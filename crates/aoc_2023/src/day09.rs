use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Vec<i64>> {
    file_lines("inputs/day09.txt").map(|l| {
        l.split_whitespace().map(|s| s.parse().unwrap()).to_vec()
    }).to_vec()
}

fn extrapolate(seed: Vec<i64>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    let mut current = seed;
    loop {
        stack.push(current[current.len() - 1]);
        let next = current.windows(2).map(|w| w[1] - w[0]).to_vec();
        if next.iter().all(|n| *n == 0) {
            break;
        }
        current = next;
    }

    stack.iter().sum()
}

#[test]
fn part1() {
    let answer: i64 = input().into_iter().map(|seed| {
        extrapolate(seed)
    }).sum();

    assert_eq!(1789635132, answer);
}

#[test]
fn part2() {
    let answer: i64 = input().into_iter().map(|mut seed| {
        seed.reverse();
        extrapolate(seed)
    }).sum();

    assert_eq!(913, answer);
}