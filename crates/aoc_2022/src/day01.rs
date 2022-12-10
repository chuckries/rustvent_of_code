use aoc_common::{file_lines, IteratorExt};

fn input() -> impl Iterator<Item = impl Iterator<Item = i32>> {
    let mut lines = file_lines("inputs/day01.txt");
    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            elves.push(current);
            current = Vec::new();
        } else {
            current.push(line.parse().unwrap());
        }
    }
    elves.push(current);

    elves.into_iter().map(|v| v.into_iter())
}

#[test]
fn part1() {
    let answer: i32 = input().map(|v| v.sum()).max().unwrap();
    assert_eq!(answer, 66616);
}

#[test]
fn part2() {
    let answer: i32 = input().map(|v| v.sum::<i32>()).sorted_by(|a, b| b.cmp(a)).take(3).sum();
    assert_eq!(answer, 199172);
}