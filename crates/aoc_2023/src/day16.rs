use std::collections::VecDeque;

use aoc_common::{Grid, Vec2i32};

type Map = Grid<char>;

fn input() -> Map {
    Map::file_as_grid("inputs/day16.txt", &mut |b, _| b as char)
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

fn run (mut map: Map, start_pos: Vec2i32, start_dir: Vec2i32) -> i32 {
    let mut energized = vec![vec!['.'; map.width()]; map.height()];
    let bounds = Vec2i32::new(map.width() as i32, map.height() as i32);
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
                        ( 1,  0) | (-1,  0) => dir.rotate_right(),
                        ( 0,  1) | ( 0, -1) => dir.rotate_left(),
                        _ => panic!()
                    }
                }
                '/' => {
                    match (dir.x, dir.y) {
                        ( 1,  0) | (-1,  0) => dir.rotate_left(),
                        ( 0,  1) | ( 0, -1) => dir.rotate_right(),
                        _ => panic!()
                    }
                }
                '-' => {
                    if matches!((dir.x, dir.y), (0, -1) | (0, 1)) {
                        queue.push_back((pos, dir.rotated_left()));
                        dir.rotate_right();
                    }
                }
                '|' => {
                    if matches!((dir.x, dir.y), (-1, 0) | (1, 0)) {
                        queue.push_back((pos, dir.rotated_left()));
                        dir.rotate_right();
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

    let mut answer = 0;
    for i in 0..map.width() {
        let cand = run(map.clone(), (i as i32, -1).into(), (0, 1).into());
        if cand > answer {
            answer = cand;
        }

        let cand = run(map.clone(), (i as i32, map.height() as i32).into(), (0, -1).into());
        if cand > answer {
            answer = cand;
        }
    }

    for j in 0..map.height() {
        let cand = run(map.clone(), (-1, j as i32).into(), (1, 0).into());
        if cand > answer {
            answer = cand;
        }

        let cand = run(map.clone(), (map.width() as i32, j as i32).into(), (-1, 0).into());
        if cand > answer {
            answer = cand;
        }
    }

    assert_eq!(9064, answer);
}