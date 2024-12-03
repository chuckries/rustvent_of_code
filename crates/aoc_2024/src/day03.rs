use aoc_common::file_string;
use lazy_static::lazy_static;
use regex::Regex;

fn input() -> String {
    file_string("inputs/day03.txt")
}

#[test]
fn part1() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    }

    let input = input();
    let answer = RE.captures_iter(&input).map(|c| {
        c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap()
    }).sum::<i32>();

    assert_eq!(answer, 189600467);
}

use Token::*;

#[derive(Copy, Clone)]
enum Token {
    Do,
    Dont,
    Mul,
    OpenParen,
    CloseParen,
    Comma,
    Number(i32)
}

fn scan(bytes: &[u8]) -> Option<(Token, usize)> {
    if bytes.len() == 0 {
        return None;
    }

    fn test(cand: &[u8], pattern: &[u8]) -> bool {
        cand.len() >= pattern.len() && &cand[0..pattern.len()] == pattern
    }

    match bytes[0] {
        b'd' if test(&bytes[1..], b"o()") => Some((Do, 4)),
        b'd' if test(&bytes[1..], b"on't()") => Some((Dont, 7)),
        b'm' if test(&bytes[1..], b"ul") => Some((Mul, 3)),
        b'(' => Some((OpenParen, 1)),
        b')' => Some((CloseParen, 1)),
        b',' => Some((Comma, 1)),
        c if c.is_ascii_digit() => {
            let mut i = 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let n = String::from_utf8(bytes[0..i].to_vec()).unwrap().parse::<i32>().unwrap();
            Some((Number(n), i))
        }
        _ => None
    }
}

#[test]
fn part2() {
    let input = input();
    let mut bytes = input.as_bytes();

    let mut answer = 0;
    let mut on = true;
    while bytes.len() > 0 {
        if let Some((tok, len)) = scan(&bytes) {
            match tok {
                Do => {
                    on = true;
                    bytes = &bytes[len..];
                }
                Dont => {
                    on = false;
                    bytes = &bytes[len..];
                }
                Mul => {
                    bytes = &bytes[len..];

                    if let Some((OpenParen, 1)) = scan(&bytes) {
                        bytes = &bytes[1..];
                    } else {
                        bytes = &bytes[1..];
                        continue;
                    }

                    let a: i32;
                    let b: i32;
                    if let Some((Number(n), len)) = scan(&bytes) {
                        bytes = &bytes[len..];
                        if len > 3 {
                            continue;
                        }
                        a = n;
                    } else {
                        bytes = &bytes[1..];
                        continue;
                    }

                    if let Some((Comma, 1)) = scan(&bytes) {
                        bytes = &bytes[1..];
                    } else {
                        bytes = &bytes[1..];
                        continue;
                    }

                    if let Some((Number(n), len)) = scan(&bytes) {
                        bytes = &bytes[len..];
                        if len > 3 {
                            continue;
                        }
                        b = n;
                    } else {
                        bytes = &bytes[1..];
                        continue;
                    }

                    if let Some((CloseParen, 1)) = scan(&bytes) {
                        bytes = &bytes[1..];
                    } else {
                        bytes = &bytes[1..];
                        continue;
                    }

                    if on {
                        answer += a * b;
                    }
                },
                _ => {
                    bytes = &bytes[len..]
                }
            }
        } else {
            bytes = &bytes[1..];
        }
    }

    assert_eq!(answer, 107069718);
}