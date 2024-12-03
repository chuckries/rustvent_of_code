use aoc_common::file_string;
use lazy_static::lazy_static;
use regex::Regex;

fn input() -> String {
    file_string("inputs/day03.txt")
}

fn run(enable_override: bool) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }

    let input = input();
    let mut total = 0;
    let mut on = true;
    for capture in RE.captures_iter(&input) {
        match &capture[0] {
            "do()" => on = true,
            "don't()" => on = false,
            _ => {
                if enable_override || on  {
                    total += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap()
                }
            }
        }
    }

    total
}

#[test]
fn part1() {
    let answer = run(true);
    assert_eq!(answer, 189600467);
}

#[test]
fn part2() {
    let answer = run(false);
    assert_eq!(answer, 107069718);
}