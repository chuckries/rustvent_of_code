use aoc_common::{file_lines, IteratorExt};

fn input() -> impl Iterator<Item = ((u32, u32), (u32, u32))> {
    file_lines("inputs/day04.txt").map(|l| {
        let nums = l.split([',', '-']).map(|s| s.parse::<u32>().unwrap()).to_vec();

        ((nums[0], nums[1]), (nums[2], nums[3]))
    })
}

fn range_contains_other(r0: (u32, u32), r1: (u32, u32)) -> bool {
    (r0.0 <= r1.0 && r0.1 >= r1.1) || (r1.0 <= r0.0 && r1.1 >= r0.1)
}

fn ranges_overlap(r0: (u32, u32), r1: (u32, u32)) -> bool {
    !(r0.0 > r1.1 || r0.1 < r1.0)
}

fn count<F: Fn((u32, u32), (u32, u32)) -> bool>(f: F) -> usize {
    input().filter(|(r0, r1)| f(*r0, *r1)).count()
}

#[test]
fn part1() {
    let answer = count(range_contains_other);

    assert_eq!(answer, 503);
}

#[test]
fn part2() {
    let answer = count(ranges_overlap);

    assert_eq!(answer, 827);
}