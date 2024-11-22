use aoc_common::IteratorExt;

const INPUT: &'static str = "1321131112";

fn input() -> Vec<i32> {
    INPUT.bytes().map(|b| (b - b'0') as i32).to_vec()
}

fn run(iterations: i32) -> usize {
    let mut current = input();
    let mut next: Vec<i32> = Vec::new();

    for _ in 0..iterations {
        let mut idx = 0;

        while idx < current.len() {
            let cand = current[idx];
            let mut count = 1;
            idx += 1;

            while idx < current.len() && current[idx] == cand {
                count += 1;
                idx += 1;
            }

            next.push(count);
            next.push(cand);
        }

        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    current.len()
}

#[test]
fn part1() {
    let answer = run(40);
    assert_eq!(answer, 492982);
}

#[test]
fn part2() {
    let answer = run(50);
    assert_eq!(answer, 6989950);
}