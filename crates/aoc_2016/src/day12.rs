use crate::computer::Computer;

#[test]
fn part1() {
    let mut machine = Computer::from_file("inputs/day12.txt");
    machine.run();

    let answer = machine.regs()[0];
    assert_eq!(answer, 317993);
}

#[test]
fn part2() {
    let mut machine = Computer::from_file("inputs/day12.txt");
    machine.regs_mut()[2] = 1;
    machine.run();

    let answer = machine.regs()[0];
    assert_eq!(answer, 9227647);
}