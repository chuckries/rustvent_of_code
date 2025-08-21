use aoc_common::{file_lines, map_points_to_string, IteratorExt, Vec2i32};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"position=<\s*(-?\d+), \s*(-?\d+)> velocity=<\s*(-?\d+), \s*(-?\d+)>").unwrap();
}

fn input() -> (Vec<Vec2i32>, Vec<Vec2i32>) {
    file_lines("inputs/day10.txt").map(|l| {
        let c = RE.captures(&l).unwrap();
        (Vec2i32::new(c[1].parse().unwrap(), c[2].parse().unwrap()), Vec2i32::new(c[3].parse().unwrap(), c[4].parse().unwrap()))
    }).unzip()
}

fn run() -> (Vec<Vec2i32>, i32) {
    let (mut points, velos) = input();

    let mut ticks = 0;
    let mut area = i64::MAX;
    loop {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y: i32 = i32::MIN;

        let next_points = points.iter().enumerate().map(|(idx, p)| {
            let next = *p + velos[idx];

            min_x = min_x.min(next.x);
            max_x = max_x.max(next.x);
            min_y = min_y.min(next.y);
            max_y = max_y.max(next.y);

            next
        }).to_vec();

        let next_area = (max_x as i64 - min_x as i64) * (max_y as i64 - min_y as i64);
        if area != i64::MAX && next_area > area {
            break;
        }

        ticks += 1;
        area = next_area;
        points = next_points;
    }

    (points, ticks)
}

#[test]
fn part1() {
    let (points, _) = run();
    let answer = map_points_to_string(points.into_iter());

    let expected = "
█████   █    █  ██████  ██████   ████   ██████  ██████  █    █
█    █  █    █  █            █  █    █  █            █  █    █
█    █  █    █  █            █  █       █            █   █  █ 
█    █  █    █  █           █   █       █           █    █  █ 
█████   ██████  █████      █    █       █████      █      ██  
█       █    █  █         █     █       █         █       ██  
█       █    █  █        █      █       █        █       █  █ 
█       █    █  █       █       █       █       █        █  █ 
█       █    █  █       █       █    █  █       █       █    █
█       █    █  █       ██████   ████   ██████  ██████  █    █";

    assert_eq!(expected, answer);
}

#[test]
fn part2() {
    let (_, answer) = run();
    assert_eq!(answer, 10634);
}