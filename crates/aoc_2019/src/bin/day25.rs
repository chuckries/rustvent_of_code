use intcode::*;

fn main() {
    let mut computer = IntCode::from_file("inputs/day25.txt");
    let mut output = String::new();
    loop {
        match computer.run() {
            IntCodeResult::Output(o) => {
                let o = o as u8 as char;
                if o == '\n' {
                    println!("{}", output);
                    output.clear();
                } else {
                    output.push(o);
                }
            }
            IntCodeResult::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                computer.write_line(input.trim());

            }
            IntCodeResult::Halt => break
        }
    }
}