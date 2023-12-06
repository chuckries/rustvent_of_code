use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Vec<i64>> {
    file_lines("inputs/day06.txt").map(|l| {
        l.split_whitespace().skip(1).map(|n| n.parse().unwrap()).to_vec()
    }).to_vec()
}

fn solve_bounds(time: i64, distance: i64) -> (i64, i64) {
    let sqrt = f64::sqrt((time * time - 4 * distance) as f64);

    let low = f64::ceil((time as f64 - sqrt) / 2.0) as i64;
    let high = f64::floor((time as f64 + sqrt) / 2.0) as i64;
    (low, high)
}

#[test]
fn part1() {
    let input = input();
    let times = &input[0];
    let distances = &input[1];

    let mut total = 1;
    for (t, d) in times.iter().zip(distances) {
        let (low, high) = solve_bounds(*t, *d);
        total *= high - low + 1;
    }

    assert_eq!(741000, total);
}

#[test]
fn part2() {
    let input = input();
    let input = input.iter().map(|v| v.iter().map(|n| n.to_string()).to_vec().join("").parse::<i64>().unwrap()).to_vec();

    let (low, high) = solve_bounds(input[0], input[1]);
    let answer = high - low + 1;

    assert_eq!(38220708, answer);
}