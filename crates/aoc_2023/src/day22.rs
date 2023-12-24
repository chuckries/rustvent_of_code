use std::collections::{HashMap, HashSet};

use aoc_common::{Vec3i32, file_lines, IteratorExt, Vec2i32, PriorityQueue};

#[derive(Clone, Copy)]
struct Piece {
    p0: Vec3i32,
    p1: Vec3i32,
}

impl Piece {
    fn new(p0: Vec3i32, p1: Vec3i32) -> Self {
        if p0.x > p1.x || p0.y > p1.y || p0.z > p1.z {
            panic!("bad shape");
        }

        Self { p0, p1 }
    }

    fn height(&self) -> i32 {
        self.p1.z - self.p0.z + 1
    }
}

fn input() -> Vec<Piece> {
    file_lines("inputs/day22.txt").map(|l| {
        let mut split = l.split('~');

        fn parse_vec3(s: &str) -> Vec3i32 {
            let mut split = s.split(',');
            Vec3i32::new(split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap(), split.next().unwrap().parse().unwrap())
        }

        let p0 = parse_vec3(split.next().unwrap());
        let p1 = parse_vec3(split.next().unwrap());

        Piece::new(p0, p1)
    }).sorted_by_key(|p| p.p0.z).to_vec()
}

#[derive(Default, Clone)]
struct ResolvedPiece {
    z: i32,
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}

#[test]
fn part1() {
    let pieces = input();
    let mut tower: HashMap<Vec2i32, (usize, i32)> = HashMap::new();
    let mut graph = vec![ResolvedPiece::default(); pieces.len()];

    for (idx, piece) in pieces.iter().enumerate() {
        let mut footprint = Vec::new();
        for i in piece.p0.x ..= piece.p1.x {
            for j in piece.p0.y ..= piece.p1.y {
                footprint.push(tower.get(&(i, j).into()));
            }
        }

        let max = footprint.iter().map(|c| {
            c.unwrap_or(&(0, 0)).1
        }).max().unwrap();

        if max > 0 {
            for existing in footprint.iter() {
                if let Some(existing) = existing {
                    if existing.1 == max {
                        graph[existing.0].supports.insert(idx);
                        graph[idx].supported_by.insert(existing.0);
                    }
                }
            }
        }

        graph[idx].z = max + 1;

        for i in piece.p0.x ..= piece.p1.x {
            for j in piece.p0.y ..= piece.p1.y {
                *tower.entry((i, j).into()).or_default() = (idx, max + piece.height());
            }
        }
    }

    let answer = graph.iter().enumerate().filter(|(idx, p)| {
        p.supports.len() == 0 || p.supports.iter().all(|s| graph[*s].supported_by.len() > 1)
    }).count();

    assert_eq!(443, answer);
}

#[test]
fn part2() {
    let pieces = input();
    let mut tower: HashMap<Vec2i32, (usize, i32)> = HashMap::new();
    let mut graph = vec![ResolvedPiece::default(); pieces.len()];

    for (idx, piece) in pieces.iter().enumerate() {
        let mut footprint = Vec::new();
        for i in piece.p0.x ..= piece.p1.x {
            for j in piece.p0.y ..= piece.p1.y {
                footprint.push(tower.get(&(i, j).into()));
            }
        }

        let max = footprint.iter().map(|c| {
            c.unwrap_or(&(0, 0)).1
        }).max().unwrap();

        if max > 0 {
            for existing in footprint.iter() {
                if let Some(existing) = existing {
                    if existing.1 == max {
                        graph[existing.0].supports.insert(idx);
                        graph[idx].supported_by.insert(existing.0);
                    }
                }
            }
        }

        graph[idx].z = max + 1;

        for i in piece.p0.x ..= piece.p1.x {
            for j in piece.p0.y ..= piece.p1.y {
                *tower.entry((i, j).into()).or_default() = (idx, max + piece.height());
            }
        }
    }

    fn search(idx: usize, graph: &[ResolvedPiece]) -> usize {
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(idx);

        let mut queue: PriorityQueue<usize, i32> = PriorityQueue::new();
        for p in graph[idx].supports.iter().copied() {
            queue.enqueue(p, graph[p].z);
        }

        while let Some(current) = queue.dequeue() {
            if graph[current].supported_by.iter().all(|p| visited.contains(p)) {
                visited.insert(current);
                for adj in graph[current].supports.iter().copied() {
                    queue.enqueue(adj, graph[adj].z);
                }
            }
        }

        visited.len() - 1
    }

    let answer: usize = (0..graph.len()).map(|idx| search(idx, &graph)).sum();
    assert_eq!(69915, answer);
}