const A_SEED: u64 = 722;
const A_FACTOR: u64 = 16807;
const A_MASK: u16 = 4 - 1;
const B_SEED: u64 = 354;
const B_FACTOR: u64 = 48271;
const B_MASK: u16 = 8 - 1;
const MODULO: u64 = 2147483647;


struct Generator {
    seed: u64,
    factor: u64,
    mask: u16,
}

impl Generator {
    fn new(seed: u64, factor: u64, mask: u16) -> Self {
        Self { 
            seed,
            factor,
            mask,
        }
    }

    fn next(&mut self) -> u16 {
        self.seed = (self.seed * self.factor) % MODULO;
        (self.seed & 0xFFFF) as u16
    }

    fn next_masked(&mut self) -> u16 {
        loop {
            let next = self.next();
            if next & self.mask == 0 {
                return next;
            }
        }
    }
}

fn run<F1, F2>(mut a: F1, mut b: F2, count: usize) -> usize 
    where F1: FnMut() -> u16, F2: FnMut() -> u16
{
    let mut matches = 0;
    for _ in 0..count {
        if a() == b() {
            matches += 1;
        }
    }

    matches
}

fn input() -> (Generator, Generator) {
    (Generator::new(A_SEED, A_FACTOR, A_MASK),
    Generator::new(B_SEED, B_FACTOR, B_MASK))
}

#[test]
fn part1() {
    let (mut a, mut b) = input();
    let answer = run(|| a.next(), || b.next(), 40000000);
    assert_eq!(answer, 612);
}

#[test]
fn part2() {
    let (mut a, mut b) = input();
    let answer = run(|| a.next_masked(), || b.next_masked(), 5000000);
    assert_eq!(answer, 285);
}