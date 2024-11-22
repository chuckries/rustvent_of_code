use aoc_common::*;

fn input() -> String {
    file_string("inputs/day01.txt")
}

#[test]
fn part1() {
    let answer: i32 = input().bytes().map(|c| {
        match c {
            b'(' => 1,
            b')' => -1,
            _ => panic!()
        }
    }).sum();

    assert_eq!(answer, 138);
}

#[test]
fn part2() {
    let mut floor = 0;
    let mut answer = 0;
    for (idx, c) in input().bytes().enumerate() {
        floor += match c {
            b'(' => 1,
            b')' => -1,
            _ => panic!(),
        };

        if floor < 0 {
            answer = idx + 1;
            break;
        }
    }

    assert_eq!(answer, 1771);
}