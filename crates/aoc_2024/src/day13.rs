use std::i32;

use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> Vec<(Vec2i32, Vec2i32, Vec2i32)> {
    let mut lines = file_lines("inputs/day13.txt");

    let mut inputs = Vec::new();
    while let Some(line) = lines.next() {
        let split = line.split([' ', '+']).to_vec();
        let x = split[3].trim_end_matches(',').parse().unwrap();
        let y = split[5].parse().unwrap();
        let a = (x, y).into();

        let line = lines.next().unwrap();
        let split = line.split([' ', '+']).to_vec();
        let x = split[3].trim_end_matches(',').parse().unwrap();
        let y = split[5].parse().unwrap();
        let b = (x, y).into();

        let line = lines.next().unwrap();
        let split = line.split([' ', '=']).to_vec();
        let x = split[2].trim_end_matches(',').parse().unwrap();
        let y = split[4].parse().unwrap();
        let target = (x, y).into();

        inputs.push((a, b, target));

        _ = lines.next();
    }

    inputs
}

#[test]
fn part1() {
    let input = input();

    let mut total = 0;
    for (a, b, target) in input {
        // A * a + B * b == target
        // minimize 3 * A + B
        // A <= 100, B <= 100

        let mut min = i32::MAX;
        for A in 0..=100 {
            for B in 0..=100 {
                if a * A + b * B == target {
                    let cand = 3 * A + B;
                    if cand < min {
                        min = cand;
                    }
                }
            }
        }

        if min != i32::MAX {
            total += min;
        }
    }

    assert_eq!(total, 29388);
}