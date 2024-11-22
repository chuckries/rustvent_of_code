use intcode::IntCode;

fn run(id: i64) -> i64 {
    let mut int_code = IntCode::from_file("inputs/day05.txt");
    let outputs = int_code.run_input_to_halt(&[id]).unwrap();

    *outputs.last().unwrap()
}

#[test]
fn part1() {
    let answer = run(1);
    assert_eq!(answer, 8332629);
}

#[test]
fn part2() {
    let answer = run(5);
    assert_eq!(answer, 8805067);
}