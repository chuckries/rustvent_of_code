use aoc_common::{file_string, IteratorExt};

fn input() -> Vec<i32> {
    file_string("inputs/day09.txt").into_bytes().into_iter().map(|b| (b - b'0') as i32).to_vec()
}

#[test]
fn part1() {
    let nums = input();
    let mut blocks: Vec<i32> = Vec::new();

    let mut id = 0;
    let mut iter = nums.into_iter();

    while let Some(block_size) = iter.next() {
        for _ in 0..block_size {
            blocks.push(id);
        }
        id += 1;

        if let Some(empty_size) = iter.next() {
            for _ in 0..empty_size {
                blocks.push(-1);
            }
        }
    }

    let mut next = 0;
    let mut end = blocks.len() - 1;
    while next < end {
        if blocks[next] != -1 {
            next += 1;
        } else if blocks[end] == -1 {
            end -= 1;
        } else {
            blocks[next] = blocks[end];
            blocks[end] = -1;
            next += 1;
            end -= 1;
        }
    }

    let answer = blocks
        .iter()
        .copied()
        .take_while(|c| *c != -1)
        .enumerate()
        .map(|(idx, id)| idx as i64 * id as i64)
        .sum::<i64>();

    assert_eq!(answer, 6337367222422);
}

use Entry::*;
enum Entry {
    File(i32, usize),
    Empty(i32),
}

impl Entry {
    fn is_file(&self) -> bool {
        matches!(self, File(_, _))
    }

    fn is_empty(&self) -> bool {
        matches!(self, Empty(_))
    }
}

#[test]
fn part2() {
    let mut iter = input().into_iter();
    let mut id = 0;
    let mut entries: Vec<Entry> = Vec::new();

    while let Some(file_size) = iter.next() {
        entries.push(File(file_size, id));
        id += 1;
        if let Some(empty_size) = iter.next() {
            entries.push(Empty(empty_size));
        }
    }

    let mut front = 0;
    let mut back = entries.len() - 1;

    while back > 0 {
        match entries[back] {
            File(file_size, id) => {
                for i in 0..back {
                    if let Empty(empty_size) = entries[i] {
                        if empty_size >= file_size {
                            entries[i] = File(file_size, id);
                            if empty_size > file_size {
                                entries.insert(i + 1, Empty(empty_size - file_size));
                                back += 1;
                            }
                            entries[back] = Empty(file_size);
                            break;
                        }
                    }
                }
                back -= 1;
            }
            Empty(_) => {
                back -=1;
            }
        }
    }

    let mut pos = 0;
    let mut total = 0;
    for e in entries {
        match e {
            File(size, id) => {
                for _ in 0..size {
                    total += pos as i64 * id as i64;
                    pos += 1;
                }
            }
            Empty(size) => {
                pos += size
            }
        }
    }

    assert_eq!(total, 6361380647183);
}