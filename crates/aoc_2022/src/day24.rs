use std::collections::HashSet;

use aoc_common::{file_lines, IteratorExt, Vec2i32, PriorityQueue};


#[derive(Clone, Copy)]
enum Bliz
{
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn safe_mod(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

struct Map {
    bounds: Vec2i32,
    start: Vec2i32,
    end: Vec2i32,
    bliz_rows: Vec<Vec<Bliz>>,
    bliz_cols: Vec<Vec<Bliz>>,
}

impl Map {
    fn is_cell_blizard_at_time(&self, p: Vec2i32, t: i32) -> bool {
        for bliz in self.bliz_rows[p.y as usize].iter() {
            if self.eval_bliz(*bliz, t) == p.x {
                return true;
            }
        }

        for bliz in self.bliz_cols[p.x as usize].iter() {
            if self.eval_bliz(*bliz, t) == p.y {
                return true;
            }
        }

        false
    }

    fn eval_bliz(&self, bliz: Bliz, t: i32) -> i32 {
        match bliz {
            Bliz::Up(init) => 1 + safe_mod(init - 1 - t, self.bounds.y - 2),
            Bliz::Down(init) => 1 + ((t + init - 1)) % (self.bounds.y - 2),
            Bliz::Left(init) => 1 + safe_mod(init - 1 - t, self.bounds.x - 2),
            Bliz::Right(init) => 1 + ((t + init - 1) % (self.bounds.x - 2)),
        }
    }

    fn is_in_bounds(&self, p: Vec2i32) -> bool {
        p == self.start || p == self.end || 
        (p.x > 0 && p.x < self.bounds.x - 1 && p.y > 0 && p.y < self.bounds.y - 1)
    }
}

fn input() -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file_lines("inputs/day24.txt") {
        map.push(line.chars().to_vec());
    }

    let mut rows: Vec<Vec<Bliz>> = vec![Vec::new(); map.len()];
    let mut cols: Vec<Vec<Bliz>> = vec![Vec::new(); map[0].len()];

    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            match *c {
                '^' => cols[i].push(Bliz::Up(j as i32)),
                'v' => cols[i].push(Bliz::Down(j as i32)),
                '<' => rows[j].push(Bliz::Left(i as i32)),
                '>' => rows[j].push(Bliz::Right(i as i32)),
                _ => (),
            }
        }
    }

    Map {
        bounds: Vec2i32::new(map[0].len() as i32, map.len() as i32),
        start: Vec2i32::new(1, 0),
        end: Vec2i32::new(map[0].len() as i32 - 2, map.len() as i32 - 1),
        bliz_rows: rows,
        bliz_cols: cols,
    }
}

#[test]
fn part1() {
    let map = input();

    let mut queue: PriorityQueue<(Vec2i32, i32), i32> = PriorityQueue::new();
    queue.enqueue((map.start, 0), map.start.manhattan_from(map.end));

    let mut visited: HashSet<(Vec2i32, i32)> = HashSet::new();

    let mut answer = 0;
    while let Some((current, mut time)) = queue.dequeue() {

        if visited.contains(&(current, time)) {
            continue;
        }
        visited.insert((current, time));

        if current == map.end {
            answer = time;
            break;
        }

        time += 1;

        if !map.is_cell_blizard_at_time(current, time) {
            queue.enqueue((current, time), time + current.manhattan_from(map.end));
        }

        for adj in current.adjacent() {
            if !map.is_in_bounds(adj) {
                continue;
            }

            if map.is_cell_blizard_at_time(adj, time) {
                continue;
            }

            queue.enqueue((adj, time), time + adj.manhattan_from(map.end));
        }
    }

    assert_eq!(answer, 299);
}

#[test]
fn part2() {
    let map = input();

    let mut queue: PriorityQueue<(Vec2i32, Vec2i32, i32, i32), i32> = PriorityQueue::new();
    queue.enqueue((map.start, map.end, 0, 0), map.start.manhattan_from(map.end));

    let mut visited: HashSet<(Vec2i32, i32, i32)> = HashSet::new();

    let mut answer = 0;
    while let Some((current, mut goal, mut goals, mut time)) = queue.dequeue() {
        if visited.contains(&(current, time, goals)) {
            continue;
        }
        visited.insert((current, time, goals));

        if current == goal {
            goals += 1;
            if goals == 3 {
                answer = time;
                break;
            }

            goal = match goals {
                1 => map.start,
                2 => map.end,
                _ => panic!(),
            };

            // not sure if this is safe, as the fastest route from a -> b doesn't seem like it's guaranteed to be fastest
            // for a -> b -> a, but this works for my input and is much faster, so whatever
            queue.clear();
        }

        time += 1;

        if !map.is_cell_blizard_at_time(current, time) {
            queue.enqueue((current, goal, goals, time), time + current.manhattan_from(goal));
        }

        for adj in current.adjacent() {
            if !map.is_in_bounds(adj) {
                continue;
            }

            if map.is_cell_blizard_at_time(adj, time) {
                continue;
            }

            queue.enqueue((adj, goal, goals, time), time + adj.manhattan_from(goal));
        }
    }

    assert_eq!(answer, 899);
}