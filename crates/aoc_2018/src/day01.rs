use std::{collections::HashSet};

use aoc_common::{file_lines_as, IteratorExt};

fn input() -> Vec<i32> {
    file_lines_as("inputs/day01.txt").collect()
}

#[test]
fn part1() {
    let answer = input().into_iter().sum();
    assert_eq!(420, answer);
}

#[test]
fn part2() {
    let input = input();
    let mut set: HashSet<i32> = HashSet::new();

    let mut total = 0;
    for i in input.iter().repeat(){
        total += i;
        if !set.insert(total) {
            break;
        }
    }

    assert_eq!(227, total);
}