use aoc_common::{file_string, IteratorExt};


fn input() -> String {
    file_string("inputs/day06.txt")
}

fn find(n: usize) -> usize {
    input()
        .chars()
        .enumerate()
        .to_vec()
        .windows(n)
        .filter(|w| 
            w.iter()
                .map(|p| p.1)
                .to_set()
                .len() == n
            )
        .next()
        .unwrap()[n - 1].0 + 1
}

#[test]
fn part1() {
    let answer = find(4);

    assert_eq!(answer, 1542);
}

#[test]
fn part2() {
    let answer = find(14);

    assert_eq!(answer, 3153);
}