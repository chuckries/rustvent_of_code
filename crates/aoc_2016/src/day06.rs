use aoc_common::file_lines;

fn input() -> Vec<String> {
    file_lines("inputs/day06.txt").collect()
}

#[test]
fn part1() {
    let input = input();
    let mut counts = vec![vec![0; 26]; input[0].len()];

    for l in input {
        for (idx, b) in l.bytes().enumerate() {
            counts[idx][(b - b'a') as usize] += 1;
        }
    }

    let answer: String = counts.into_iter().map(|v| {
        let max_idx = v.iter().enumerate().max_by_key(|(_, n)| *n).unwrap().0;
        (max_idx as u8 + b'a') as char
    }).collect();

    assert_eq!(answer, "cyxeoccr");
}

#[test]
fn part2() {
    let input = input();
    let mut counts = vec![vec![0; 26]; input[0].len()];

    for l in input {
        for (idx, b) in l.bytes().enumerate() {
            counts[idx][(b - b'a') as usize] += 1;
        }
    }

    let answer: String = counts.into_iter().map(|v| {
        let max_idx = v.iter().enumerate().min_by_key(|(_, n)| *n).unwrap().0;
        (max_idx as u8 + b'a') as char
    }).collect();

    assert_eq!(answer, "batwpask");
}