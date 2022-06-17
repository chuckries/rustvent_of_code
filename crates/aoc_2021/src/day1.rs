use aoc_common::file_lines_as;

fn input() -> Vec<i32> {
    file_lines_as("inputs/day1.txt").collect()
}

#[test]
fn part1() {
    let input = input();

    let count = input.windows(2).filter(|w| w[0] < w[1]).count();

    assert_eq!(count, 1342);
}

#[test]
fn part2() {
    let input = input();

    let first = input.windows(3);
    let second = input[1..].windows(3);

    let count = first.zip(second).filter(|(l, r)| l.iter().sum::<i32>() < r.iter().sum::<i32>()).count();
    assert_eq!(count, 1378);
}