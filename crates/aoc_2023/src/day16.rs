use std::collections::VecDeque;

use aoc_common::{file_2d_map, Vec2i32};

fn input() -> Vec<Vec<char>> {
    file_2d_map("inputs/day16.txt")
}

fn dir_char(dir: Vec2i32) -> char {
    match (dir.x, dir.y) {
        (-1,  0) => '<',
        ( 1,  0) => '>',
        ( 0, -1) => '^',
        ( 0,  1) => 'v',
        _ => panic!(),
    }
}

fn run (mut map: Vec<Vec<char>>, start_pos: Vec2i32, start_dir: Vec2i32) -> i32 {
    let mut energized = vec![vec!['.'; map[0].len()]; map.len()];
    let bounds = Vec2i32::new(map[0].len() as i32, map.len() as i32);
    let mut queue: VecDeque<(Vec2i32, Vec2i32)> = VecDeque::new();
    queue.push_back((start_pos, start_dir));

    while let Some((mut pos, mut dir)) = queue.pop_front() {
        loop {
            pos += dir;

            if !pos.is_in_bounds(bounds) {
                break;
            }

            let c = &mut map[pos.y as usize][pos.x as usize];
            energized[pos.y as usize][pos.x as usize] = '#';

            match *c {
                '.' => {
                    *c = dir_char(dir);
                }
                '\\' => {
                    match (dir.x, dir.y) {
                        ( 1,  0) | (-1,  0) => dir = dir.rotate_right(),
                        ( 0,  1) | ( 0, -1) => dir = dir.rotate_left(),
                        _ => panic!()
                    }
                }
                '/' => {
                    match (dir.x, dir.y) {
                        ( 1,  0) | (-1,  0) => dir = dir.rotate_left(),
                        ( 0,  1) | ( 0, -1) => dir = dir.rotate_right(),
                        _ => panic!()
                    }
                }
                '-' => {
                    if matches!((dir.x, dir.y), (0, -1) | (0, 1)) {
                        queue.push_back((pos, dir.rotate_left()));
                        dir = dir.rotate_right();
                    }
                }
                '|' => {
                    if matches!((dir.x, dir.y), (-1, 0) | (1, 0)) {
                        queue.push_back((pos, dir.rotate_left()));
                        dir = dir.rotate_right();
                    }
                }
                'v' | '^' => {
                    match (dir.x, dir.y) {
                        (1, 0) | (-1, 0) => *c = '2',
                        (0, 1) | (0, -1) => break,
                        _ => panic!()
                    }
                }
                '<' | '>' => {
                    match (dir.x, dir.y) {
                        (0, 1) | (0, -1) => *c = '2',
                        (1, 0) | (-1, 0) => break,
                        _ => panic!()
                    }
                }
                '2' => break,
                _ => panic!()
            }
        }
    }

    energized.iter().map(|r| r.iter()).flatten().filter(|c| **c != '.').count() as i32
}

#[test]
fn part1() {
    let map = input();
    let answer = run(map, (-1, 0).into(), (1, 0).into());

    assert_eq!(8901, answer);
}

#[test]
fn part2() {
    let map = input();
    let bounds = Vec2i32::new(map[0].len() as i32, map.len() as i32);

    let mut answer = 0;
    for i in 0..bounds.x {
        let cand = run(map.clone(), (i, -1).into(), (0, 1).into());
        if cand > answer {
            answer = cand;
        }

        let cand = run(map.clone(), (i, bounds.y).into(), (0, -1).into());
        if cand > answer {
            answer = cand;
        }
    }

    for j in 0..bounds.y {
        let cand = run(map.clone(), (-1, j).into(), (1, 0).into());
        if cand > answer {
            answer = cand;
        }

        let cand = run(map.clone(), (bounds.x, j).into(), (-1, 0).into());
        if cand > answer {
            answer = cand;
        }
    }

    assert_eq!(9064, answer);
}