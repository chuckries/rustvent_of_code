use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Vec<i64>> {
    file_lines("inputs/day09.txt").map(|l| {
        l.split_whitespace().map(|s| s.parse().unwrap()).to_vec()
    }).to_vec()
}

fn extrapolate<S, O>(seed: Vec<i64>, selector: &S, operation: &O) -> i64 
    where S: Fn(&[i64]) -> i64, O: Fn(i64, i64) -> i64
{
    let mut stack: Vec<i64> = Vec::new();
    let mut current = seed;
    loop {
        stack.push(selector(&current));
        let next = current.windows(2).map(|w| w[1] - w[0]).to_vec();
        if next.iter().all(|n| *n == 0) {
            break;
        }
        current = next;
    }

    let mut accum = 0;
    while let Some(n) = stack.pop() {
        accum = operation(accum, n);
    }

    accum
}

fn run<S, O>(selector: S, operation: O) -> i64 
    where S: Fn(&[i64]) -> i64, O: Fn(i64, i64) -> i64
{
    input().into_iter().map(|seed| {
        extrapolate(seed, &selector, &operation)
    }).sum()
}

#[test]
fn part1() {
    let answer = run(|seq| *seq.last().unwrap(), |acc, x| acc + x);
    assert_eq!(1789635132, answer);
}

#[test]
fn part2() {
    let answer = run(|seq| seq[0], |acc, x| x - acc);
    assert_eq!(913, answer);
}