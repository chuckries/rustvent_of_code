use std::i32;

use aoc_common::{file_lines, IteratorExt, Vec2i32, Vec2i64};

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

#[test]
fn part2() {
    // A * ax + B * bx = tx
    // A * ay + B * by = ty

    // A * ax = tx - B * bx
    // A = (tx - B * bx) / ax

    // ((tx - B * bx) / ax) * ay + B * by = ty
    // (ay / ax) * (tx - B * bx) + B * by = ty
    // tx * (ay / ax) - (B * bx) * (ay /ax) + B * by = ty
    // B * by - B * bx * (ay / ax) = ty - tx * (ay / ax)
    // B * (by - bx * (ay /ax)) = ty - tx * (ay / ax)
    // B = (ty - tx * (ay / ax)) / (by - bx * (ay / ax))
}

#[test]
fn part2_2() {
    let input = input().into_iter().map(|(a, b, t)| {
        (a.cast::<i64>(), b.cast::<i64>(), Vec2i64::new(t.x as i64 + 10000000000000, t.y as i64 + 10000000000000))
    }).to_vec();

    let mut total = 0;
    for (a, b, t) in input {
        let ay_ax: f64 = a.y as f64 / a.x as f64;

        let B = (t.y as f64 - t.x as f64 * ay_ax) / (b.y as f64 - b.x as f64 * ay_ax);
        let B = B.round();

        let A = (t.x as f64 - B * b.x as f64) / a.x as f64;
        let A = A.round();

        if a * A as i64 + b * B as i64 != t {
            continue;
        }

        total += 3 * A as i64 + B as i64;
    }

    assert_eq!(total, 99548032866004);
}