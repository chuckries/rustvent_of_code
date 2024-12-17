
use aoc_common::{file_lines, IteratorExt, Vec2i32};
use console::{Key, Style, Term};
use std::io::Write;

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
    let mut lines = file_lines(r"crates\aoc_2024\inputs\day15.txt");

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

fn main() -> std::io::Result<()> {
    let mut term = Term::buffered_stdout();

    term.hide_cursor()?;

    let (map, dirs) = input();
    let mut pos = Vec2i32::default();
    let mut modified: Vec<Vec<u8>> = Vec::with_capacity(map.len());
    for (j, row) in map.iter().enumerate() {
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
    let mut map = modified;

    term.clear_screen()?;
    for row in map.iter() {
        for c in row.iter() {
            write!(term, "{}", *c as char)?;
        }
        writeln!(term)?;
    }
    term.flush()?;

    loop {
        let key = term.read_key()?;
        let dir = match key {
            Key::ArrowUp => U,
            Key::ArrowDown => D,
            Key::ArrowLeft => L,
            Key::ArrowRight => R,
            _ => continue
        };

        let unit = dir.unit_vec();
        let cand = pos + unit;

        match map[cand.y as usize][cand.x as usize] {
            b'#' => continue,
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
                        } else {
                            continue;
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
                        } else {
                            continue;
                        }
                    }
                }
            }
            _ => panic!()
        }

        term.clear_screen()?;
        for row in map.iter() {
            for c in row.iter() {
                let style = match *c {
                    b'.' => Style::new().dim(),
                    b'@' => Style::new().cyan().bold(),
                    b'#' => Style::new().green(),
                    _ => Style::new().black().on_white(),
                };
                write!(term, "{}", style.apply_to(*c as char))?;
            }
            writeln!(term)?;
        }
        term.flush()?;
    }

    Ok(())
}