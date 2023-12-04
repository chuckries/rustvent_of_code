use std::collections::HashSet;

use aoc_common::file_lines;

fn input() -> Vec<i32> {
    file_lines("inputs/day04.txt").map(|l| {
        let mut nums = l.split(": ").skip(1).next().unwrap().split_whitespace();

        let mut winning_nums: HashSet<i32> = HashSet::new();
        let mut my_nums: Vec<i32> = Vec::new();

        loop {
            let n = nums.next().unwrap();
            if n == "|" { break; }

            winning_nums.insert(n.parse().unwrap());
        }

        while let Some(n) = nums.next() {
            my_nums.push(n.parse().unwrap())
        }

        my_nums.iter().filter(|n| winning_nums.contains(n)).count() as i32
    }).collect()
}

#[test]
fn part1() {
    let input = input();

    let answer: i32 = input.iter().map(|g| {
        let score: i32 = if *g == 0 { 0 } else { i32::pow(2, *g as u32 - 1) };

        score
    }).sum();

    assert_eq!(23941, answer);
}

#[test]
fn part2() {
    let wins = input();
    let mut counts = vec![1; wins.len()];

    for i in 0..wins.len() {
        let count = counts[i];
        for n in counts[i + 1..i + 1 + wins[i] as usize].iter_mut() {
            *n += count;
        }
    }

    let answer: i32 = counts.iter().sum();

    assert_eq!(5571760, answer);
}