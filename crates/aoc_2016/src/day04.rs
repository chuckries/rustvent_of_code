use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

struct Name {
    parts: Vec<String>,
    id: i32,
    checksum: String,
}

fn input() -> Vec<Name> {
    file_lines("inputs/day04.txt").map(|l| {
        let split = l.split('-').to_vec();
        let mut parts: Vec<String> = Vec::new();

        let mut i = 0;
        while i < split.len() - 1 {
            parts.push(split[i].to_string());
            i += 1;
        }

        let split = split[i].split('[').to_vec();

        let id = split[0].parse::<i32>().unwrap();
        let checksum = split[1].trim_end_matches(']').to_string();

        Name {
            parts,
            id,
            checksum
        }
    }).collect()
}

#[test]
fn part1() {
    let answer: i32 = input().into_iter().filter_map(|n| {
        let mut map: HashMap<char, i32> = HashMap::new();
        for p in n.parts {
            for c in p.chars() {
                *map.entry(c).or_default() += 1;
            }
        }

        let calculated: String = map.into_iter().sorted_by(|lhs, rhs| {
            let mut ord = rhs.1.cmp(&lhs.1);
            if ord == std::cmp::Ordering::Equal {
                ord = lhs.0.cmp(&rhs.0);
            }
            ord
        }).take(5).map(|(k, _)| k).collect();

        if calculated == n.checksum {
            Some(n.id)
        } else {
            None
        }
    }).sum();

    assert_eq!(answer, 185371);
}

#[test]
fn part2() {
    let answers = input().into_iter().map(|n| {
        let shift = n.id % 26;

        let decrypt = n.parts.iter().map(|s| {
            s.chars().map(|c| {
                let mut c = c as u8 + shift as u8;
                if c > b'z' {
                    c -= 26
                }
                c as char
            }).collect::<String>()
        }).to_vec().join(" ");

        (decrypt, n.id)
    }).to_vec();

    let answer = answers.into_iter().filter(|(name, _)| name == "northpole object storage").next().unwrap().1;
    assert_eq!(answer, 984);
}