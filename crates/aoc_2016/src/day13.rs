use std::collections::{HashMap, VecDeque, HashSet};
use aoc_common::Vec2us;

const INPUT: usize = 1364;

fn run(target: Vec2us, max_steps: i32) -> (i32, usize) {
    let mut map: HashMap<Vec2us, bool> = HashMap::new();

    fn get(idx: Vec2us, map: &mut HashMap<Vec2us, bool>) -> bool {
        *map.entry(idx).or_insert_with(|| {
            let mut val = idx.x * idx.x  + 3 * idx.x + 2 * idx.x * idx.y + idx.y + idx.y * idx.y;
            val += INPUT;
            val.count_ones() & 1 == 0
        })
    }

    let origin: Vec2us = (1, 1).into();
    let mut queue: VecDeque<(Vec2us, i32)> = VecDeque::new();
    let mut visited: HashSet<Vec2us> = HashSet::new();

    queue.push_back((origin, 0));
    visited.insert(origin);

    while let Some((current, dist)) = queue.pop_front() {
        if current == target {
            return (dist, 0);
        }

        if dist < max_steps {
            for adj in current.adjacent_non_negative() {
                if !visited.contains(&adj) && get(adj, &mut map) {
                    visited.insert(adj);
                    queue.push_back((adj, dist + 1));
                }
            }
        }
    }

    return (0, visited.len())
}

#[test]
fn part1() {

    let (answer, _) = run((31, 39).into(), i32::MAX);
    assert_eq!(answer, 86);
}

#[test]
fn part2() {
    let (_, answer) = run(Vec2us::max_value(), 50);
    assert_eq!(answer, 127);
}