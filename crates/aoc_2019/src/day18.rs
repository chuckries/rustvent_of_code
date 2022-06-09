use std::collections::{VecDeque, HashMap, HashSet};

use aoc_common::{file_lines, ToVec, Vec2us};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Entry,
    Wall,
    Space,
    Key(usize),
    Door(usize),
}

#[derive(Copy, Clone, Default, Debug)]
struct Adj {
    dist: usize,
    keys_required: u32
}

type Map = Vec<Vec<Cell>>;
type Graph = Vec<Vec<Adj>>;

struct Maze {
    graph: Graph,
    solved_mask: u32,
}

impl Maze {
    fn new(map: &Map, bounds: (Vec2us, Vec2us)) -> Self {
        let mut origin = Vec2us::zero();
        let mut keys: Vec<Vec2us> = Vec::new();

        for j in bounds.0.y..bounds.1.y {
            for i in bounds.0.x..bounds.1.x {
                match map[j][i] {
                    Cell::Entry => origin = (i, j).into(),
                    Cell::Key(key) => {
                        if key >= keys.len() {
                            keys.resize(key + 1, Vec2us::default());
                        }
                        keys[key] = (i, j).into();
                    }
                    _ => ()
                }
            }
        }

        fn explore_paths(pos: Vec2us, map: &Map) -> Vec<Adj> {
            if pos == Vec2us::zero() {
                return Vec::new();
            }

            let mut visited: HashSet<Vec2us> = HashSet::new();
            let mut paths: Vec<Adj> = Vec::new();
            let mut to_visit: VecDeque<(Vec2us, usize, u32)> = VecDeque::new();
            to_visit.push_back((pos, 0, 0));

            while let Some((current, dist, mut required_keys)) = to_visit.pop_front() {
                if !visited.insert(current) {
                    continue;
                }

                match map[current.y][current.x] {
                    Cell::Door(door) => {
                        required_keys |= 1 << door;
                    }
                    Cell::Key(key) => {
                        if key >= paths.len() {
                            paths.resize(key + 1, Adj::default());
                        }
                        paths[key] = Adj { dist, keys_required: required_keys };
                    }
                    _ => ()
                }

                for adj in current.adjacent() {
                    if map[adj.y][adj.x] == Cell::Wall {
                        continue;
                    }
                    to_visit.push_back((adj, dist + 1, required_keys));
                }
            }
            paths
        }

        let graph = keys
            .iter()
            .chain(std::iter::once(&origin))
            .map(|k| { explore_paths(*k, &map) }).to_vec();

        let mut solved_mask = 0;
        for (idx, k) in keys.iter().enumerate() {
            if *k != Vec2us::zero() {
                solved_mask |= 1 << idx;
            }
        }

        Self { 
            graph,
            solved_mask,
        }
    }

    fn shortest_path(&self) -> usize {
        let mut cache: HashMap<(usize, u32), usize> = HashMap::new();
        self.recurse(self.graph.len() - 1, 0, &mut cache)
    }

    fn recurse(&self, idx: usize, keys: u32, cache: &mut HashMap<(usize, u32), usize>) -> usize {
        if keys == self.solved_mask {
            return 0;
        }

        if let Some(cached) = cache.get(&(idx, keys)) {
            return *cached;
        }

        let mut min = usize::MAX;
        for (adj_idx, adj) in self.graph[idx].iter().enumerate() {
            if adj.dist == 0 {
                continue;
            }

            if (keys & (1 << adj_idx)) != 0 {
                continue;
            }

            if (keys & adj.keys_required) != adj.keys_required {
                continue;
            }

            let dist = adj.dist + self.recurse(adj_idx, keys | (1 << adj_idx), cache);
            if dist < min {
                min = dist;
            }
        }
        cache.insert((idx, keys), min);
        min
    }



    fn _print_graph(&self) {
        println!("{:?}", self.graph);
    }
}

struct MultiMaze {
    mazes: Vec<Maze>,
    solved_mask: u32,
}

type MultiMazeCacheKey = (u32, Vec<usize>);
type MultiMazeCache = HashMap<MultiMazeCacheKey, usize>;

impl MultiMaze {
    fn new(mazes: Vec::<Maze>) -> Self {
        let solved_mask = mazes.iter().map(|m| m.solved_mask).reduce(|accum, next| { accum | next }).unwrap();
        Self {
            mazes,
            solved_mask
        }
    }

