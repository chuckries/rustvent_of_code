use std::iter;

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<(Vec<u8>, Vec<usize>)> {
    file_lines("inputs/day12.txt").map(|l| {
        let mut split = l.split(' ');
        let puzzle = split.next().unwrap().bytes().collect();
        let clues = split.next().unwrap().split(',').map(|s| s.parse().unwrap()).to_vec();
        (puzzle, clues)
    }).to_vec()
}

fn generate_solutions(puzzle: &[u8], clues: &[usize], buff: Vec<u8>, idx: usize) -> usize {
    if clues.len() == 0 {
        panic!();
    }

    let available_len = buff.len() - idx;
    if available_len == 0 {
        panic!();
    }

    let min_total = clues.iter().sum::<usize>() + clues.len() - 1;
    if min_total > buff.len() - idx {
        panic!();
    }

    let mut solutions = 0;

    let range = available_len - min_total;
    for i in 0..=range {
        let mut buff = buff.clone();
        for j in 0..clues[0] {
            buff[idx + i + j] = b'#'
        }

        let mut is_ok_solution = true;
        for j in idx..idx + i + clues[0] {
            if puzzle[j] != b'?' && puzzle[j] != buff[j] {
                is_ok_solution = false;
                break;
            }
        }

        if is_ok_solution {
            if clues.len() > 1 {
                let next_idx = idx + i + clues[0];
                if puzzle[next_idx] == b'?' || puzzle[next_idx] == buff[next_idx] {
                    solutions += generate_solutions(puzzle, &clues[1..], buff, next_idx + 1);
                }
            } else {
                for j in idx + i + clues[0] .. buff.len() {
                    if puzzle[j] != b'?' && puzzle[j] != buff[j] {
                        is_ok_solution = false;
                        break;
                    }
                }

                if is_ok_solution {
                    solutions += 1;

                    for b in buff.iter() {
                        print!("{}", *b as char);
                    }
                    println!();
                }
            }
        }
    }

    solutions
}

#[test]
fn part1() {
    let input = input();

    let mut total_solutions = 0;
    for (puzzle, clues) in input.iter() {
        for b in puzzle.iter() {
            print!("{}", *b as char);
        }
        
        for c in clues.iter() {
            print!(" {}", c);
        }
        println!();
        total_solutions += generate_solutions(&puzzle, &clues, vec![b'.'; puzzle.len()], 0);
        println!();
    }

    assert_eq!(7653, total_solutions);
}

#[test]
fn part2() {
    let input = input().iter().map(|(puzzle, clues)| {
        let mut big_puzzle = Vec::new();
        let mut big_clues = Vec::new();

        for i in 0..5 {
            big_puzzle.append(&mut puzzle.clone());
            if i != 4 {
                big_puzzle.push(b'?');
            }
            big_clues.append(&mut clues.clone());
        }

        (big_puzzle, big_clues)
    }).to_vec();

    let mut total_solutions = 0;
    for (puzzle, clues) in input.iter().take(1) {
        for b in puzzle.iter() {
            print!("{}", *b as char);
        }
        
        for c in clues.iter() {
            print!(" {}", c);
        }
        println!();
        total_solutions += generate_solutions(&puzzle, &clues, vec![b'.'; puzzle.len()], 0);
        println!();
    }

    assert_eq!(0, total_solutions);
}