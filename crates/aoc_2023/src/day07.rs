use std::{collections::HashMap, cmp::Ordering};
use HandType::*;

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<(Vec<char>, i32)> {
    file_lines("inputs/day07.txt").map(|l| {
        let mut split = l.split_whitespace();

        (split.next().unwrap().chars().collect(), split.next().unwrap().parse().unwrap())
    }).to_vec()
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_value(c: char, jacks_wild: bool) -> i32 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap() as i32,
        'T' => 10,
        'J' => if jacks_wild { 1 } else { 11 },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unknown card"),
    }
}

fn get_hand_type(hand: &[char], jacks_wild: bool) -> HandType {
    let mut map: HashMap<char, i32> = HashMap::new();

    let mut wilds = 0;
    for c in hand {
        if jacks_wild && *c == 'J' {
            wilds += 1;
        } else {
            *map.entry(*c).or_default() += 1;
        }
    }

    let count_buckets = map.len();
    let max_bucket = map.values().copied().max().unwrap_or_default();

    match (wilds, count_buckets, max_bucket) {
        (_, 1, _) => FiveOfAKind,
        (5, _, _) => FiveOfAKind,
        (4, _, _) => FiveOfAKind,
        (3, _, _) => FourOfAKind,
        (2, 2, _) => FourOfAKind,
        (2, 3, _) => ThreeOfAKind,
        (1, 2, 3) => FourOfAKind,
        (1, 2, 2) => FullHouse,
        (1, 3, _) => ThreeOfAKind,
        (1, 4, _) => OnePair,
        (0, 2, 4) => FourOfAKind,
        (0, 2, 3) => FullHouse,
        (0, 3, 3) => ThreeOfAKind,
        (0, 3, 2) => TwoPair,
        (0, 4, _) => OnePair,
        (0, 5, _) => HighCard,
        _ => panic!(),
    }
}

fn run(jacks_wild: bool) -> i64 {
    let input = input();

    let mut scored_hands = input.into_iter().map(|(hand, bid)| {
        let hand_type = get_hand_type(&hand, jacks_wild);
        (hand, bid, hand_type)
    }).to_vec();

    scored_hands.sort_by(|lhs, rhs| {
        let mut ord = lhs.2.cmp(&rhs.2);
        let mut idx = 0;
        while ord == Ordering::Equal {
            ord = card_value(lhs.0[idx], jacks_wild).cmp(&card_value(rhs.0[idx], jacks_wild));
            idx += 1;
        }
        ord
    });

    scored_hands.into_iter().enumerate().map(|(idx, (_, bid, _))| { (idx as i64 + 1) * bid as i64 }).sum()
}

#[test]
fn part1() {
    let answer = run(false);
    assert_eq!(256448566, answer);
}

#[test]
fn part2() {
    let answer = run(true);
    assert_eq!(254412181, answer);
}