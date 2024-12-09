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

    // for c in blocks.iter().copied() {
    //     match c {
    //         -1 => print!("."),
    //         _ => print!("{}", c)
    //     }
    // }
    // println!();

    let mut next = 0;
    while blocks[next] != -1 {
        next += 1;
    }

    let mut end = blocks.len() - 1;
    while blocks[end] == -1 {
        end -= 1;
    }

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

    // for c in blocks.iter().copied() {
    //     match c {
    //         -1 => print!("."),
    //         _ => print!("{}", c)
    //     }
    // }
    // println!();

    let mut answer: i64 = 0;
    for (idx, id) in blocks.iter().copied().enumerate() {
        if id == -1 {
            break;
        }

        answer += idx as i64 * id as i64;
    }

    assert_eq!(answer, 6337367222422);
}