use aoc_common::{file_lines, IteratorExt};

fn input<F, T>(f: F) -> (T, T)
    where F: Fn(&[&str]) -> T
{
    let lines = file_lines("inputs/day06.txt");
    let mut items = lines.map(|l| {
        let numbers = l.split_whitespace().skip(1).to_vec();
        f(&numbers)
    });

    (items.next().unwrap(), items.next().unwrap())
}

fn solve_bounds(time: i64, distance: i64) -> i64 {
    let sqrt = f64::sqrt((time * time - 4 * distance) as f64);

    let low = f64::ceil((time as f64 - sqrt) / 2.0) as i64;
    let high = f64::floor((time as f64 + sqrt) / 2.0) as i64;

    high - low + 1
}

#[test]
fn part1() {
    let (times, distances) = input(|nums| {
        nums.iter().map(|n| n.parse::<i64>().unwrap()).to_vec()
    });

    let answer: i64 = times.into_iter().zip(distances).map(|(t, d)| solve_bounds(t, d)).product();

    assert_eq!(741000, answer);
}

#[test]
fn part2() {
    let (time, distance) = input(|nums| {
        nums.join("").parse::<i64>().unwrap()
    });

    let answer = solve_bounds(time, distance);

    assert_eq!(38220708, answer);
}