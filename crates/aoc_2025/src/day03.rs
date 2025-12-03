use aoc_common::file_lines;

fn input() -> Vec<Vec<u64>> {
    file_lines("inputs/day03.txt")
        .map(|l| l.bytes().map(|b| (b - b'0') as u64).collect())
        .collect()
}

fn calc_maxes<const DIGITS: usize>() -> u64 {
    input().into_iter().map(|line| {
        let mut num = 0;
        let mut first_idx = 0;

        for digit in 0 .. DIGITS {
            let mut max = 0;
            let mut max_idx = 0;
            for i in first_idx .. line.len() - (DIGITS - digit - 1) {
                if line[i] > max {
                    max = line[i];
                    max_idx = i;
                }
            }
            first_idx = max_idx + 1;
            num = num * 10 + max;
        }
        num
    }).sum()
}

#[test]
fn part1() {
    let answer = calc_maxes::<2>();
    assert_eq!(17158, answer);
}

#[test]
fn part2() {
    let answer = calc_maxes::<12>();
    assert_eq!(170449335646486, answer)
}