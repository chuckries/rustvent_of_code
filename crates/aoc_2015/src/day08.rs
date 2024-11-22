use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<String> {
    file_lines("inputs/day08.txt").to_vec()
}

#[test]
fn part1() {
    let answer = input().into_iter().map(|s| {
        let bytes = s.as_bytes();
        let mut count = bytes.len() - 2;

        let mut i = 1;
        while i < bytes.len() - 1 {
            if bytes[i] == b'\\' {
                i += 1;
                if bytes[i] == b'x' {
                    i += 2;
                    count -= 3;
                } else {
                    count -= 1;
                }
            }
            i += 1;
        }

        bytes.len() - count
    }).sum::<usize>();

    assert_eq!(answer, 1371);
}

#[test]
fn part2() {
    let answer = input().into_iter().map(|s| {
        let bytes = s.as_bytes();
        let mut total = 2;

        for b in bytes.iter() {
            total += match b {
                b'\\' | b'"' => 2,
                _ => 1
            };
        }

        total - bytes.len()
    }).sum::<usize>();

    assert_eq!(answer, 2117);
}