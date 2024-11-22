use std::collections::HashMap;

use aoc_common::{Vec2i32, map_points_to_string};

use intcode::{IntCode, IntCodeResult};

fn run(start: bool) -> HashMap<Vec2i32, bool> {
    let mut robot = IntCode::from_file("inputs/day11.txt");
    let mut map: HashMap<Vec2i32, bool> = HashMap::new();
    let mut pos = Vec2i32::zero();
    let mut dir = -Vec2i32::unit_y();

    map.insert(pos, start);

    loop {
        let panel = map.entry(pos).or_default();
        robot.push_input_back(if *panel { 1 } else { 0 });

        if let IntCodeResult::Output(output) = robot.run() {
            if output == 1 {
                *panel = true;
            } else {
                *panel = false;
            }

            let turn = robot.run().unwrap();
            dir = if turn == 0 {
                dir.rotated_left()
            } else {
                dir.rotated_right()
            };
            pos += dir;
        } else {
            break;
        }
    }

    map
}

#[test]
fn part1() {
    let answer = run(false).len();
    assert_eq!(answer, 2018);
}

#[test]
fn part2() {
    let map = run(true);

    let answer = map_points_to_string(map.iter().filter_map(|p| {
        if *p.1 {
            Some(p.0)
        } else {
            None
        }
    }).copied());

    let known = "
 ██  ███  ████ █  █ ███  █  █ ███  ███ 
█  █ █  █ █    █ █  █  █ █ █  █  █ █  █
█  █ █  █ ███  ██   █  █ ██   ███  █  █
████ ███  █    █ █  ███  █ █  █  █ ███ 
█  █ █    █    █ █  █ █  █ █  █  █ █ █ 
█  █ █    █    █  █ █  █ █  █ ███  █  █";

    assert_eq!(answer, known);
}