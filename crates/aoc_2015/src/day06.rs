use aoc_common::{file_lines, IteratorExt, Vec2us};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    On,
    Off,
    Toggle,
}

struct Instruction {
    operation: Operation,
    p0: Vec2us,
    p1: Vec2us,
}

const SPLIT_CHARS: [char; 2] = [' ', ','];

fn input() -> Vec<Instruction> {
    file_lines("inputs/day06.txt").map(|s| {
        let split = s.split(SPLIT_CHARS).to_vec();

        let p0_idx;
        let p1_idx;
        let operation: Operation;

        if split[0] == "toggle" {
            operation = Operation::Toggle;
            p0_idx = 1;
            p1_idx = 4;
        } else {
            if split[1] == "on" {
                operation = Operation::On
            } else {
                operation = Operation::Off
            }
            p0_idx = 2;
            p1_idx = 5;
        }

        Instruction {
            operation,
            p0: Vec2us::new(split[p0_idx].parse().unwrap(), split[p0_idx + 1].parse().unwrap()),
            p1: Vec2us::new(split[p1_idx].parse().unwrap(), split[p1_idx + 1].parse().unwrap()),
        }

    }).to_vec()
}

#[test]
fn part1() {
    let mut map = vec![vec![false; 1000]; 1000];

    fn set_true(b: &mut bool) { *b = true; }
    fn set_false(b: &mut bool) { *b = false; }
    fn toggle(b: &mut bool) { *b = !*b; }

    for instruction in input() {
        let f = match instruction.operation {
            Operation::On => set_true,
            Operation::Off => set_false,
            Operation::Toggle => toggle,
        };

        for i in instruction.p0.x ..= instruction.p1.x {
            for j in instruction.p0.y ..= instruction.p1.y {
                f(&mut map[i][j]);
            }
        }
    }

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] {
                total += 1;
            }
        }
    }

    assert_eq!(total, 377891);
}

#[test]
fn part2() {
    let mut map = vec![vec![0; 1000]; 1000];

    for instruction in input() {
        let add = match instruction.operation {
            Operation::On => 1,
            Operation::Off => -1,
            Operation::Toggle => 2,
        };

        for i in instruction.p0.x ..= instruction.p1.x {
            for j in instruction.p0.y ..= instruction.p1.y {
                let cell = &mut map[i][j];
                *cell += add;
                if *cell < 0 {
                    *cell  = 0;
                }
            }
        }
    }

    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            total +=  map[i][j];
        }
    }

    assert_eq!(total, 14110788);
}