use std::collections::{HashSet, HashMap};
use aoc_common::{Vec2i32, file_lines};

static DIRS: [([Vec2i32; 3], Vec2i32); 4] = [
    ([Vec2i32::new( 0, -1), Vec2i32::new( 1, -1), Vec2i32::new(-1, -1)], Vec2i32::new( 0, -1)),
    ([Vec2i32::new( 0,  1), Vec2i32::new( 1,  1), Vec2i32::new(-1,  1)], Vec2i32::new( 0,  1)),
    ([Vec2i32::new(-1,  0), Vec2i32::new(-1, -1), Vec2i32::new(-1,  1)], Vec2i32::new(-1,  0)),
    ([Vec2i32::new( 1,  0), Vec2i32::new( 1, -1), Vec2i32::new( 1,  1)], Vec2i32::new( 1,  0)),
];

fn dir_iter(idx: usize) -> impl Iterator<Item = &'static ([Vec2i32; 3], Vec2i32)> {
    DIRS[idx..4].iter().chain(DIRS[0..idx].iter())
}

fn input() -> HashSet<Vec2i32> {

    let mut elves: HashSet<Vec2i32> = HashSet::new();

    for (j, line) in file_lines("inputs/day23.txt").enumerate() {
        for (i, c) in line.char_indices() {
            if c == '#' {
                elves.insert(Vec2i32::new(i as i32, j as i32));
            }
        }
    }

    elves
}

#[test]
fn part1() {
    let mut elves = input();

    let mut proposed: HashMap<Vec2i32, usize> = HashMap::with_capacity(elves.len());
    let mut proposals: Vec<(Vec2i32, Vec2i32)> = Vec::with_capacity(elves.len());

    let mut dir_idx = 0;
    for _ in 0..10 {
        proposed.clear();
        proposals.clear();
        'outer: for current in elves.iter().cloned() {
            if current.surrounding_unbounded().all(|p| !elves.contains(&p)) {
                proposals.push((current, current));
                *proposed.entry(current).or_default() += 1;
            } else {
                for (dirs, dir) in dir_iter(dir_idx) {
                    if dirs.iter().all(|d| !elves.contains(&(current + d))) {
                        proposals.push((current, current + dir));
                        *proposed.entry(current + dir).or_default() += 1;
                        continue 'outer;
                    }
                }
                proposals.push((current, current));
                *proposed.entry(current).or_default() += 1;
            }
        }

        elves.clear();
        for (current, next) in proposals.iter() {
            if proposed[&next] >= 2 {
                elves.insert(*current);
            } else {
                elves.insert(*next);
            }
        }

        dir_idx = (dir_idx + 1) % 4;
    }

    let mut min = Vec2i32::new(i32::MAX, i32::MAX);
    let mut max = Vec2i32::new(i32::MIN, i32::MIN);

    for e in elves.iter().copied() {
        if e.x < min.x {
            min.x = e.x;
        }
        if e.x > max.x {
            max.x = e.x;
        }
        if e.y < min.y {
            min.y = e.y;
        }
        if e.y > max.y {
            max.y = e.y;
        }
    }

    let answer = (max.x - min.x + 1) * (max.y - min.y + 1) - elves.len() as i32;
    assert_eq!(answer, 4288);
}

#[test]
fn part2() {
    let mut elves = input();

    let mut proposed: HashMap<Vec2i32, usize> = HashMap::with_capacity(elves.len());
    let mut proposals: Vec<(Vec2i32, Vec2i32)> = Vec::with_capacity(elves.len());

    let mut dir_idx = 0;
    let mut round = 1;
    loop {
        proposed.clear();
        proposals.clear();
        'outer: for current in elves.iter().cloned() {
            if current.surrounding_unbounded().all(|p| !elves.contains(&p)) {
                proposals.push((current, current));
                *proposed.entry(current).or_default() += 1;
            } else {
                for (dirs, dir) in dir_iter(dir_idx) {
                    if dirs.iter().all(|d| !elves.contains(&(current + d))) {
                        proposals.push((current, current + dir));
                        *proposed.entry(current + dir).or_default() += 1;
                        continue 'outer;
                    }
                }
                proposals.push((current, current));
                *proposed.entry(current).or_default() += 1;
            }
        }

        let mut has_moved = false;
        elves.clear();
        for (current, next) in proposals.iter() {
            if proposed[&next] >= 2 {
                elves.insert(*current);
            } else {
                if next != current {
                    has_moved = true;
                }
                elves.insert(*next);
            }
        }

        if !has_moved {
            break;
        }

        round += 1;
        dir_idx = (dir_idx + 1) % 4;
    }

    assert_eq!(round, 940);
}