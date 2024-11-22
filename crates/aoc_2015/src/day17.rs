use std::i32;

use aoc_common::file_lines_as;

fn input() -> Vec<i32> {
    file_lines_as("inputs/day17.txt").collect()
}

const SIZE: i32 = 150;

fn backtrack(start: usize, used: &mut [bool], sizes: &[i32], total: i32, count: &mut i32, used_count: i32, min: &mut i32, min_count: &mut i32) {
    if total == SIZE {
        *count += 1;

        if used_count < *min {
            *min = used_count;
            *min_count = 1;
        } else if used_count == *min {
            *min_count += 1;
        }

    } else if total < SIZE  {
        for i in start..used.len() {
            if !used[i] {
                used[i] = true;
                backtrack(i + 1, used, sizes, total + sizes[i], count, used_count + 1, min, min_count);
                used[i] = false;
            }
        }
    }
}

#[test]
fn part1() {
    let input = input();
    let mut used = vec![false; input.len()];
    let mut count = 0;
    let mut min = i32::MAX;
    let mut min_count = 0;
    backtrack(0, &mut used, &input, 0, &mut count, 0, &mut min, &mut min_count);

    assert_eq!(count, 654);
}

#[test]
fn part2() {
    let input = input();
    let mut used = vec![false; input.len()];
    let mut count = 0;
    let mut min = i32::MAX;
    let mut min_count = 0;
    backtrack(0, &mut used, &input, 0, &mut count, 0, &mut min, &mut min_count);

    assert_eq!(min_count, 57);
}