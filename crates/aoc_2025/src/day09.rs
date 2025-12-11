use std::{cmp::Reverse, collections::BTreeMap, ops::Bound, usize};

use aoc_common::{PriorityQueueBuilder, Vec2, Vec2i64, file_lines};

use Dir::*;

fn input() -> Vec<Vec2i64> {
    file_lines("inputs/day09.txt").map(|l| Vec2::from_split_comma(l)).collect()
}

#[test]
fn part1() {
    let input = input();

    let mut max = 0;
    for i in 0 .. input.len() - 1 {
        for j in i + 1 .. input.len() {
            let diff = (input[i] - input[j]).abs() + Vec2::one();
            let area = diff.x * diff.y;
            if area > max {
                max = area;
            }
        }
    }

    assert_eq!(4782268188, max);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn from_diff(p: Vec2i64) -> Self {
        match p.signum().into() {
            ( 0,  1) => South,
            ( 0, -1) => North,
            ( 1,  0) => East,
            (-1,  0) => West,
            _ => panic!(),
        }
    }
}

fn normalize(p0: Vec2i64, p1: Vec2i64) -> (Vec2i64, Vec2i64) {
    let (mut x0, mut x1) = (p0.x, p1.x);
    if x0 > x1 {
        (x0, x1) = (x1, x0);
    }

    let (mut y0, mut y1) = (p0.y, p1.y);
    if y0 > y1 {
        (y0, y1) = (y1, y0);
    }

    ((x0, y0).into(), (x1, y1).into())
}

struct Segment {
    p0: Vec2i64,
    p1: Vec2i64,
    dir: Dir,
}

impl Segment {
    fn new(p0: Vec2i64, p1: Vec2i64, dir: Dir) -> Self {
        let (p0, p1) = normalize(p0, p1);

        Self {
            p0,
            p1,
            dir,
        }
    }

    fn range(&self) -> (Vec2i64, Vec2i64) {
        (self.p0, self.p1)
    }
}

#[test]
fn part2() {
    let input = input();

    // find the left most vertical line segment so that we can assume this is a "Left" wall
    let mut previous = input[input.len() - 1];
    let mut min_x = i64::MAX;
    let mut dir: i64 = 0;
    let mut start_idx = usize::MAX;
    for i in 0..input.len() {
        let current = input[i];

        if current.x == previous.x {
            // vertical segment
            if current.x < min_x {
                min_x = current.x;
                dir = i64::signum(current.y - previous.y);
                start_idx = i;
            }
        }

        previous = current;
    }

    let mut hori_map: BTreeMap<i64, Vec<Segment>> = BTreeMap::new();
    let mut vert_map: BTreeMap<i64, Vec<Segment>> = BTreeMap::new();

    let mut previous_dir = if dir < 0 { North } else { South };
    let mut previous_wall = West;
    let mut previous_point = input[start_idx];
    for p in input[start_idx + 1..].iter().chain(input[..=start_idx].iter()) {
        let p0 = previous_point;
        let p1 = *p;

        let dir = Dir::from_diff(p1 - p0);

        let wall = match (previous_wall, previous_dir, dir) {
            (West, North, West) => South,
            (West, North, East) => North,
            (West, South, West) => North,
            (West, South, East) => South,
            (East, North, West) => North,
            (East, North, East) => South,
            (East, South, West) => South,
            (East, South, East) => North,
            (North, West, North) => East,
            (North, West, South) => West,
            (North, East, North) => West,
            (North, East, South) => East,
            (South, West, North) => West,
            (South, West, South) => East,
            (South, East, North) => East,
            (South, East, South) => West,
            _ => panic!(),
        };

        let seg = Segment::new(p0, p1, wall);
        match wall {
            North | South => hori_map.entry(p0.y).or_default().push(seg),
            East | West => vert_map.entry(p0.x).or_default().push(seg),
        }

        previous_dir = dir;
        previous_wall = wall;
        previous_point = p1;
    }

    fn overlaps(r0: (i64, i64), r1: (i64, i64)) -> bool {
        !(r0.1 <= r1.0 || r0.0 >= r1.1)
    }

    fn overlaps_x(r0: (Vec2i64, Vec2i64), r1: (Vec2i64, Vec2i64)) -> bool {
        overlaps((r0.0.x, r0.1.x), (r1.0.x, r1.1.x))
    }

    fn overlaps_y(r0: (Vec2i64, Vec2i64), r1: (Vec2i64, Vec2i64)) -> bool {
        overlaps((r0.0.y, r0.1.y), (r1.0.y, r1.1.y))
    }

    let check_hori_matches = |y: i64, r: (Vec2i64, Vec2i64), dir: Dir| -> bool {
        for hori in hori_map.get(&y).iter().copied().flatten() {
            if overlaps_x(r, hori.range()) && hori.dir != dir {
                return false;
            }
        }

        true
    };

    let check_vert_matches = |x: i64, r: (Vec2i64, Vec2i64), dir: Dir| -> bool {
        for vert in vert_map.get(&x).iter().copied().flatten() {
            if overlaps_y(r, vert.range()) && vert.dir != dir {
                return false;
            }
        }

        true
    };

    let mut builder: PriorityQueueBuilder<(Vec2i64, Vec2i64), Reverse<i64>> = PriorityQueueBuilder::with_capacity(input.len() * input.len() - 1);
    for i in 0 .. input.len() - 1 {
        for j in i + 1 .. input.len() {
            let p0 = input[i];
            let p1 = input[j];
            let diff = (p0 - p1).abs() + Vec2::one();
            let area = diff.x * diff.y;
            builder.push((p0, p1), Reverse(area));
        }
    }

    let queue = builder.build();
    let mut max = 0;
    // for i in 0..input.len() - 1 {
    //     'outer: for j in i + 1 .. input.len() {
    'outer: for ((p0, p1), Reverse(area)) in queue.into_iter_sorted() {
        let (p0, p1) = normalize(p0, p1);

        if p0.x == p1.x || p0.y == p1.y {
            // line, figure it out later.
            continue;
        }

        let r = (p0, p1);

        // check for segments coincidental with the boundary

        if !check_hori_matches(p0.y, r, North) {
            continue;
        }

        if !check_hori_matches(p1.y, r, South) {
            continue;
        }

        if !check_vert_matches(p0.x, r, West) {
            continue;
        }

        if !check_vert_matches(p1.x, r, East) {
            continue;
        }

        // check for any segment collision in the interior
        for (_, horis) in hori_map.range((Bound::Excluded(p0.y), Bound::Excluded(p1.y))) {
            for hori in horis {
                if overlaps_x(r, hori.range()) {
                    continue 'outer;
                }
            }
        }

        for (_, verts) in vert_map.range((Bound::Excluded(p0.x), Bound::Excluded(p1.x))) {
            for vert in verts {
                if overlaps_y(r, vert.range()) {
                    continue 'outer;
                }
            }
        }

        max = area;
        break;
    }

    assert_eq!(1574717268, max);
}