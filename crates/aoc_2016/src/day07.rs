use std::collections::HashSet;

use aoc_common::file_lines;

fn input() -> Vec<String> {
    file_lines("inputs/day07.txt").collect()
}

#[test]
fn part1() {
    let answer = input().into_iter().filter(|s| {
        let mut i = 0;
        let bytes = s.as_bytes();
        let mut in_brackets = false;
        let mut good = false;
        while i < bytes.len() - 3 {
            if bytes[i] == b'[' {
                in_brackets = true;
            } else if bytes[i] == b']' {
                in_brackets = false;
            } else if bytes[i] != bytes[i + 1] && bytes[i] == bytes[i + 3] && bytes[i + 1] == bytes[i + 2] {
                if in_brackets {
                    return false;
                } else {
                    good = true;
                }
            }

            i += 1;
        }

        good
    }).count();

    assert_eq!(answer, 105);
}

#[test]
fn part2() {
    let answer = input().into_iter().filter(|s| {
        let mut abas: HashSet<&[u8]> = HashSet::new();
        let mut babs: HashSet<&[u8]> = HashSet::new();
        let mut i = 0;
        let bytes = s.as_bytes();
        let mut in_brackets = false;
        while i < bytes.len() - 2 {
            if bytes[i] == b'[' {
                in_brackets = true;
            } else if bytes[i] == b']' {
                in_brackets = false;
            } else {
                if bytes[i] != bytes[i + 1] && bytes[i] == bytes[i + 2] {
                    if in_brackets {
                        &mut babs
                    } else {
                        &mut abas
                    }.insert(&bytes[i..i+3]);
                }
            }

            i += 1;
        }

        for a in abas.iter() {
            for b in babs.iter() {
                if a[0] == b[1] && a[1] == b[0] {
                    return true;
                }
            }
        }
        false
    }).count();

    assert_eq!(answer, 258);
}