use std::collections::{VecDeque, HashMap, HashSet};

use aoc_common::{Grid, IteratorExt, Vec2us};

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

type Map = Grid<Cell>;
type Graph = Vec<Vec<Adj>>;

struct Maze {
    graph: Graph,
    solved_mask: u32,
    entry_count: usize,
}

impl Maze {
    fn new(map: &Map) -> Self {
        let mut entrances: Vec<Vec2us> = Vec::new();
        let mut keys: Vec<Vec2us> = Vec::new();

        for (p, b) in map.enumerate() {
            match b {
                Cell::Entry => entrances.push(p),
                Cell::Key(key) => {
                    let key = *key;
                    if key >= keys.len() {
                        keys.resize(key + 1, Vec2us::default());
                    }
                    keys[key] = p;
                }
                _ => (),
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

                match map[current]{
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
                    if map[adj] == Cell::Wall {
                        continue;
                    }
                    to_visit.push_back((adj, dist + 1, required_keys));
                }
            }
            paths
        }

        let graph = keys
            .iter()
            .chain(entrances.iter())
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
            entry_count: entrances.len(),
        }
    }

    fn shortest_path(&self) -> usize {
        let mut cache: HashMap<(u32, Vec<usize>), usize> = HashMap::new();
        let positions = (0..self.entry_count).map(|i| self.graph.len() - i - 1).to_vec();
        self.recurse(positions, 0, &mut cache)
    }

    fn recurse(&self, positions: Vec<usize>, keys: u32, cache: &mut HashMap<(u32, Vec<usize>), usize>) -> usize {
        if keys == self.solved_mask {
            return 0;
        }

        let dict_key = (keys, positions);

        if let Some(cached) = cache.get(&dict_key) {
            return *cached;
        }

        let mut min = usize::MAX;
        for (pos_idx, idx) in dict_key.1.iter().enumerate() {
            for (adj_idx, adj) in self.graph[*idx].iter().enumerate() {
                if adj.dist == 0 {
                    continue;
                }

                if (keys & (1 << adj_idx)) != 0 {
                    continue;
                }

                if (keys & adj.keys_required) != adj.keys_required {
                    continue;
                }

                let mut next_positions = dict_key.1.clone();
                next_positions[pos_idx] = adj_idx;

                let delta = self.recurse(next_positions, keys | (1 << adj_idx), cache);
                if delta == usize::MAX {
                    continue;
                }

                let dist = adj.dist + delta;
                if dist < min {
                    min = dist;
                }
            }
        }
        cache.insert(dict_key, min);
        min
    }

    fn _print_graph(&self) {
        println!("{:?}", self.graph);
    }
}

fn parse_map() -> Map {
    Map::file_as_grid("inputs/day18.txt", &mut |b, _| {
        match b {
            b'@' => Cell::Entry,
            b'#' => Cell::Wall,
            b'.' => Cell::Space,
            key @ b'a'..=b'z' => Cell::Key((key - b'a') as usize),
            door @ b'A'..=b'Z' => Cell::Door((door - b'A') as usize),
            _ => panic!()
        }
    })
}

fn _print_map(map: &Map) {
    let mut string = String::new();

    for row in map.rows() {
        for cell in row.iter() {
            let c = match cell {
                Cell::Wall => '#',
                Cell::Entry => '@',
                Cell::Space => '.',
                Cell::Key(key) => (*key as u8 + b'a') as char,
                Cell::Door(door) => (*door as u8 + b'A') as char
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
    let maze = Maze::new(&map);
    let answer = maze.shortest_path();
    assert_eq!(answer, 3764);
}

#[test]
fn part2() {
    let mut map = parse_map();

    let (origin, _) = map.enumerate().filter(|(_, b)| { matches!(b, Cell::Entry) }).next().unwrap();

    map[origin] = Cell::Wall;
    for p in origin.adjacent() {
        map[p] = Cell::Wall;
    }

    for p in origin.diags() {
        map[p] = Cell::Entry;
    }

    let maze = Maze::new(&map);
    let answer = maze.shortest_path();
    assert_eq!(answer, 1738);
}