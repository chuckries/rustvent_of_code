use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

fn input(repeats: usize) -> Vec<(Vec<u8>, Vec<usize>)> {
    file_lines("inputs/day12.txt").map(|l| {
        let mut split = l.split(' ');

        let puzzle = split.next().unwrap();
        let puzzle = vec![puzzle; repeats].join("?");
        let puzzle = puzzle.bytes().collect();

        let clues = split.next().unwrap();
        let clues = vec![clues; repeats].join(",");
        let clues = clues.split(',').map(|s| s.parse().unwrap()).to_vec();
        (puzzle, clues)
    }).to_vec()
}

fn generate_solutions(puzzle: &[u8], clues: &[usize]) -> usize {
    return generate_solutions_recursive(puzzle, clues, 0, &mut HashMap::new());

    fn generate_solutions_recursive(puzzle: &[u8], clues: &[usize], idx: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if let Some(cached) = cache.get(&(idx, clues.len())) {
            return *cached;
        }

        let mut solutions = 0;
        
        let available_len = puzzle.len() - idx;
        let min_total = clues.iter().sum::<usize>() + clues.len() - 1;
        let range = available_len - min_total + 1;

        'outer: for i in 0 .. range {
            for j in idx .. idx + i {
                if puzzle[j] == b'#' {
                    continue 'outer;
                }
            }

            for j in idx + i .. idx + i + clues[0] {
                if puzzle[j] == b'.' {
                    continue 'outer;
                }
            }

            let next_idx = idx + i + clues[0];

            if clues.len() > 1 {
                if puzzle[next_idx] == b'#' {
                    continue 'outer;
                }
                solutions += generate_solutions_recursive(puzzle, &clues[1..], next_idx + 1, cache);
            } else {
                for j in next_idx .. puzzle.len() {
                    if puzzle[j] == b'#' {
                        continue 'outer;
                    }
                }

                solutions += 1;
            }
        }

        cache.insert((idx, clues.len()), solutions);

        solutions
    }
}

fn run(repeats: usize) -> usize {
    let input = input(repeats);
    input.into_iter().map(|(puzzle, clues)| {
        generate_solutions(&puzzle, &clues)
    }).sum()
}

#[test]
fn part1() {
    let answer = run(1);
    assert_eq!(7653, answer);
}

#[test]
fn part2() {
    let answer = run(5);
    assert_eq!(60681419004564, answer);
}