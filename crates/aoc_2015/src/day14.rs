use std::i32;

use aoc_common::{file_lines, IteratorExt};

struct Deer {
    speed: i32,
    duration: i32,
    cooldown: i32,
}

impl Deer {
    fn pos(&self, t: i32) -> i32 {
        let period = self.duration + self.cooldown;
        let cycles = t / period;
        let remainder = (t % period).min(self.duration);

        (cycles * self.duration + remainder) * self.speed
    }
}

fn input() -> Vec<Deer> {
    file_lines("inputs/day14.txt").map(|l| {
        let split = l.split(' ').to_vec();

        Deer {
            speed: split[3].parse().unwrap(),
            duration: split[6].parse().unwrap(),
            cooldown: split[13].parse().unwrap(),
        }
    }).to_vec()
}

const SECONDS: i32 = 2503;

#[test]
fn part1() {
    let answer = input().into_iter().map(|d| d.pos(SECONDS)).max().unwrap();
    assert_eq!(answer, 2660);
}

#[test]
fn part2() {
    let input = input();
    let mut points = vec![0; input.len()];
    let mut positions = vec![0; input.len()];

    for i in 1..=SECONDS {
        let mut max = i32::MIN;

        for (idx, pos) in input.iter().map(|d| d.pos(i)).enumerate() {
            if pos > max {
                max = pos;
            }
            positions[idx] = pos;
        }

        for (idx, pos) in positions.iter().enumerate() {
            if max == *pos {
                points[idx] += 1;
            }
        }
    }

    let answer = points.into_iter().max().unwrap();
    assert_eq!(answer, 1256);
}