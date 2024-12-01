use aoc_common::file_lines_as;

fn input() -> Vec<i32> {
    file_lines_as("inputs/day05.txt").collect()
}

#[test]
fn part1() {
    let mut instructions = input();

    let mut idx = 0;
    let mut count = 0;

    loop {
        count += 1;
        let delta = instructions[idx];
        instructions[idx] += 1;

        let next = idx as i32 + delta;
        if next < 0 || next >= instructions.len() as i32 {
            break;
        }
        idx = next as usize;
    }
    
    assert_eq!(count, 391540);
}

#[test]
fn part2() {
    let mut instructions = input();

    let mut idx = 0;
    let mut count = 0;

    loop {
        count += 1;
        let delta = instructions[idx];
        if delta < 3 {
            instructions[idx] += 1;
        } else {
            instructions[idx] -= 1;
        }

        let next = idx as i32 + delta;
        if next < 0 || next >= instructions.len() as i32 {
            break;
        }
        idx = next as usize;
    }
    
    assert_eq!(count, 30513679);
}