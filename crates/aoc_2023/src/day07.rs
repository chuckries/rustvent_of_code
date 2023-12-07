use std::{collections::HashMap, cmp::Ordering};

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Hand> {
    file_lines("inputs/day07.txt").map(|l| {
        let mut split = l.split_whitespace();

        Hand::new(split.next().unwrap(), split.next().unwrap().parse().unwrap())
    }).to_vec()
}

const FIVE_OF_A_KIND: i32 = 6;
const FOUR_OF_A_KIND: i32 = 5;
const FULL_HOUSE: i32 = 4;
const THREE_OF_A_KIND: i32 = 3;
const TWO_PAIR: i32 = 2;
const ONE_PAIR: i32 = 1;
const HIGH_CARD: i32 = 0;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    ordered: Vec<char>,
    hand_type: i32,
}

impl Hand {
    fn new(cards: &str, bid: i32) -> Hand {
        let cards = cards.chars().to_vec();

        let hand_type = Self::score_hand_wild(&cards);

        Hand {
            cards: cards.to_vec(),
            bid,
            ordered: cards.to_vec(),
            hand_type
        }
    }

    fn score_hand(hand: &[char]) -> (Vec<char>, i32) {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in hand {
            *map.entry(*c).or_default() += 1;
        }

        let mut counts = map.iter().map(|kvp| (*kvp.0, *kvp.1)).to_vec();
        counts.sort_by(|lhs, rhs| {
            let mut ord = rhs.1.cmp(&lhs.1);
            if ord == Ordering::Equal {
                ord = Self::card_value(rhs.0).cmp(&Self::card_value(lhs.0))
            }
            ord
        });

        let hand_type = if counts.len() == 1 {
            FIVE_OF_A_KIND
        } else if counts.len() == 2 {
            if counts[0].1 == 4 {
                FOUR_OF_A_KIND
            } else {
                FULL_HOUSE
            } 
        } else if counts.len() == 3 {
            if counts[0].1 == 3 {
                THREE_OF_A_KIND
            } else {
                TWO_PAIR
            }
        } else if counts.len() == 4 { 
            ONE_PAIR
        } else {
            HIGH_CARD
        };

        let mut ordered: Vec<char> = Vec::new();
        for (c, count) in counts {
            for _ in 0..count {
                ordered.push(c)
            }
        }

        (ordered, hand_type)
    }

    fn score_hand_wild(hand: &[char]) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();

        let mut wilds = 0;
        for c in hand {
            if *c == 'J' {
                wilds += 1; 
            } else {
                *map.entry(*c).or_default() += 1;
            }
        }

        let mut counts = map.iter().map(|kvp| (*kvp.0, *kvp.1)).to_vec();
        counts.sort_by(|lhs, rhs| {
            let mut ord = rhs.1.cmp(&lhs.1);
            if ord == Ordering::Equal {
                ord = Self::card_value(rhs.0).cmp(&Self::card_value(lhs.0))
            }
            ord
        });

        if wilds == 0 {
            Self::score_hand(hand).1
        } else if counts.len() == 1 {
            FIVE_OF_A_KIND
        } else if wilds == 5 || wilds == 4 {
            FIVE_OF_A_KIND
        } else if wilds == 3 {
            FOUR_OF_A_KIND
        } else if wilds == 2 {
            if counts.len() == 3 {
                THREE_OF_A_KIND
            } else {
                FOUR_OF_A_KIND
            }
        } else if wilds == 1 {
            if counts.len() == 2 {
                if counts[0].1 == 3 {
                    FOUR_OF_A_KIND
                } else {
                    FULL_HOUSE
                }
            } else if counts.len() == 3 {
                THREE_OF_A_KIND
            } else {
                ONE_PAIR
            }
        } else {
            panic!()
        }
    }

    fn card_value(c: char) -> i32 {
        match c {
            '2'..='9' => c.to_digit(10).unwrap() as i32,
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unknown card"),
        }
    }
}

#[test]
fn part1() {
    let mut hands = input();

    hands.sort_by(|lhs, rhs| {
        let mut ord = lhs.hand_type.cmp(&rhs.hand_type);
        if ord == Ordering::Equal {
            let mut idx = 0;
            while ord == Ordering::Equal {
                ord = Hand::card_value(lhs.cards[idx]).cmp(&Hand::card_value(rhs.cards[idx]));
                idx += 1;
            }
        }

        ord
    });

    let mut total: i64 = 0;
    for (rank, hand) in hands.iter().enumerate() {
        total += (rank as i64 + 1) * hand.bid as i64;
    }

    for h in hands {
        println!("{:?}", h.ordered);
    }

    assert_eq!(0, total);
}