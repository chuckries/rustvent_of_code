use aoc_common::{file_lines, Vec2us};
use regex::Regex;

fn input() -> Vec<(i32, Vec2us, Vec2us)> {
    let regex = r"^\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$";
    let regex = Regex::new(regex).unwrap();

    file_lines("inputs/day03.txt").map(|l| {
        let caputres = regex.captures(&l).unwrap();
        let n = caputres[1].parse().unwrap();
        let x = caputres[2].parse().unwrap();
        let y = caputres[3].parse().unwrap();
        let u = caputres[4].parse().unwrap();
        let v = caputres[5].parse().unwrap();

        (n, (x, y).into(), (u, v).into())
    }).collect()
}

#[test]
fn part1() {
    let mut map = vec![[0; 1000]; 1000];

    for (num, start, dims) in input() {
        for j in start.y .. start.y + dims.y {
            for i in start.x .. start.x + dims.x {
                let cell = &mut map[j][i];
                if *cell == 0 {
                    *cell = num;
                } else {
                    *cell = -1;
                }
            }
        }
    }

    let answer = map.iter().flatten().filter(|n| **n == -1).count();
    assert_eq!(110389, answer);
}

#[test]
fn part2() {
    let input = input();
    let mut map = vec![vec![0; 1000]; 1000];

    for (num, start, dims) in input.iter() {
        for j in start.y .. start.y + dims.y {
            for i in start.x .. start.x + dims.x {
                let cell = &mut map[j][i];
                if *cell == 0 {
                    *cell = *num;
                } else {
                    *cell = -1;
                }
            }
        }
    }

    let mut answer = -1;
    'outer: for (num, start, dims) in input.iter() {
        for j in start.y .. start.y + dims.y {
            for i in start.x .. start.x + dims.x {
                if map[j][i] != *num {
                    continue 'outer;
                }
            }
        }
        answer = *num;
        break;
    }

    assert_eq!(552, answer);
}