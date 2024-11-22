use aoc_common::{file_lines, IteratorExt, Vec3i32};

fn input() -> impl Iterator<Item = Vec3i32> {
    file_lines("inputs/day02.txt").map(|s| {
        let nums = s.split('x').map(|i| i.parse::<i32>().unwrap()).to_vec();
        Vec3i32::new(nums[0], nums[1], nums[2])
    })
}

fn run<F: Fn(Vec3i32) -> i32>(f: F) -> i32 {
    input().map(f).sum()
}

#[test]
fn part1() {
    fn map(v: Vec3i32) -> i32 {
        let a = v.x * v.y;
        let b = v.x * v.z;
        let c = v.y * v.z;
        let min = a.min(b.min(c));

        a * 2 + b * 2 + c * 2 + min
    }

    let answer = run(map);
    assert_eq!(answer, 1588178);
}

#[test]
fn part2() {
    fn map(v: Vec3i32) -> i32 {
        let max = v.x.max(v.y.max(v.z));
        let perimeter = (v.x + v.y + v.z - max) * 2;
        let volume = v.x * v.y * v.z;

        perimeter + volume
    }

    let answer = run(map);
    assert_eq!(answer, 3783758);
}