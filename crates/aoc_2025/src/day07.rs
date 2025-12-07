use std::collections::{HashMap, HashSet, VecDeque};

use aoc_common::{Grid, Vec2us};

fn input() -> (Grid<u8>, Vec2us) {
    let grid = Grid::file_as_grid("inputs/day07.txt", &mut |b, _| b);
    let row = grid.row(0);
    let mut start = Vec2us::zero();
    for i in 0 .. row.len() {
        if row[i] == b'S' {
            start = Vec2us::new(i, 0);
            break;
        }
    }

    (grid, start)
}

#[test] 
fn part1() {
    let (map, start) = input();

    let mut visited: HashSet<Vec2us> = HashSet::new();
    let mut queue: VecDeque<Vec2us> = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for y in current.y .. map.height() {
            let current = Vec2us::new(current.x, y);
            if map[current] == b'^' {
                if visited.insert((current.x, y).into()) {
                    queue.push_back(current.west_of());
                    queue.push_back(current.east_of());
                }
                break;
            }
        }
    }

    let answer = visited.len();
    assert_eq!(1626, answer);
}

#[test]
fn part2() {
    let (map, start) = input();

    fn backtrack(p: Vec2us, map: &Grid<u8>, count: &mut u64, cache: &mut HashMap<Vec2us, u64>) {
        if p.y >= map.height() {
            *count += 1;
        } else if matches!(map[p], b'.' | b'S') {
            backtrack(p.south_of(), map, count, cache);
        } else if map[p] == b'^' {
            if let Some(cached) = cache.get(&p) {
                *count += cached;
                return;
            }

            let mut local_count = 0;
            backtrack(p.west_of(), map, &mut local_count, cache);
            backtrack(p.east_of(), map, &mut local_count, cache);
            cache.insert(p, local_count);
            *count += local_count;
        } else {
            panic!();
        }
    }

    let mut count = 0;
    let mut cache: HashMap<Vec2us, u64> = HashMap::new();
    backtrack(start, &map, &mut count, &mut cache);
    assert_eq!(48989920237096, count);
}