    fn shortest_path(&self) -> usize {
        let mut cache = MultiMazeCache::new();
        let key = (0, self.mazes.iter().map(|m| m.graph.len() - 1).to_vec());
        self.recurse(key, &mut cache)
    }

    fn recurse(&self, key: MultiMazeCacheKey, cache: &mut MultiMazeCache) -> usize {
        if key.0 == self.solved_mask {
            return 0;
        }

        if let Some(cached) = cache.get(&key) {
            return *cached;
        }

        let mut min = usize::MAX;
        let keys = key.0;
        for (idx, (maze, pos)) in self.mazes.iter().zip(key.1.iter()).enumerate() {
            for (adj_idx, adj) in maze.graph[*pos].iter().enumerate() {
                if adj.dist == 0 {
                    continue;
                }

                if (keys & (1 << adj_idx)) != 0 {
                    continue;
                }

                if (keys & adj.keys_required) != adj.keys_required {
                    continue;
                }

                let mut next_key = key.clone();
                next_key.0 = keys | (1 << adj_idx);
                next_key.1[idx] = adj_idx;

                let delta = self.recurse(next_key, cache);
                if delta == usize::MAX {
                    continue;
                }
                let dist = adj.dist + delta;
                if dist < min {
                    min = dist;
                }
            }
        }
        cache.insert(key.clone(), min);
        min
    }
}

fn parse_map() -> Map {
    let map = file_lines("inputs/day18.txt").map(|l| l.bytes().map(|c| {
        match c {
            b'@' => Cell::Entry,
            b'#' => Cell::Wall,
            b'.' => Cell::Space,
            key @ b'a'..=b'z' => Cell::Key((key - b'a') as usize),
            door @ b'A'..=b'Z' => Cell::Door((door - b'A') as usize),
            _ => panic!()
        }}).to_vec()).to_vec();
    map
}

fn _print_map(map: &Map, bounds: (Vec2us, Vec2us)) {
    let mut string = String::new();

    for j in bounds.0.y..bounds.1.y {
        for i in bounds.0.x..bounds.1.x {
            let c = match map[j][i] {
                Cell::Wall => '#',
                Cell::Entry => '@',
                Cell::Space => '.',
                Cell::Key(key) => (key as u8 + b'a') as char,
                Cell::Door(door) => (door as u8 + b'A') as char
            };
            string.push(c);
        }
        string.push('\n');
    }
    println!("{}", string);
}

#[test]
fn part1() {
    let map = parse_map();
    let bounds = (Vec2us::zero(), Vec2us::new(map[0].len(), map.len()));
    let maze = Maze::new(&map, bounds);
    let answer = maze.shortest_path();
    assert_eq!(answer, 3764);
}

#[test]
fn part2() {
    let mut map = parse_map();
    let mut origin = Vec2us::default();
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if let Cell::Entry = map[j][i] {
                origin = (i, j).into();
                break;
            }
        }
    }

    map[origin.y    ][origin.x    ] = Cell::Wall;
    map[origin.y + 1][origin.x    ] = Cell::Wall;
    map[origin.y - 1][origin.x    ] = Cell::Wall;
    map[origin.y    ][origin.x + 1] = Cell::Wall;
    map[origin.y    ][origin.x - 1] = Cell::Wall;

    map[origin.y + 1][origin.x + 1] = Cell::Entry;
    map[origin.y - 1][origin.x + 1] = Cell::Entry;
    map[origin.y + 1][origin.x - 1] = Cell::Entry;
    map[origin.y - 1][origin.x - 1] = Cell::Entry;

    let bounds: [(Vec2us, Vec2us); 4] = [
        ((0, 0).into(), (origin.x + 1, origin.y + 1).into()),
        ((0, origin.y).into(), (origin.x + 1, map.len()).into()),
        ((origin.x, 0).into(), (map[0].len(), origin.y + 1).into()),
        (origin, (map[0].len(), map.len()).into())
    ];

    let mazes = bounds.into_iter().map(|b| Maze::new(&map, b)).to_vec();
    let multi = MultiMaze::new(mazes);
    let answer = multi.shortest_path();
    assert_eq!(answer, 1738);

}