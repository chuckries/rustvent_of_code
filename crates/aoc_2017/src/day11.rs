use aoc_common::{file_string, Vec3i32};

// https://www.redblobgames.com/grids/hexagons/

fn input() -> Vec<Vec3i32> {
    file_string("inputs/day11.txt").split(',').map(|s| {
        match s {
            "n" => Vec3i32::new(0, -1, 1),
            "ne" => Vec3i32::new(1, -1,0),
            "se" => Vec3i32::new(1, 0, -1),
            "s" => Vec3i32::new(0, 1, -1),
            "sw" => Vec3i32::new(-1, 1, 0),
            "nw" => Vec3i32::new(-1, 0, 1),
            _ => panic!()
        }
    }).collect()
}

#[test]
fn part1() {
    let pos = input().into_iter().sum::<Vec3i32>();
    let answer = pos.manhattan() / 2;
    assert_eq!(answer, 670);
}

#[test]
fn part2() {
    let input = input();

    let mut p = Vec3i32::zero();
    let mut max = 0;
    for delta in input {
        p += delta;
        let dist = p.manhattan() / 2;
        if dist > max {
            max = dist;
        }
    }

    assert_eq!(max, 1426);
}