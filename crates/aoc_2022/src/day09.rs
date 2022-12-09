use std::collections::HashSet;

use aoc_common::{Vec2i32, file_lines, IteratorExt};



fn input() -> impl Iterator<Item = (Vec2i32, i32)> {
    file_lines("inputs/day09.txt").map(|l| {
        let split = l.split(' ').to_vec();

        let count: i32 = split[1].parse().unwrap();

        let dir = match split[0] {
            "L" => -Vec2i32::unit_x(),
            "R" => Vec2i32::unit_x(),
            "U" => -Vec2i32::unit_y(),
            "D" => Vec2i32::unit_y(),
            _ => panic!()
        };

        (dir, count)
    })
}

fn fix(head: &mut Vec2i32, tail: &mut Vec2i32) {
    if head != tail {
        if head.x == tail.x {
            let diff = head.y - tail.y;
            if diff.abs() > 1 {
                tail.y += diff.signum();
            }
        } else if head.y == tail.y {
            let diff = head.x - tail.x;
            if diff.abs() > 1 {
                tail.x += diff.signum();
            }
        } else if head.manhattan_from(*tail) > 2 {
            *tail = match (*head - *tail).into() {
                ( 2,  1) => (head.x - 1, head.y),
                (-2,  1) => (head.x + 1, head.y),
                ( 2, -1) => (head.x - 1, head.y),
                (-2, -1) => (head.x + 1, head.y),
                ( 1,  2) => (head.x, head.y - 1),
                (-1,  2) => (head.x, head.y - 1),
                ( 1, -2) => (head.x, head.y + 1),
                (-1, -2) => (head.x, head.y + 1),
                ( 2,  2) => (head.x - 1, head.y - 1),
                (-2,  2) => (head.x + 1, head.y - 1),
                ( 2, -2) => (head.x - 1, head.y + 1),
                (-2, -2) => (head.x + 1, head.y + 1),
                _ => panic!(),
            }.into()
        }
    }
}

#[test]
fn part1() {
    let mut head = Vec2i32::zero();
    let mut tail = Vec2i32::zero();
    let mut visited: HashSet<Vec2i32> = HashSet::new();
    visited.insert(tail);

    for (dir, count) in input() {
        for _ in 0..count {
            head += dir;
            fix(&mut head, &mut tail);
            visited.insert(tail);
        }
    }

    let answer = visited.len();
    assert_eq!(answer, 5874);
}

#[test]
fn part2() {
    let mut knots = vec![Vec2i32::zero(); 10];
    let mut visited: HashSet<Vec2i32> = HashSet::new();
    visited.insert(Vec2i32::zero());

    for (dir, count) in input() {
        for _ in 0..count {
            knots[0] += dir;

            for i in 0..knots.len() - 1 {
                let mut head = knots[i];
                let mut tail = knots[i + 1];

                fix(&mut head, &mut tail);

                knots[i] = head;
                knots[i + 1] = tail;
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    let answer = visited.len();
    assert_eq!(answer, 2467);
}