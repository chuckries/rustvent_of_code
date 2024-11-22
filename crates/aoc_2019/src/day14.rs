use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

type Map = HashMap<String, Vec<(i64, i64, String)>>;

struct Graph {
    map: Map
}

impl Graph {
    fn new() -> Self {
        let mut map = Map::new();

        for (left, right) in file_lines("inputs/day14.txt").map(|l| {
            let mut sides = l.split(" => ");
            let left = sides.next().unwrap().split(',').flat_map(|tok| tok.trim().split(' ')).map(|s| s.to_string()).to_vec();
            let right = sides.next().unwrap().split(' ').map(|s| s.to_string()).to_vec();
            (left, right)
        }) {
            let right_num = right[0].parse::<i64>().unwrap();
            let right = right[1].clone();

            for chunk in left.chunks(2) {
                let left_num = chunk[0].parse::<i64>().unwrap();
                let left = chunk[1].clone();
                map.entry(left).or_default().push((left_num, right_num, right.clone()));
            }
        }

        Self {
            map
        }
    }

    fn get_total_required(&self, name: &str, required: i64) -> i64 {
        self.map.get(name).map_or(required, |deps| {
            deps.iter().map(|dep| {
                let downstream = self.get_total_required(&dep.2, required);
                let multiplier = downstream / dep.1 + i64::signum(downstream % dep.1);
                dep.0 * multiplier
            }).sum()
        })
    }
}

#[test]
fn part1() {
    let answer = Graph::new().get_total_required("ORE", 1);
    assert_eq!(answer, 443537);
}

#[test]
fn part2() {
    let graph = Graph::new();

    let ore_per_fuel = graph.get_total_required("ORE", 1);
    let input_ore: i64 = 1000000000000;

    let mut lower_bound = input_ore / ore_per_fuel;
    let mut upper_bound = lower_bound * 2;

    let answer;
    loop {
        if lower_bound >= upper_bound {
            answer = lower_bound;
            break;
        }

        let half = lower_bound + ((upper_bound - lower_bound + 1) / 2);
        if graph.get_total_required("ORE", half) <= input_ore {
            lower_bound = half;
        } else {
            upper_bound = half - 1;
        }
    }

    assert_eq!(answer, 2910558);
}