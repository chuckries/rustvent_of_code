use aoc_common::{file_line_blocks, IteratorExt};
    
fn input() -> Vec<Vec<String>> {
    file_line_blocks("inputs/day06.txt")
}

#[test]
fn part1() {
    let input = input();
    let answer = input.into_iter().map(|block| {
        block.into_iter().map(|s| s.into_bytes()).flatten().to_set().len()
    }).sum::<usize>();

    assert_eq!(6310, answer);
}

#[test]
fn part2() {
    let answer: usize = input().into_iter().map(|block| {
        let start = block[0].bytes().to_set();
        block[1..].iter().fold(start, |set, s| {
            set.intersection(&s.bytes().to_set()).copied().to_set()
        }).len()
    }).sum();

    assert_eq!(3193, answer)
}