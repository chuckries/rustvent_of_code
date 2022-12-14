use crate::computer::Computer;

fn run(seed: i32) -> i32 {
    let mut machine = Computer::from_file("inputs/day23.txt");
    machine.regs_mut()[0] = seed;
    machine.run();
    machine.regs()[0]
}

#[test]
fn part1() {
    let answer = run(7);
    assert_eq!(answer, 10365);
}

#[test]
#[ignore] //slow
fn part2() {
    let answer = run(12);
    assert_eq!(answer, 479006925);
}