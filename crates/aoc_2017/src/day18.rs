use std::{collections::VecDeque, rc::Rc};

use aoc_common::{file_lines, IteratorExt};

enum Value {
    Reg(usize),
    Lit(i64),
}

impl Value {
    fn from_str(s: &str) -> Self {
        if let Ok(val) = s.parse::<i64>() {
            Value::Lit(val)
        } else {
            Value::Reg((s.as_bytes()[0] - b'a') as usize)
        }
    }

    fn resolve(&self, regs: &[i64]) -> i64 {
        match self {
            Value::Reg(idx) => regs[*idx],
            Value::Lit(val) => *val,
        }
    }

    fn reg(&self) -> usize {
        match self {
            Value::Reg(idx) => *idx,
            _ => panic!(),
        }
    }
}

enum UnaryOp {
    Snd,
    Rcv,
}

#[derive(Copy, Clone)]
enum BinaryOp {
    Set,
    Add,
    Mul,
    Mod,
    Jgz,
}

enum Instruction {
    Unary(UnaryOp, Value),
    Binary(BinaryOp, Value, Value),
}

enum TickResult {
    Ok,
    Snd(i64),
    Rcv(usize),
}

struct Machine {
    code: Rc<Vec<Instruction>>,
    ip: usize,
    regs: [i64; 26],
}

impl Machine {
    fn new(code: Rc<Vec<Instruction>>, id: i64) -> Self {
        let mut regs = [0; 26];
        regs[(b'p' - b'a') as usize] = id;
        Self {
            code,
            ip: 0,
            regs,
        }
    }

    fn tick(&mut self) -> TickResult {
        let mut result = TickResult::Ok;
        let current = &self.code[self.ip];
        match current {
            Instruction::Binary(op, arg0, arg1) => {
                match *op {
                    BinaryOp::Jgz => {
                        if arg0.resolve(&self.regs) > 0 {
                            self.ip = (self.ip as i64 + arg1.resolve(&self.regs) - 1) as usize;
                        }
                    }
                    _ => {
                        let value = arg1.resolve(&self.regs);
                        let reg = &mut self.regs[arg0.reg()];
                        match op {
                            BinaryOp::Set => *reg = value,
                            BinaryOp::Add => *reg = *reg + value,
                            BinaryOp::Mul => *reg = *reg * value,
                            BinaryOp::Mod => *reg = *reg % value,
                            _ => panic!()
                        }
                    }
                }
            }
            Instruction::Unary(op, arg0) => {
                match op {
                    UnaryOp::Snd => result = TickResult::Snd(arg0.resolve(&self.regs)),
                    UnaryOp::Rcv => result = TickResult::Rcv(arg0.reg()),
                }
            }
        }
        self.ip += 1;
        result
    }


}

fn input() -> Vec<Instruction> {
    file_lines("inputs/day18.txt").map(|s| {
        let mut split = s.split(' ');
        let op = split.next().unwrap();
        let arg0 = Value::from_str(split.next().unwrap());
        let arg1 = split.next().and_then(|v| Some(Value::from_str(v)));

        match op {
            "snd" => Instruction::Unary(UnaryOp::Snd, arg0),
            "rcv" => Instruction::Unary(UnaryOp::Rcv, arg0),
            "set" => Instruction::Binary(BinaryOp::Set, arg0, arg1.unwrap()),
            "add" => Instruction::Binary(BinaryOp::Add, arg0, arg1.unwrap()),
            "mul" => Instruction::Binary(BinaryOp::Mul, arg0, arg1.unwrap()),
            "mod" => Instruction::Binary(BinaryOp::Mod, arg0, arg1.unwrap()),
            "jgz" => Instruction::Binary(BinaryOp::Jgz, arg0, arg1.unwrap()),
            _ => panic!(),
        }
    }).to_vec()
}

#[test]
fn part1() {
    let input = input();
    let mut machine = Machine::new(Rc::new(input), 0);

    let mut sound = 0;
    loop {
        match machine.tick() {
            TickResult::Ok => (),
            TickResult::Snd(val) => sound = val,
            TickResult::Rcv(reg) => if machine.regs[reg] > 0 { break; }
        }
    }
    assert_eq!(sound, 3188);
}

#[test]
fn part2() {
    let input = input();

    let code = Rc::new(input);
    let mut machine0 = Machine::new(code.clone(), 0);
    let mut machine1 = Machine::new(code, 1);
    let mut queue0: VecDeque<i64> = VecDeque::new();
    let mut queue1: VecDeque<i64> = VecDeque::new();

    let mut prog1_sends = 0;

    loop {
        let res0 = machine0.tick();
        let res1 = machine1.tick();

        let mut stalled0 = false;
        let mut stalled1 = false;

        match res0 {
            TickResult::Snd(val) => queue1.push_back(val),
            TickResult::Rcv(reg) => {
                if let Some(val) = queue0.pop_front() {
                    machine0.regs[reg] = val;
                } else {
                    machine0.ip -= 1;
                    stalled0 = true;
                }
            }
            TickResult::Ok => ()
        };

        match res1 {
            TickResult::Snd(val) => {
                queue0.push_back(val);
                prog1_sends += 1;
            }
            TickResult::Rcv(reg) => {
                if let Some(val) = queue1.pop_front() {
                    machine1.regs[reg] = val;
                } else {
                    machine1.ip -= 1;
                    stalled1 = true;
                }
            }
            TickResult::Ok => ()
        };

        if stalled0 && stalled1 {
            break;
        }
    }

    assert_eq!(prog1_sends, 7112);
}