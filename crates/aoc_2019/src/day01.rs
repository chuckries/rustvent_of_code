use aoc_common::file_lines_as;

fn input() -> Vec<i32> {
    file_lines_as("inputs/day01.txt").collect()
}

#[test]
fn part1() {
    let answer = input().into_iter().map(|l| l / 3 - 2).sum::<i32>();
    assert_eq!(answer, 3363929);
}

#[test]
fn part2() {
    let answer = input().into_iter().map(|f| {

        let mut next = f / 3 - 2;
        let mut total = 0;
        while next > 0 {
            total += next;
            next = next / 3 - 2;
        }

        total
    }).sum::<i32>();

    assert_eq!(answer, 5043026);
}