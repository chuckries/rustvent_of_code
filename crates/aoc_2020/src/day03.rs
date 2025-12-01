use aoc_common::file_string;
use regex::Regex;

fn input() -> String {
    file_string("inputs/day03.txt")
}

#[test]
fn part1() {
    let input = input();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let answer = regex.captures_iter(&input).map(|cap| {
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();
        a * b
    }).sum::<i32>();

    assert_eq!(189600467, answer)
}

#[test]
fn part2() {
    let input = input();
    let regex = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    
    let mut enable = true;
    let mut total = 0;
    for cap in regex.captures_iter(&input) {
        if &cap[0] == "do()" {
            enable = true;
        } else if &cap[0] == "don't()" {
            enable = false;
        } else {
            if enable {
                let a: i32 = cap[1].parse().unwrap();
                let b: i32 = cap[2].parse().unwrap();

                total += a * b;
            }
        }
    };

    assert_eq!(107069718, total);
}