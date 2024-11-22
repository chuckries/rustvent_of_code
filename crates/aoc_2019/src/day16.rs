use aoc_common::{file_string, IteratorExt};

const BASE_SEQUENCE: [i32; 4] = [0, 1, 0, -1];

struct Sequence {
    idx: usize,
    count: usize,
    current: usize
}

impl Sequence {
    fn new(count: usize) -> Self {
        Self {
            idx: 0,
            count,
            current: 0
        }
    }
}

impl Iterator for Sequence {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = BASE_SEQUENCE[self.idx];
        self.current += 1;
        if self.current == self.count {
            self.current = 0;
            self.idx += 1;
            if self.idx == BASE_SEQUENCE.len() {
                self.idx = 0;
            }
        }

        Some(result)
    }
}

fn input() -> Vec<i32> {
    file_string("inputs/day16.txt").bytes().map(|b| (b - b'0') as i32).to_vec()
}

#[test]
fn part1() {
    let mut current = input();

    println!("{}", current.len() * 10000);

    for _ in 0..100 {
        current = (0..current.len()).map(|i| {
            let seq = Sequence::new(i + 1).skip(1);
            i32::abs(current.iter().zip(seq).map(|(i, s)| i * s).sum::<i32>()) % 10
        }).to_vec();
    }

    let answer = current.into_iter().take(8).reduce(|accum, next| accum * 10 + next).unwrap();
    assert_eq!(answer, 84487724);
}

#[test]
fn part2() {
    let input = input().into_iter().map(|i| i as u64).to_vec();
    let len = input.len();
    let pos = input.iter().take(7).copied().reduce(|accum, next| accum * 10 + next).unwrap() as usize;
    let mut current = input.into_iter().cycle().skip(pos).take(len * 10000 - pos).to_vec();

    for _ in 0..100 {
        let mut partial = current.iter().sum::<u64>();
        for c in current.iter_mut() {
            let previous = *c;
            *c = partial % 10;
            partial -= previous
        }
    }

    let answer = current.into_iter().take(8).reduce(|accum, next| accum * 10 + next).unwrap();
    assert_eq!(answer, 84692524);
}