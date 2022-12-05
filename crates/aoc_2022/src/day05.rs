use aoc_common::{file_lines, IteratorExt};


fn input() -> (Vec<Vec<char>>, impl Iterator<Item = (usize, usize, usize)>)
{
    let mut lines = file_lines("inputs/day05.txt");

    let mut stacks: Vec<Vec<char>> = Vec::new();

    while let Some(line) = lines.next() {
        let chars = line.chars().to_vec();
        if chars[1] == '1' { break; }

        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '[' {
                let stack_idx = i / 4;
                while stack_idx >= stacks.len() {
                    stacks.push(Vec::new());
                }
                stacks[stack_idx].push(chars[i + 1]);
            }
            i += 4;
        }
    }

    for s in stacks.iter_mut() {
        s.reverse();
    }

    lines.next();

    let instructions = lines.into_iter().map(|l| {
        let split = l.split(' ').to_vec();

        (split[1].parse().unwrap(), split[3].parse().unwrap(), split[5].parse().unwrap())
    });

    (stacks, instructions)

}

#[test]
fn part1() {
    let (mut stacks, instructions) = input();

    for (count, from, to) in instructions {
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }

    let answer: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    assert_eq!(answer, "");
}

#[test]
fn part2() {
    let (mut stacks, instructions) = input();
    let mut tmp: Vec<char> = Vec::new();

    for (count, from, to) in instructions {
        for _ in 0..count {
            tmp.push(stacks[from - 1].pop().unwrap());
        }

        while let Some(c) = tmp.pop() {
            stacks[to - 1].push(c);
        }
    }

    let answer: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    assert_eq!(answer, "");
}