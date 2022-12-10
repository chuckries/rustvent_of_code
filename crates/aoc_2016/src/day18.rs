use aoc_common::{file_string};

fn input() -> Vec<char> {
    file_string("inputs/day18.txt").chars().collect()
}

fn run(len: usize) -> usize {
    let mut current = input();
    let mut next: Vec<char> = vec!['.'; current.len()];

    let mut count = current.iter().filter(|c| **c == '.').count();

    for _ in 0..len - 1 {
        for i in 0..next.len() {
            let left = if i == 0 { '.' } else { current[i - 1] };
            let center = current[i];
            let right = if i == current.len() - 1 { '.' } else { current[i + 1] };

            next[i] = match (left, center, right) {
                ('^', '^', '.') | ('.', '^','^') | ('^', '.', '.') | ('.', '.', '^') => '^',
                _ => '.'
            };
        }

        std::mem::swap(&mut current, &mut next);
        count += current.iter().filter(|c| **c == '.').count();
    }
    
    count
}

#[test]
fn part1() {
    let answer = run(40);
    assert_eq!(answer, 1987)
}

#[test]
fn part2() {
    let answer = run(400000);
    assert_eq!(answer, 19984714)
}