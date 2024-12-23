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
                let digits = f64::floor(f64::log10(i as f64) + 1.0) as i64;
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

#[test]
fn part3() {
    let input = input();
    let mut map: HashMap<(i64, i32), usize> = HashMap::new();

    let answer = input.into_iter().map(|n| recursive(n, 75, &mut map)).sum::<usize>();
    assert_eq!(answer, 257335372288947);
}

fn recursive(n: i64, depth: i32, map: &mut HashMap<(i64, i32), usize>) -> usize {
    if let Some(cached) = map.get(&(n, depth)) {
        return *cached;
    } else {
        if depth == 0 {
            return 1;
        } else {
            let total = if n == 0 {
                recursive(1, depth - 1, map)
            } else {
                let digits = f64::floor(f64::log10(n as f64) + 1.0) as i64;
                if digits & 1 == 0 {
                    let div = i64::pow(10, digits as u32 / 2);
                    let left = recursive(n / div, depth - 1, map);
                    let right = recursive(n % div, depth - 1, map);
                    left + right
                } else {
                    recursive(n * 2024, depth - 1, map)
                }
            };

            map.insert((n, depth), total);
            total
        }
    }
}