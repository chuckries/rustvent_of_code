use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> Vec<Vec<u8>> {
    file_lines("inputs/day04.txt").map(|l| l.into_bytes()).collect()
}

#[test]
fn part1() {
    let map = input();
    let bounds = Vec2i32::new(map.len() as i32, map[0].len() as i32);
    let dirs = Vec2i32::cardinal_dirs_and_diags().to_vec();
    const TARGET: &[u8] = b"XMAS";

    let mut total = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] == TARGET[0] {
                let p = Vec2i32::new(i as i32, j as i32);
                'dir: for dir in dirs.iter().copied() {
                    let mut p = p + dir;
                    for i in 1..TARGET.len() {
                        if !p.is_in_bounds(bounds) || map[p.y as usize][p.x as usize] != TARGET[i] {
                            continue 'dir;
                        }
                        p += dir;
                    }
                    total += 1;
                }
            }
        }
    }

    assert_eq!(total, 2593);
}

#[test]
fn part2() {
    let map = input();
    let mut total = 0;
    for j in 1..map.len() - 1 {
        for i in 1..map[j].len() - 1 {
            if map[j][i] == b'A' && 
                ((map[j - 1][i - 1] == b'M' && map[j + 1][i + 1] == b'S') ||
                (map[j - 1][i - 1] == b'S' && map[j + 1][i + 1] == b'M')) &&
                ((map[j - 1][i + 1] == b'M' && map[j + 1][i - 1] == b'S') ||
                (map[j - 1][i + 1] == b'S' && map[j + 1][i - 1] == b'M')) {
                    total += 1;
                }
        }
    }
    
    assert_eq!(total, 1950);
}