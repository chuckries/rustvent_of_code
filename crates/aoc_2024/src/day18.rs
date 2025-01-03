use std::collections::{HashSet, VecDeque};

use aoc_common::{file_lines, Grid, PriorityQueue, Vec2us};

const BOUNDS: Vec2us = Vec2us::new(71, 71);
const END: Vec2us = Vec2us::new(70, 70);

fn input() -> Vec<Vec2us> {
    file_lines("inputs/day18.txt").map(|l| {
        l.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect()
}

#[test]
fn part1() {
    let mut map: Grid<bool> = Grid::with_dimensions(BOUNDS);
    for p in input().into_iter().take(1024) {
        map[p] = true;
    }

    let mut visisted: Grid<bool> = Grid::with_dimensions(BOUNDS);
    let mut queue: PriorityQueue<(Vec2us, usize), usize> = PriorityQueue::new();
    queue.enqueue((Vec2us::zero(), 0), 0);

    let mut answer = 0;
    while let Some((current, dist)) = queue.dequeue() {
        if current == END {
            answer = dist;
            break;
        }

        if visisted[current] {
            continue;
        }
        visisted[current] = true;

        for p in current.adjacent_bounded(&BOUNDS) {
            if !visisted[p] && !map[p] {
                queue.enqueue((p, dist + 1), dist + 1 + p.manhattan_from(END));
            }
        }
    }

    assert_eq!(answer, 438);
}

fn try_find_path(map: &Grid<bool>) -> Option<HashSet<Vec2us>> {
    let mut visited: Grid<Option<Vec2us>> = Grid::with_dimensions(map.bounds());
    let mut queue: VecDeque<Vec2us> = VecDeque::new();
    queue.push_back(Vec2us::zero());
    visited[Vec2us::zero()] = Some(Vec2us::zero());

    while let Some(p) = queue.pop_front() {
        if p == END {
            let mut set: HashSet<Vec2us> = HashSet::new();
            set.insert(p);

            let mut current = p;
            loop {
                let prev = visited[current].unwrap();
                set.insert(prev);
                if prev.is_zero() {
                    break;
                }
                current = prev;
            }
            return Some(set);
        }

        for adj in p.adjacent_bounded(&BOUNDS) {
            if visited[adj].is_none() && !map[adj] {
                visited[adj] = Some(p);
                queue.push_back(adj);
            }
        }
    }

    None
}

#[test]
fn part2() {
    let mut map: Grid<bool> = Grid::with_dimensions(BOUNDS);
    let input = input();

    for p in input[0..1024].iter() {
        map[*p] = true;
    }

    let mut path = try_find_path(&map).unwrap();
    let mut answer = Vec2us::default();
    for p in input[1024..].iter() {
        map[*p] = true;

        if path.contains(p) {
            if let Some(new_path) = try_find_path(&map) {
                path = new_path;
            } else {
                answer = *p;
                break;
            }
        }
    }

    let answer = format!("{},{}", answer.x, answer.y);
    assert_eq!(answer, "26,22");
}