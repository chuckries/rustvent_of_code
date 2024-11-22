use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

type Items = HashMap<String, i32>;

fn input() -> (Vec<Items>, Items) {
    let input: Vec<Items> = file_lines("inputs/day16.txt").map(|l| {
        let split = l.split(' ').to_vec();
        split.chunks(2).skip(1).map(|chunk| {
            (chunk[0].trim_end_matches(':').to_string(), chunk[1].trim_end_matches(',').parse::<i32>().unwrap())
        }).collect()
    }).collect();

    let target: Items = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1)
    ].into_iter().map(|(s, i)| (s.to_string(), i)).collect();

    (input, target)
}

fn run(test: fn(key: &str, candidate: &i32, target: &i32) -> bool) -> usize {
    let (input, target) = input();

    let mut i = 0;
    'outer: loop {
        i += 1;
        let input = &input[i - 1];
        for (k, v) in target.iter() {
            if let Some(found) = input.get(k) {
                if !test(k, found, v) {
                    continue 'outer;
                }
            }
        }
        break;
    }

    i
}

#[test]
fn part1() {
    fn test(_: &str, cand: &i32, target: &i32) -> bool {
        cand == target
    }

    let answer = run(test);
    assert_eq!(answer, 373);
}

#[test]
fn part2() {
    fn test(key: &str, cand: &i32, target: &i32) -> bool {
        match key {
            "cats" | "trees" => cand > target,
            "pomeranian" | "goldfish" => cand < target,
            _ => cand == target
        }
    }

    let answer = run(test);
    assert_eq!(answer, 260);
}