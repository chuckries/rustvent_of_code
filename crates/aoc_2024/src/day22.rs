use aoc_common::file_lines_as;

const MODULO: u64 = 16777216;

fn next_secret_number(mut secret: u64) -> u64 {
    secret = (secret ^ (secret << 6)) % MODULO;
    secret = (secret ^ (secret >> 5)) % MODULO;
    secret = (secret ^ (secret << 11)) % MODULO;
    secret
}

fn calc_secret_number(mut seed: u64, iterations: usize) -> u64 {
    for _ in 0..iterations {
        seed = next_secret_number(seed);
    }
    seed
}

fn input() -> Vec<u64> {
    file_lines_as("inputs/day22.txt").collect()
}

#[test]
fn part1() {
    let answer = input().into_iter().map(|s| calc_secret_number(s, 2000)).sum::<u64>();
    assert_eq!(answer, 16039090236);
}

fn run_seed(seed: u64, idx: u16, seen: &mut [u16], totals: &mut [i32]) {
    // prime the encoded deltas
    let mut encoded = 0;
    let mut previous = seed;
    let mut previous_digit = (seed % 10) as i8;
    let mut current;
    let mut digit;

    for _ in 0..3 {
        current = next_secret_number(previous);
        digit = (current % 10) as i8;
        encoded <<= 5;
        encoded |= (digit - previous_digit + 9) as u8 as u32;
        previous = current;
        previous_digit = digit;
    }

    for _ in 3..2000 {
        current = next_secret_number(previous);
        digit = (current % 10) as i8;
        encoded <<= 5;
        encoded |= (digit - previous_digit + 9) as u8 as u32;
        encoded &= 0x000FFFFF;

        if seen[encoded as usize] != idx {
            seen[encoded as usize] = idx;
            totals[encoded as usize] += digit as i32;
        }

        previous = current;
        previous_digit = digit;
    }
}

#[test]
fn part2() {
    const SPACE: usize = usize::pow(2, 20);
    let mut seen = vec![0; SPACE];
    let mut totals = vec![0; SPACE];

    for (idx, seed) in input().into_iter().enumerate() {
        run_seed(seed, idx as u16 + 1, &mut seen, &mut totals);
    }

    let mut max = 0;
    let mut encoded;
    let mut cand;
    for a in 0u32..=18 {
        for b in 0u32..=18 {
            for c in 0u32..=18 {
                for d in 0u32..18 {
                    encoded = (a << 15) | (b << 10) | (c << 5) | d;
                    cand = totals[encoded as usize];
                    if cand > max {
                        max = cand;
                    }
                }
            }
        }
    }

    assert_eq!(max, 1808);
}