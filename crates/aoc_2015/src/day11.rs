use aoc_common::IteratorExt;

const INPUT: &'static str = "hepxcrrq";

fn input() -> Vec<u8> {
    INPUT.bytes().map(|b| b).to_vec()
}

fn increment(slice: &mut [u8]) {
    for i in (0..slice.len()).rev() {
        let val = &mut slice[i];
        *val += 1;
        
        if matches!(*val, b'i' | b'l' | b'o') {
            *val += 1;
        }

        if *val > b'z' {
            *val = b'a'
        } else {
            break;
        }
    }
}

fn check(slice: &[u8]) -> bool {
    let mut i = 0;

    let mut found_run = false;
    while i < slice.len() - 2 {
        let b = slice[i];
        if slice[i + 1] == b + 1 && slice[i + 2] == b + 2 {
            found_run = true;
            break;
        }

        i += 1;
    }

    if !found_run {
        return false;
    }

    i = 0;
    let mut pairs = 0;
    while i < slice.len() - 1 {
        if slice[i] == slice[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    pairs >= 2
}

fn run(iterations: i32) -> String {
    let mut input = input();
    let mut answers = 0;

    while answers < iterations {
        increment(&mut input);
        if check(&input) {
            answers += 1;
        }
    }

    String::from_utf8(input).unwrap()
}

#[test]
fn part1() {
    let answer = run(1);
    assert_eq!(answer, "hepxxyzz");
}

#[test]
fn part2() {
    let answer = run(2);
    assert_eq!(answer, "heqaabcc");
}