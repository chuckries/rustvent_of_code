use aoc_common::{file_string, IteratorExt};

fn input() -> String {
    file_string("inputs/day09.txt")
}

fn run(s: &str, count_str: fn(&str) -> usize) -> usize {
    let mut idx = 0;
    let mut total = 0;
    while idx < s.len() {
        if let Some(open_idx) = s[idx..].find('(') {
            total += open_idx;
            idx += open_idx;
            let close_idx = idx + s[idx..].find(')').unwrap();

            let repeat = s[idx + 1 .. close_idx].split('x').map(|i| i.parse::<usize>().unwrap()).to_vec();

            idx = close_idx + 1 + repeat[0];
            total += repeat[1] * count_str(&s[close_idx + 1 .. idx]);
        } else {
            total += s[idx..].len();
            break;
        }
    }

    total
}

#[test] 
fn part1() {
    fn count(s: &str) -> usize {
        s.len()
    }

    let answer = run(&input(), count);
    assert_eq!(answer, 183269);
}

#[test] 
fn part2() {
    fn count(s: &str) -> usize {
        run(&s, count)
    }

    let answer = run(&input(), count);
    assert_eq!(answer, 11317278863);
}