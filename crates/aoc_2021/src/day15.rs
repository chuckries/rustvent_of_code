use std::{collections::{HashSet}};

use aoc_common::{file_lines, Vec2us, PriorityQueue};

fn input() -> Vec<Vec<usize>> {
    file_lines("inputs/day15.txt").map(|l| {
        l.bytes().map(|b| (b - b'0' - 1) as usize).collect()
    }).collect()
}

fn search(map: &Vec<Vec<usize>>) -> usize {
    let mut to_visit: PriorityQueue<Vec2us, usize> = PriorityQueue::new();

    let mut visited: HashSet<Vec2us> = HashSet::new();

    let bounds = Vec2us::new(map[0].len(), map.len());
    let target = Vec2us::new(bounds.x - 1, bounds.y - 1);

    to_visit.enqueue(Vec2us::zero(), 0);

    while let Some((current, dist)) = to_visit.try_dequeue() {
        if current == target {
            return dist;
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        for adj in current.adjacent_bounded(&bounds) {
            if visited.contains(&adj) {
                continue;
            }
            to_visit.enqueue(adj, dist + map[adj.y][adj.x] + 1);
        }
    }

    panic!();
}

#[test]
fn part1() {
    let map = input();
    let answer = search(&map);
    assert_eq!(answer, 595);
}

#[test]
fn part2() {
    let start = input();
    let start_bounds = (start[0].len(), start.len());
    let bounds = (start_bounds.0 * 5, start_bounds.1 * 5);

    let mut map = vec![vec![0; bounds.0]; bounds.1];

    for x in 0usize..5 {
        for y in 0usize..5 {
            for u in 0..start[0].len() {
                for v in 0..start.len() {
                    map[y * start_bounds.1 + v][x * start_bounds.0 + u] = (start[v][u] + x + y) % 9;
                }
            }
        }
    }

    let answer = search(&map);
    assert_eq!(answer, 2914);
}