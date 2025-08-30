use std::collections::HashSet;

use aoc_common::{file_lines, IteratorExt};

type Graph = Vec<Vec<u8>>;

fn input() -> (Graph, Graph) {
    let mut requires: Graph = vec![vec![]; 26];
    let mut required_by: Graph = vec![vec![]; 26];

    for line in file_lines("inputs/day07.txt") {
        let split = line.split_ascii_whitespace().to_vec();
        let src = split[1].as_bytes()[0] - b'A';
        let dst = split[7].as_bytes()[0] - b'A';

        required_by[src as usize].push(dst);
        requires[dst as usize].push(src);
    }

    (requires, required_by)
}

#[test]
fn part1() {
    let (requires, required_by) = input();
    let mut complete = [false; 26];

    let mut answer = String::new();
    let mut available: HashSet<u8> = requires.iter().enumerate().filter_map(|(idx, nodes)| {
        if nodes.len() == 0 {
            Some(idx as u8) 
        } else {
            None
        }
    }).to_set();

    while available.len() > 0 {
        let next = available.iter().copied().sorted().next().unwrap();
        available.remove(&next);
        answer.push((next + b'A') as char);
        complete[next as usize] = true;

        for adj in required_by[next as usize].iter().copied() {
            if requires[adj as usize].iter().all(|n| complete[*n as usize]) {
                available.insert(adj);
            }
        }
    }

    assert_eq!("BETUFNVADWGPLRJOHMXKZQCISY", answer);
}

#[derive(Copy, Clone)]
struct Worker {
    time_remaining: i32,
    letter: u8,
}

#[test]
fn part2() {
    let (requires, required_by) = input();

    let mut workers: [Option<Worker>; 5] = [None; 5];

    let mut available: HashSet<u8> = requires.iter().enumerate().filter_map(|(idx, nodes)| {
        if nodes.len() == 0 {
            Some(idx as u8) 
        } else {
            None
        }
    }).to_set();

    const ADD: i32 = 60;

    let mut complete = [false; 26];
    let mut total = 0;
    let mut cands: HashSet<u8> = HashSet::new(); 
    loop {
        // fill available workers
        let sorted = available.iter().copied().sorted().to_vec();
        let available_workers = workers.iter_mut().filter(|w| w.is_none());
        for (c, w) in sorted.into_iter().zip(available_workers.into_iter()) {
            *w = Some(Worker {
                time_remaining: c as i32 + 1 + ADD,
                letter: c
            });
            available.remove(&c);
        }

        // stop if all workers are empty
        if workers.iter().flatten().count() == 0 {
            break;
        }

        // determine next time slice
        let run_time = workers.iter().flatten().min_of(|w| w.time_remaining).unwrap();

        // accrue the time
        total += run_time;

        // apply run_time to each worker, marking them as complete if they are 0
        // we also need to accrue the new candidates and validate them before we add them to available
        for w in workers.iter_mut() {
            if let Some(ww) = w {
                ww.time_remaining -= run_time;

                if ww.time_remaining == 0 {
                    let current = ww.letter;
                    complete[current as usize] = true;
                    for adj in required_by[current as usize].iter().copied() {
                        cands.insert(adj);
                    }
                    *w = None;
                }
            }
        }

        // add any qualifying candidates to available
        for cand in cands.drain() {
            if requires[cand as usize].iter().all(|n| complete[*n as usize]) {
                available.insert(cand);
            }
        }
    }

    assert_eq!(848, total);
}