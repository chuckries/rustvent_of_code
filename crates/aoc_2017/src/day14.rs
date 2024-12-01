use std::collections::VecDeque;

use aoc_common::Vec2us;

use crate::knot_hash::knot_hash;

const INPUT: &str = "vbqugkhl";

fn get_map() -> Vec<[u8; 16]> {
    (0..128)
        .map(|i| format!("{}-{}", INPUT, i))
        .map(|s| knot_hash(&s))
        .collect()
}

#[test]
fn part1() {
    let answer = get_map()
        .iter()
        .map(|hash| hash
            .iter()
            .map(|b| b.count_ones()))
        .flatten()
        .sum::<u32>();
    assert_eq!(answer, 8148);
}

#[test]
fn part2() {
    const BOUNDS: Vec2us = Vec2us::new(128, 128);

    let map = get_map();
    let mut regions = [[-1; BOUNDS.y]; BOUNDS.x];
    let mut region_num: i32 = 0;

    let get_map = |i: usize, j: usize| -> bool {
        (map[j][i >> 3] & (0b10000000 >> (i & 0b111))) != 0
    };

    let mut queue: VecDeque<Vec2us> = VecDeque::new();

    for j in 0..BOUNDS.y {
        for i in 0..BOUNDS.x {
            if regions[j][i] == -1 && get_map(i, j) {
                queue.push_back((i, j).into());
                while let Some(current) = queue.pop_front() {
                    regions[current.y][current.x] = region_num;
                    for adj in current.adjacent_bounded(&BOUNDS) {
                        if regions[adj.y][adj.x] == -1 && get_map(adj.x, adj.y) {
                            queue.push_back(adj);
                        }
                    }
                }

                region_num += 1;
            }
        }
    }

    assert_eq!(region_num, 1180);
}