use std::collections::HashSet;

use aoc_common::{file_string, IteratorExt, Vec2i32};

fn input() -> Vec<(bool, i32)> {
    file_string("inputs/day01.txt").split(", ").map(|l| {
        let chars = l.chars().to_vec();
        (chars[0] == 'L', chars[1..].iter().collect::<String>().parse().unwrap())
    }).collect()
}

#[test]
fn part1() {
    let input = input();
    let mut p = Vec2i32::zero();
    let mut d = -Vec2i32::unit_y();
    for (dir, length) in input {
        if dir {
            d.rotate_left();
        } else {
            d.rotate_right();
        }

        p += d * length;
    }

    let answer = p.manhattan();
    assert_eq!(answer, 226);
}

#[test]
fn part2() {
    let input = input();
    let mut p = Vec2i32::zero();
    let mut d = -Vec2i32::unit_y();
    let mut set: HashSet<Vec2i32> = HashSet::new();
    set.insert(p);
    'outer: for (dir, length) in input {
        if dir {
            d.rotate_left();
        } else {
            d.rotate_right();
        }

        for _ in 0..length {
            p += d;
            if !set.insert(p) {
                break 'outer;
            }
        }
    }

    let answer = p.manhattan();
    assert_eq!(answer, 79);
}