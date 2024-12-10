use std::collections::HashSet;

use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day10.txt").map(|l| l.into_bytes().into_iter().map(|b| (b - b'0') as i32).to_vec()).to_vec()
}

#[test]
fn part1() {
    let map = input();
    let bounds: Vec2i32 = Vec2i32::new(map[0].len() as i32, map.len() as i32);

    fn test(p: Vec2i32, map: &Vec<Vec<i32>>, bounds: Vec2i32, n: i32, positions: &mut HashSet<Vec2i32>) {
        if n == 9 {
            positions.insert(p);
        } else {
            for adj in p.adjacent_bounded(&bounds) {
                if map[adj.y as usize][adj.x as usize] == n + 1 {
                    test(adj, map, bounds, n + 1, positions);
                }
            };
        }
    }

    let mut total = 0;
    let mut positions = HashSet::new();
    for j in 0..bounds.y {
        for i in 0..bounds.x {
            if map[j as usize][i as usize] == 0 {
                positions.clear();
                test((i, j).into(), &map, bounds, 0, &mut positions);
                total += positions.len();
            }
        }
    }

    assert_eq!(total, 482);
}

#[test]
fn part2() {
    let map = input();
    let bounds: Vec2i32 = Vec2i32::new(map[0].len() as i32, map.len() as i32);

    fn test(p: Vec2i32, map: &Vec<Vec<i32>>, bounds: Vec2i32, n: i32) -> i32 {
        if n == 9 {
            1
        } else {
            p.adjacent_bounded(&bounds).map(|adj| {
                if map[adj.y as usize][adj.x as usize] == n + 1 {
                    test(adj, map, bounds, n + 1)
                } else {
                    0
                }
            }).sum::<i32>()
        }
    }

    let mut total = 0;
    for j in 0..bounds.y {
        for i in 0..bounds.x {
            if map[j as usize][i as usize] == 0 {
                total += test((i, j).into(), &map, bounds, 0);
            }
        }
    }

    assert_eq!(total, 1094);
}