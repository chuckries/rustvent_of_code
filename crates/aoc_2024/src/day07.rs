use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<(i64, Vec<i64>)> {
    file_lines("inputs/day07.txt").map(|l| {
        let mut split = l.split(' ');

        let left = split.next().unwrap().trim_end_matches(':').parse().unwrap();
        let right = split.map(|s| s.parse().unwrap()).to_vec();

        (left, right)
    }).to_vec()
}

fn test<F: Fn(i64, &[i64], i64) -> bool>(acc: i64, remaining: &[i64], target: i64, f: &F) -> bool {
    if acc > target {
        return false;
    }

    if remaining.len() == 0 {
        return acc == target;
    }

    if test(acc * remaining[0], &remaining[1..], target, f) {
        return true;
    }

    if test(acc + remaining[0], &remaining[1..], target, f) {
        return true;
    }

    f(acc, remaining, target)
}

fn run<F: Fn(i64, &[i64], i64) -> bool>(f: &F) -> i64 {
    input().into_iter().filter_map(|(total, nums)| {
        if test(nums[0], &nums[1..], total, f) {
            Some(total)
        } else {
            None
        }
    }).sum::<i64>()
}


#[test]
fn part1 () {
    #[inline(always)]
    fn nop(_: i64, _: &[i64], _: i64) -> bool {
        false
    }

    let answer = run(&nop);
    assert_eq!(answer, 6083020304036);
}

#[test]
fn part2 () {
    #[inline(always)]
    fn concat(acc: i64, remaining: &[i64], target: i64) -> bool {
        let mut mul = 1;
        let mut num = remaining[0];
        while num > 0 {
            mul *= 10;
            num /= 10;
        }
        let next = acc * mul + remaining[0];

        if test(next, &remaining[1..], target, &concat) {
            return true;
        }

        false
    }

    let answer = run(&concat);
    assert_eq!(answer, 59002246504791);
}