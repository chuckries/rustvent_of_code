use std::collections::HashSet;

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<String> {
    file_lines("inputs/day04.txt").collect()
}

#[test]
fn part1() {
    let input = input();
    let answer = input.into_iter().filter(|p| {
        let mut set: HashSet<&str> = HashSet::new();
        for w in p.split(' ') {
            if !set.insert(w) {
                return false;
            }
        }
        true
    }).count();

    assert_eq!(answer, 383);
}

#[test]
fn part2() {
    let input = input();
    let answer = input.into_iter().filter(|p| {
        let mut set: HashSet<Vec<u8>> = HashSet::new();
        for w in p.split(' ') {
            let w = w.bytes().sorted().to_vec();
            if !set.insert(w) {
                return false;
            }
        }
        true
    }).count();

    assert_eq!(answer, 265);
}