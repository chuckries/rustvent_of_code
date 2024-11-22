use aoc_common::{file_lines, IteratorExt, Vec3i32};

fn input() -> Vec<Vec3i32> {
    file_lines("inputs/day03.txt").map(|l| {
        let split = l.split_whitespace().map(|s| s.parse().unwrap()).to_vec();
        (split[0], split[1], split[2]).into()
    }).collect()
}

fn count<I: Iterator<Item = Vec3i32>>(iter: I) -> usize {
    iter.into_iter().filter(|t| {
        if t.x + t.y <= t.z {
            false
        } else if t.x + t.z <= t.y {
            false
        } else if t.y + t.z <= t.x {
            false
        } else {
            true
        }
    }).count()
}

#[test]
fn part1() {
    let answer = count(input().into_iter());
    assert_eq!(answer, 982);
}

#[test] 
fn part2() {
    let answer = count(input().chunks_exact(3).map(|c| -> [Vec3i32; 3] {
        [
            Vec3i32::new(c[0].x, c[1].x, c[2].x),
            Vec3i32::new(c[0].y, c[1].y, c[2].y),
            Vec3i32::new(c[0].z, c[1].z, c[2].z),
        ]
    }).flatten());

    assert_eq!(answer, 1826);
}