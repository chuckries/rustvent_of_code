use intcode::IntCode;

fn run(val: i64) -> i64 {
    IntCode::from_file("inputs/day09.txt").run_input_to_halt(&[val]).unwrap()[0]
}

#[test]
fn part1() {
    let answer = run(1);
    assert_eq!(answer, 2518058886);
}

#[test]
fn part2() {
    let answer = run(2);
    assert_eq!(answer, 44292);
}