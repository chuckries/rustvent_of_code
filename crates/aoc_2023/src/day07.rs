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

    if map.len() == 1 || wilds == 5 || wilds == 4 {
        FiveOfAKind
    } else if wilds == 3 {
        FourOfAKind
    } else if wilds == 2 {
        if map.len() == 3 {
            ThreeOfAKind
        } else {
            FourOfAKind
        }
    } else if wilds == 1 {
        if map.len() == 2 {
            if *map.values().max().unwrap() == 3 {
                FourOfAKind
            } else {
                FullHouse
            }
        } else if map.len() == 3 {
            ThreeOfAKind
        } else {
            OnePair
        }
    } else if wilds == 0 {
        if map.len() == 2 {
            if *map.values().max().unwrap() == 4 {
                FourOfAKind
            } else {
                FullHouse
            } 
        } else if map.len() == 3 {
            if *map.values().max().unwrap() == 3 {
                ThreeOfAKind
            } else {
                TwoPair
            }
        } else if map.len() == 4 { 
            OnePair
        } else {
            HighCard
        }
    }  else {
        panic!()
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

    let mut total: i64 = 0;
    for (idx, hand) in scored_hands.iter().enumerate() {
        total += (idx as i64 + 1) * hand.1 as i64;
    }

    total
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