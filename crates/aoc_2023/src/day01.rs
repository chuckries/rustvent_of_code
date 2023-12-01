use aoc_common::file_lines;

fn input() -> impl Iterator<Item = String> {
    file_lines("inputs/day01.txt")
}

struct Digits<'a, F> {
    bytes: &'a [u8],
    filter: F,
}

impl<'a, F> Digits<'a, F> {
    fn new(bytes: &'a [u8], filter: F) -> Self {
        Self { 
            bytes: bytes,
            filter: filter
        }
    }
}

impl<'a, F> Iterator for Digits<'a, F> 
    where F: Fn(&'a [u8]) -> Option<(i32, usize)>
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.bytes.len() == 0 {
                return None
            }

            if let Some((value, delta)) = (self.filter)(&self.bytes) {
                self.bytes = &self.bytes[delta..];
                return Some(value);
            }

            self.bytes = &self.bytes[1..];
        }
    }
}

fn run<F>(filter: F) -> i32 
    where F: Fn(&[u8]) -> Option<(i32, usize)>
{
    input().map(|l| {
        let mut digits = Digits::new(l.as_bytes(), &filter);
        let first = digits.next().unwrap();
        let second = digits.last().unwrap_or(first);
        first * 10 + second
    }).sum()
}

#[test]
fn part1() {
    fn filter(bytes: &[u8]) -> Option<(i32, usize)> {
        if bytes[0].is_ascii_digit() {
            Some(((bytes[0] - b'0') as i32, 1))
        } else {
            None
        }
    }

    let answer = run(filter);
    assert_eq!(54953, answer);
}

#[test]
fn part2() {
    fn test(cand: &[u8], pattern: &[u8]) -> bool {
        cand.len() >= pattern.len() && &cand[0..pattern.len()] == pattern
    }

    fn filter(bytes: &[u8]) -> Option<(i32, usize)> {
        match bytes[0] {
            b if b.is_ascii_digit() => Some(((b - b'0') as i32, 1)),
            b'o' if test(&bytes[1..], b"ne") => Some((1, 1)),
            b't' if test(&bytes[1..], b"wo") => Some((2, 2)),
            b't' if test(&bytes[1..], b"hree") => Some((3, 3)),
            b'f' if test(&bytes[1..], b"our") => Some((4, 1)),
            b'f' if test(&bytes[1..], b"ive") => Some((5, 3)),
            b's' if test(&bytes[1..], b"ix") => Some((6, 3)),
            b's' if test(&bytes[1..], b"even") => Some((7, 1)),
            b'e' if test(&bytes[1..], b"ight") => Some((8, 4)),
            b'n' if test(&bytes[1..], b"ine") => Some((9, 2)),
            _ => None,
        }
    }

    let answer = run(filter);
    assert_eq!(53868, answer);
}