use aoc_common::{Vec2us, PriorityQueue, VirtualGrid};

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
    cache: VirtualGrid<usize>
}

impl Regions {
    fn new() -> Regions {
        Regions {
            cache: VirtualGrid::new()
        }
    }

    fn risk(&mut self, p: Vec2us) -> usize {
        self.erosion(p) % 3
    }

    fn erosion(&mut self, p: Vec2us) -> usize {
        (self.index(p) + DEPTH) % 20183
    }

    fn index(&mut self, p: Vec2us) -> usize {
        if let Some(idx) = self.cache[p] {
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

            self.cache[p] = Some(idx);
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

fn search() -> usize {
    let mut to_visit: PriorityQueue<Search, usize> = PriorityQueue::new();
    let mut distances = [
        VirtualGrid::<usize>::new(),
        VirtualGrid::<usize>::new(),
        VirtualGrid::<usize>::new(),
    ];
    let mut regions = Regions::new();

    let start = Search {
        pos: (0, 0).into(),
        tool: TORCH,
        risk: regions.risk((0, 0).into()),
        time: 0,
    };
    distances[start.tool].insert(Vec2us::zero(), 0);
    to_visit.enqueue(start, TARGET.x + TARGET.y);

    while let Some(current) = to_visit.dequeue() {
        if current.pos == TARGET && current.tool == TORCH {
            return current.time;
        }

        let other_tool = OTHER_VALID_TOOL[current.risk][current.tool];
        let entry = distances[other_tool].entry(current.pos).or_insert(usize::MAX);
        if current.time + 7 < *entry {
            *entry = current.time + 7;
            to_visit.enqueue(Search {
                pos: current.pos,
                tool: other_tool,
                risk: current.risk,
                time: current.time + 7,
            }, current.time + 7);
        }

        for adj in current.pos.adjacent_non_negative().filter_map(|adj| {
            let risk = regions.risk(adj);
            if VALID_TOOLS[risk][current.tool] {
                Some((adj, risk))
            } else {
                None
            }
        }) {
            let entry = distances[current.tool].entry(adj.0).or_insert(usize::MAX);
            if current.time + 1 < *entry {
                *entry = current.time + 1;
                to_visit.enqueue(Search {
                    pos: adj.0,
                    risk: adj.1,
                    tool: current.tool,
                    time: current.time + 1,
                }, current.time + 1 + adj.0.manhattan_from(TARGET));
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
    let answer = search();
    assert_eq!(answer, 999);
}