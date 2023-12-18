use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt, Vec2i32};

#[derive(Copy, Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

type Map = HashMap<Vec2i32, State>;

fn input() -> (Map, Vec2i32) {
    let lines = file_lines("inputs/day22.txt").to_vec();
    let start = Vec2i32::new(lines[0].len() as i32, lines.len() as i32) / 2;

    let mut map = Map::new();

    for (j, line) in lines.into_iter().enumerate() {
        for (i, c) in line.char_indices() {
            if c == '#' {
                map.insert((i as i32, j as i32).into(), State::Infected);
            }
        }
    }

    (map, start)
}

fn run(iterations: usize, matcher: impl Fn(Vec2i32, State) -> (Vec2i32, State)) -> usize {
    let (mut map, mut pos) = input();
    let mut dir = -Vec2i32::unit_y();

    let mut infected = 0;
    for _ in 0..iterations {
        let current = map.entry(pos).or_insert(State::Clean);
        (dir, *current) = matcher(dir, *current);
        if matches!(*current, State::Infected) {
            infected += 1;
        }
        pos += dir;
    }
    infected
}

#[test]
fn part1() {
    let answer = run(10000, |dir, state| {
        match state {
            State::Clean => (dir.rotated_left(), State::Infected),
            State::Infected => (dir.rotated_right(), State::Clean),
            _ => panic!(),
        }
    });

    assert_eq!(answer, 5565);
}

#[test]
fn part2() {
    let answer = run(10000000, |dir, state| {
        match state {
            State::Clean => (dir.rotated_left(), State::Weakened),
            State::Weakened => (dir, State::Infected),
            State::Infected => (dir.rotated_right(), State::Flagged),
            State::Flagged => (-dir, State::Clean),
        }
    });

    assert_eq!(answer, 2511978);
}