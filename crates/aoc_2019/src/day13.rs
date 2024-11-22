use intcode::{IntCode, IntCodeResult};

#[test]
fn part1() {
    let mut computer = IntCode::from_file("inputs/day13.txt");
    let blocks = computer
        .run_to_halt()
        .unwrap()
        .chunks(3)
        .filter(|i| { i[2] == 2 })
        .count();

    assert_eq!(blocks, 173);
}

#[test]
fn part2() {
    let mut computer = IntCode::from_file("inputs/day13.txt");
    computer.mem_mut()[0] = 2;

    let mut ball = 0;
    let mut paddle = 0;
    let mut score = 0;

    loop {
        match computer.run() {
            IntCodeResult::Output(x) => {
                let y = computer.run().unwrap();
                let id = computer.run().unwrap();

                if id == 4 {
                    ball = x;
                } else if id == 3 {
                    paddle = x;
                } else if x == -1 && y == 0 {
                    score = id;
                }
            }
            IntCodeResult::Input => computer.push_input_back(i64::signum(ball - paddle)),
            IntCodeResult::Halt => break
        }
    }

    assert_eq!(score, 8942);
}