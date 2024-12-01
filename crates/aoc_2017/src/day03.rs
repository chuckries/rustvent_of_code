use std::collections::HashMap;

use aoc_common::Vec2i32;

const INPUT: i32 = 361527;

#[test]
fn part1() {
    let mut pos = Vec2i32::zero();
    let mut loc = 1;
    let mut size = 1;

    fn take_steps(loc: &mut i32, size: i32, dir: Vec2i32, pos: &mut Vec2i32) -> bool {
        let remaining = INPUT - *loc;
        let min = remaining.min(size);
        *pos += dir * min;
        *loc += min;

        return min == remaining;
    }

    loop {
        if take_steps(&mut loc, size, Vec2i32::unit_x(), &mut pos) { break; }
        if take_steps(&mut loc, size, -Vec2i32::unit_y(), &mut pos) { break; }
        size += 1;
        if take_steps(&mut loc, size, -Vec2i32::unit_x(), &mut pos) { break; }
        if take_steps(&mut loc, size, Vec2i32::unit_y(), &mut pos) { break; }
        size += 1;
    }

    let answer = pos.manhattan();
    assert_eq!(answer, 326);
}

static SPIRAL_DIRS: [Vec2i32; 4] = [
    Vec2i32::new(1, 0),
    Vec2i32::new(0, -1),
    Vec2i32::new(-1, 0),
    Vec2i32::new(0, 1),
];

struct SpiralIter {
    pos: Vec2i32,
    dir_idx: usize,
    cur_size: usize,
    max_size: usize,
    size_count: usize,
}

impl SpiralIter {
    fn new() -> Self {
        Self {
            pos: Vec2i32::zero(),
            dir_idx: SPIRAL_DIRS.len() - 1,
            cur_size: 0,
            max_size: 0,
            size_count: 1,
        }
    }
}

impl Iterator for SpiralIter {
    type Item = Vec2i32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.pos;

        if self.cur_size == self.max_size {
            self.cur_size = 0;
            self.size_count += 1;
            if self.size_count == 2 {
                self.max_size += 1;
                self.size_count = 0;
            }
            self.dir_idx += 1;
            if self.dir_idx == SPIRAL_DIRS.len() {
                self.dir_idx = 0;
            }
        }

        self.pos += SPIRAL_DIRS[self.dir_idx];
        self.cur_size += 1;

        Some(next)
    }
}

#[test]
fn part2() {
    let mut map: HashMap<Vec2i32, i32> = HashMap::new();
    map.insert((0, 0).into(), 1);
    let mut spiral = SpiralIter::new();
    spiral.next().unwrap(); // skip (0, 0);

    let answer;
    loop {
        let p = spiral.next().unwrap();
        let sum_adj = p.surrounding_unbounded().filter_map(|p| map.get(&p)).copied().sum::<i32>();
        if sum_adj > INPUT {
            answer = sum_adj;
            break;
        }
        map.insert(p, sum_adj);
    }

    assert_eq!(answer, 363010);
}