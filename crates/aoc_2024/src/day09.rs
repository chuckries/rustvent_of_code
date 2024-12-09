
use aoc_common::{file_string, IteratorExt};

fn input() -> Vec<i64> {
    file_string("inputs/day09.txt").into_bytes().into_iter().map(|b| (b - b'0') as i64).to_vec()
}

#[test]
fn part1() {
    let nums = input();
    let mut total: i64 = 0;

    let mut front_idx = 0;
    let mut back_idx = nums.len() - 1;

    // ensure we start on even idx (file)
    if back_idx & 1 == 1 {
        back_idx -= 1;
    }

    let mut back_remaining = nums[back_idx];

    let mut pos = 0;
    let mut back_pos = nums[0..=back_idx].iter().sum::<i64>() - 1;
    'outer: while pos <= back_pos {
        // add current file to total
        let id = front_idx as i64 / 2;
        for _ in 0..nums[front_idx] {
            total += pos * id;
            pos += 1;
            if pos > back_pos {
                break 'outer;
            }
        }

        front_idx += 1;

        let mut id = back_idx as i64 / 2;
        // read num[front_idx] number of blocks from the back to fill the empty slot
        for _ in 0..nums[front_idx] {
            total += pos * id;

            pos += 1;
            back_pos -= 1;
            if pos > back_pos {
                break 'outer;
            }

            back_remaining -= 1;
            if back_remaining == 0 {
                back_pos -= nums[back_idx - 1];
                if pos > back_pos {
                    break 'outer;
                }
                back_idx -= 2;
                back_remaining = nums[back_idx];
                id -= 1;
            }
        }

        front_idx += 1;
    }

    assert_eq!(total, 6337367222422);
}

struct File {
    size: i64,
    id: i64,
    pos: i64,
}

struct Emtpy {
    size: i64,
    pos: i64,
}

#[test]
fn part2() {
    let input = input();
    let mut files: Vec<File> = Vec::with_capacity(input.len() / 2  + 1);
    let mut empties: Vec<Emtpy> = Vec::with_capacity(input.len() / 2 + 1);
    let mut iter = input.into_iter();
    let mut id = 0;
    let mut pos = 0;
    while let Some(size) = iter.next() {
        files.push(File {
            size,
            id,
            pos
        });

        id += 1;
        pos += size;

        if let Some(size) = iter.next() {
            empties.push(Emtpy {
                size,
                pos,
            });

            pos += size;
        }
    }

    let mut file_idx = files.len() - 1;
    loop {
        let file = &mut files[file_idx];

        if file.pos <= empties[0].pos {
            break;
        }

        for i in 0..empties.len() {
            let empty = &mut empties[i];
            if empty.pos > file.pos {
                break;
            }

            if empty.size >= file.size {
                file.pos = empty.pos;
                empty.size -= file.size;
                if empty.size == 0 {
                    empties.remove(i);
                } else {
                    empty.pos += file.size;
                }

                break;
            }
        }

        file_idx -= 1;
    }

    let mut total: i64 = 0;
    for f in files {
        for i in 0 .. f.size {
            total += f.id * (f.pos + i);
        }
    }
    assert_eq!(total, 6361380647183);
}