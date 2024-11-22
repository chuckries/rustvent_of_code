use std::collections::HashSet;

use aoc_common::{Vec2i64, PriorityQueue};

use intcode::IntCode;

const DIRS: [Vec2i64; 4] = [
    Vec2i64::new( 0, -1),
    Vec2i64::new( 0,  1),
    Vec2i64::new(-1,  0),
    Vec2i64::new( 1,  0),
];
const OPPOSITES: [i64; 4] = [2, 1, 4, 3];

fn explore() -> (HashSet<Vec2i64>, Vec2i64) {
    let mut robot = IntCode::from_file("inputs/day15.txt");
    let mut map: HashSet<Vec2i64> = HashSet::new();
    let start = Vec2i64::zero();
    map.insert(start);
    let mut target = Vec2i64::zero();
    explore_recurse(&mut robot, &mut map, start, &mut target);
    (map, target)
}

fn explore_recurse(robot: &mut IntCode, map: &mut HashSet<Vec2i64>, pos: Vec2i64, target: &mut Vec2i64) {
    for cand in 0..4 {
        let new_pos = pos + DIRS[cand as usize];
        if map.contains(&new_pos) {
            continue;
        }

        match robot.run_input(&[cand + 1]).unwrap() {
            result @ (1 | 2) => {
                if result == 2 {
                    *target = new_pos;
                }

                map.insert(new_pos);
                explore_recurse(robot, map, new_pos, target);
                robot.run_input(&[OPPOSITES[cand as usize]]).unwrap();
            }
            _ => ()
        }
    }
}

#[test]
fn part1() {
    let (map, target) = explore();

    let mut to_visit: PriorityQueue<Vec2i64, usize> = PriorityQueue::new();
    to_visit.enqueue(Vec2i64::zero(), 0);

    let mut visited: HashSet<Vec2i64> = HashSet::new();

    let mut answer = 0;
    while let Some((current, dist)) = to_visit.dequeue_with_priority() {
        if current == target {
            answer = dist;
            break;
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        for adj in current.adjacent().filter(|adj| map.contains(adj)) {
            to_visit.enqueue(adj, dist + 1);
        }
    }

    assert_eq!(answer, 380);
}

#[test]
fn part2() {
    let (map, target) = explore();
    let mut current = vec![target];
    let mut visited: HashSet<Vec2i64> = HashSet::new();

    let mut count = 0;
    loop {
        for p in current.drain(..).collect::<Vec<_>>() {
            visited.insert(p);
            for adj in p.adjacent().filter(|p|map.contains(p) && !visited.contains(p)) {
                current.push(adj);
            }
        }

        if current.len() > 0 {
            count += 1;
        } else {
            break;
        }
    }

    assert_eq!(count, 410);
}