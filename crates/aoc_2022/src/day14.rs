use std::collections::VecDeque;
use aoc_common::{file_lines, IteratorExt, Vec2us, Vec2i32};

type Map = Vec<Vec<char>>;

const SOURCE: Vec2us = Vec2us::new(500, 0);
const DIRS: [Vec2i32; 3] = [Vec2i32::new(0, 1), Vec2i32::new(-1, 1), Vec2i32::new(1, 1)];

fn dirs(p: Vec2us) -> impl Iterator<Item = Vec2us> {
    DIRS.iter().map(move |d| (p.cast::<i32>() + *d).cast::<usize>())
}

fn input() -> (Vec<Vec<Vec2us>>, (Vec2us, Vec2us)) {
    let lines = file_lines("inputs/day14.txt").map(|l| {
        l.split(" -> ").map(|s| {
            let split = s.split(',').map(|i| i.parse::<usize>().unwrap()).to_vec();
            Vec2us::new(split[0], split[1])
        }).to_vec()
    }).to_vec();

    let bounding_rect = Vec2us::bounding_box(lines.iter().flatten().copied());

    (lines, bounding_rect)
}

fn get_map_from_rect(lines: &Vec<Vec<Vec2us>>, low: Vec2us, high: Vec2us) -> (Map, Vec2us) {
    let width = high.x - low.x + 1;
    let height = high.y + 1;
    let offset: Vec2us = (low.x, 0).into();

    let mut map = vec![vec!['.'; width]; height];

    for line in lines {
        for w in line.windows(2) {
            let p0 = w[0] - offset;
            let p1 = w[1] - offset;

            for p in p0.area(p1) {
                map[p.y][p.x] = '#';
            }
        }
    }

    let source = SOURCE - offset;
    map[source.y][source.x] = '+';

    (map, source)
}

fn get_min_bounded_map() -> (Map, Vec2us) {
    let (lines, (mut low, mut high)) = input();

    low.x -= 1;
    high.x += 1;

    get_map_from_rect(&lines, low, high)
}

fn get_max_bounded_map() -> (Map, Vec2us) {
    let (lines, (mut low, mut high)) = input();

    high.y += 2;
    low.x = SOURCE.x - high.y;
    high.x = SOURCE.x + high.y;

    let (mut map, source) = get_map_from_rect(&lines, low, high);

    for i in 0..map[high.y].len() {
        map[high.y][i] = '#';
    }

    (map, source)
}

struct StackNode
{
    p: Vec2us,
    i: usize,
}

impl StackNode {
    fn new(p: Vec2us) -> StackNode {
        StackNode {
            p,
            i: 0,
        }
    }
}

impl Iterator for StackNode {
    type Item = Vec2us;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= DIRS.len() {
            None
        } else {
            self.i += 1;
            Some((self.p.cast::<i32>() + DIRS[self.i - 1]).cast::<usize>())
        }
    }
}

fn fill_sand(map: &mut Map, source: Vec2us) -> usize {
    let mut stack: Vec<StackNode> = Vec::new();
    stack.push(StackNode::new(source));

    'outer: while let Some(current) = stack.last_mut() {
        if current.p.y == map.len() - 1 {
            break;
        }

        while let Some(next) = current.next() {
            if map[next.y][next.x] == '.' {
                stack.push(StackNode::new(next));
                continue 'outer;
            }
        }

        map[current.p.y][current.p.x] = 'o';
        stack.pop();
    }

    map.iter().flatten().filter(|c| **c == 'o').count()
}

fn _print_map(map: &Map) {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

#[test]
fn part1() {
    let (mut map, source) = get_min_bounded_map();
    let answer = fill_sand(&mut map, source);
    assert_eq!(answer, 638);
}

#[test]
fn part2() {
    let (mut map, source) = get_max_bounded_map();

    let mut queue: VecDeque<Vec2us> = VecDeque::new();
    queue.push_back(source);
    map[source.y][source.x] = 'o';

    while let Some(idx) = queue.pop_front() {
        for adj in dirs(idx) {
            if map[adj.y][adj.x] == '.' {
                map[adj.y][adj.x] = 'o';
                queue.push_back(adj);
            }
        }
    }

    let answer = map.iter().flatten().filter(|c| **c == 'o').count();
    assert_eq!(answer, 31722);
}