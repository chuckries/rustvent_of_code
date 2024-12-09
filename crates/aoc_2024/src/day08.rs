use std::collections::{HashMap, HashSet};

use aoc_common::{file_lines, gcf, IteratorExt, Vec2i32};

fn input() -> (HashMap<u8, Vec<Vec2i32>>, Vec2i32) {
    let map = file_lines("inputs/day08.txt").map(|l| l.into_bytes()).to_vec();
    let mut points: HashMap<u8, Vec<Vec2i32>> = HashMap::new();
    let bounds = Vec2i32::new(map[0].len() as i32, map.len() as i32);
    for j in 0..bounds.y as usize {
        for i in 0..bounds.x as usize {
            if map[j][i] != b'.' {
                points.entry(map[j][i]).or_default().push((i as i32, j as i32).into());
            }
        }
    }

    (points, bounds)
}

#[test]
fn part1() {
    let (points, bounds) = input();
    let mut positions: HashSet<Vec2i32> = HashSet::new();

    for v in points.values() {
        for i in 0..v.len() - 1 {
            for j in i + 1 .. v.len() {
                let a = v[i];
                let b = v[j];

                let delta = b - a;
                if (b + delta).is_in_bounds(bounds) {
                    positions.insert(b + delta);
                }
                if (a - delta).is_in_bounds(bounds) {
                    positions.insert(a - delta);
                }
            }
        }
    }

    let answer = positions.len();
    assert_eq!(answer, 400);
}

#[test]
fn part2() {
    let (points, bounds) = input();
    let mut positions: HashSet<Vec2i32> = HashSet::new();

    for v in points.values() {
        for i in 0..v.len() - 1 {
            for j in i + 1 .. v.len() {
                let a = v[i];
                let b = v[j];

                let mut delta = b - a;
                if delta.x == 0 {
                    delta.y = 1;
                } else if delta.y == 0 {
                    delta.x = 1;
                } else {
                    let gcf = gcf(delta.x, delta.y);
                    delta /= gcf;
                }

                let mut p = a;
                while p.is_in_bounds(bounds) {
                    positions.insert(p);
                    p += delta;
                }
                p = a - delta;
                while p.is_in_bounds(bounds) {
                    positions.insert(p);
                    p -= delta;
                }
            }
        }
    }

    let answer = positions.len();
    assert_eq!(answer, 1280);
}