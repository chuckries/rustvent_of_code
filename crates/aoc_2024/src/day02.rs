use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day02.txt").map(|l| {
        l.split_ascii_whitespace().map(|s| s.parse().unwrap()).to_vec()
    }).to_vec()
}

fn run<F>(f: F) -> usize 
    where F: Fn(&[i32]) -> bool
{
    input().into_iter().filter(|l| f(&l)).count()
}

fn test(list: &[i32]) -> bool {
    let ord = list[0].cmp(&list[1]);

    for w in list.windows(2) {
        if w[0].cmp(&w[1]) != ord {
            return false;
        }

        if matches!((w[0] - w[1]).abs(), i if i < 1 || i > 3) {
            return false;
        }
    }

    true
}

fn test_2(list: &[i32]) -> bool {
    for i in 0..list.len() {
        let mut list = list.to_vec();
        list.remove(i);
        if test(&list) {
            return true;
        }
    }
    false
}

#[test]
fn part1() {
    let answer = run(test);
    assert_eq!(answer, 326);
}

#[test]
fn part2() {
    let answer= run(test_2);
    assert_eq!(answer, 381);
}