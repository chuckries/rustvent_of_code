use aoc_common::file_string;

fn input() -> String {
    file_string("inputs/day01.txt")
}

fn run(bytes: &[u8], skip: usize) -> usize {
    let mut count = 0;
    for i in 0..bytes.len() {
        if bytes[i] == bytes[(i + skip) % bytes.len()] {
            count += (bytes[i] - b'0') as usize;
        }
    }
    count
}

#[test]
fn part1() {
    let input = input();
    let answer = run(input.as_bytes(), 1);
    assert_eq!(answer, 995);
}

#[test]
fn part2() {
    let input = input();
    let bytes = input.as_bytes();
    let answer = run(bytes, bytes.len() / 2);
    assert_eq!(answer, 1130);
}