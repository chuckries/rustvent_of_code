use std::{cmp::Ordering, collections::{HashMap, VecDeque}};

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

#[test]
fn part2() {
    let input = input();
    // to avoid a lot of painful bounds checking, we are going to copy the map into a larger one that has a border of 0's
    let mut map = vec![vec![0; input[0].len() + 2]; input.len() + 2];
    for j in 0..input.len() {
        for i in 0..input[0].len() {
            map[j + 1][i + 1] = input[j][i];
        }
    }
    let map = map;

    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    fn explore(start: Vec2us, map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, area: &mut i32, edges: &mut Vec<Vec2us>) {
        let mut queue: VecDeque<Vec2us> = VecDeque::new();
        queue.push_back(start);
        let c = map[start.y][start.x];
        visited[start.y][start.x] = true;
        *area += 1;

        while let Some(current) = queue.pop_front() {
            let mut perimeter = 0;
            for adj in current.adjacent() {
                if map[adj.y][adj.x] == c {
                    if !visited[adj.y][adj.x] {
                        *area += 1;
                        visited[adj.y][adj.x] = true;
                        queue.push_back(adj);
                    }
                } else {
                    perimeter += 1;
                }
            }
            if perimeter > 0 {
                edges.push(current);
            }
        }
    }

    struct Side(Vec2us, Vec2us);

    fn count_sides(edges: &[Vec2us], map: &Vec<Vec<u8>>) -> i32 {
        const N: u8 = 0;
        const E: u8 = 1;
        const S: u8 = 2;
        const W: u8 = 3;

        let c = map[edges[0].y][edges[0].x];
        let mut sides: Vec<Side> = Vec::new();
        let mut side_map: HashMap<(Vec2us, u8), usize> = HashMap::new();
    
        for pos in edges.iter().cloned() {
            // does pos contribute to a north edge?
            if map[pos.y - 1][pos.x] != c {
                // do we have an existing edge to extend?
                if let Some(idx) = side_map.get(&((pos.x - 1, pos.y).into(), N)) {
                    sides[*idx].1 = pos;
                    side_map.insert((pos, N), *idx);
                } else {
                    let idx = sides.len();
                    sides.push(Side(pos, pos));
                    side_map.insert((pos, N), idx);
                }
            }

            // does pos contribute to a south edge?
            if map[pos.y + 1][pos.x] != c {
                // do we have an existing edge to extend?
                if let Some(idx) = side_map.get(&((pos.x - 1, pos.y).into(), S)) {
                    sides[*idx].1 = pos;
                    side_map.insert((pos, S), *idx);
                } else {
                    let idx = sides.len();
                    sides.push(Side(pos, pos));
                    side_map.insert((pos, S), idx);
                }
            }

            // does pos contribute to a west edge?
            if map[pos.y][pos.x - 1] != c {
                // do we have an existing edge to extend?
                if let Some(idx) = side_map.get(&((pos.x, pos.y - 1).into(), W)) {
                    sides[*idx].1 = pos;
                    side_map.insert((pos, W), *idx);
                } else {
                    let idx = sides.len();
                    sides.push(Side(pos, pos));
                    side_map.insert((pos, W), idx);
                }
            }

            // does pos contribute to a east edge?
            if map[pos.y][pos.x + 1] != c {
                // do we have an existing edge to extend?
                if let Some(idx) = side_map.get(&((pos.x, pos.y - 1).into(), E)) {
                    sides[*idx].1 = pos;
                    side_map.insert((pos, E), *idx);
                } else {
                    let idx = sides.len();
                    sides.push(Side(pos, pos));
                    side_map.insert((pos, E), idx);
                }
            }
        }

        sides.len() as i32
    }

    let mut total = 0;
    for j in 1..map.len() - 1 {
        for i in 1..map.len() - 1 {
            if !visited[j][i] {
                let mut area = 0;
                let mut edges: Vec<Vec2us> = Vec::new();

                explore((i, j).into(), &map, &mut visited, &mut area, &mut edges);

                edges.sort_by(|lhs, rhs| {
                    let mut ord = lhs.y.cmp(&rhs.y);
                    if ord == Ordering::Equal {
                        ord = lhs.x.cmp(&rhs.x)
                    }
                    ord
                });

                let sides = count_sides(&edges, &map);

                total += area * sides;
            }
        }
    }

    assert_eq!(total, 0);
}