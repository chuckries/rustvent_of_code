use aoc_common::{IteratorExt, Vec2us, file_lines};

fn input() -> Vec<(Vec2us, Vec<usize>)> {
    let mut lines = file_lines("inputs/day12.txt").peekable();
    while !lines.peek().unwrap().contains("x") { lines.next(); }

    let mut rects = Vec::new();
    for line in lines {
        let mut split = line.split(' ');
        let dims: Vec2us = split.next().unwrap().trim_end_matches(":").split('x').map(|s| s.parse().unwrap()).collect();
        let required = split.map(|s| s.parse().unwrap()).to_vec();
        rects.push((dims, required));
    }

    rects
}

#[test]
fn part1() {
    // make the assumption that shape doesn't matter, and we are simply trying to fit 
    // however many 3x3 squares into the available space.
    let mut total = 0;
    for (mut dims, required) in input() {
        let total_required: usize = required.iter().sum();
        
        dims /= 3;
        let available = dims.x * dims.y;
        if available >= total_required {
            total += 1;
        }
    }

    assert_eq!(0, total);
}