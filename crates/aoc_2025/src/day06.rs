use aoc_common::{IteratorExt, file_lines};

fn input() -> (Vec<Vec<u64>>, Vec<u8>) {
    let lines = file_lines("inputs/day06.txt").to_vec();
    
    let nums = lines[0..lines.len() - 1].iter().map(|l| {
        l.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect()
    }).collect();

    let ops = lines[lines.len() - 1].split_ascii_whitespace().map(|s| {
        s.as_bytes()[0]
    }).collect();

    (nums, ops)
}

#[test]
fn part1() {
    let (nums, ops) = input();

    let mut total = 0;
    for i in 0 .. nums[0].len() {
        let iter = (0..nums.len()).map(|j| nums[j][i]);
        let result: u64 = match ops[i] {
            b'+' => iter.sum(),
            b'*' => iter.product(),
            _ => panic!(),
        };
        total += result;
    }

    assert_eq!(0, total);
}

#[test]
fn part2() {
    let grid = file_lines("inputs/day06.txt").map(|l| {
        l.into_bytes()
    }).to_vec();

    let mut total: u64 = 0;
    let mut nums: Vec<u64> = Vec::with_capacity(grid.len());
    for i in (0..grid[0].len()).rev() {
        let mut bytes: Vec<u8> = Vec::with_capacity(grid.len());
        for j in 0..grid.len() {
            match grid[j][i] {
                b if b.is_ascii_whitespace() => (),
                b@ b'*' | b@ b'+' => {
                    nums.push(String::from_utf8(bytes).unwrap().parse().unwrap());
                    bytes = Vec::with_capacity(grid.len());

                    match b {
                        b'*' => total += nums.iter().product::<u64>(),
                        b'+' => total += nums.iter().sum::<u64>(),
                        _ => panic!(),
                    };

                    nums.clear();
                },
                b => bytes.push(b),
            }
        }
        if bytes.len() > 0 {
            nums.push(String::from_utf8(bytes.clone()).unwrap().parse().unwrap());
            bytes.clear();
        }
    }

    assert_eq!(0, total);
}