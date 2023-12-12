use std::collections::{HashSet, VecDeque};

use aoc_common::{Vec2us, file_lines, IteratorExt};

fn input() -> (Vec<Vec<char>>, Vec2us) {
    let map = file_lines("inputs/day10.txt").map(|l| l.chars().to_vec()).to_vec();
    let start = map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(move |(x, c)| {
            (x, y, *c)
        })
    }).flatten().find_map(|(x, y, c)| {
        if c == 'S' {
            Some(Vec2us::new(x, y))
        } else {
            None
        }
     }).unwrap();

    (map, start)
}

fn adjacent(p: Vec2us, c: char) -> [Vec2us; 2] {
    match c {
        '|' => [Vec2us::new(p.x, p.y - 1), Vec2us::new(p.x, p.y + 1)],
        '-' => [Vec2us::new(p.x - 1, p.y), Vec2us::new(p.x + 1, p.y)],
        'L' => [Vec2us::new(p.x, p.y - 1), Vec2us::new(p.x + 1, p.y)],
        'J' => [Vec2us::new(p.x, p.y - 1), Vec2us::new(p.x - 1, p.y)],
        '7' => [Vec2us::new(p.x - 1, p.y), Vec2us::new(p.x, p.y + 1)],
        'F' => [Vec2us::new(p.x + 1, p.y), Vec2us::new(p.x, p.y + 1)],
        _ => panic!()
    }
}

#[test]
fn part1() {
    let (map, start) = input();

    let mut visited: HashSet<Vec2us> = HashSet::new();
    let mut queue: VecDeque<(Vec2us, i32)> = VecDeque::new();

    match map[start.y - 1][start.x] {
        '|' | '7' | 'F' => queue.push_back((start - Vec2us::unit_y(), 1)),
        _ => ()
    };

    match map[start.y + 1][start.x] {
        '|' | 'J' | 'L' => queue.push_back((start + Vec2us::unit_y(), 1)),
        _ => ()
    };

    match map[start.y][start.x - 1] {
        '-' | 'F' | 'L' => queue.push_back((start - Vec2us::unit_x(), 1)),
        _ => ()
    };

    match map[start.y][start.x + 1] {
        '-' | '7' | 'J' => queue.push_back((start + Vec2us::unit_x(), 1)),
        _ => ()
    };

    visited.insert(start);

    let mut max = 0;
    while let Some((p, steps)) = queue.pop_front() {  
        max = steps;
        visited.insert(p);
        for s in adjacent(p, map[p.y][p.x]) {
            if !visited.contains(&s) {
                queue.push_back((s, steps + 1));
            }
        }
    }

    assert_eq!(6757, max);
}