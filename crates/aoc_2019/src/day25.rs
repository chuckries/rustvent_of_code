use intcode::{IntCode, IntCodeAscii, IntCodeResult};

#[test]
fn part1() {
    let mut game = IntCode::from_file("inputs/day25.txt");

    for command in [
        "north",
        "take mutex",
        "south",
        "west",
        "take space law space brochure",
        "south",
        "take hologram",
        "west",
        "take manifold",
        "east",
        "north",
        "east",
        "south",
        "west",
        "south",
        "south",
        "south",
    ] {
        game.write_line(command);
    }

    let mut output = String::new();
    loop {
        match game.read_line() {
            Ok(line) => output = line,
            Err(IntCodeResult::Halt) => break,
            _ => panic!()
        }
    }

    let known = "\"Oh, hello! You should be able to get in by typing 262848 on the keypad at the main airlock.\"";
    assert_eq!(output, known);
}