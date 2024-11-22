use aoc_common::full_permutations;

use intcode::{IntCode, IntCodeResult};

fn run_once(phases: &[&i64]) -> i64 {
    let mut computers = vec![IntCode::from_file("inputs/day07.txt"); 5];
    for pair in computers.iter_mut().zip(phases) {
        pair.0.push_input_back(**pair.1);
    }

    let mut result = 0;
    for c in computers.iter_mut() {
        result = c.run_input(&[result]).unwrap();
    }

    result
}

fn run_multiple(phases: &[&i64]) -> i64 {
    let mut computers = vec![IntCode::from_file("inputs/day07.txt"); 5];
    for pair in computers.iter_mut().zip(phases) {
        pair.0.push_input_back(**pair.1);
    }

    let mut result = 0;
    while computers.iter().any(|c| !c.is_halt()) {
        for c in computers.iter_mut() {
            if let IntCodeResult::Output(o) = c.run_input(&[result]) {
                result = o;
            }
        }
    }

    result
}

#[test]
fn part1() {
    let permutations = full_permutations(&[0, 1, 2, 3, 4]);

    let mut max = 0;
    for current in permutations {
        let answer = run_once(&current);
        if answer > max {
            max = answer;
        }
    }

    assert_eq!(max, 17440);
}

#[test]
fn part2() {
    let permutations = full_permutations(&[5, 6, 7, 8, 9]);

    let mut max = 0;
    for current in permutations {
        let answer = run_multiple(&current);
        if answer > max {
            max = answer;
        }
    }

    assert_eq!(max, 27561242);
}