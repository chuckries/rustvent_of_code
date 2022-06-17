use std::collections::HashMap;

use aoc_common::{file_lines, ToVec};

fn input() -> Vec<(Vec<u8>, Vec<u8>)> {
    file_lines("inputs/day8.txt").map(|l| {
        let mut tok = l.split(" | ");
        let left = tok.next().unwrap().split(' ');
        let right = tok.next().unwrap().split(' ');

        fn to_byte(s: &str) -> u8 {
            s.bytes().fold(0, |accum, b| accum | (1 << (b - b'a'))) as u8
        }

        (left.map(to_byte).to_vec(), right.map(to_byte).to_vec())
    }).to_vec()
}

#[test]
fn part1() {
    let input = input();
    let answer = input.into_iter().flat_map(|(_, outputs)| outputs).filter(|c| {
        matches!(c.count_ones(), 2 | 3 | 4 | 7)
    }).count();

    assert_eq!(answer, 488);
}

#[test]
fn part2() {
    let input = input();

    let answer: usize = input.into_iter().map(|(inputs, outputs)| {
        let mut one = 0;
        let mut four = 0;
        let mut seven = 0;
        let mut eight = 0;
        inputs.iter().for_each(|id| {
            match id.count_ones() {
                2 => one = *id,
                3 => seven = *id,
                4 => four = *id,
                7 => eight = *id,
                _ => ()
            };
        });

        let masker = |size: u32, mask: u8| {
            inputs.iter().filter_map(|i| {
                if i.count_ones() == size {
                    let res = i & !mask;
                    if res.count_ones() == 1 {
                        return Some(res);
                    } 
                }
                None
            }).next().unwrap()
        };

        let a = seven & !one;
        let g = masker(6, four | seven);
        let e = masker(6, four | seven | g);
        let d = masker(5, one | a | e | g);
        let b = masker(5, one | a | d | e | g);
        let c = masker(5, a | d | e | g);
        let f = eight & !(a | b | c | d | e | g);

        let id_to_num: HashMap<u8, usize> = [
            (one                      , 1),
            (four                     , 4),
            (seven                    , 7),
            (eight                    , 8),
            (a | b | c | 0 | e | f | g, 0),
            (a | 0 | c | d | e | 0 | g, 2),
            (a | 0 | c | d | 0 | f | g, 3),
            (a | b | 0 | d | 0 | f | g, 5),
            (a | b | 0 | d | e | f | g, 6),
            (a | b | c | d | 0 | f | g, 9),
        ].into_iter().collect();

        outputs.iter().fold(0, |accum, next| accum * 10 + id_to_num.get(next).unwrap())
    }).sum();

    assert_eq!(answer, 1040429);
}