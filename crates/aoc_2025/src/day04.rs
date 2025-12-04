use aoc_common::{Grid, Vec2us};

fn input() -> Grid<u8> {
    Grid::file_as_grid("inputs/day04.txt", &mut |b, _| b)
}

fn find_available(map: &Grid<u8>) -> Vec<Vec2us> {
        map.enumerate().filter_map(|(p, c)| {
        if *c == b'@' {
            let count = map.surrounding(p).filter(|c| **c == b'@').count();
            if count < 4 {
                return Some(p);
            }
        }
        None
    }).collect()
}

#[test]
fn part1() {
    let map = input();
    let answer = find_available(&map).len();
    assert_eq!(1435, answer);
}

#[test]
fn part2() {
    let mut map = input();
    let mut total = 0;
    loop {
        let available = find_available(&map);
        if available.is_empty() {
            break;
        }
        total += available.len();
        for p in available {
            map[p] = b'.';
        }
    }

    assert_eq!(8623, total);
}