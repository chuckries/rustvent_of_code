use std::collections::{HashSet};

use aoc_common::{file_lines, IteratorExt, Vec2us, PriorityQueue};


struct Node {
    idx: Vec2us,
    size: usize,
    used: usize,
    available: usize,
}

fn input() -> Vec<Node> {
    file_lines("inputs/day22.txt").skip(2).map(|l| {
        let split = l.split_whitespace().to_vec();

        let size: usize = split[1].trim_end_matches('T').parse().unwrap();
        let used: usize = split[2].trim_end_matches('T').parse().unwrap();
        let available: usize = split[3].trim_end_matches('T').parse().unwrap();

        let split = split[0].split('-').to_vec();

        let x: usize = split[1].trim_start_matches('x').parse().unwrap();
        let y: usize = split[2].trim_start_matches('y').parse().unwrap();

        Node { idx: (x, y).into(), size, used, available }
    }).to_vec()
}

#[test]
fn part1() {
    let nodes = input();

    #[inline]
    fn test_pair(a: &Node, b: &Node) -> bool {
        a.used != 0 && b.available >= a.used
    }

    let mut count = 0;
    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            if test_pair(&nodes[i], &nodes[j]) {
                count += 1;
            }
            if test_pair(&nodes[j], &nodes[i]) {
                count += 1;
            }
        }
    }

    assert_eq!(count, 860);
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Search {
    data_idx: Vec2us,
    empty_idx: Vec2us,
    target_idx: Vec2us,
}

struct Graph {
    graph: Vec<Vec<bool>>,
    goal_idx: Vec2us,
    empty_idx: Vec2us,
    bounds: Vec2us,
}

impl Graph {
    fn new() -> Graph {
        let nodes = input();
        let bounds = Vec2us::bounds(nodes.iter().map(|n| n.idx));
        let goal_idx = Vec2us::new(bounds.x - 1, 0);
        let empty = nodes.iter().find(|n| n.used == 0).unwrap();
        let empty_idx = empty.idx;
        let max_available = empty.size;
    
        let mut graph = vec![vec![false; bounds.x]; bounds.y];
        for n in nodes {
            if n.used <= max_available {
                graph[n.idx.y][n.idx.x] = true
            }
        }

        Graph {
            graph,
            goal_idx,
            empty_idx,
            bounds,
        }
    }

    fn min_steps(&self) -> usize {
        let mut queue: PriorityQueue<(Search, usize), usize> = PriorityQueue::new();
        let mut visited: HashSet<Search> = HashSet::new();

        for adj in self.adjacent(self.goal_idx) {
            let s = Search { data_idx: self.goal_idx, empty_idx: self.empty_idx, target_idx: adj };
            queue.enqueue((s, 0), Self::weighted_distance(&s, 0));
        }

        while let Some((current, dist)) = queue.dequeue() {
            if current.data_idx.is_zero() {
                return dist;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            if current.empty_idx == current.target_idx {
                for adj in self.adjacent(current.empty_idx).filter(|a| *a != current.data_idx) {
                    let next = Search {
                        data_idx: current.empty_idx,
                        empty_idx: current.data_idx,
                        target_idx: adj,
                    };
                    if !visited.contains(&next) {
                        queue.enqueue((next, dist + 1), Self::weighted_distance(&next, dist + 1));
                    }
                }
            } else {
                for adj in self.adjacent(current.empty_idx).filter(|a| *a != current.data_idx) {
                    let mut next = current;
                    next.empty_idx = adj;
                    if !visited.contains(&next) {
                        queue.enqueue((next, dist + 1), Self::weighted_distance(&next, dist + 1));
                    }
                }
            }
        }

        panic!();
    }

    fn adjacent<'a>(&'a self, idx: Vec2us) -> impl 'a + Iterator<Item = Vec2us> {
        idx.adjacent_bounded(&self.bounds).filter(|i| self.graph[i.y][i.x])
    }

    fn weighted_distance(s: &Search, dist: usize) -> usize {
        dist + s.data_idx.manhattan() + s.target_idx.manhattan() + s.empty_idx.manhattan_from(s.target_idx)
    }
}

#[test]
fn part2() {
    let answer = Graph::new().min_steps();
    assert_eq!(answer, 200);
}