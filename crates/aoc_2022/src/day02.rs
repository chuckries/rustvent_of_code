use aoc_common::file_lines;

use Type::*;
use self::Result::*;

#[derive(Copy, Clone)]
enum Type {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn input() -> Vec<(char, char)> {
    file_lines("inputs/day02.txt").map(|l| {
        let mut split = l.split(' ');
        (split.next().unwrap().chars().nth(0).unwrap(), split.next().unwrap().chars().nth(0).unwrap())
    }).collect()
}

fn get_type(c: char) -> Type {
    match c {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
        _ => panic!(),
    }
}

fn type_score(t: Type) -> i32 {
    match t {
        Rock => 1,
        Paper => 2,
        Scissors => 3
    }
}

fn win_score(me: Type, them: Type) -> i32 {
    match me {
        Rock => match them {
            Rock => 3,
            Paper => 0,
            Scissors => 6,
        }
        Paper => match them {
            Rock => 6,
            Paper => 3,
            Scissors => 0,
        }
        Scissors => match them {
            Rock => 0,
            Paper => 6,
            Scissors => 3,
        }
    }
}

fn get_result(c: char) -> Result {
    match c {
        'X' => Lose,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!(),
    }
}

fn get_me(them: Type, me: Result) -> Type {
    match them {
        Rock => match me {
            Win => Paper,
            Lose => Scissors,
            Draw => Rock
        }
        Paper => match me {
            Win => Scissors,
            Lose => Rock,
            Draw => Paper
        }
        Scissors => match me {
            Win => Rock,
            Lose => Paper,
            Draw => Scissors
        }
    }
}

#[test]
fn part1() {
    let input = input();

    let answer: i32 = input.into_iter().map(|(them, me)| {
        let me = get_type(me);
        let them = get_type(them);
        let type_score = type_score(me);
        let win_score = win_score(me, them);
        type_score + win_score
    }).sum();

    assert_eq!(answer, 14297);
}

#[test]
fn part2() {
    let input = input();

    let answer: i32 = input.into_iter().map(|(them, me)| {
        let them = get_type(them);
        let me = get_result(me);
        let me = get_me(them, me);
        let type_score = type_score(me);
        let win_score = win_score(me, them);
        type_score + win_score
    }).sum();

    assert_eq!(answer, 10498);
}