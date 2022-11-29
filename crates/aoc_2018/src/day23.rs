use std::{str::FromStr, cmp::Ordering, collections::BinaryHeap};
use aoc_common::{Vec3i64, file_lines_as};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Bot {
    pos: Vec3i64,
    rad: i64,
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)$").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        Ok(Bot {
            pos: Vec3i64::new(caps[1].parse().unwrap(), caps[2].parse().unwrap(), caps[3].parse().unwrap()),
            rad: caps[4].parse().unwrap()
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Box {
    lo: Vec3i64,
    hi: Vec3i64,
}

impl Box {
    fn new<T: Into<Vec3i64>>(lo: T, hi: T) -> Box {
        Box {
            lo: lo.into(),
            hi: hi.into(),
        }
    }

    fn is_unit(&self) -> bool {
        self.lo.x == self.hi.x && self.lo.y == self.hi.y && self.lo.z == self.hi.z
    }

    fn in_range_of(&self, b: &Bot) -> bool {
        self.point_closest_to(b.pos).manhattan_from(b.pos) <= b.rad
    }

    fn point_closest_to(&self, p: Vec3i64) -> Vec3i64 {
        Vec3i64::new(
            p.x.clamp(self.lo.x, self.hi.x),
            p.y.clamp(self.lo.y, self.hi.y),
            p.z.clamp(self.lo.z, self.hi.z),
        )
    }

    fn subdivide(&self) -> Vec<Box> {
        let split_x = self.lo.x < self.hi.x;
        let split_y = self.lo.y < self.hi.y;
        let split_z = self.lo.z < self.hi.z;

        let mid_x = self.lo.x + (self.hi.x - self.lo.x) / 2;
        let mid_y = self.lo.y + (self.hi.y - self.lo.y) / 2;
        let mid_z = self.lo.z + (self.hi.z - self.lo.z) / 2;

        let mut subs: Vec<Box> = Vec::with_capacity(8);
        subs.push(Box::new((self.lo.x, self.lo.y, self.lo.z), (mid_x, mid_y, mid_z)));
        if split_x { subs.push(Box::new((mid_x + 1, self.lo.y, self.lo.z), (self.hi.x, mid_y, mid_z))); }
        if split_y { subs.push(Box::new((self.lo.x, mid_y + 1, self.lo.z), (mid_x, self.hi.y, mid_z))); }
        if split_z { subs.push(Box::new((self.lo.x, self.lo.y, mid_z + 1), (mid_x, mid_y, self.hi.z))); }
        if split_x && split_y { subs.push(Box::new((mid_x + 1, mid_y + 1, self.lo.z), (self.hi.x, self.hi.y, mid_z))); }
        if split_x && split_z { subs.push(Box::new((mid_x + 1, self.lo.y, mid_z + 1), (self.hi.x, mid_y, self.hi.z))); }
        if split_y && split_z { subs.push(Box::new((self.lo.x, mid_y + 1, mid_z + 1), (mid_x, self.hi.y, self.hi.z))); }
        if split_x && split_y && split_z { subs.push(Box::new((mid_x + 1, mid_y + 1, mid_z + 1), (self.hi.x, self.hi.y, self.hi.z))); }

        subs
    }
}

#[derive(PartialEq, Eq)]
struct Search {
    region: Box,
    in_range: usize,
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Search {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut cmp_val = self.in_range as i64 - other.in_range as i64;
        if cmp_val == 0 {
            cmp_val = other.region.point_closest_to(Vec3i64::zero()).manhattan() - self.region.point_closest_to(Vec3i64::zero()).manhattan();
        }

        match cmp_val {
            0 => Ordering::Equal,
            x if x < 0 => Ordering::Less,
            x if x > 0 => Ordering::Greater,
            _ => unreachable!()
        }
    }
}

fn input() -> Vec<Bot> {
    file_lines_as("inputs/day23.txt").collect()
}

#[test]
fn part1() {
    let bots = input();
    let strongest = bots.iter().max_by_key(|b| b.rad).unwrap();

    let answer = bots.iter().filter(|b| {
        strongest.pos.manhattan_from(b.pos) <= strongest.rad
    }).count();

    assert_eq!(answer, 270);
}

#[test]
fn part2() {
    let bots = input();

    let mut min = bots[0].pos;
    let mut max = bots[0].pos;

    for b in bots.iter().skip(1) {
        if b.pos.x < min.x {
            min.x = b.pos.x;
        }
        if b.pos.x > max.x {
            max.x = b.pos.x;
        }
        if b.pos.y < min.y {
            min.y = b.pos.y;
        }
        if b.pos.y > max.y {
            max.y = b.pos.y;
        }
        if b.pos.z < min.z {
            min.z = b.pos.z;
        }
        if b.pos.z > max.z {
            max.z = b.pos.z;
        }
    }

    let start = Box {
        lo: min,
        hi: max,
    };

    let mut to_search: BinaryHeap<Search> = BinaryHeap::new();
    to_search.push(Search {
        region: start,
        in_range: bots.len(),
    });

    let mut answer = 0;
    while let Some(current) = to_search.pop() {
        if current.region.is_unit() {
            answer = current.region.lo.manhattan();
            break;
        }

        for div in current.region.subdivide() {
            let in_range = bots.iter().filter(|b| div.in_range_of(b)).count();

            to_search.push(Search {
                region: div,
                in_range
            });
        }
    }

    assert_eq!(answer, 106323091);
}