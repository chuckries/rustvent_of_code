use std::{collections::HashMap};

use aoc_common::file_lines;


enum Input {
    Mask(String),
    Write(u64, u64),
}

fn input() -> impl Iterator<Item = Input> {
    file_lines("inputs/day14.txt").map(|l| {
        let split = l.split(" = ").collect::<Vec<_>>();

        if split[0].eq("mask") {
            Input::Mask(split[1].to_string())
        } else {
            let address: u64 = split[0][4..split[0].len() - 1].parse().unwrap();
            let value: u64 = split[1].parse().unwrap();

            Input::Write(address, value)
        }
    })
}

#[test]
fn part1()
{
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and_mask = (1 << 36) - 1;
    let mut or_mask = 0;

    for line in input() {
        match line {
            Input::Mask(mask) => {
                and_mask = (1 << 36) - 1;
                or_mask = 0;
                let mut idx = 35;
                for c in mask.chars() {
                    match c {
                        '1' => or_mask |= 1 << idx,
                        '0' => and_mask ^= 1 << idx,
                        'X' => (),
                        _ => panic!()
                    }
                    idx -= 1;
                }

            }
            Input::Write(address, mut value) => {
                value &= and_mask;
                value |= or_mask;
                memory.insert(address, value);
            }
        }
    }

    let answer: u64 = memory.values().sum();

    assert_eq!(answer, 5875750429995);
}

fn apply_address_mask(address: u64, mask: &str) -> String {
    let mut base = String::new();
    let mut idx = 35;
    for c in mask.chars() {
        match c {
            '0' => {
                if address & 1 << idx > 0 {
                    base.push('1')
                } else {
                    base.push('0')
                }
            }
            '1' | 'X' => base.push(c),
            _ => panic!()
        };
        idx -= 1;
    }

    base
}

fn enumerate_masked_addresses(address: u64, mask: &str) -> Vec<String> {
    let mut addresses: Vec<String> = Vec::new();

    let base: Vec<char> = apply_address_mask(address, mask).chars().collect();

    fn backtrack(base: &[char], idx: usize, current: &mut String, addresses: &mut Vec<String>) {
        if current.len() == 36 {
            addresses.push(current.to_string());
        } else {
            match base[idx] {
                '1' | '0' => {
                    current.push(base[idx]);
                    backtrack(base, idx + 1, current, addresses);
                    current.pop();
                }
                'X' => {
                    current.push('1');
                    backtrack(base, idx + 1, current, addresses);
                    current.pop();
                    current.push('0');
                    backtrack(base, idx + 1, current, addresses);
                    current.pop();
                }
                _ => panic!()
            };
        }
    }

    backtrack(&base, 0, &mut String::new(), &mut addresses);
    addresses
}

#[test]
fn part2() {
    let mut memory: HashMap<String, u64> = HashMap::new();
    let mut mask = String::new();

    for input in input() {
        match input {
            Input::Mask(m) => mask = m,
            Input::Write(address, value) => {
                for address in enumerate_masked_addresses(address, &mask) {
                    memory.insert(address, value);
                }
            }
        }
    }

    let answer: u64 = memory.values().sum();

    assert_eq!(answer, 5272149590143);
}