use std::cmp::Ordering;

use aoc_common::{IteratorExt, Vec2u64, file_lines};

fn input() -> (Vec<Vec2u64>, Vec<u64>) {
    let mut lines = file_lines("inputs/day05.txt");

    let mut ranges: Vec<Vec2u64> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        ranges.push(line.split('-').map(|s| s.parse().unwrap()).to_vec2());
    }

    let ids = lines.map(|l| l.parse().unwrap()).collect();
    
    (ranges, ids)
}

#[test]
fn part1() {
    let (ranges, ids) = input();

    let count = ids.into_iter().filter(|id| {
        ranges.iter().any(|r| r.x <= *id && r.y >= *id)
    }).count();

    assert_eq!(737, count);
}

#[test]
fn part2() {
    let (ranges, _) = input();
    let mut ranges = ranges.into_iter();
    let mut space: Vec<Vec2u64> = vec![ranges.next().unwrap()];

    for mut range in ranges {
        for existing in space.drain(..).to_vec() {
            if existing.y < range.x - 1 || existing.x > range.y + 1 {
                // no connection, add existing back to space
                space.push(existing);
            } else {
                // some connection, adjust range to take over existing
                range.x = range.x.min(existing.x);
                range.y = range.y.max(existing.y);
            }
        }
        space.push(range);
    }

    let answer: u64 = space.into_iter().map(|r| r.y - r.x + 1).sum();
    assert_eq!(357485433193284, answer);
}

// this is the "correct" way to solve this problem.
// for the given input, it's not appreciably faster than what I came up with above
// but this was a good learning. I got this answer from ChatGPT because I've never looked up
// the canonical 1d range reduction alg before. oh well.
#[test]
fn part2_canonical() {
    let (mut ranges, _) = input();
    ranges.sort_by(|lhs, rhs| {
        let mut ord = lhs.x.cmp(&rhs.x);
        if ord == Ordering::Equal {
            ord = lhs.y.cmp(&rhs.y);
        }
        ord
    });

    let mut total = 0;
    let mut ranges = ranges.into_iter();
    let mut current = ranges.next().unwrap();
    for range in ranges {
        if range.x > current.y + 1 {
            total += current.y - current.x + 1;
            current = range;
        } else {
            current.y = current.y.max(range.y);
        }
    }
    total += current.y - current.x + 1;

    assert_eq!(357485433193284, total);
}