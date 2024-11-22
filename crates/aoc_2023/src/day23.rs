use std::collections::VecDeque;

use aoc_common::{file_2d_map, Vec2i32, Vec2us, IteratorExt};

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

#[test]
fn part2() {
    let mut input = input();
    for row in input.iter_mut() {
        for c in row.iter_mut() {
            if *c != '#' {
                *c = '.';
            }
        }
    }
    let input = input;

    let start = Vec2i32::new(1, 0);
    let end = Vec2i32::new(input[0].len() as i32 - 2, input.len() as i32 - 1);

    let mut intersections: Vec<Vec2i32> = Vec::new();
    for j in 1..input.len() - 1 {
        for i in 1..input[0].len() - 1 {
            if input[j][i] == '.' {
                if Vec2us::new(i, j).adjacent().filter(|adj| input[adj.y][adj.x] == '.').count() > 2 {
                    intersections.push((i as i32, j as i32).into());
                }
            }
        }
    }

    fn bfs(p: Vec2i32, map: &Vec<Vec<char>>, intersections: &[Vec2i32]) -> Vec<(usize, i32)> {
        let bounds = Vec2i32::new(map[0].len() as i32, map.len() as i32);
        let mut queue: VecDeque<(Vec2i32, Vec2i32, usize)> = VecDeque::new();
        for adj in p.adjacent_bounded(&bounds) {
            if map[adj.y as usize][adj.x as usize] == '.' {
                queue.push_back((adj, adj - p, 1));
            }
        }

        let mut edges: Vec<(usize, i32)> = Vec::new();
        while let Some((current, dir, len)) = queue.pop_front() {
            if let Some(idx) = intersections.iter().enumerate().find_map(|(idx, p)| {
                if *p == current {
                    Some(idx)
                } else {
                    None
                } 
            }) {
                edges.push((idx, len as i32));
            } else {
                for next_dir in [dir, dir.rotated_left(), dir.rotated_right()] {
                    let next = current + next_dir;
                    if next.is_in_bounds(bounds) && map[next.y as usize][next.x as usize] == '.' {
                        queue.push_back((next, next_dir, len + 1));
                    }
                }
            }
        }

        edges
    }

    let edges = intersections.iter().map(|int| bfs(*int, &input, &intersections)).to_vec();

    let (start_idx, start_len) = bfs(start, &input, &intersections)[0];
    let (end_idx, end_len) = bfs(end, &input, &intersections)[0];

    fn dfs_max(start: usize, end: usize, graph: &Vec<Vec<(usize, i32)>>) -> i32 {
        let mut stack: Vec<(usize, u64, i32)> = Vec::new();
        stack.push((start, 1 << start, 0));
        let mut max = 0;

        while let Some((current, state, len)) = stack.pop() {
            if current == end {
                if len > max {
                    max = len;
                }
            } else {
                for (adj, delta) in graph[current].iter().cloned() {
                    let flag = 1 << adj;
                    if (state & flag) == 0 {
                        stack.push((adj, state | flag, len + delta));
                    }
                }
            }
        }

        max
    }

    let max = dfs_max(start_idx, end_idx, &edges);
    let answer = max + start_len + end_len;
    assert_eq!(6622, answer);
}