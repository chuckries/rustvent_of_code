use std::collections::HashSet;

use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day10.txt").map(|l| l.into_bytes().into_iter().map(|b| (b - b'0') as i32).to_vec()).to_vec()
}

trait Trailer: Default {
    fn after_trailhead(&mut self);
    fn end_found(&mut self, p: Vec2i32);
    fn total(&self) -> i32;
}

fn run<T: Trailer>() -> i32 {
    let map = input();
    let bounds: Vec2i32 = Vec2i32::new(map[0].len() as i32, map.len() as i32);
    let mut trailer = T::default();

    fn test<T: Trailer>(p: Vec2i32, map: &Vec<Vec<i32>>, bounds: Vec2i32, n: i32, t: &mut T) {
        if n == 9 {
            t.end_found(p);
        } else {
            for adj in p.adjacent_bounded(&bounds) {
                if map[adj.y as usize][adj.x as usize] == n + 1 {
                    test(adj, map, bounds, n + 1, t);
                }
            };
        }
    }

    for j in 0..bounds.y {
        for i in 0..bounds.x {
            if map[j as usize][i as usize] == 0 {
                test((i, j).into(), &map, bounds, 0, &mut trailer);
                trailer.after_trailhead();
            }
        }
    }

    trailer.total()
}

#[derive(Default)]
struct Trailer1 {
    positions: HashSet<Vec2i32>,
    total: i32,
}

impl Trailer for Trailer1 {
    fn after_trailhead(&mut self) {
        self.total += self.positions.len() as i32;
        self.positions.clear();
    }

    fn end_found(&mut self, p: Vec2i32) {
        self.positions.insert(p);
    }

    fn total(&self) -> i32 {
        self.total
    }
}

#[test]
fn part1() {
    let answer = run::<Trailer1>();
    assert_eq!(answer, 482);
}

#[derive(Default)]
struct Trailer2 {
    total: i32
}

impl Trailer for Trailer2 {
    fn after_trailhead(&mut self) { }

    fn end_found(&mut self, _: Vec2i32) {
        self.total += 1;
    }

    fn total(&self) -> i32 {
        self.total
    }
}

#[test]
fn part2() {
    let answer = run::<Trailer2>();
    assert_eq!(answer, 1094);
}