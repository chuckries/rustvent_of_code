use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> Vec<Vec2i32> {
    file_lines("inputs/day06.txt").map(|l| {
        let split = l.split(", ").map(|s| s.parse::<i32>().unwrap()).to_vec();
        Vec2i32::new(split[0], split[1])
    }).to_vec()
}

#[test]
fn part1() {
    let input = input();

    // do this the naive way
    
    // find the max dist between pairs
    let mut max = 0;
    for i in 0 .. input.len() - 1 {
        for j in i + 1 .. input.len()
        {
            let dist = input[i].manhattan_from(input[j]);
            if dist > max {
                max = dist;
            }
        }
    }

    
}