use std::collections::HashSet;

use aoc_common::{file_string, Vec2i32};

fn input() -> String {
    file_string("inputs/day03.txt")
}

#[test]
fn part1() {
    let mut p = Vec2i32::zero();
    let mut set: HashSet<Vec2i32> = HashSet::new();
    set.insert(p);

    for b in input().bytes() {
        p += match b {
            b'<' => -Vec2i32::unit_x(),
            b'>' =>  Vec2i32::unit_x(),
            b'^' => -Vec2i32::unit_y(),
            b'v' =>  Vec2i32::unit_y(),
            _ => panic!(),
        };
        set.insert(p);
    }

    let answer = set.len();
    assert_eq!(answer, 2592);
}

#[test]
fn part2() {
    let mut current = Vec2i32::zero();
    let mut next = Vec2i32::zero();

    let mut set: HashSet<Vec2i32> = HashSet::new();
    set.insert(current);

    for b in input().bytes() {
        current += match b {
            b'<' => -Vec2i32::unit_x(),
            b'>' =>  Vec2i32::unit_x(),
            b'^' => -Vec2i32::unit_y(),
            b'v' =>  Vec2i32::unit_y(),
            _ => panic!(),
        };
        set.insert(current);

        std::mem::swap(&mut current, &mut next);
    }

    let answer = set.len();
    assert_eq!(answer, 2360);
}