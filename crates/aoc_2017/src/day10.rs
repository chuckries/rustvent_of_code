use aoc_common::{bytes_to_hex_string, file_string, IteratorExt};

use crate::knot_hash::{self, knot_hash};

fn input() -> String {
    file_string("inputs/day10.txt")
}

#[test]
fn part1() {
    let lengths = input().split(',').map(|s| s.parse().unwrap()).to_vec();

    let mut skip_size = 0;
    let mut position = 0;
    let mut list = (0u8..=255).to_vec();

    knot_hash::knot_hash_round(&mut list, &lengths, &mut position, &mut skip_size);

    let answer = list[0] as u32 * list[1] as u32;
    assert_eq!(answer, 23715);
}

#[test]
fn part2() {
    let hash = knot_hash(&input());
    let answer = bytes_to_hex_string(&hash);

    assert_eq!(answer, "541dc3180fd4b72881e39cf925a50253");
}