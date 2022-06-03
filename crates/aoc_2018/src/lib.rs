#![cfg(test)]

mod day18 {
    use core::borrow;
    use std::{collections::{HashMap, BinaryHeap, HashSet}, cell::RefCell};

    use aoc_common::{Vec2i32, Vec2us};

    const ZERO: Vec2i32 = Vec2i32::new(0, 0);
    const DEPTH: usize = 10689;
    const TARGET: Vec2i32 = Vec2i32::new(11, 722);
    const ROCKY: usize = 0;
    const WET: usize = 1;
    const NARROW: usize = 2;
    const NONE: usize = 0;
    const TORCH: usize = 1;
    const CLIMBING: usize = 2;

    const VALID_TOOLS: [[bool; 3]; 3] = [
        [ false, true, true ],
        [ true, false, true ],
        [ true, true, false ]
    ];

    const OTHER_VALID_TOOL: [[usize; 3]; 3] = [
        [ usize::MAX, 2, 1 ],
        [ 2, usize::MAX, 0 ],
        [ 1, 0, usize::MAX ],
    ];

    struct Regions {
        //cache: RefCell<HashMap<Vec2i32, usize>>
        cache: Vec<Vec<Option<usize>>>
    }

    impl Regions {
        fn new() -> Regions {
            //Regions { cache: RefCell::new(HashMap::new()) }
            Regions {
                cache: vec![vec![None; (TARGET.x * 2) as usize]; (TARGET.y * 2) as usize],
            }
        }

        fn risk(&mut self, p: Vec2i32) -> usize {
            self.erosion(p) % 3
        }

        fn erosion(&mut self, p: Vec2i32) -> usize {
            (self.index(p) + DEPTH) % 20183
        }

        fn index(&mut self, p: Vec2i32) -> usize {
            if p.y as usize >= self.cache.len() || p.x as usize >= self.cache[0].len() {
                let y_size = (p.y as usize).max(self.cache.len() * 2);
                let x_size = (p.x as usize).max(self.cache[0].len() * 2);

                for row in self.cache.iter_mut() {
                    row.resize(x_size, None);
                }
                self.cache.resize(y_size, vec![None; x_size]);
            }

            let existing = &mut self.cache[p.y as usize][p.x as usize];
            if let Some(idx) = existing {
                *idx
            } else {
                drop(existing);

                let idx = if p == TARGET || p == ZERO {
                    0
                } else if p.x == 0 {
                    (p.y * 48271) as usize
                } else if p.y == 0 {
                    (p.x * 16807) as usize
                } else {
                    self.erosion(Vec2i32::new(p.x - 1, p.y)) * self.erosion(Vec2i32::new(p.x, p.y - 1))
                };

                //*existing = Some(idx);
                self.cache[p.y as usize][p.x as usize] = Some(idx);
                idx
            }

            // let borrow = self.cache.borrow();
            // if let Some(idx) = borrow.get(&p) {
            //     *idx
            // } else {
            //     drop(borrow);

            //     let idx = if p == TARGET || p == ZERO {
            //         0
            //     } else if p.x == 0 {
            //         (p.y * 48271) as usize
            //     } else if p.y == 0 {
            //         (p.x * 16807) as usize
            //     } else {
            //         self.erosion(Vec2i32::new(p.x - 1, p.y)) * self.erosion(Vec2i32::new(p.x, p.y - 1))
            //     };

            //     self.cache.borrow_mut().insert(p, idx);
            //     idx
            // }
        }
    }

    struct Search {
        pos: Vec2i32,
        tool: usize,
        risk: usize,
        time: i32,
        heuristic: i32,
    }

    impl PartialEq for Search {
        fn eq(&self, other: &Self) -> bool {
            self.heuristic.eq(&other.heuristic)
        }
    }

    impl Eq for Search { }

    impl PartialOrd for Search {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.heuristic.partial_cmp(&self.heuristic)
        }
    }

    impl Ord for Search {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.heuristic.cmp(&self.heuristic)
        }
    }

    fn search() -> i32 {
        let mut to_visit: BinaryHeap<Search> = BinaryHeap::new();
        let mut distances: HashMap<(Vec2i32, usize), i32> = HashMap::new();
        let mut regions = Regions::new();

        let start = Search {
            pos: (0, 0).into(),
            tool: TORCH,
            risk: regions.risk((0i32, 0i32).into()),
            time: 0,
            heuristic: TARGET.x + TARGET.y
        };
        distances.insert((start.pos, start.tool), 0);
        to_visit.push(start);

        let mut answer= 0;
        while let Some(current) = to_visit.pop() {
            if current.pos == TARGET && current.tool == TORCH {
                return current.time;
            }

            let current_min = *distances.get(&(current.pos, current.tool)).unwrap();
            if current_min != current.time {
                continue;
            }

            let other_tool = OTHER_VALID_TOOL[current.risk][current.tool];

            let entry = distances.entry((current.pos, other_tool)).or_insert(i32::max_value());
            if current.time + 7 < *entry {
                *entry = current.time + 7;
                to_visit.push(Search {
                    pos: current.pos,
                    tool: other_tool,
                    risk: current.risk,
                    time: current.time + 7,
                    heuristic: current.heuristic + 7,
                });
            }

            for adj in current.pos.adjacent().filter_map(|adj| {
                if adj.x < 0|| adj.y < 0 {
                    return None;
                }

                let risk = regions.risk(adj);
                if VALID_TOOLS[risk][current.tool] {
                    Some((adj, risk))
                } else {
                    None
                }
            }) {
                let entry = distances.entry((adj.0, current.tool)).or_insert(i32::max_value());
                if current.time + 1 < *entry {
                    *entry = current.time + 1;
                    to_visit.push(Search {
                        pos: adj.0,
                        risk: adj.1,
                        tool: current.tool,
                        time: current.time + 1,
                        heuristic: current.time + 1 + dist(adj.0, TARGET),
                    });
                }
            }
        }

        panic!()
    }

    #[test]
    fn part1() {
        let bounds = Vec2i32::new(TARGET.x + 1, TARGET.y + 1);
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

    fn dist(a: Vec2i32, b: Vec2i32) -> i32 {
        i32::abs(a.x - b.x) + i32::abs(a.y - b.y)
    }
}