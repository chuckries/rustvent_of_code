use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day02.txt").map(|l| {
        l.split_ascii_whitespace().map(|s| s.parse().unwrap()).to_vec()
    }).to_vec()
}

#[test]
fn part1() {
    let input = input();

    let answer = input.into_iter().filter(|list| {
        let delta = list[0] - list[1];
        if matches!(delta.abs(), i if i < 1 || i > 3) {
            return false;
        }
        let sign = delta.signum();

        for w in list.windows(2).skip(1) {
            let delta = w[0] - w[1];
            if delta.signum() != sign {
                return false;
            }
            if matches!(delta.abs(), i if i < 1 || i > 3) {
                return false;
            }
        }

        true
    }).count();

    assert_eq!(answer, 326);
}

#[test]
fn part2() {
    let input = input();

    let answer = input.into_iter().filter(|list| {
        'outer: for i in 0..list.len() {
            let mut list = list.clone();
            list.remove(i);
            let list = list;
            let delta = list[0] - list[1];
            if matches!(delta.abs(), i if i < 1 || i > 3) {
                continue 'outer;
            }
            let sign = delta.signum();
    
            for w in list.windows(2).skip(1) {
                let delta = w[0] - w[1];
                if delta.signum() != sign {
                    continue 'outer;
                }
                if matches!(delta.abs(), i if i < 1 || i > 3) {
                    continue 'outer;
                }
            }
    
            return true;
        }
        false
    }).count();

    assert_eq!(answer, 381);
}