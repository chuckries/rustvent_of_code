use std::collections::VecDeque;

use aoc_common::{file_lines, IteratorExt, Vec2us};

fn input() -> Vec<Vec<u8>> {
    file_lines("inputs/day12.txt").map(|l| l.into_bytes().to_vec()).to_vec()
}

#[test]
fn part1() {
    let map = input();
    let bounds = Vec2us::new(map[0].len(), map.len());

    let mut visited = vec![vec![false; bounds.x]; bounds.y];

    fn explore(start: Vec2us, map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, bounds: Vec2us, area: &mut i32, perimeter: &mut i32) {
        let mut queue: VecDeque<Vec2us> = VecDeque::new();
        queue.push_back(start);
        let c = map[start.y][start.x];
        visited[start.y][start.x] = true;
        *area += 1;

        while let Some(current) = queue.pop_front() {
            *perimeter += 4;
            for adj in current.adjacent_bounded(&bounds) {
                if map[adj.y][adj.x] == c {
                    *perimeter -= 1;
                
                    if !visited[adj.y][adj.x] {
                        *area += 1;
                        visited[adj.y][adj.x] = true;
                        queue.push_back(adj);
                    }
                }
            }
        }
    }

    let mut total = 0;
    for j in 0..bounds.y {
        for i in 0..bounds.x {
            if !visited[j][i] {
                let mut area = 0;
                let mut perimeter = 0;

                explore((i, j).into(), &map, &mut visited, bounds, &mut area, &mut perimeter);

                total += area * perimeter;
            }
        }
    }

    assert_eq!(total, 1402544);
}