use aoc_common::{file_lines, Vec2us};
use regex::Regex;

fn input() -> Vec<(Vec2us, char, String)> {
    let regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    file_lines("inputs/day02.txt").map(|l| {
        let caps = regex.captures(&l).unwrap();
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let c = caps[3].chars().next().unwrap();
        let s = caps[4].to_string();

        ((x, y).into(), c, s)
    }).collect()
}

#[test]
fn part1() {
    let answer = input().into_iter().filter(|(r, c, s)| {
        let count = s.chars().filter(|cc| cc == c).count();
        count >= r.x && count <= r.y
    }).count();

    assert_eq!(640, answer);
}

#[test]
fn part2() {
    let answer = input().into_iter().filter(|(r, c, s)| {
        let bytes = s.as_bytes();
        let a = bytes[r.x - 1];
        let b = bytes[r.y - 1];
        let c = *c as u8;
        (a == c) ^ (b == c)
    }).count();

    assert_eq!(472, answer);
}