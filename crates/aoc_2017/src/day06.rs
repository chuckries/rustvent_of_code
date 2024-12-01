use std::collections::{HashMap, HashSet};

use aoc_common::file_string;

fn input() -> Vec<i32> {
    file_string("inputs/day06.txt").split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn redistrubte(buckets: &mut[i32]) {
    let mut max_idx = 0;
    let mut max = buckets[0];
    for (idx, val) in buckets.iter().enumerate().skip(1) {
        if *val > max {
            max_idx = idx;
            max = buckets[idx];
        }
    }

    let mut idx = max_idx;
    let num = buckets[idx];
    buckets[idx] = 0;
    for _ in 0..num {
        idx += 1;
        idx %= buckets.len();
        buckets[idx] += 1;
    }
}

#[test]
fn part1() {
    let mut buckets = input();
    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    seen.insert(buckets.clone());

    loop {
        redistrubte(&mut buckets);

        if !seen.insert(buckets.clone()) { 
            break;
        }
    }

    assert_eq!(seen.len(), 6681);
}

#[test]
fn part2() {
    let mut buckets = input();
    let mut seen: HashMap<Vec<i32>, usize> = HashMap::new();
    seen.insert(buckets.clone(), 0);

    let answer;
    loop {
        redistrubte(&mut buckets);

        if let Some(idx) = seen.get(&buckets) {
            answer = seen.len() - *idx;
            break;
        }

        seen.insert(buckets.clone(), seen.len());
    }

    assert_eq!(answer, 2392);
}