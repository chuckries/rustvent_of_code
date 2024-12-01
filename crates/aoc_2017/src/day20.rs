use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt, Vec3i64};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$").unwrap();
}

fn input() -> Vec<(Vec3i64, Vec3i64, Vec3i64)> {
    file_lines("inputs/day20.txt").map(|l| {
        let nums = RE.captures(&l).unwrap().iter().skip(1).map(|s| s.unwrap().as_str().parse::<i64>().unwrap()).to_vec();
        let p = (nums[0], nums[1], nums[2]).into();
        let v = (nums[3], nums[4], nums[5]).into();
        let a = (nums[6], nums[7], nums[8]).into();
        (p, v, a)
    }).collect()
}

#[test]
fn part1() {
    let mut input = input();

    for _ in 0..500 {
        for (p, v, a) in input.iter_mut() {
            *v += *a;
            *p += *v;
        }
    }

    let answer = input.into_iter().enumerate().min_by_key(|(_, (p, _, _))| p.manhattan()).unwrap().0;
    assert_eq!(answer, 308);
}

#[test]
fn part2() {
    let mut input = input().into_iter().map(|p| Some(p)).to_vec();
    let mut map: HashMap<Vec3i64, Vec<usize>> = HashMap::with_capacity(input.len());

    for _ in 0..40 {
        for i in 0..input.len() {
            if let Some((p, v, a)) = &mut input[i] {
                *v += *a;
                *p += *v;

                map.entry(*p).or_default().push(i);
            }
        }

        for (_, v) in map.drain() {
            if v.len() > 1 {
                for idx in v.iter() {
                    input[*idx] = None;
                }
            }
        }
    }

    let answer = input.into_iter().filter(|p| p.is_some()).count();
    assert_eq!(answer, 504);
}