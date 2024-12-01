use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<(i32, i32)> {
    file_lines("inputs/day01.txt").map(|l| { 
        let split = l.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).to_vec();
        (split[0], split[1])
     }).to_vec()
}

#[test]
fn part1() {
    let input = input();
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.iter().cloned().unzip();
    left.sort();
    right.sort();
    let total = left.into_iter().zip(right.into_iter()).map(|(l, r)| i32::abs(l - r)).sum::<i32>();
    assert_eq!(total, 0);
}

#[test]
fn part2() {
    let input = input();
    let (left, right): (Vec<_>, Vec<_>) = input.iter().cloned().unzip();

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in right.into_iter() {
        *counts.entry(i).or_default() += 1;
    }

    let total = left.into_iter().map(|l| l * counts.get(&l).unwrap_or(&0)).sum::<i32>();
    assert_eq!(total, 0);
}