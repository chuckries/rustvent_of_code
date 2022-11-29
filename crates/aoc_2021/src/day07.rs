use aoc_common::file_string;

fn input() -> (Vec<i32>, i32, i32) {
    let input: Vec<i32> = file_string("inputs/day07.txt").split(',').map(|s| s.parse().unwrap()).collect();

    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in input.iter() {
        if *i < min {
            min = *i;
        }
        if *i > max {
            max = *i;
        }
    }
    (input, min, max)
}

fn fuel_const(pos: i32, crabs: &[i32]) -> i32 {
    crabs.iter().map(|c| i32::abs(*c - pos)).sum()
}

fn fuel_variable(pos: i32, crabs: &[i32]) -> i32 {
    crabs.iter().map(|c| {
        let dist = i32::abs(*c - pos);
        (dist * (dist + 1)) / 2
    }).sum()
}

fn search(fuel: fn(i32, &[i32]) -> i32) -> i32 {
    let (input, min, max) = input();
    let mut min_fuel = i32::MAX;

    for i in min..max {
        let cost = fuel(i, &input);
        if cost < min_fuel {
            min_fuel = cost;
        }
    }

    min_fuel
}

#[test]
fn part1() {
    assert_eq!(search(fuel_const), 329389);
}

#[test]
fn part2() {
    assert_eq!(search(fuel_variable), 86397080);
}