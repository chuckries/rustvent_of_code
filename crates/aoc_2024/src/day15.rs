use aoc_common::{file_lines, Grid, IteratorExt, Vec2i32};

use Dir::*;

enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn unit_vec(&self) -> Vec2i32 {
        match self {
            U => -Vec2i32::unit_y(),
            D => Vec2i32::unit_y(),
            L => -Vec2i32::unit_x(),
            R => Vec2i32::unit_x(),
        }
    }
}

fn input() -> (Grid<u8>, Vec<Dir>) {
    let mut lines = file_lines("inputs/day15.txt");

    let mut map: Vec<Vec<u8>> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        map.push(line.into_bytes());
    }

    let dirs = lines.flat_map(|l| l.into_bytes()).map(|c| {
        match c {
            b'^' => U,
            b'v' => D,
            b'<' => L,
            b'>' => R,
            _ => panic!()
        }
    }).to_vec();

    (map.into(), dirs)
}

#[test]
fn part1() {
    let (mut map, dirs) = input();

    let mut pos = Vec2i32::default();
    for (p, c) in map.enumerate() {
        if *c == b'@' {
            pos = p.cast();
            break;
        }
    }

    for dir in dirs {
        let dir = dir.unit_vec();

        let cand = pos + dir;
        match map[cand] {
            b'#' => (),
            b'.' => {
                map[cand] = b'@';
                map[pos] = b'.';
                pos = cand;
            }
            b'O' => {
                let mut check = cand;
                loop {
                    check += dir;
                    match map[check] {
                        b'O' => (),
                        b'#' => break,
                        b'.' => {
                            map[check] = b'O';
                            map[cand] = b'@';
                            map[pos] = b'.';
                            pos = cand;
                            break;
                        }
                        _ => panic!(),
                    }
                }
            }
            _ => panic!(),
        }
    }

    let mut total = 0;
    for (p, c) in map.enumerate() {
        if *c == b'O' {
            total += 100 * p.y + p.x;
        }
    }

    assert_eq!(total, 1538871);
}

#[test]
fn part2() {
    let (map, dirs) = input();
    let mut pos = Vec2i32::default();

    let mut modified: Vec<Vec<u8>> = Vec::with_capacity(map.height());
    for (j, row) in map.rows().enumerate() {
        let mut new_row: Vec<u8> = Vec::with_capacity(row.len() * 2);
        for (i, b) in row.iter().enumerate() {
            match b {
                b'#' => new_row.extend_from_slice(b"##"),
                b'O' => new_row.extend_from_slice(b"[]"),
                b'.' => new_row.extend_from_slice(b".."),
                b'@' => { 
                    new_row.extend_from_slice(b"@.");
                    pos = Vec2i32::new(i as i32 * 2, j as i32);
                }
                _ => panic!(),
            }
        }
        modified.push(new_row);
    }
    let mut map = Grid::new(modified);

    for dir in dirs {
        let unit = dir.unit_vec();
        let cand = pos + unit;

        match map[cand] {
            b'#' => (),
            b'.' => {
                map[cand] = b'@';
                map[pos] = b'.';
                pos = cand;
            }
            c@ b'[' | c @ b']' => {
                match dir {
                    U | D => {
                        fn can_move_up_or_down(pos: Vec2i32, dir: Vec2i32, map: &Grid<u8>) -> bool {
                            let next_left = pos + dir;
                            let c_left = map[next_left];
                            let c_right = map[next_left.east_of()];

                            if c_left == b'#' || c_right == b'#' {
                                return false;
                            }

                            if c_left == b'[' {
                                return can_move_up_or_down(next_left, dir, map);
                            }

                            let can_left = if c_left == b']' {
                                can_move_up_or_down(next_left - Vec2i32::unit_x(), dir, map)
                            } else if c_left == b'.' {
                                true
                            } else {
                                panic!();
                            };

                            can_left && if c_right == b'[' {
                                can_move_up_or_down(next_left + Vec2i32::unit_x(), dir, map)
                            } else if c_right == b'.' {
                                true
                            } else {
                                panic!()
                            }
                        }

                        fn move_up_or_down(pos: Vec2i32, dir: Vec2i32, map: &mut Grid<u8>) {
                            let next_left = pos + dir;
                            let c_left = map[next_left];
                            let c_right = map[next_left.east_of()];

                            if c_left == b'[' {
                                move_up_or_down(next_left, dir, map);
                            }
                            else {
                                if c_left == b']' {
                                    move_up_or_down(next_left.west_of(), dir, map);
                                }
                                if c_right == b'[' {
                                    move_up_or_down(next_left.east_of(), dir, map);
                                }
                            }

                            map[next_left] = b'[';
                            map[next_left.east_of()] = b']';
                            map[pos] = b'.';
                            map[pos.east_of()] = b'.';
                        }

                        let mut box_pos = cand;
                        if c == b']' { box_pos.x -= 1; }
                        if can_move_up_or_down(box_pos, unit, &map) {
                            move_up_or_down(box_pos, unit, &mut map);
                            map[cand] = b'@';
                            map[pos] = b'.';
                            pos = cand;
                        }
                    }
                    L | R => {
                        let mut can_move = false;
                        let mut current = cand;
                        loop {
                            current += unit;
                            match map[current] {
                                b'.' => {
                                    can_move = true;
                                    break;
                                }
                                b'#' => break,
                                b'[' | b']' => (),
                                _ => panic!()
                            }
                        }

                        if can_move {
                            loop {
                                if current == pos {
                                    break;
                                }
                                map[current] = map[((current.x - unit.x), current.y)];
                                current -= unit;
                            }

                            map[pos] = b'.';
                            pos = cand;
                        }
                    }
                }
            }
            _ => panic!()
        }
    }

    let mut total = 0;
    for (p, c) in map.enumerate() {
        if *c == b'[' {
            total += 100 * p.y + p.x;
        }
    }

    assert_eq!(total, 1543338);
}