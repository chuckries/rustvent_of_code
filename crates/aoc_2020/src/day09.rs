use std::{collections::HashSet};

use aoc_common::file_lines_as;


fn input() -> Vec<i64> {
    file_lines_as("inputs/day09.txt").collect()
}

fn find_invalid_number(input: &[i64]) -> i64 {
    let mut front = 0;
    let mut back = 25;
    let mut set: HashSet<i64> = input[front..back].iter().cloned().collect();

    let mut answer = 0;
    while back < input.len() {
        let cand = input[back];

        let mut found = false;
        for i in set.iter() {
            let pair = cand - i;
            if pair != *i && set.contains(&pair) {
                found = true;
                break;
            }
        }
        
        if !found {
            answer = input[back];
            break;
        }

        set.remove(&input[front]);
        set.insert(input[back]);
        front += 1;
        back += 1;
    }

    answer
}

#[test]
fn part1() {
    let input = input();
    let answer = find_invalid_number(&input);
    assert_eq!(answer, 530627549);
}

#[test]
fn part2() {
    let input = input();
    let target = find_invalid_number(&input);

    let mut front = 0;
    let mut back = 1;
    let mut total: i64 = input[front..=back].iter().sum();

    loop {
        if total == target { break; }
        
        if total < target {
            back += 1;
            total += input[back];
        } else {
            total -= input[front];
            front += 1;
        }
    }

    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for i in input[front..=back].iter().cloned() {
        if i < min {
            min = i;
        }

        if i > max {
            max = i;
        }
    }

    let answer = min + max;
    assert_eq!(answer, 77730285);
}