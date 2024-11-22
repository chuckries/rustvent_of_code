use std::{collections::HashMap};

use aoc_common::{Vec2i32, file_lines};

struct Turn(Vec2i32, i32);

fn input() -> [Vec<Turn>; 2] {
    let mut iter = file_lines("inputs/day03.txt").map(|l| {
        l.split(',').map(|t| {
            let (dir, num) = t.split_at(1);
            let dir = match dir {
                "U" => -Vec2i32::unit_y(),
                "D" => Vec2i32::unit_y(),
                "L" => -Vec2i32::unit_x(),
                "R" => Vec2i32::unit_x(),
                _ => panic!()
            };

            let num = num.parse::<i32>().unwrap();
            Turn(dir, num)
        }).collect::<Vec<_>>()
    });

    [iter.next().unwrap(), iter.next().unwrap()]
}

fn map() -> HashMap<Vec2i32, (i32, [i32; 2])> {
    let mut map: HashMap<aoc_common::Vec2<i32>, (i32, [i32; 2])> = HashMap::new();

    for (id, turns) in input().into_iter().enumerate() {
        let mut pos = Vec2i32::zero();
        let mut count_steps = 0;
        for turn in turns {
            for _ in 0..turn.1 {
                pos += turn.0;
                count_steps += 1;
                let entry = map.entry(pos).or_default();
                let steps = &mut entry.1[id];
                if *steps == 0 {
                    *steps = count_steps;
                }
                entry.0 |= (id + 1) as i32;
            }
        }
    }

    map
}

#[test]
fn part1() {
    let answer = map().iter().filter_map(|p| {
        if p.1.0 == 3 {
            Some(i32::abs(p.0.x) + i32::abs(p.0.y))
        } else {
            None
        }
    }).min().unwrap();

    assert_eq!(answer, 248);
}

#[test]
fn part2() {
    let answer = map().values().filter_map(|p| {
        if p.0 == 3 {
            Some(p.1[0] + p.1[1])
        } else {
            None
        }
    }).min().unwrap();

    assert_eq!(answer, 28580);
}