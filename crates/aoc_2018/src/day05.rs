use std::usize;

use aoc_common::file_string;

fn input() -> Vec<u8> {
    file_string("inputs/day05.txt").bytes().collect()
}

const DELTA: u8 = b'a' - b'A';

fn react<T: Iterator<Item = u8>>(poly: T) -> usize {
    let mut stack: Vec<u8> = Vec::new();

    for c in poly {
        if let Some(top) = stack.last().copied() {
            if (top < c && top + DELTA == c) || (c < top && c + DELTA == top) {
                stack.pop().unwrap();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    stack.len()
}

#[test]
fn part1() {
    let answer = react(input().iter().copied());
    assert_eq!(9172, answer);
}

#[test]
fn part2() {
    let input = input();
    let mut min = usize::MAX;
    for i in b'A' ..= b'Z' {
        let j = i + DELTA;

        let cand = input.iter().copied().filter(|c| !(*c == i || *c == j));
        let count = react(cand);
        if count < min {
            min = count;
        }
    }

    assert_eq!(6550, min);
}