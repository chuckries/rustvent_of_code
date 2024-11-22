use std::collections::VecDeque;

use aoc_common::Vec2i64;

use intcode::{IntCode, IntCodeResult};

fn input() -> (Vec<IntCode>, Vec<VecDeque<Vec2i64>>) {
    let computer = IntCode::from_file("inputs/day23.txt");

    let mut network = vec![computer; 50];
    for (address, computer) in network.iter_mut().enumerate() {
        computer.push_input_back(address as i64);
    }

    let queues = vec![VecDeque::new(); network.len()];
    (network, queues)
}

#[test]
fn part1() {
    let (mut network, mut queues) = input();
    let answer;
    'outer: loop {
        for (current, computer) in network.iter_mut().enumerate() {
            if let Some(next) = queues[current].pop_front() {
                computer.push_input_back(next.x);
                computer.push_input_back(next.y);
            } else {
                computer.push_input_back(-1);
            }

            if let IntCodeResult::Output(dest) = computer.run() {
                let x = computer.run().unwrap();
                let y = computer.run().unwrap();
                if dest == 255 {
                    answer = y;
                    break 'outer;
                } else {
                    queues[dest as usize].push_back((x, y).into());
                }
            }
        }
    }

    assert_eq!(answer, 22134);
}

#[test]
fn part2() {
    let (mut network, mut queues) = input();
    let mut nat = Vec2i64::zero();
    let mut answer = 0;
    'outer: loop {
        for (current, computer) in network.iter_mut().enumerate() {
            if let Some(next) = queues[current].pop_front() {
                computer.push_input_back(next.x);
                computer.push_input_back(next.y);
            } else {
                computer.push_input_back(-1);
            }

            if let IntCodeResult::Output(dest) = computer.run() {
                let x = computer.run().unwrap();
                let y = computer.run().unwrap();
                if dest == 255 {
                    nat = (x, y).into();
                } else {
                    queues[dest as usize].push_back((x, y).into());
                }
            }
        }

        if queues.iter().all(|q| q.is_empty()) {
            if answer == nat.y {
                break 'outer;
            }
            answer = nat.y;
            queues[0].push_back(nat);
        }
    }

    assert_eq!(answer, 16084);
}