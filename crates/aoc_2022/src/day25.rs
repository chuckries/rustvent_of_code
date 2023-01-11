use aoc_common::{file_lines, IteratorExt};

fn snafu_sum(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut idx = 0;
    let mut carry = 0;

    loop {
        if idx >= a.len() && idx >= b.len() && carry == 0 {
            break;
        }

        let mut sum = carry;

        if idx < a.len() {
            sum += a[idx];
        }
        if idx < b.len() {
            sum += b[idx];
        }

        if sum < -2 {
            carry = -1;
            sum = sum + 5;
        } else if sum > 2 {
            carry = 1;
            sum = sum - 5;
        } else {
            carry = 0;
        }

        if idx < a.len() {
            a[idx] = sum;
        } else {
            a.push(sum);
        }

        idx += 1;
    }

    a
}

#[test]
fn part1() {
    let nums = file_lines("inputs/day25.txt").map(|l| l.chars().rev().map(|c| match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!(),
    }).to_vec()).to_vec();

    let answer = nums.into_iter().reduce(|accum, item| snafu_sum(accum, item)).unwrap().into_iter().rev().map(|d| match d {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!(),
    }).collect::<String>();

    assert_eq!(answer, "2-=0-=-2=111=220=100");
}