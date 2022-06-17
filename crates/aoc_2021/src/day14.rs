use std::{collections::HashMap};

use aoc_common::file_lines;

fn input() -> (Vec<u8>, HashMap<[u8; 2], u8>) {
    let mut lines = file_lines("inputs/day14.txt");
    let start = lines.next().unwrap().into_bytes();

    let mut lines = lines.skip(1);

    let mut map: HashMap<[u8; 2], u8> = HashMap::new();
    while let Some(line) = lines.next() {
        let mut tok = line.split(" -> ");
        let key_bytes = tok.next().unwrap().as_bytes();
        let key = [key_bytes[0], key_bytes[1]];
        let value = tok.next().unwrap().as_bytes()[0];
        map.insert(key, value);
    }

    (start, map)
}

fn run(iterations: i32) -> usize {
    let (start, rules) = input();

    let mut counts: HashMap<u8, usize> = HashMap::new();
    let mut pairs: HashMap<[u8; 2], usize> = HashMap::new();

    for c in start.iter() {
        *counts.entry(*c).or_default() += 1;
    }

    for p in start.windows(2) {
        *pairs.entry([p[0], p[1]]).or_default() += 1;
    }

    for _ in 0..iterations {
        let current: Vec<_> = pairs.drain().collect();

        for (pair, count) in current {
            let next = rules.get(&pair).unwrap();
            *counts.entry(*next).or_default() += count;
            *pairs.entry([pair[0], *next]).or_default() += count;
            *pairs.entry([*next, pair[1]]).or_default() += count;
        }
    }

    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for (_, c) in counts {
        if c < min { min = c; }
        if c > max { max = c; }
    }

    max - min
}

#[test]
fn part1() {
    let answer = run(10);
    assert_eq!(answer, 2587);
}

#[test]
fn part2() {
    let answer = run(40);
    assert_eq!(answer, 3318837563123);
}