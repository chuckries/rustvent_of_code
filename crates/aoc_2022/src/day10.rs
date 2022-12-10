use std::collections::HashSet;
use aoc_common::{file_lines, IteratorExt, Vec2i32, map_points_to_string};
use Instr::*;

enum Instr {
    Add(i32),
    Nop,
}

fn input() -> impl Iterator<Item = Instr> {
    file_lines("inputs/day10.txt").map(|l| {
        let split = l.split(' ').to_vec();
        match split[0] {
            "addx" => Add(split[1].parse().unwrap()),
            "noop" => Nop,
            _ => panic!()
        }
    })
}

fn run<F>(mut f: F)
    where F: FnMut(i32, i32)
{
    let mut reg_x = 1;
    let mut ticks = 1;

    for instr in input() {
        f(ticks, reg_x);
        ticks += 1;

        match instr {
            Add(x) => {
                f(ticks, reg_x);
                ticks += 1;
                reg_x += x;
            }
            Nop => (),
        }
    }
}

#[test]
fn part1() {
    let checks = vec![20, 60, 100, 140, 180, 220];
    let mut idx = 0;
    let mut answer = 0;

    run(|ticks, x| {
        if idx < checks.len() {
            if checks[idx] == ticks {
                answer += ticks * x;
                idx += 1;
            }
        }
    });

    assert_eq!(answer, 14160);
}

#[test]
fn part2() {
    let mut x = 0;
    let mut y = 0;
    let mut screen: HashSet<Vec2i32> = HashSet::new();

    run(|_, reg_x| {
        if x >= reg_x - 1 && x <= reg_x + 1 {
            screen.insert((x, y).into());
        }

        x += 1;
        if x == 40 {
            x = 0;
            y += 1;
            if y == 6 {
                y = 0;
            }
        }
    });

    let answer = map_points_to_string(screen.iter().copied());

    let known = "
███    ██ ████ ███  ███  ████ ████  ██ 
█  █    █ █    █  █ █  █ █    █    █  █
█  █    █ ███  █  █ █  █ ███  ███  █   
███     █ █    ███  ███  █    █    █   
█ █  █  █ █    █ █  █    █    █    █  █
█  █  ██  ████ █  █ █    ████ █     ██ ";

    assert_eq!(answer, known);
}