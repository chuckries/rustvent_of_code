use std::collections::HashSet;
use aoc_common::{Vec2i32, file_lines, IteratorExt};

fn input() -> impl Iterator<Item = (Vec2i32, i32)> {
    file_lines("inputs/day09.txt").map(|l| {
        let split = l.split(' ').to_vec();
        let count: i32 = split[1].parse().unwrap();
        let dir = match split[0] {
            "L" => -Vec2i32::unit_x(),
            "R" =>  Vec2i32::unit_x(),
            "U" => -Vec2i32::unit_y(),
            "D" =>  Vec2i32::unit_y(),
            _ => panic!()
        };

        (dir, count)
    })
}

fn fix(head: Vec2i32, tail: Vec2i32) -> Vec2i32 {
    let diff = head - tail;
    let manhattan = diff.manhattan();
    if (diff.x == 0 || diff.y == 0) && manhattan > 1 {
        head - Vec2i32::new(diff.x.signum(), diff.y.signum())
    } else if manhattan > 2 {
        head - Vec2i32::new(diff.x - diff.x.signum(), diff.y - diff.y.signum())
    } else {
        tail
    }
}

fn move_rope(len: usize) -> usize {
    let mut knots = vec![Vec2i32::zero(); len];
    let mut visited: HashSet<Vec2i32> = HashSet::new();

    for (dir, count) in input() {
        for _ in 0..count {
            knots[0] += dir;

            for i in 0..knots.len() - 1 {
                let next = fix(knots[i], knots[i + 1]);
                if next == knots[i + 1] {
                    break;
                }
                knots[i + 1] = next;
            }

            visited.insert(knots[len - 1]);
        }
    }

    visited.len()
}

#[test]
fn part1() {
    let answer = move_rope(2);
    assert_eq!(answer, 5874);
}

#[test]
fn part2() {
    let answer = move_rope(10);
    assert_eq!(answer, 2467);
}