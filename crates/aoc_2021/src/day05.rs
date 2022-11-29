use std::collections::HashMap;

use aoc_common::file_lines;

type Point = (i32, i32);

fn input() -> Vec<(Point, Point)> {
    file_lines("inputs/day05.txt").map(|l| {
        let mut split = l.split(" -> ");
        let mut left = split.next().unwrap().split(',');
        let left = (left.next().unwrap().parse::<i32>().unwrap(), left.next().unwrap().parse::<i32>().unwrap());
        let mut right = split.next().unwrap().split(',');
        let right = (right.next().unwrap().parse::<i32>().unwrap(), right.next().unwrap().parse::<i32>().unwrap());
        (left, right)
    }).collect()
}

fn non_diagonals(lines: &Vec<(Point, Point)>, map: &mut HashMap<Point, i32>) {
    for ((x0, y0), (x1, y1)) in lines.iter().cloned() {
        if x0 == x1 {
            let range = if y0 < y1 { y0..=y1 } else { y1..=y0 };

            for i in range {
                *map.entry((x0, i)).or_default() += 1;
            }
        } else if y0 == y1 {
            let range = if x0 < x1 { x0..=x1 } else { x1..= x0 };

            for i in range {
                *map.entry((i, y0)).or_default() += 1;
            }
        }
    }
}

fn diagonals(lines: &Vec<(Point, Point)>, map: &mut HashMap<Point, i32>) {
    for ((x0, y0), (x1, y1)) in lines.iter().filter(|(p0, p1)| p0.0 != p1.0 && p0.1 != p1.1).cloned() {
        let (left, right) = if x0 < x1 { ((x0, y0), (x1, y1)) } else { ((x1, y1), (x0, y0)) };

        if left.1 < right.1 {
            for p in (left.0..=right.0).zip(left.1..=right.1) {
                *map.entry(p).or_default() += 1;
            }
        } else {
            for p in (left.0..=right.0).zip((right.1..=left.1).rev()) {
                *map.entry(p).or_default() += 1;
            }
        }

    }
}

#[test]
fn part1() {
    let input = input();
    let mut map: HashMap<Point, i32>  = HashMap::new();

    non_diagonals(&input, &mut map);

    let answer = map.values().filter(|v| **v > 1).count();
    assert_eq!(answer, 5576)
}

#[test]
fn part2() {
    let input = input();
    let mut map: HashMap<Point, i32> = HashMap::new();

    non_diagonals(&input, &mut map);
    diagonals(&input, &mut map);

    let answer = map.values().filter(|v| **v > 1).count();
    assert_eq!(answer, 18144);
}