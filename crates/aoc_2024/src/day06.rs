use std::collections::HashSet;

use aoc_common::{file_as_byte_grid, Grid, Vec2i32};

fn input() -> (Grid<u8>, Vec2i32) {
    let map = file_as_byte_grid("inputs/day06.txt");

    let mut start = Vec2i32::default();
    for (p, c) in map.enumerate() {
        if *c == b'^' {
            start = p.cast();
            break;
        }
    }
    (map, start)
}

use MarchResult::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MarchResult {
    Forward,
    Turn,
    OOB,
}

fn march(pos: &mut Vec2i32, dir: &mut Vec2i32, map: &Grid<u8>) -> MarchResult {
    let mut result = Forward;
    loop {
        let next = *pos + *dir;

        if !next.is_in_bounds(map.bounds().cast()) {
            return OOB;
        }

        if map[next] == b'#' {
            dir.rotate_right();
            result = Turn;
        } else {
            *pos = next;
            return result;
        }
    }
}

fn find_visited(mut pos: Vec2i32, mut dir: Vec2i32, map: &Grid<u8>) -> Grid<bool> {
    let mut visited = map.same_of_type::<bool>();
    visited[pos] = true;

    while march(&mut pos, &mut dir, &map) != OOB {
        visited[pos] = true;
    }

    visited
}

fn check_cycle(mut pos: Vec2i32, mut dir: Vec2i32, map: &Grid<u8>) -> bool {
    let mut visited: HashSet<(Vec2i32, Vec2i32)> = HashSet::new();

    loop {
        let result = march(&mut pos, &mut dir, map);
        if result == OOB {
            break;
        } else if result == Turn {
            if visited.contains(&(pos, dir)) {
                return true;
            }
            visited.insert((pos, dir));
        }
    }

    false
}

#[test]
fn part1() {
    let (map, start) = input();
    let visited = find_visited(start, -Vec2i32::unit_y(), &map);
    let answer = visited.iter().filter(|b| **b).count();
    assert_eq!(answer, 4580);
}

#[test]
fn part2() {
    let (mut map, start) = input();
    let dir = -Vec2i32::unit_y();
    let mut visited = find_visited(start, dir, &map);
    visited[start.y as usize][start.x as usize] = false;

    let mut total = 0;
    for (p, v) in visited.enumerate() {
        if *v {
            map[p] = b'#';
            if check_cycle(start, dir, &map) {
                total += 1;
            }
            map[p] = b'.';
        }
    }

    assert_eq!(total, 1480);
}