use aoc_common::{Grid, IteratorExt, Vec2i32};

fn input() -> Grid<u8> {
    Grid::file_as_grid("inputss/day04.txt", &mut |b, _| b)
}

#[test]
fn part1() {
    let map = input();
    let bounds = map.bounds();
    let dirs = Vec2i32::unit_dirs_and_diags().to_vec();
    const TARGET: &[u8] = b"XMAS";

    let mut total = 0;
    for (p, c) in map.enumerate() {
        if *c == TARGET[0] {
            'dir: for dir in dirs.iter().copied() {
                let mut p = p.cast::<i32>() + dir;
                for i in 1..TARGET.len() {
                    if !p.is_in_bounds(bounds.cast()) || map[p] != TARGET[i] {
                        continue 'dir;
                    }
                    p += dir;
                }
                total += 1;
            }
        }
    }

    assert_eq!(total, 2593);
}

#[test]
fn part2() {
    let map = input();
    let mut total = 0;
    for j in 1..map.height() - 1 {
        for i in 1..map.width() - 1 {
            if map[(i, j)] == b'A' && 
                ((map[(i - 1, j - 1)] == b'M' && map[(i + 1, j + 1)] == b'S') ||
                (map[(i - 1, j - 1)] == b'S' && map[(i + 1, j + 1)] == b'M')) &&
                ((map[(i + 1, j - 1)] == b'M' && map[(i - 1, j + 1)] == b'S') ||
                (map[(i + 1, j - 1)] == b'S' && map[(i - 1, j + 1)] == b'M')) {
                    total += 1;
                }
        }
    }
    
    assert_eq!(total, 1950);
}