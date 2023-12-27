use std::collections::{VecDeque, HashSet};

use aoc_common::{file_2d_map, Vec2i32, Vec2us};

fn input() -> Vec<Vec<char>> {
    file_2d_map("inputs/day23.txt")
}

#[test]
fn part1() {
    let map = input();

    let start = Vec2i32::new(1, 0);
    let dir = Vec2i32::unit_y();

    let target = Vec2i32::new(map[0].len() as i32 - 2, map.len() as i32 - 1);

    let mut queue: VecDeque<(Vec2i32, Vec2i32, usize)> = VecDeque::new();
    queue.push_back((start, dir, 0));

    let mut answer = 0;
    while let Some((p, dir, len)) = queue.pop_front() {       
        if p == target {
            if len > answer {
                answer = len;
            }
            continue;
        }

        for dir in [dir, dir.rotated_left(), dir.rotated_right()] {
            let adj = p + dir;

            let c = map[adj.y as usize][adj.x as usize];
            if c == '.' || 
                (c == '<' && dir == -Vec2i32::unit_x()) ||
                (c == '>' && dir == Vec2i32::unit_x()) ||
                (c == '^' && dir == -Vec2i32::unit_y()) ||
                (c == 'v' && dir == Vec2i32::unit_y()) {
                    queue.push_back((adj, dir, len + 1));
                }
        }
    }

    assert_eq!(2246, answer);
}