use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

fn input() -> (Vec<i32>, Vec<i32>) {
    let lines = file_lines("inputs/day01.txt").to_vec();
    let mut left = Vec::with_capacity(lines.len());
    let mut right = Vec::with_capacity(lines.len());
    for l in lines {
        let split = l.split_ascii_whitespace().map(|s| s.parse().unwrap()).to_vec();
        left.push(split[0]);
        right.push(split[1]);
    }

    (left, right)
}

#[test]
fn part1() {
    let (mut left, mut right) = input();
    left.sort();
    right.sort();
    let total = left.into_iter().zip(right.into_iter()).map(|(l, r)| i32::abs(l - r)).sum::<i32>();
    assert_eq!(total, 2264607);
}

#[test]
fn part2() {
    let (left, right) = input();
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in right.into_iter() {
        *counts.entry(i).or_default() += 1;
    }

    let total = left.into_iter().map(|l| l * counts.get(&l).unwrap_or(&0)).sum::<i32>();
    assert_eq!(total, 19457120);
}