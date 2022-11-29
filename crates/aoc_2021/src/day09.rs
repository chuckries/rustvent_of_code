use std::{collections::{HashSet, VecDeque}};

use aoc_common::{file_lines, Vec2us};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day09.txt")
        .map(|l| {
            l.bytes().map(|c| (c - b'0') as i32).collect()
        }).collect()
}

fn basins(map: &Vec<Vec<i32>>) -> Vec<Vec2us> {
    let mut basins = Vec::new();
    let bounds: Vec2us = (map[0].len(), map.len()).into();

    for p in bounds.iter() {
        if p.adjacent_bounded(&bounds).all(|adj| {
            map[adj.y][adj.x] > map[p.y][p.x]
        }) {
            basins.push(p);
        }
    }

    basins
}

#[test]
fn part1() {
    let map = input();
    let risk: i32 = basins(&map).iter().map(|b| map[b.y][b.x] + 1).sum();

    assert_eq!(risk, 486);
}

#[test]
fn part2() {
    let map = input();
    let mut visited: HashSet<Vec2us> = HashSet::new();
    let mut to_visit: VecDeque<Vec2us> = VecDeque::new();

    let mut sizes: Vec<usize> = Vec::new();

    let bounds = Vec2us::new(map[0].len(), map.len());

    for basin in basins(&map) {
        visited.clear();
        to_visit.clear();

        to_visit.push_back(basin);
        while let Some(current) = to_visit.pop_front() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

            for adj in current.adjacent_bounded(&bounds).filter(|adj| map[adj.y][adj.x] != 9) {
                to_visit.push_back(adj);
            }
        }

        sizes.push(visited.len());
    }

    sizes.sort_by(|a, b| b.cmp(a));
    let answer = sizes
        .into_iter()
        .take(3)
        .reduce(|accum, item| accum * item)
        .unwrap();

    assert_eq!(answer, 1059300);
}