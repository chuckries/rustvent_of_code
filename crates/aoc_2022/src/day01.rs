use aoc_common::file_lines;

fn input() -> Vec<Vec<i32>> {
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

    elves
}

#[test]
fn part1() {
    let answer: i32 = input().iter().map(|v| v.iter().sum()).max().unwrap();
    assert_eq!(answer, 66616);
}

#[test]
fn part2() {
    let mut totals: Vec<i32> = input().iter().map(|v| v.iter().sum()).collect();
    totals.sort_by(|a, b| b.cmp(a));
    let answer: i32 = totals[0..3].iter().sum();
    assert_eq!(answer, 199172);
}