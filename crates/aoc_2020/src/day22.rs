use std::collections::{VecDeque, HashSet};

use aoc_common::{file_lines};

fn input() -> (VecDeque<i32>, VecDeque<i32>) {
    let mut lines = file_lines("inputs/day22.txt");

    let mut first: VecDeque<i32> = VecDeque::new();
    let mut second: VecDeque<i32> = VecDeque::new();

    lines.next().unwrap();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        first.push_back(line.parse().unwrap());
    }

    lines.next().unwrap();
    while let Some(line) = lines.next() {
        second.push_back(line.parse().unwrap());
    }

    (first, second)
}

#[test]
fn part1() {
    let (mut one, mut two) = input();

    while !one.is_empty() && !two.is_empty() {
        let (a, b) = (one.pop_front().unwrap(), two.pop_front().unwrap());

        if a > b {
            one.push_back(a);
            one.push_back(b);
        } else {
            two.push_back(b);
            two.push_back(a);
        }
    }

    let winner = if one.is_empty() { &two } else { &one };

    let answer = winner.iter().rev().enumerate().map(|(idx, n)| (idx + 1) as i32 * *n).sum::<i32>();
    assert_eq!(answer, 33559);
}

fn recursive_combate(one: &mut VecDeque<i32>, two: &mut VecDeque<i32>) -> bool {
    let mut states: HashSet<(VecDeque<i32>, VecDeque<i32>)> = HashSet::new();

    while !one.is_empty() && !two.is_empty() {
        let state = (one.clone(), two.clone());
        if states.contains(&state) {
            return false;
        }
        states.insert(state.clone());


        let a = one.pop_front().unwrap();
        let b = two.pop_front().unwrap();

        let winner = if a as usize <= one.len() && b as usize <= two.len() {
            let mut one = one.iter().take(a as usize).copied().collect();
            let mut two = two.iter().take(b as usize).copied().collect();

            recursive_combate(&mut one, &mut two)
        } else {
            a < b
        };

        if !winner {
            one.push_back(a);
            one.push_back(b);
        } else {
            two.push_back(b);
            two.push_back(a);
        }
    }

    one.is_empty()
}

#[test]
fn part2() {
    let (mut one, mut two) = input();

    let winner = if recursive_combate(&mut one, &mut two) {
        &two
    } else {
        &one
    };

    let answer = winner.iter().rev().enumerate().map(|(idx, n)| (idx + 1) as i32 * *n).sum::<i32>();
    assert_eq!(answer, 32789);
}