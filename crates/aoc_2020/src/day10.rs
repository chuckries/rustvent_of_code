use aoc_common::{file_lines_as, IteratorExt};


fn input() -> Vec<i32> {
    file_lines_as("inputs/day10.txt").sorted().collect()
}

#[test]
fn part1() {
    let input = input();

    let mut ones = 0;
    let mut threes = 1;

    let mut current = 0;

    for i in input {
        match i - current {
            1 => ones += 1,
            3 => threes += 1,
            _ => ()
        };

        current = i;
    }

    let answer = ones * threes;
    assert_eq!(answer, 1998);
}

#[test]
fn part2() {
    let input = input();

    let mut counts: Vec<u64> = vec![0; input.len()];
    let mut i = 0;
    while input[i] <= 3 {
        counts[i] = 1;
        i += 1;
    }

    for i in 0..input.len() - 1 {
        let mut j = i + 1; 
        while j < input.len() && input[j] - input[i] <= 3 {
            counts[j] += counts[i];
            j += 1;
        }
    }

    let answer = *counts.last().unwrap();
    assert_eq!(answer, 347250213298688);
}