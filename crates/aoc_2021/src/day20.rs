use std::collections::HashSet;

use aoc_common::file_lines;

fn input() -> (Vec<char>, Vec<Vec<char>>) {
    let mut lines = file_lines("inputs/day20.txt");

    let algo: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next().unwrap();
    let start: Vec<Vec<char>> = lines.map(|l| {
        l.chars().collect()
    }).collect();

    (algo, start)
}

fn run(iterations: i32) -> usize {
    let (algo, start) = input();

    let mut bounds = ((0, 0), ((start[0].len() - 1) as i32, (start.len() - 1) as i32));

    let mut canvas: HashSet<(i32, i32)> = HashSet::new();

    for (j, cj) in start.into_iter().enumerate() {
        for (i, ci) in cj.into_iter().enumerate() {
            if ci == '#' {
                canvas.insert((i as i32, j as i32));
            }
        }
    }

    let is_toggle = algo[0] == '#';
    let mut toggle = false;
    for _ in 0..iterations {
        let mut next: HashSet<(i32, i32)> = HashSet::new();

        for j in bounds.0.1 - 1..=bounds.1.1 + 1 {
            for i in bounds.0.0 - 1..=bounds.1.0 + 1 {
                let mut idx = 0;
                for v in j - 1..=j + 1 {
                    for u in i - 1..=i + 1 {
                        let is_set = if u < bounds.0.0 || u > bounds.1.0 || v < bounds.0.1 || v > bounds.1.1 {
                            toggle
                        } else {
                            canvas.contains(&(u, v))
                        };

                        idx = (idx << 1) | if is_set { 1 } else { 0 };
                    }
                }
                if algo[idx] == '#' {
                    next.insert((i, j));
                }
            }
        }

        std::mem::swap(&mut next, &mut canvas);
        next.clear();

        if is_toggle {
            toggle = !toggle;
        }

        bounds = ((bounds.0.0 - 1, bounds.0.1 - 1), (bounds.1.0 + 1, bounds.1.1 + 1));
    }

    canvas.len()
}

#[test]
fn part1() {
    let answer = run(2);

    assert_eq!(answer, 5663);
}

#[test]
fn part2() {
    let answer = run(50);

    assert_eq!(answer, 19638);
}