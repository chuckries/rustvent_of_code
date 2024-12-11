use std::collections::HashMap;

use aoc_common::{file_string, IteratorExt};

fn input() -> Vec<i64> {
    file_string("inputs/day11.txt").split(' ').map(|s| s.parse::<i64>().unwrap()).to_vec()
}

fn run(iterations: usize) -> usize {
    let input = input();

    let mut current: HashMap<i64, usize> = HashMap::new();
    for i in input.into_iter() {
        current.insert(i, 1);
    }
    let mut next: HashMap<i64, usize> = HashMap::new();

    for _ in 0..iterations {
        for (i, count) in current.drain() {
            if i == 0 {
                *next.entry(1).or_default() += count;
            } else {
                let mut tmp = i;
                let mut digits = 0;
                while tmp > 0 {
                    digits += 1;
                    tmp /= 10;
                }
                if digits & 1 == 0 {
                    let div = i64::pow(10, digits as u32 / 2);
                    let left = i / div;
                    let right = i % div;
                    *next.entry(left).or_default() += count;
                    *next.entry(right).or_default() += count;
                } else {
                    let num = i * 2024;
                    *next.entry(num).or_default() += count;
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    current.values().sum::<usize>()
}

#[test]
fn part1() {
    let answer = run(25);
    assert_eq!(answer, 216996);
}

#[test]
fn part2() {
    let answer = run(75);
    assert_eq!(answer, 257335372288947);
}