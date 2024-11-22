use std::{collections::HashMap};

use aoc_common::file_lines;

type Map = HashMap<String, (String, Vec<String>)>;

fn input() -> Map {
    let mut map = Map::new();

    for s in file_lines("inputs/day06.txt") {
        let tok = s.split(')').collect::<Vec<_>>();

        map.entry(tok[0].to_string()).or_default().1.push(tok[1].to_string());
        map.entry(tok[1].to_string()).or_default().0 = tok[0].to_string();
    }

    map
}

fn count(key: &str, map: &Map, depth: usize) -> usize {
    depth + if let Some((_, children)) = map.get(key) {
        children.iter().map(|c| count(c, map, depth + 1)).sum::<usize>()
    } else {
        0
    }
}

#[test]
fn part1() {
    let answer = count("COM", &input(), 0);
    assert_eq!(answer, 417916);
}

#[test]
fn part2() {
    let map = input();

    let mut distances: HashMap<&str, i32> = HashMap::new();
    let mut current = "YOU";
    let mut steps = 0;
    while let Some((parent, _)) = map.get(current) {
        distances.insert(parent, steps);
        current = parent;
        steps += 1;
    }

    current = &map.get("SAN").unwrap().0;
    steps = 0;
    loop {
        if let Some(dist) = distances.get(current) {
            steps += *dist;
            break;
        } else {
            current = &map.get(current).unwrap().0;
            steps += 1;
        }
    }

    assert_eq!(steps, 523);
}
