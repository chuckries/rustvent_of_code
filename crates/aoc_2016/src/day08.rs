use aoc_common::{file_lines, map_points_to_string, IteratorExt, Vec2us};

fn input() -> Vec<String> {
    file_lines("inputs/day08.txt").collect()
}

fn run() -> Vec<Vec2us> {
    let mut map = vec![vec![false; 50]; 6];
    for l in input() {
        let split = l.split([' ', '=']).to_vec();

        if split[0] == "rect" {
            let split = split[1].split('x').to_vec();
            let x: usize = split[0].parse().unwrap();
            let y: usize = split[1].parse().unwrap();

            for j in 0..y {
                for i in 0..x {
                    map[j][i] = true;
                }
            }
        } else if split[1] == "row" {
            let j: usize = split[3].parse().unwrap();
            let count: usize = split[5].parse().unwrap();
            let count = count % map[j].len();

            let mut rotated = vec![false; map[j].len()];
            for i in 0..map[j].len() {
                let mut rot = i + count;
                if rot >= rotated.len() {
                    rot -= rotated.len();
                }
                rotated[rot] = map[j][i];
            }
            for i in 0..rotated.len() {
                map[j][i] = rotated[i];
            }
        } else if split[1] == "column" {
            let i: usize = split[3].parse().unwrap();
            let count: usize = split[5].parse().unwrap();
            let count = count % map.len();

            let mut rotated = vec![false; map.len()];
            for j in 0..map.len() {
                let mut rot = j + count;
                if rot >= rotated.len() {
                    rot -= rotated.len();
                }
                rotated[rot] = map[j][i];
            }
            for j in 0..rotated.len() {
                map[j][i] = rotated[j];
            }
        } else {
            panic!();
        }
    }

    let mut points: Vec<Vec2us> = Vec::new();
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] {
                points.push((i, j).into());
            }
        }
    }

    points
}

#[test]
fn part1() {
    let points = run();
    let answer = points.len();

    assert_eq!(answer, 128);
}

#[test]
fn part2() {
    let points = run();
    let answer = map_points_to_string(points.into_iter());

const KNOWN: &str = "
████  ██   ██  ███   ██  ███  █  █ █   █ ██   ██ 
█    █  █ █  █ █  █ █  █ █  █ █  █ █   ██  █ █  █
███  █  █ █  █ █  █ █    █  █ ████  █ █ █  █ █  █
█    █  █ ████ ███  █ ██ ███  █  █   █  ████ █  █
█    █  █ █  █ █ █  █  █ █    █  █   █  █  █ █  █
████  ██  █  █ █  █  ███ █    █  █   █  █  █  ██ ";

    assert_eq!(answer, KNOWN);
}