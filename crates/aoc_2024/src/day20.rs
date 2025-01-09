use aoc_common::{Grid, Vec2i32, Vec2us};

fn input() -> (Grid<u8>, Vec2i32, Vec2i32) {
    let mut start = Vec2us::default();
    let mut end = Vec2us::default();
    let grid = Grid::file_as_grid("inputs/day20.txt", &mut |b, p| {
        match b {
            b'S' => start = p,
            b'E' => end = p,
            _ => (),
        };

        b
    });

    (grid, start.cast(), end.cast())
}

#[derive(Default, Clone)]
struct Cell {
    next: Vec2i32,
    dist: i32,
}

fn process_map() -> (Grid<u8>, Grid<Cell>, Vec2i32, Vec2i32) {
    let (map, start, end) = input();

    let mut distances: Grid<Cell> = map.same_of_type();

    let mut current = start.cast::<i32>();
    let mut dir = Vec2i32::zero();
    for adj in current.adjacent() {
        if map[adj] == b'.' {
            dir = adj - current;
            break;
        }
    }

    let mut dist = 0;
    loop {
        distances[current].dist = dist;
        if current == end.cast() {
            break;
        }

        let mut cand = current + dir;
        let mut cand_dir = dir;
        if map[cand] == b'#' {
            cand_dir = dir.rotated_left();
            cand = current + cand_dir;
            if map[cand] == b'#' {
                cand_dir = dir.rotated_right();
                cand = current + cand_dir;
                if map[cand] == b'#' {
                    panic!();
                }
            }
        }

        distances[current].next = cand;
        current = cand;
        dir = cand_dir;
        dist += 1;
    }

    (map, distances, start, end)
}

fn run<const MAX_CHEAT: i32>() -> i32 {
    let (map, distances, start, end) = process_map();
    let standard_distance = distances[end].dist;

    let mut count_under_100 = 0;
    let mut pos: Vec2i32 = start.cast();
    loop {
        if pos == end.cast() {
            break;
        }

        let top = i32::max(pos.y - MAX_CHEAT,0);
        let bot = i32::min(pos.y + MAX_CHEAT, map.height() as i32 - 1);

        let mut j = top;
        while j <= bot {
            let remaining = MAX_CHEAT - i32::abs(pos.y - j);

            let left = i32::max(pos.x - remaining, 0);
            let right = i32::min(pos.x + remaining, map.width() as i32 - 1);

            let mut i = left;
            while i <= right {
                let end = Vec2i32::new(i, j);
                if map[end] != b'#' {
                    let time = distances[pos].dist + (standard_distance - distances[end].dist) + pos.manhattan_from(end);
                    if standard_distance - time >= 100 {
                        count_under_100 += 1;
                    }
                }

                i += 1;
            }

            j += 1;
        }

        pos = distances[pos].next;
    }

    count_under_100
}

#[test]
fn part1() {
    let answer = run::<2>();
    assert_eq!(answer, 1365);
}

#[test]
fn part2() {
    let answer = run::<20>();
    assert_eq!(answer, 986082);
}