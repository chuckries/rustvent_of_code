use intcode::{IntCode, IntCodeAscii, IntCodeResult};

fn run_program(program: &[&str]) -> Result<i64, Vec<String>> {
    let mut droid = IntCode::from_file("inputs/day21.txt");

    for command in program {
        droid.write_line(command);
    }

    let mut output: Vec<String> = Vec::new();
    loop {
        match droid.read_line() {
            Ok(line) => output.push(line),
            Err(IntCodeResult::Output(o)) => return Ok(o),
            Err(IntCodeResult::Halt) => break,
            _ => panic!()
        }
    }

    return Err(output);
}

#[test]
fn part1() {
    let answer = run_program(&[
        "NOT T T",
        "AND A T",
        "AND B T",
        "AND C T",
        "NOT T J",
        "AND D J",
        "WALK"
    ]).unwrap();

    assert_eq!(answer, 19358870);
}

#[test]
fn part2() {
    let answer = run_program(&[
        "NOT T T",
        "AND A T",
        "AND B T",
        "AND C T",
        "NOT T J",
        "AND D J",
        "NOT E T",
        "NOT T T",
        "OR H T",
        "AND T J",
        "RUN"
    ]).unwrap();

    assert_eq!(answer, 1143356492);
}