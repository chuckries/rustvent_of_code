use std::{collections::HashMap, fmt::Display, str::FromStr};

use aoc_common::file_lines;
use Action::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TimeStamp {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl FromStr for TimeStamp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(&['[', ']']);
        let mut split = s.split(&['-', ' ', ':']);

        Ok(Self {
            year: split.next().unwrap().parse().unwrap(),
            month: split.next().unwrap().parse().unwrap(),
            day: split.next().unwrap().parse().unwrap(),
            hour: split.next().unwrap().parse().unwrap(),
            minute: split.next().unwrap().parse().unwrap()
        })
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute)
    }
}

#[derive(Clone, Copy)]
enum Action {
    Start(usize),
    Sleep,
    Wake,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let result = match split.next().unwrap() {
            "falls" => Sleep,
            "wakes" => Wake,
            "Guard" => {
                let num = split.next().unwrap().trim_start_matches('#').parse().unwrap();
                Start(num)
            }
            _ => panic!(),
        };
        Ok(result)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Start(n) => write!(f, "Guard #{} begins shift", n),
            Sleep => write!(f, "falls asleep"),
            Wake => write!(f, "wakes up"),
        }
    }
}

fn input() -> Vec<(TimeStamp, Action)> {
    file_lines("inputs/day04.txt").map(|l| {
        let space = l.find(' ').unwrap();
        let space = space + 1 + l[space + 1 ..].find(' ').unwrap();

        let time: TimeStamp = l[.. space].parse().unwrap();
        let action: Action = l[space + 1 ..].parse().unwrap();

        (time, action)
    }).collect()
}

fn process_log() -> HashMap<usize, [u32; 60]> {
    let mut input = input();
    input.sort_by_cached_key(|(t, _)| *t);
    let input = input;

    let mut guards: HashMap<usize, [u32; 60]> = HashMap::new();

    let mut current_guard = 0;
    let mut sleep_minute = 0;
    for (t, a) in input {
        match a {
            Start(idx) => {
                guards.entry(idx).or_insert([0; 60]);
                current_guard = idx;
            },
            Sleep => {
                sleep_minute = t.minute;
            }
            Wake => {
                let guard = guards.get_mut(&current_guard).unwrap();
                for i in sleep_minute .. t.minute {
                    guard[i as usize] += 1;
                }
            }
        }
    }

    guards
}

#[test]
fn part1() {
    let guards = process_log();
    let max_guard = guards.iter().max_by_key(|(_, g)| g.iter().sum::<u32>()).unwrap();
    let max_minute = max_guard.1.iter().enumerate().max_by_key(|(_, n)| **n).unwrap().0;

    let answer = max_guard.0 * max_minute;
    assert_eq!(14346, answer);
}

#[test]
fn part2() {
    let guards = process_log();

    let mut max_count = 0;
    let mut max_minute = 0;
    let mut max_guard_id = 0;

    for (id, log) in guards {
        for (idx, n) in log.iter().enumerate() {
            if *n > max_count {
                max_count = *n;
                max_minute = idx;
                max_guard_id = id;
            }
        }
    }

    let answer = max_guard_id * max_minute;
    assert_eq!(5705, answer);
}