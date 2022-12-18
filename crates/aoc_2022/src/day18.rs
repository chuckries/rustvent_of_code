use std::collections::{HashSet, VecDeque};

use aoc_common::{file_lines, IteratorExt, Vec3, Vec3i32, Aabb};

fn input() -> Vec<Vec3i32> {
    file_lines("inputs/day18.txt").map(|l| {
        let split = l.split(',').map(|l| l.parse::<i32>().unwrap()).to_vec();
        Vec3::new(split[0], split[1], split[2])
    }).to_vec()
}

#[test]
fn part1() {
    let mut space: HashSet<Vec3i32> = HashSet::new();

    let mut total = 0;
    for new in input() {
        space.insert(new);

        total += 6;
        for adj in new.adjacent() {
            if space.contains(&adj) {
                total -= 2;
            }
        }
    }

    assert_eq!(total, 3470);
}

#[test]
fn part2() {
    let space: HashSet<Vec3i32> = input().into_iter().collect();

    let aabb = Aabb::<i32>::bounding(space.iter().copied());
    let aabb = Aabb::<i32>::new(aabb.p0() - Vec3i32::one(), aabb.p1() + Vec3i32::one());

    let mut queue: VecDeque<Vec3i32> = VecDeque::new();
    let mut visited: HashSet<Vec3i32> = HashSet::new();
    queue.push_back(aabb.p0());
    visited.insert(aabb.p0());

    let mut total = 0;
    while let Some(p) = queue.pop_front() {
        for adj in p.adjacent().filter(|p| aabb.contains(*p)) {
            if space.contains(&adj) {
                total += 1;
            } else if !visited.contains(&adj) {
                visited.insert(adj);
                queue.push_back(adj);
            }
        }
    }

    assert_eq!(total, 1986);
}