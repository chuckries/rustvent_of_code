use aoc_common::{file_lines, IteratorExt};

fn input() -> impl Iterator<Item = String> {
    file_lines("inputs/day03.txt")
}

fn letter_score(c: u8) -> u32 {
    let score = match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!()
    };
    score as u32
}

fn find<I, F>(mut it: I, f: F) -> u32 
    where I: Iterator<Item = u8>, F: Fn(&u8) -> bool
{
    letter_score(it.find(f).unwrap())
}

#[test]
fn part1() {
    let answer: u32 = input().map(|line| {
        let (front, back) = line.split_at(line.len() / 2);
        let back = back.bytes().to_set();

        find(front.bytes(), |c| back.contains(c))
    }).sum();

    assert_eq!(answer, 7793);
}

#[test]
fn part2() {
    let answer: u32 = input().to_vec().chunks(3).map(|lines| {
        let l1 = lines[1].bytes().to_set();
        let l2 = lines[2].bytes().to_set();

        find(lines[0].bytes(), |c| l1.contains(c) && l2.contains(c))
    }).sum();

    assert_eq!(answer, 2499);
}