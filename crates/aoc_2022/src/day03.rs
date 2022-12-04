use std::collections::HashSet;

use aoc_common::file_lines;

fn input() -> impl Iterator<Item = String> {
    file_lines("inputs/day03.txt")
}

fn letter_score(c: u8) -> u32 {
    let score = match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!()
    };
    score as u32
}

#[test]
fn part1() {
    let answer: u32 = input().map(|line| {
        let (front, back) = line.split_at(line.len() / 2);
        let front: HashSet<u8> = front.bytes().collect();
        let back: HashSet<u8> = back.bytes().collect();

        front.intersection(&back).copied().map(letter_score).sum::<u32>()
    }).sum();

    assert_eq!(answer, 7793);
}

#[test]
fn part2() {
    let input: Vec<String> = input().collect();

    let answer: u32 = input.chunks(3).map(|lines| {
        let l0: HashSet<u8> = lines[0].bytes().collect();
        let l1: HashSet<u8> = lines[1].bytes().collect();
        let l2: HashSet<u8> = lines[2].bytes().collect();

        l0.intersection(&l1).copied().collect::<HashSet<_>>().intersection(&l2).copied().map(letter_score).sum::<u32>()
    }).sum();

    assert_eq!(answer, 2499);
}