use aoc_common::{Vec2u64, file_lines};

fn input() -> Vec<Vec2u64>{
    file_lines("inputs/day20.txt").map(|l| {
        l.split('-').map(|s| s.parse().unwrap()).collect()
    }).collect()
}

fn get_merged() -> Vec<Vec2u64> {
    let mut ranges = input();
    ranges.sort_by_cached_key(|r| r.x);

    let mut ranges = ranges.into_iter();
    let mut merged = Vec::with_capacity(ranges.len());
    merged.push(ranges.next().unwrap());
    for range in ranges {
        let last = merged.last_mut().unwrap();
        if last.y + 1 < range.x {
            merged.push(range);
        } else {
            last.y = last.y.max(range.y);
        }
    }
    merged
}

#[test]
fn part1() {
    let merged = get_merged();
    let answer = merged[0].y + 1;
    assert_eq!(answer, 22887907);
}

#[test]
fn part2() {
    let merged = get_merged();
    let mut total = 0;
    total += merged[0].x - 0;
    total += u32::MAX as u64 - merged[merged.len() - 1].y;
    for wind in merged.windows(2) {
        total += wind[1].x - wind[0].y - 1;
    }
    assert_eq!(109, total);
}