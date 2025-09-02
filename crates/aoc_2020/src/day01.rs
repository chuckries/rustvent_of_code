use aoc_common::{file_lines_as, IteratorExt};

fn input() -> Vec<i64> {
    file_lines_as("inputs/day01.txt").collect()
}

#[test]
fn part1() {
    let input = input();
    let set = input.iter().to_set();

    let mut answer = 0;
    for i in input.iter() {
        let cand = 2020 - *i;
        if set.contains(&cand) {
            answer = i * cand;
            break;
        }
    }

    assert_eq!(388075, answer);
}

#[test]
fn part2() {
    let input = input();
    let set = input.iter().to_set();

    let mut answer = 0;
    for i in 0 .. input.len() - 1 {
        for j in i + 1 .. input.len() {
            let cand = 2020 - input[i] - input[j];
            if set.contains(&cand) {
                answer = input[i] * input[j] * cand;
                break;
            }
        }
    }

    assert_eq!(293450526, answer);
}