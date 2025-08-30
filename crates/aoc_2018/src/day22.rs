use aoc_common::file_lines;
use Op::*;
use Arg::*;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Ban,
    Bor,
    Set,
    Gt,
    Eq,
}

#[derive(Clone, Copy)]
enum Arg {
    Imm(usize),
    Reg(usize),
}

impl Arg {
    fn read(&self, regs: &[usize]) -> usize {
        match self {
            Imm(imm) => *imm,
            Reg(idx) => regs[*idx],
        }
    }

    fn write(&self, regs: &mut[usize], val: usize) {
        match self {
            Reg(idx) => regs[*idx] = val,
            _ => panic!()
        };
    }
}

fn input() -> (Vec<(Op, Arg, Arg, Arg)>, usize) {
    let mut ip_arg = 0;
    let mut instructions = Vec::new();
    for l in file_lines("inputs/day22.txt") {
        let mut split = l.split(' ');
        let first = split.next().unwrap();
        if first == "#ip" {
            ip_arg = split.next().unwrap().parse().unwrap()
        } else {
            let args: Vec<usize> = split.map(|s| s.parse::<usize>().unwrap()).collect();
            let a = args[0];
            let b = args[1];
            let c = args[2];
            
            instructions.push(match first {
                "addr" => (Add, Reg(a), Reg(b), Reg(c)),
                "addi" => (Add, Reg(a), Imm(b), Reg(c)),
                "mulr" => (Mul, Reg(a), Reg(b), Reg(c)),
                "muli" => (Mul, Reg(a), Imm(b), Reg(c)),
                "banr" => (Ban, Reg(a), Reg(b), Reg(c)),
                "bani" => (Ban, Reg(a), Imm(b), Reg(c)),
                "borr" => (Bor, Reg(a), Reg(b), Reg(c)),
                "bori" => (Bor, Reg(a), Imm(b), Reg(c)),
                "setr" => (Set, Reg(a), Imm(0), Reg(c)),
                "seti" => (Set, Imm(a), Imm(0), Reg(c)),
                "gtir" => (Gt, Imm(a), Reg(b), Reg(c)),
                "gtri" => (Gt, Reg(a), Imm(b), Reg(c)),
                "gtrr" => (Gt, Reg(a), Reg(b), Reg(c)),
                "eqir" => (Eq, Imm(a), Reg(b), Reg(c)),
                "eqri" => (Eq, Reg(a), Imm(b), Reg(c)),
                "eqrr" => (Eq, Reg(a), Reg(b), Reg(c)),
                _ => panic!(),
            });
        }
    }

    (instructions, ip_arg)
}

fn run(seed: usize) -> usize {
    let (instructions, ip_reg) = input();
    let mut regs = [0; 6];
    regs[0] = seed;

    loop {
        if regs[ip_reg] >= instructions.len() {
            break;
        }

        let (op, a, b, c) = instructions[regs[ip_reg]];
        let a = a.read(&regs);
        let b = b.read(&regs);

        let result = match op {
            Add => a + b,
            Mul => a * b,
            Ban => a & b,
            Bor => a | b,
            Set => a,
            Gt => if a > b { 1 } else  { 0 },
            Eq => if a == b { 1 } else { 0 },
        };

        c.write(&mut regs, result);

        regs[ip_reg] += 1;
    }

    return regs[0];
}

#[test]
fn part1() {
    let answer = run(0);
    assert_eq!(1824, answer);
}

#[test]
fn part2() {
    assert_eq!(true, false);
}