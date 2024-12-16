use aoc_common::{file_lines, IteratorExt, Vec2i32};

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

fn input() -> (Vec<Vec<u8>>, Vec<Dir>) {
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

    (map, dirs)
}

#[test]
fn part1() {
    let (mut map, dirs) = input();

    let mut pos = Vec2i32::default();
    'outer: for j in 0.. map.len() {
        for i in 0..map[0].len() {
            if map[j][i] == b'@' {
                pos = Vec2i32::new(i as i32, j as i32);
                break 'outer;
            }
        }
    }

    let mut total = 0;
    for dir in dirs {
        let dir = dir.unit_vec();

        let cand = pos + dir;
        match map[cand.y as usize][cand.x as usize] {
            b'#' => (),
            b'.' => {
                map[cand.y as usize][cand.x as usize] = b'@';
                map[pos.y as usize][pos.x as usize] = b'.';
                pos = cand;
            }
            b'O' => {
                let mut check = cand;
                loop {
                    check += dir;
                    match map[check.y as usize][check.x as usize] {
                        b'O' => (),
                        b'#' => break,
                        b'.' => {
                            map[check.y as usize][check.x as usize] = b'O';
                            map[cand.y as usize][cand.x as usize] = b'@';
                            map[pos.y as usize][pos.x as usize] = b'.';
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

    total = 0;
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if map[j as usize][i as usize] == b'O' {
                total += 100 * j + i;
            }
            //print!("{}", map[j as usize][i as usize] as char);
        }
        //println!();
    }
    //println!();

    assert_eq!(total, 1538871);
}

#[test]
fn part2() {
    let (map, dirs) = input();
    let mut modified: Vec<Vec<u8>> = Vec::with_capacity(map.len());
    for row in map {
        let mut new_row: Vec<u8> = Vec::with_capacity(row.len() * 2);
        for b in row {
            match b {
                b'#' => new_row.extend_from_slice(b"##"),
                b'O' => new_row.extend_from_slice(b"[]"),
                b'.' => new_row.extend_from_slice(b".."),
                b'@' => new_row.extend_from_slice(b"@."),
                _ => panic!(),
            }
        }
        modified.push(new_row);
    }
    let mut map = modified;

    let mut pos = Vec2i32::default();
    'outer: for j in 0.. map.len() {
        for i in 0..map[0].len() {
            if map[j][i] == b'@' {
                pos = Vec2i32::new(i as i32, j as i32);
                break 'outer;
            }
        }
    }

    for dir in dirs {
        let unit = dir.unit_vec();
        let cand = pos + unit;

        match map[cand.y as usize][cand.x as usize] {
            b'#' => (),
            b'.' => {
                map[cand.y as usize][cand.x as usize] = b'@';
                map[pos.y as usize][pos.x as usize] = b'.';
                pos = cand;
            }
            c@ b'[' | c @ b']' => {
                match dir {
                    U | D => {
                        fn can_move_up_or_down(pos: Vec2i32, dir: Vec2i32, map: &Vec<Vec<u8>>) -> bool {
                            let next_left = pos + dir;
                            let c_left = map[next_left.y as usize][next_left.x as usize];
                            let c_right = map[next_left.y as usize][next_left.x as usize + 1];

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

                        fn move_up_or_down(pos: Vec2i32, dir: Vec2i32, map: &mut Vec<Vec<u8>>) {
                            let next_left = pos + dir;
                            let c_left = map[next_left.y as usize][next_left.x as usize];
                            let c_right = map[next_left.y as usize][next_left.x as usize + 1];

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

                            map[next_left.y as usize][next_left.x as usize] = b'[';
                            map[next_left.y as usize][next_left.x as usize + 1] = b']';
                            map[pos.y as usize][pos.x as usize] = b'.';
                            map[pos.y as usize][pos.x as usize + 1] = b'.';
                        }

                        let mut box_pos = cand;
                        if c == b']' { box_pos.x -= 1; }
                        if can_move_up_or_down(box_pos, unit, &map) {
                            move_up_or_down(box_pos, unit, &mut map);
                            map[cand.y as usize][cand.x as usize] = b'@';
                            map[pos.y as usize][pos.x as usize] = b'.';
                            pos = cand;
                        }
                    }
                    L | R => {
                        let mut can_move = false;
                        let mut current = cand;
                        loop {
                            current += unit;
                            match map[current.y as usize][current.x as usize] {
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
                                map[current.y as usize][current.x as usize] = map[current.y as usize][(current.x - unit.x) as usize];
                                current -= unit;
                            }

                            map[pos.y as usize][pos.x as usize] = b'.';
                            pos = cand;
                        }
                    }
                }
            }
            _ => panic!()
        }

        // for row in map.iter() {
        //     for c in row.iter() {
        //         print!("{}", *c as char)
        //     }
        //     println!();
        // }
        // println!();
    }

    let mut total = 0;
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if map[j][i] == b'[' {
                total += 100 * j + i;
            }
        }
    }

    assert_eq!(total, 1543338);
}