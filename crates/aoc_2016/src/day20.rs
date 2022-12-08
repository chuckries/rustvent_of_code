use aoc_common::{file_lines, IteratorExt};


fn input() -> impl Iterator<Item = (u64, u64)> {
    file_lines("inputs/day20.txt").map(|l| {
        let split = l.split('-').map(|n| n.parse::<u64>().unwrap()).to_vec();
        (split[0], split[1])
    })
}

fn get_reduced() -> Vec<(u64, u64)> {
    let mut space: Vec<(u64, u64)> = Vec::new();

    for mut new in input() {
        for existing in space.drain(..).to_vec() {
            if new.0 > existing.1 + 1 || new.1 + 1 < existing.0 {
                space.push(existing);
            } else {
                new.0 = new.0.min(existing.0);
                new.1 = new.1.max(existing.1);
            }
        }
        space.push(new);
    }

    space
}

#[test]
fn part1() {
    let space = get_reduced();
    let answer = space.iter().min_by_key(|r| r.0).unwrap().1 + 1;
    assert_eq!(answer, 22887907);
}

#[test]
fn part2() {
    let mut space = get_reduced();
    space.sort_by_key(|r| r.0);

    if space.last().unwrap().1 != u32::MAX as u64 {
        space.push((u32::MAX as u64 + 1, u32::MAX as u64 + 1));
    }

    let answer: u64 = space.windows(2).map(|w| {
        w[1].0 - w[0].1 - 1
    }).sum();

    assert_eq!(answer, 109);
}