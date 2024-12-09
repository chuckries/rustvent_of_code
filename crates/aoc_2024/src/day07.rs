use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<(i64, Vec<i64>)> {
    file_lines("inputs/day07.txt").map(|l| {
        let mut split = l.split(' ');

        let left = split.next().unwrap().trim_end_matches(':').parse().unwrap();
        let right = split.map(|s| s.parse().unwrap()).to_vec();

        (left, right)
    }).to_vec()
}

fn test<const N: u8>(acc: i64, remaining: &[i64], target: i64) -> bool {
    if acc > target {
        return false;
    }

    if remaining.len() == 0 {
        return acc == target;
    }

    if test::<N>(acc * remaining[0], &remaining[1..], target) {
        return true;
    }

    if test::<N>(acc + remaining[0], &remaining[1..], target) {
        return true;
    }

    if N != 0 {
        let mut mul = 1;
        let mut num = remaining[0];
        while num > 0 {
            mul *= 10;
            num /= 10;
        }
        let next = acc * mul + remaining[0];

        if test::<N>(next, &remaining[1..], target) {
            return true;
        }
    }

    false
}

fn run<const N: u8>() -> i64 {
    input().into_iter().filter_map(|(total, nums)| {
        if test::<N>(nums[0], &nums[1..], total) {
            Some(total)
        } else {
            None
        }
    }).sum::<i64>()
}


#[test]
fn part1 () {
    let answer = run::<0>();
    assert_eq!(answer, 6083020304036);
}

#[test]
fn part2 () {
    let answer = run::<1>();
    assert_eq!(answer, 59002246504791);
}