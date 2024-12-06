use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> (Vec<Vec<u8>>, Vec2i32) {
    let map = file_lines("inputs/day06.txt").map(|l| l.into_bytes().to_vec()).to_vec();

    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] == b'^' {
                return (map, (i as i32, j as i32).into());
            }
        }
    }
    panic!();
}

fn march(pos: &mut Vec2i32, dir: &mut Vec2i32, map: &[Vec<u8>]) -> bool {
    loop {
        let next = *pos + *dir;

        if !next.is_in_bounds((map.len() as i32, map[0].len() as i32).into()) {
            return false;
        }

        if map[next.y as usize][next.x as usize] == b'#' {
            dir.rotate_right();
        } else {
            *pos = next;
            return true;
        }
    }
}

fn find_visited(mut pos: Vec2i32, mut dir: Vec2i32, map: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut visited = vec![vec![false; map[0].len() as usize]; map.len() as usize];
    visited[pos.y as usize][pos.x as usize] = true;

    while march(&mut pos, &mut dir, &map) {
        visited[pos.y as usize][pos.x as usize] = true;
    }

    visited
}

fn check_cycle(mut pos: Vec2i32, mut dir: Vec2i32, map: &[Vec<u8>]) -> bool {
    let mut visited: Vec<Vec<Option<Vec<Vec2i32>>>> = vec![vec![None; map[0].len()]; map.len()];
    visited[pos.y as usize][pos.x as usize] = Some(vec![dir]);

    while march(&mut pos, &mut dir, &map) {
        let dirs = &mut visited[pos.y as usize][pos.x as usize].get_or_insert(Default::default());

        if dirs.contains(&dir) {
            return true;
        }
        dirs.push(dir);
    }

    false
}

#[test]
fn part1() {
    let (map, start) = input();
    let visited = find_visited(start, -Vec2i32::unit_y(), &map);
    let answer = visited.iter().flatten().filter(|b| **b).count();
    assert_eq!(answer, 4580);
}

#[test]
fn part2() {
    let (mut map, start) = input();
    let dir = -Vec2i32::unit_y();
    let mut visited = find_visited(start, dir, &map);
    visited[start.y as usize][start.x as usize] = false;

    let mut total = 0;
    for j in 0..visited.len() as usize {
        for i in 0..visited[j].len() as usize {
            if visited[j][i] {
                map[j][i] = b'#';
                if check_cycle(start, dir, &map) {
                    total += 1;
                }
                map[j][i] = b'.';
            }
        }
    }

    assert_eq!(total, 1480);
}