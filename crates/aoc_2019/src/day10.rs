use std::collections::HashSet;

use aoc_common::{file_lines, Vec2i32, gcf};

type Points = HashSet<Vec2i32>;

fn input() -> Points {
    file_lines("inputs/day10.txt")
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, c)| {
                if c == '#' {
                    Some(Vec2i32::new(x as i32, y as i32))
                } else {
                    None
                }
            }).collect::<Vec<_>>()
        }).collect()
}

fn reduce(p: Vec2i32) -> Vec2i32 {
    let reduced = match (p.x, p.y) {
        (0, 0) => panic!(),
        (x, y) if x == 0 || y == 0 => (i32::signum(x), i32::signum(y)),
        (x, y) => {
            let gcf = gcf(i32::abs(x), i32::abs(y));
            (p / gcf).into()
        }
    };

    reduced.into()
}

fn station() -> (Vec2i32, Points) {
    let points = input();
    points.iter()
        .map(|cand| {
            let points = points.iter().filter(|p| **p != *cand).map(|p| {
                reduce(*p - *cand)
            }).collect::<Points>();
            (*cand, points)
        })
        .max_by_key(|(_origin, in_sight)| in_sight.len())
        .unwrap()
}

#[test]
fn part1() {
    let answer = station().1.len();
    assert_eq!(answer, 340);
}

#[test]
fn part2() {
    let (station, in_sight) = station();
    assert!(in_sight.len() >= 200);

    let in_sight: Vec<Vec2i32> = in_sight.into_iter().collect();

    let mut in_sight = in_sight.into_iter().map(|p| {
        let quad = match (i32::signum(p.x), i32::signum(p.y)) {
            ( 0, -1) => 0,
            ( 1, -1) => 1,
            ( 1,  0) => 2,
            ( 1,  1) => 3,
            ( 0,  1) => 4,
            (-1,  1) => 5,
            (-1,  0) => 6,
            (-1, -1) => 7,
            _ => unreachable!()
        };

        (p, quad)
    }).collect::<Vec<_>>();

    in_sight.sort_by(|(a, a_quad), (b, b_quad)| {
        let mut sort_value = *a_quad - *b_quad;
        if sort_value == 0 {
            let left_top = i32::abs(a.y * b.x);
            let right_top = i32::abs(b.y * a.x);

            sort_value = if i32::signum(a.x) == i32::signum(a.y) {
                left_top - right_top
            } else {
                right_top - left_top
            }
        }
        match sort_value {
            0 => std::cmp::Ordering::Equal,
            i if i < 0 => std::cmp::Ordering::Less,
            i if i > 0 => std::cmp::Ordering::Greater,
            _ => unreachable!(),
        }
    });

    let answer = station + in_sight[199].0;
    let answer = answer.x * 100 + answer.y;
    assert_eq!(answer, 2628);
}