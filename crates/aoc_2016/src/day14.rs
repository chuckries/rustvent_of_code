use std::collections::VecDeque;
use bcrypt::md5::Md5;

const INPUT: &str = "ngcjuoqr";

struct HashIter {
    hasher: Md5,
    suffix: usize,
    cached: VecDeque<String>,
    extra_hashes: usize,
}

impl HashIter {
    fn new(extra_hashes: usize) -> Self {
        Self {
            hasher: Md5::new(),
            suffix: 0,
            cached: VecDeque::new(),
            extra_hashes,
        }
    }

    fn idx(&self) -> usize {
        self.suffix - 1
    }

    fn next(&mut self) -> String {
        self.suffix += 1;
        if let Some(cached) = self.cached.pop_front() {
            cached
        } else {
            self.calc_suffix(self.suffix - 1)
        }
    }

    fn peek(&mut self, offset: usize) -> &String {
        if offset < 1 {
            panic!();
        }

        if offset <= self.cached.len() {
            &self.cached[offset - 1]
        } else {
            let suffix = self.suffix + offset - 1;
            let hash = self.calc_suffix(suffix);
            self.cached.push_back(hash);
            &self.cached.back().unwrap()
        }
    }

    fn calc_suffix(&mut self, suffix: usize) -> String {
        let mut s = format!("{}{}", INPUT, suffix);
        for _ in 0 .. self.extra_hashes + 1 {
            let digest = self.hasher.compute(s);
            s = Self::digest_to_string(&digest);
        }
        s
    }

    fn digest_to_string(bytes: &[u8]) -> String {
        fn byte_to_char(b: u8) -> u8 {
            if b < 10 {
                b + b'0'
            } else {
                b - 10 + b'a'
            }
        }

        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            s.push(byte_to_char(b >> 4) as char);
            s.push(byte_to_char(b & 0x0F) as char);
        }
        s
    }
}

fn run(extra_hashes: usize) -> usize {
    let mut hash_iter = HashIter::new(extra_hashes);

    let mut count_found = 0;
    while count_found < 64 {
        let hash = hash_iter.next();

        let mut cand: Option<u8> = None;
        let bytes = hash.as_bytes();
        for i in 0..hash.len() - 2 {
            if bytes[i] == bytes[i + 1] && bytes[i + 1] == bytes[i + 2] {
                cand = Some(bytes[i]);
                break;
            }
        }

        if let Some(cand) = cand {
            'outer: for offset in 1..=1000 {
                let hash = hash_iter.peek(offset);
                let bytes = hash.as_bytes();

                for i in 0..hash.len() - 4 {
                    if bytes[i..i + 5].iter().all(|b| *b == cand) {
                        count_found += 1;
                        break 'outer;
                    }
                }
            }
        }
    }

    hash_iter.idx()
}

#[test]
fn part1() {
    let answer = run(0);
    assert_eq!(answer, 18626);
}

#[test]
fn part2() {
    let answer = run(2016);
    assert_eq!(answer, 20092);
}