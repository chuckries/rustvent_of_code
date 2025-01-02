use std::collections::VecDeque;

use aoc_common::{file_lines, IteratorExt, PriorityQueue, Vec2us};

const BOUNDS: Vec2us = Vec2us::new(71, 71);
const END: Vec2us = Vec2us::new(70, 70);

fn input() -> Vec<Vec2us> {
    file_lines("inputs/day18.txt").map(|l| {
        l.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
    }).collect()
}

#[test]
fn part1() {
    let mut map = vec![vec![false; BOUNDS.x]; BOUNDS.y];
    for p in input().into_iter().take(1024) {
        map[p.y][p.x] = true;
    }

    let mut visisted = vec![vec![false; BOUNDS.x]; BOUNDS.y];
    let mut queue: PriorityQueue<(Vec2us, usize), usize> = PriorityQueue::new();
    queue.enqueue((Vec2us::zero(), 0), 0);

    let mut answer = 0;
    while let Some((current, dist)) = queue.dequeue() {
        if current == END {
            answer = dist;
            break;
        }

        if visisted[current.y][current.x] {
            continue;
        }
        visisted[current.y][current.x] = true;

        for p in current.adjacent_bounded(&BOUNDS) {
            if !visisted[p.y][p.x] && !map[p.y][p.x] {
                queue.enqueue((p, dist + 1), dist + 1 + p.manhattan_from(END));
            }
        }
    }

    assert_eq!(answer, 438);
}

fn try_find_path(map: &Vec<Vec<bool>>) -> Option<Vec<Vec2us>> {
    let mut visited = vec![vec![false; BOUNDS.x]; BOUNDS.y];
    let mut queue: VecDeque<(Vec2us, Vec<Vec2us>)> = VecDeque::new();
    queue.push_back((Vec2us::zero(), vec![Vec2us::zero()]));

    while let Some((p, path)) = queue.pop_front() {
        if p == END {
            return Some(path);
        }

        if visited[p.y][p.x] {
            continue;
        }
        visited[p.y][p.x] = true;

        for adj in p.adjacent_bounded(&BOUNDS) {
            if !visited[adj.y][adj.x] && !map[adj.y][adj.x] {
                let mut next_path = path.clone();
                next_path.push(adj);
                queue.push_back((adj, next_path));
            }
        }
    }

    None
}

#[test]
fn part2() {
    let mut map = vec![vec![false; BOUNDS.x]; BOUNDS.y];
    let input = input();

    for p in input[0..1024].iter() {
        map[p.y][p.x] = true;
    }

    let path = try_find_path(&map).unwrap();
    let mut path_set = path.into_iter().to_set();
    let mut answer = Vec2us::default();
    for p in input[1024..].iter() {
        map[p.y][p.x] = true;

        if path_set.contains(p) {
            if let Some(path) = try_find_path(&map) {
                path_set = path.into_iter().to_set();
            } else {
                answer = *p;
                break;
            }
        }
    }

    let answer = format!("{},{}", answer.x, answer.y);
    assert_eq!(answer, "26,22");
}