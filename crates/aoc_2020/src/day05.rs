use std::array;

use aoc_common::file_lines;

fn input() -> Vec<usize> {
    file_lines("inputs/day05.txt").map(|l| {
        let mut total = 0;
        for b in l.into_bytes() {
            let digit = match b {
                b'B' | b'R' => 1,
                b'F' | b'L' => 0,
                _ => panic!(),
            };

            total = (total << 1) | digit;
        }
        total
    }).collect()
}

#[test]
fn part1() {
    let answer = input().into_iter().max().unwrap();
    assert_eq!(980, answer);
}

#[test]
fn part2() {
    let mut seats: [Option<Box<[bool; 8]>>; 128] = array::from_fn(|_| None);

for i in input() {
        let row = i >> 3;
        let seat = i & 7;
        seats[row].get_or_insert_default()[seat] = true;
    }

    let mut answer = 0;
    'outer: for (row_idx, seats) in seats.iter().enumerate() {
        if let Some(seats) = seats {
            for (seat_idx, taken) in seats.iter().enumerate() {
                if !*taken {
                    answer = (row_idx << 3) + seat_idx;
                    break 'outer;
                }
            }
        }
    }

    assert_eq!(607, answer);
}