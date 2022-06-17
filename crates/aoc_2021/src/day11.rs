use aoc_common::{file_lines, Vec2us};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day11.txt")
        .map(|l| l.bytes().map(|b| (b - b'0') as i32).collect())
        .collect()
}

fn run(map: &mut Vec<Vec<i32>>) -> usize {
    let mut to_flash: Vec<Vec2us> = Vec::new();
    let bounds = Vec2us::new(map[0].len(), map.len());

    for p in bounds.iter() { 
        map[p.y][p.x] += 1;
        if map[p.y][p.x] == 10 {
            to_flash.push(p);
        }
    }

    let mut i = 0;
    while i < to_flash.len() {
        let p = to_flash[i];

        for adj in p.surrouding_bounded(&bounds) {
            map[adj.y][adj.x] += 1;
            if map[adj.y][adj.x] == 10 {
                to_flash.push(adj);
            }
        }
        i += 1;
    }

    for p in &to_flash {
        map[p.y][p.x] = 0;
    }

    to_flash.len()
}

#[test]
fn part1() {
    let mut map = input();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += run(&mut map);
    }

    assert_eq!(flashes, 1591);
}

#[test]
fn part2() {
    let mut map = input();
    let target = map[0].len() * map.len();
    let mut iteration = 0;

    loop {
        iteration += 1;
        if run(&mut map) == target {
            break;
        }
    }

    assert_eq!(iteration, 314);
}