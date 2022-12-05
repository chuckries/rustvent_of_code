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

fn input() -> impl Iterator<Item = (char, char)> {
    file_lines("inputs/day02.txt").map(|l| {
        let mut split = l.split(' ');
        (split.next().unwrap().chars().nth(0).unwrap(), split.next().unwrap().chars().nth(0).unwrap())
    })
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

fn get_result(me: Type, them: Type) -> Result {
    match me {
        Rock => match them {
            Rock => Draw,
            Paper => Lose,
            Scissors => Win,
        }
        Paper => match them {
            Rock => Win,
            Paper => Draw,
            Scissors => Lose,
        }
        Scissors => match them {
            Rock => Lose,
            Paper => Win,
            Scissors => Draw,
        }
    }
}

fn result_score(r: Result) -> i32 {
    match r {
        Win => 6,
        Lose => 0,
        Draw => 3,
    }
}

fn get_needed_result(c: char) -> Result {
    match c {
        'X' => Lose,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!(),
    }
}

fn get_needed_type(them: Type, me: Result) -> Type {
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
    let answer: i32 = input().map(|(them, me)| {
        let me = get_type(me);
        let them = get_type(them);
        let type_score = type_score(me);
        let result = get_result(me, them);
        let result_score = result_score(result);
        type_score + result_score
    }).sum();

    assert_eq!(answer, 14297);
}

#[test]
fn part2() {
    let answer: i32 = input().map(|(them, me)| {
        let them = get_type(them);
        let result = get_needed_result(me);
        let me = get_needed_type(them, result);
        let type_score = type_score(me);
        let win_score = result_score(result);
        type_score + win_score
    }).sum();

    assert_eq!(answer, 10498);
}