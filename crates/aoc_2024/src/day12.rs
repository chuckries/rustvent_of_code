use std::collections::{HashMap, VecDeque};

use aoc_common::{file_lines, IteratorExt, Vec2i32, Vec2us};

fn input() -> Vec<Vec<u8>> {
    file_lines("inputs/day12.txt").map(|l| l.into_bytes().to_vec()).to_vec()
}

trait Runner: Default {
    fn flood_fill_visit(&mut self, p: Vec2us, map: &Vec<Vec<u8>>);
    fn after_flood_fill(&mut self, map: &Vec<Vec<u8>>) -> i32;
}

fn run<T: Runner>() -> i32 {
    let input = input();

    // make a border of zero to make us not have to do bounds checking
    let mut map = vec![vec![0; input[0].len() + 2]; input.len() + 2];
    for j in 0..input.len() {
        for i in 0..input[0].len() {
            map[j + 1][i + 1] = input[j][i];
        }
    }
    let map = map;

    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    fn flood_fill<T: Runner>(start: Vec2us, map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, runner: &mut T) -> i32 {
        let mut queue: VecDeque<Vec2us> = VecDeque::new();
        queue.push_back(start);
        let c = map[start.y][start.x];
        visited[start.y][start.x] = true;
        let mut area = 0;
    
        while let Some(current) = queue.pop_front() {
            area += 1;
            runner.flood_fill_visit(current, map);
            for adj in current.adjacent() {
                if map[adj.y][adj.x] == c && !visited[adj.y][adj.x]{
                    visited[adj.y][adj.x] = true;
                    queue.push_back(adj);
                }
            }
        }
    
        area
    }

    let mut runner = T::default();
    let mut total = 0;
    for j in 1..map.len() - 1 {
        for i in 1..map[0].len() - 1 {
            if !visited[j][i] {
                let area = flood_fill((i, j).into(), &map, &mut visited, &mut runner);
                total += area * runner.after_flood_fill(&map);
            }
        }
    }

    total
}

#[derive(Default)]
struct Runner1 {
    perimeter: i32
}

impl Runner for Runner1 {
    fn flood_fill_visit(&mut self, p: Vec2us, map: &Vec<Vec<u8>>) {
        for adj in p.adjacent() {
            if map[adj.y][adj.x] != map[p.y][p.x] {
                self.perimeter += 1;
            }
        }
    }

    fn after_flood_fill(&mut self, _: &Vec<Vec<u8>>) -> i32 {
        let perimeter = self.perimeter;
        self.perimeter = 0;
        perimeter
    }
}

#[test]
fn part1() {
    let answer = run::<Runner1>();
    assert_eq!(answer, 1402544);
}

#[derive(Default)]
struct Runner2 {
    edges: Vec<Vec2us>
}

impl Runner2 {
    fn count_sides(&self, map: &Vec<Vec<u8>>) -> i32 {
        const N: u8 = 1 << 1;
        const E: u8 = 1 << 2;
        const S: u8 = 1 << 3;
        const W: u8 = 1 << 4;

        const TO_TEST: [(u8, Vec2i32, Vec2i32); 4] = [

            (N, Vec2i32::new( 0, -1), Vec2i32::new(-1,  0)),
            (S, Vec2i32::new( 0,  1), Vec2i32::new(-1,  0)),
            (W, Vec2i32::new(-1,  0), Vec2i32::new( 0, -1)),
            (E, Vec2i32::new( 1,  0), Vec2i32::new( 0, -1)),
        ];

        let c = map[self.edges[0].y][self.edges[0].x];
        let mut count_sides = 0;
        let mut sides: HashMap<Vec2i32, u8> = HashMap::new();
        for p in self.edges.iter().copied() {
            let p = p.cast::<i32>();
            for (mask, border_check, existing_edge_check) in TO_TEST {
                let neighbor = p + border_check;
                if map[neighbor.y as usize][neighbor.x as usize] != c {
                    let neighbor = p + existing_edge_check;
                    let existing = sides.get(&neighbor);
                    if existing.is_none_or(|existing| { *existing & mask == 0 }) {
                        count_sides += 1;
                    }

                    *sides.entry(p).or_default() |= mask;
                }
            }
        }

        count_sides
    }
}

impl Runner for Runner2 {
    fn flood_fill_visit(&mut self, p: Vec2us, map: &Vec<Vec<u8>>) {
        if p.adjacent().any(|adj| map[adj.y][adj.x] != map[p.y][p.x]) {
            self.edges.push(p);
        }
    }

    fn after_flood_fill(&mut self, map: &Vec<Vec<u8>>) -> i32 {
        self.edges.sort_by(Vec2us::grid_ordering);
        let sides = self.count_sides(map);
        self.edges.clear();
        sides
    }
}

#[test]
fn part2() {
    let answer = run::<Runner2>();
    assert_eq!(answer, 862486);
}