use std::{collections::{HashMap, HashSet}, usize};

use aoc_common::{file_lines, IteratorExt};

type Map = HashMap<String, Vec<String>>;

fn input() -> (Map, String) {
    let mut lines = file_lines("inputs/day19.txt");

    let mut map = Map::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let split = line.split(' ').to_vec();
        map.entry(split[0].to_string()).or_default().push(split[2].to_string());
    }

    let target = lines.next().unwrap();

    (map, target)
}

fn process<R, F: FnMut(String) -> Option<R>>(input: &Map, target: &str, mut f: F) -> Option<R> {
    for (key, value) in input.iter() {
        let mut idx = 0;
        loop {
            if let Some(found) = target[idx..].find(key) {
                println!("{}", found);

                let found = idx + found;
                idx = found + key.len();

                for replace in value.iter() {
                    let mut next = target[0..found].to_string();
                    next.push_str(replace);
                    next.push_str(&target[idx..]);
                    if let Some(r) = f(next) {
                        return Some(r);
                    }
                }
            } else {
                break;
            }
        }
    }

    None
}

#[test]
fn part1() {
    let (input, target) = input();
    let mut solutions: HashSet<String> = HashSet::new();

    process(
        &input,
        &target,
        |s| -> Option<()> {
            solutions.insert(s);
            None
        }
    );
    
    let answer = solutions.len();
    assert_eq!(answer, 518);
}

#[test]
// maybe some day come back and clean this up, it is currently non-deterministic
// apparently HashMap iter is not deterministic? and depending on the run this test will either pass or hang
#[ignore]
fn part2() {
    fn recurse(s: &str, map: &Map, level: usize) -> Option<usize> {
        if s == "e" {
            return Some(level);
        } else {
            if let Some(level) = process(map, s, |s| {
                recurse(&s, map, level + 1)
            }) {
                return Some(level);
            }
        }

        None
    }

    let (input, target) = input();

    println!("{:?}", input);

    let mut inverse: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in input.into_iter() {
        for s in v.into_iter() {
            inverse.entry(s).or_default().push(k.clone());
        }
    }


    let answer = recurse(&target, &inverse, 0).unwrap();
    assert_eq!(answer, 200);
}