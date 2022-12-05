use std::collections::VecDeque;

use aoc_common::{file_lines, IteratorExt};


fn input() -> (Vec<Vec<char>>, impl Iterator<Item = (usize, usize, usize)>)
{
    let mut lines = file_lines("inputs/day05.txt");

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    while let Some(line) = lines.next() {
        let chars = line.chars().to_vec();
        if chars[1] == '1' { break; }

        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '[' {
                let stack_idx = i / 4;
                while stack_idx >= stacks.len() {
                    stacks.push(VecDeque::new());
                }
                stacks[stack_idx].push_front(chars[i + 1]);
            }
            i += 4;
        }
    }

    lines.next();

    let instructions = lines.into_iter().map(|l| {
        let split = l.split(' ').to_vec();

        (split[1].parse().unwrap(), split[3].parse().unwrap(), split[5].parse().unwrap())
    });

    (stacks.into_iter().map(|v| v.into_iter().collect()).collect(), instructions)

}

fn run<F>(mut f: F) -> String
    where F: FnMut(usize, usize, usize, &mut Vec<Vec<char>>)
{
    let (mut stacks, instructions) = input();

    for (count, from, to) in instructions {
        f(count, to, from, &mut stacks);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[test]
fn part1() {
    let answer = run(|count, to, from, stacks| {
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    });

    assert_eq!(answer, "VQZNJMWTR");
}

#[test]
fn part2() {
    let mut tmp: Vec<char> = Vec::new();

    let answer = run(|count, to, from, stacks| {
        for _ in 0..count {
            tmp.push(stacks[from - 1].pop().unwrap());
        }

        while let Some(c) = tmp.pop() {
            stacks[to - 1].push(c);
        }
    });

    assert_eq!(answer, "NLCDCLVMQ");
}