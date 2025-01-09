use std::collections::{HashMap, HashSet};

use aoc_common::{file_lines, Grid, PriorityQueue, Vec2i32};

const DIRS: [(Vec2i32, u8); 4] = [
    (Vec2i32::new( 0,  1), b'v'),
    (Vec2i32::new( 1,  0), b'>'),
    (Vec2i32::new(-1,  0), b'<'),
    (Vec2i32::new( 0, -1), b'^'),
];

fn num_pad() -> Grid<u8> {
    Grid::new(vec![
        vec![b'7', b'8', b'9'],
        vec![b'4', b'5', b'6'],
        vec![b'1', b'2', b'3'],
        vec![   0, b'0', b'A'],
    ])
}

fn dir_pad() -> Grid<u8> {
    Grid::new(vec![
        vec![   0, b'^', b'A'],
        vec![b'<', b'v', b'>'],
    ])
}

fn input() -> Vec<Vec<u8>> {
    file_lines("inputs/day21.txt").map(|l| l.into_bytes()).collect()
}

type KeyMap = HashMap<(u8, u8), i64>;

#[derive(Clone, Copy, Default)]
struct Search {
    pos: Vec2i32,
    prev_input: u8,
    pressed: bool,
    keys: i64,
}

fn calc_output_keystrokes(map: &Grid<u8>, input_keystrokes: &KeyMap) -> KeyMap {
    let mut outputs = KeyMap::new();

    for (ouptut_pos, output_key) in map.enumerate() {
        if *output_key == 0 {
            continue;
        }

        let mut queue: PriorityQueue<Search, i64> = PriorityQueue::new();
        let mut visisted: HashSet<(Vec2i32, u8, bool)> = HashSet::new();

        let start = Search {
            pos: ouptut_pos.cast(),
            prev_input: b'A',
            pressed: false,
            keys: 0,
        };
        queue.enqueue(start, 0);

        while let Some(current) = queue.dequeue() {
            if visisted.contains(&(current.pos, current.prev_input, current.pressed)) {
                continue;
            }
            visisted.insert((current.pos, current.prev_input, current.pressed));

            if current.pressed {
                if outputs.insert((*output_key, map[current.pos]), current.keys).is_some() {
                    panic!();
                }
            } else {
                if !(visisted.contains(&(current.pos, current.prev_input, true))) {
                    let mut next = current;
                    next.pressed = true;
                    next.keys += input_keystrokes.get(&(next.prev_input, b'A')).unwrap();
                    next.prev_input = b'A';
                    let priority = next.keys;
                    queue.enqueue(next, priority);
                }

                for (dir, dir_char) in DIRS {
                    let adj = current.pos + dir;
                    if !adj.is_in_bounds(map.bounds().cast()) || map[adj] == 0 || visisted.contains(&(adj, dir_char, false)) {
                        continue;
                    }

                    let mut next = current;
                    next.pos = adj;
                    next.keys += input_keystrokes.get(&(next.prev_input, dir_char)).unwrap();
                    next.prev_input = dir_char;
                    let priority = next.keys;
                    queue.enqueue(next, priority);
                }
            }
        }
    }

    outputs
}

fn run(robots: usize) -> i64 {
    let dir_pad = dir_pad();
    let num_pad = num_pad();

    // seed the map, 1 keystroke for every input
    let mut keymap = KeyMap::new();
    for c0 in dir_pad.iter() {
        if *c0 == 0 {
            continue;
        }
        for c1 in dir_pad.iter() {
            if *c1 == 0 {
                continue;
            }
            keymap.insert((*c0, *c1), 1);
        }
    }

    for _ in 0..robots {
        keymap = calc_output_keystrokes(&dir_pad, &keymap);
    }

    let keymap = calc_output_keystrokes(&num_pad, &keymap);

    let mut total = 0;
    for code in input() {
        let mut current = b'A';
        let mut path = 0;

        for c in code.iter().copied() {
            path += keymap.get(&(current, c)).unwrap();
            current = c;
        }

        let mut num = 0;
        for c in code.iter() {
            if c.is_ascii_digit() {
                num *= 10;
                num += (c - b'0') as i64;
            }
        }

        total += num * path;
    }

    total
}

#[test]
fn part1() {
    let answer = run(2);
    assert_eq!(answer, 246990);
}

#[test]
fn part2() {
    let answer = run(25);
    assert_eq!(answer, 306335137543664);
}