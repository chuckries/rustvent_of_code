#![cfg(test)]

mod day18 {
    use std::{collections::{BinaryHeap}};

    use aoc_common::{Vec2us, SearchNode};

    const ZERO: Vec2us = Vec2us::new(0, 0);
    const DEPTH: usize = 10689;
    const TARGET: Vec2us = Vec2us::new(11, 722);
    const NONE: usize = 0;
    const TORCH: usize = 1;
    const CLIMBING: usize = 2;
    const INVALID: usize = usize::MAX;

    const VALID_TOOLS: [[bool; 3]; 3] = [
        [ false, true, true ],
        [ true, false, true ],
        [ true, true, false ]
    ];

    const OTHER_VALID_TOOL: [[usize; 3]; 3] = [
        [ INVALID, CLIMBING, TORCH ],
        [ CLIMBING, INVALID, NONE ],
        [ TORCH, NONE, INVALID ],
    ];

    struct Regions {
        cache: Vec<Vec<Option<usize>>>
    }

    impl Regions {
        fn new() -> Regions {
            Regions {
                cache: vec![vec![None; TARGET.x * 2]; TARGET.y * 2],
            }
        }

        fn risk(&mut self, p: Vec2us) -> usize {
            self.erosion(p) % 3
        }

        fn erosion(&mut self, p: Vec2us) -> usize {
            (self.index(p) + DEPTH) % 20183
        }

        fn index(&mut self, p: Vec2us) -> usize {
            if p.x >= self.cache[0].len() {
                let size = p.x.max(self.cache[0].len() * 2);
                for row in self.cache.iter_mut() {
                    row.resize(size, None);
                }
            }

            if p.y >= self.cache.len() {
                let size = p.y.max(self.cache.len() * 2);
                self.cache.resize(size, vec![None; self.cache[0].len()]);
            }

            let existing = self.cache[p.y][p.x];
            if let Some(idx) = existing {
                idx
            } else {
                let idx = if p == TARGET || p == ZERO {
                    0
                } else if p.x == 0 {
                    p.y * 48271
                } else if p.y == 0 {
                    p.x * 16807
                } else {
                    self.erosion(Vec2us::new(p.x - 1, p.y)) * self.erosion(Vec2us::new(p.x, p.y - 1))
                };

                self.cache[p.y][p.x] = Some(idx);
                idx
            }
        }
    }

    struct Search {
        pos: Vec2us,
        tool: usize,
        risk: usize,
        time: usize,
    }

    #[inline]
    fn get_distance(p: Vec2us, tool: usize, distances: &mut [Vec<Vec<usize>>]) ->  &mut usize {
        if p.x >= distances[0][0].len() {
            let size = p.x.max(distances[0][0].len() * 2);
            for t in distances.iter_mut() {
                for row in t.iter_mut() {
                    row.resize(size, usize::MAX);
                }
            }
        }

        if p.y >= distances[0].len() {
            let size = p.y.max(distances[0].len() * 2);
            let x = distances[0][0].len();
            for y in distances.iter_mut() {
                y.resize(size, vec![usize::MAX; x]);
            }
        }

        &mut distances[tool][p.y][p.x]
    }

    fn search() -> usize {
        let mut to_visit: BinaryHeap<SearchNode<usize, Search>> = BinaryHeap::new();
        let mut distances = [
            vec![vec![usize::MAX; TARGET.x * 2]; TARGET.y * 2],
            vec![vec![usize::MAX; TARGET.x * 2]; TARGET.y * 2],
            vec![vec![usize::MAX; TARGET.x * 2]; TARGET.y * 2],
        ];
        let mut regions = Regions::new();

        let start = Search {
            pos: (0, 0).into(),
            tool: TORCH,
            risk: regions.risk((0, 0).into()),
            time: 0,
        };
        distances[start.tool][0][0] = 0;
        to_visit.push(SearchNode { dist: TARGET.x + TARGET.y, data: start });

        while let Some(current) = to_visit.pop() {
            if current.pos == TARGET && current.tool == TORCH {
                return current.time;
            }

            let other_tool = OTHER_VALID_TOOL[current.risk][current.tool];
            let entry = get_distance(current.pos, other_tool, &mut distances);
            if current.time + 7 < *entry {
                *entry = current.time + 7;
                to_visit.push(SearchNode {
                    dist: current.dist + 7,
                    data: Search {
                        pos: current.pos,
                        tool: other_tool,
                        risk: current.risk,
                        time: current.time + 7,
                    }
                });
            }

            for adj in current.pos.adjacent_non_negative().filter_map(|adj| {
                let risk = regions.risk(adj);
                if VALID_TOOLS[risk][current.tool] {
                    Some((adj, risk))
                } else {
                    None
                }
            }) {
                let entry = get_distance(adj.0, current.tool, &mut distances);
                if current.time + 1 < *entry {
                    *entry = current.time + 1;
                    to_visit.push(SearchNode {
                        dist: current.time + 1 + dist(adj.0, TARGET),
                        data: Search {
                            pos: adj.0,
                            risk: adj.1,
                            tool: current.tool,
                            time: current.time + 1,
                        }
                    });
                }
            }
        }

        panic!()
    }

    #[test]
    fn part1() {
        let bounds = Vec2us::new(TARGET.x + 1, TARGET.y + 1);
        let mut regions = Regions::new();
        let answer = bounds.iter().map(|p| regions.risk(p)).sum::<usize>();

        assert_eq!(answer, 8575);
    }

    #[test]
    fn part2() {
        let now = std::time::Instant::now();
        let answer = search();
        let elapsed = std::time::Instant::now().duration_since(now);
        println!("{:?}", elapsed.as_secs_f64());
        assert_eq!(answer, 999);
    }

    fn dist(a: Vec2us, b: Vec2us) -> usize {
        usize::abs_diff(a.x, b.x) + usize::abs_diff(a.y, b.y)
    }
}