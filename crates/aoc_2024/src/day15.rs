use aoc_common::{file_lines, IteratorExt, Vec2i32};

use Dir::*;

enum Dir {
    U,
    D,
    L,
    R,
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
        let dir = match dir {
            U => -Vec2i32::unit_y(),
            D => Vec2i32::unit_y(),
            L => -Vec2i32::unit_x(),
            R => Vec2i32::unit_x(),
        };

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

    assert_eq!(total, 0);
}