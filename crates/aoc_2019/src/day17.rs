use aoc_common::{Vec2us, Vec2i64};

use intcode::{IntCode, IntCodeAscii, IntCodeResult};

fn input() -> (IntCode, Vec<Vec<u8>>, Vec2us) {
    let mut computer = IntCode::from_file("inputs/day17.txt");
    let output = computer.run_to_halt().unwrap();

    let mut map: Vec<Vec<u8>> = vec![Vec::new()];
    let mut line = 0;
    let mut start = Vec2us::zero();
    for c in output {
        if c == b'\n'.into() {
            map.push(Vec::new());
            line += 1;
        } else {
            if c == b'^'.into() {
                start = (map[line].len(), line).into();
            }
            map[line].push(c as u8);
        }
    }
    map.retain(|l| !l.is_empty());

    (computer, map, start)
}

#[test]
fn part1() {
    let (_, map, _) = input();

    let mut total = 0;
    for j in 1..map.len() - 1 {
        for i in 1..map[0].len() - 1 {
            if map[j][i] == b'#' && Vec2us::new(i, j).adjacent().all(|p| {
                map[p.y][p.x] == b'#'
            }) {
                total += i * j;
            }
        }
    }

    assert_eq!(total, 3888);
}

#[test]
fn part2() {
    let (mut computer, map, pos) = input();

    let bounds: Vec2i64 = Vec2us::new(map[0].len(), map.len()).cast();
    let mut pos: Vec2i64 = pos.cast();
    let mut dir = Vec2i64::new(0, -1);
    let mut path: Vec<i32> = Vec::new();
    loop {
        let left = pos + dir.rotated_left();
        if left.is_in_bounds(bounds) && map[left.y as usize][left.x as usize] == b'#' {
            path.push(-1);
            dir = dir.rotated_left();
        } else {
            let right = pos + dir.rotated_right();
            if right.is_in_bounds(bounds) && map[right.y as usize][right.x as usize] == b'#' {
                path.push(-2);
                dir = dir.rotated_right();
            } else {
                break;
            }
        }

        let mut count = 0;

        loop {
            let next = pos + dir;
            if next.is_in_bounds(bounds) && map[next.y as usize][next.x as usize] == b'#' {
                count += 1;
                pos = next;
            } else {
                break;
            }
        }

        if count != 0 {
            path.push(count);
        }
    }

    let mut patterns: Vec<&[i32]> = Vec::new();
    let solution = recurse(&path, &mut patterns, Vec::new()).unwrap();

    computer.reset();
    computer.mem_mut()[0] = 2;

    let program = solution.iter().map(|c| String::from_utf8([*c as u8 + b'A'].into()).unwrap()).collect::<Vec<String>>().join(",");
    let routines: Vec<String> = patterns.iter().map(|p| to_string(*p)).collect();

    computer.write_line(&program);
    for routine in routines {
        computer.write_line(&routine);
    }
    computer.write_line("n");

    let mut answer = 0;
    loop {
        match computer.run() {
            IntCodeResult::Output(o) => answer = o,
            IntCodeResult::Halt => break,
            _ => panic!(),
        }
    }

    assert_eq!(answer, 927809);
}

fn recurse<'a>(mut remaining: &'a [i32], patterns: &mut Vec<&'a [i32]>, mut partial_solution: Vec<usize>) -> Option<Vec<usize>> {
    'outer: loop {
        for (i, pattern) in patterns.iter().enumerate() {
            if pattern.len() <= remaining.len() && *pattern == &remaining[0..pattern.len()] {
                remaining = &remaining[pattern.len()..];
                partial_solution.push(i);
                continue 'outer;
            }
        }
        break;
    }

    if remaining.len() == 0 {
        return Some(partial_solution);
    }

    if patterns.len() == 3 {
        return None;
    }

    for i in 1..remaining.len() {
        let cand = &remaining[0..i];
        if to_string(cand).len() > 20 {
            break;
        }

        partial_solution.push(patterns.len());
        patterns.push(cand);
        if let Some(solution) = recurse(&remaining[cand.len()..], patterns, partial_solution.clone()) {
            return Some(solution);
        }
        partial_solution.pop();
        patterns.pop();
    }

    None
}

fn to_string(chars: &[i32]) -> String {
    chars.iter().map(|c| {
        if *c == -1 {
            "L".to_owned()
        } else if *c == -2 {
            "R".to_owned()
        } else {
            c.to_string()
        }
    }).collect::<Vec<String>>().join(",")
}