use aoc_common::{file_lines, IteratorExt};

use Op::*;

#[derive(Clone, Copy)]
enum Op {
    SwpPos(usize, usize),
    SwpLtr(u8, u8),
    Rot(i32),
    RotLtr(u8),
    Rev(usize, usize),
    Mov(usize, usize),
}

fn input() -> Vec<Op> {
    file_lines("inputs/day21.txt").map(|l| {
        let split = l.split(' ').to_vec();
        match split[0] {
            "swap" => match split[1] {
                "position" => SwpPos(split[2].parse().unwrap(), split[5].parse().unwrap()),
                "letter" => SwpLtr(split[2].as_bytes()[0], split[5].as_bytes()[0]),
                _ => panic!(),
            },
            "rotate" => match split[1] {
                "right" => Rot(split[2].parse().unwrap()),
                "left" => Rot(-split[2].parse::<i32>().unwrap()),
                "based" => RotLtr(split[6].as_bytes()[0]),
                _ => panic!(),
            },
            "reverse" => Rev(split[2].parse().unwrap(), split[4].parse().unwrap()),
            "move" => Mov(split[2].parse().unwrap(), split[5].parse().unwrap()),
            _ => panic!(),
        }
    }).collect()
}

fn swap_letters(bytes: &mut [u8], a: u8, b: u8) {
    let mut idx = 0;
    let mut indices = [0; 2];
    for (i, c) in bytes.iter().enumerate() {
        if *c == a || *c == b {
            indices[idx] = i;
            idx += 1;
            if idx == 2 {
                break;
            }
        }
    }
    bytes.swap(indices[0], indices[1]);
}

fn rotate(bytes: &mut[u8], rot: i32) {
    if rot > 0 {
        bytes.rotate_right(rot as usize);
    } else {
        bytes.rotate_left((-rot) as usize);
    }
}

fn reverse(bytes: &mut [u8], mut start: usize, mut end: usize) {
    if end < start {
        (start, end) = (end, start);
    }
    while start < end {
        bytes.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn mov(bytes: &mut Vec<u8>, src: usize, dst: usize) {
    let c = bytes.remove(src);
    bytes.insert(dst, c);
}

fn scramble(s: &str, ops: &[Op]) -> String {
    let mut bytes = s.as_bytes().to_vec();

    for op in ops.iter().cloned() {
        match op {
            SwpPos(a, b) => bytes.swap(a, b),
            SwpLtr(a, b) => swap_letters(&mut bytes, a, b),
            Rot(rot) => rotate(&mut bytes, rot),
            RotLtr(rot) => {
                let mut pos = bytes.iter().position(|b| *b == rot).unwrap();
                if pos >= 4 {
                    pos += 1;
                }
                pos += 1;
                pos %= bytes.len();
                bytes.rotate_right(pos);
            }
            Rev(start, end) => reverse(&mut bytes, start, end),
            Mov(a, b) => mov(&mut bytes, a, b)
        }
    }

    String::from_utf8(bytes).unwrap()
}

fn unscramble(s: &str, ops: &[Op]) -> String {
    let mut bytes = s.as_bytes().to_vec();

    for op in ops.iter().rev().cloned() {
        match op {
            SwpPos(a, b) => bytes.swap(a, b),
            SwpLtr(a, b) => swap_letters(&mut bytes, a, b),
            Rot(rot) => rotate(&mut bytes, -rot),
            RotLtr(rot) => {
                let mut idx = bytes.iter().position(|c| *c == rot).unwrap();
                let mut rotations = 0;
                loop {
                    let mut cand_rot = idx;
                    if cand_rot >= 4 {
                        cand_rot += 1;
                    }
                    cand_rot += 1;
                    cand_rot %= bytes.len();
                    if cand_rot == rotations { break; }

                    if idx == 0 {
                        idx = bytes.len() - 1;
                    } else {
                        idx -= 1;
                    }
                    rotations += 1;
                }
                bytes.rotate_left(rotations);
            }
            Rev (start, end) => reverse(&mut bytes, start, end),
            Mov(a, b) => mov(&mut bytes, b, a),
        }
    }

    String::from_utf8(bytes).unwrap()
}

#[test]
fn part1() {
    let input = input();
    let answer = scramble("abcdefgh", &input);
    assert_eq!(answer, "baecdfgh");
}

#[test]
fn part2() {
    let input = input();
    let answer = unscramble("fbgdceah", &input);
    assert_eq!(answer, "cegdahbf");
}