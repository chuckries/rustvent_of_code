use aoc_common::{file_lines, IteratorExt};

use Identifier::*;
use Instruction::*;

#[derive(Copy, Clone)]
enum Identifier {
    Reg(usize),
    Const(i32),
}

enum Instruction {
    Cpy(Identifier, Identifier),
    Inc(Identifier),
    Dec(Identifier),
    Jnz(Identifier, Identifier),
    Tgl(Identifier),
}

impl Instruction {
    fn toggle(&self) -> Instruction {
        match *self {
            Inc(a) => Dec(a),
            Dec(a) => Inc(a),
            Tgl(a) => Inc(a),
            Jnz(a, b) => Cpy(a, b),
            Cpy(a, b) => Jnz(a, b),
        }
    }
}

pub struct Computer {
    code: Vec<Instruction>,
    regs: Vec<i32>,
    ip: i32,
    is_halt: bool
}

impl Computer {
    pub fn from_file(path: &str) -> Self {
        let code = file_lines(path).map(|l| {
            let split = l.split(' ').to_vec();
            let args = split[1..].iter().map(|s| {
                match *s {
                    "a" | "b" | "c" | "d" => Reg((s.as_bytes()[0] - b'a') as usize),
                    _ => Const(s.parse().unwrap())
                }
            }).to_vec();
    
            match split[0] {
                "cpy" => Cpy(args[0], args[1]),
                "inc" => Inc(args[0]),
                "dec" => Dec(args[0]),
                "jnz" => Jnz(args[0], args[1]),
                "tgl" => Tgl(args[0]),
                _ => panic!()
            }
        }).to_vec();


        Self {
            code,
            regs: vec![0; 4],
            ip: 0,
            is_halt: false
        }
    }

    pub fn regs(&self) -> &[i32] {
        &self.regs
    }

    pub fn regs_mut(&mut self) -> &mut [i32] {
        &mut self.regs
    }

    pub fn run(&mut self) {
        while !self.is_halt {
            self.step();
        }
    }

    fn step(&mut self) {
        if self.is_halt { return; }

        match self.code[self.ip as usize] {
            Cpy(a, b) => {
                if let Reg(b) = b {
                    self.regs[b] = self.read(a);
                }
            }
            Inc(a) => {
                if let Reg(a) = a {
                    self.regs[a] += 1;
                }
            }
            Dec(a) => {
                if let Reg(a) = a {
                    self.regs[a] -= 1;
                }
            }
            Jnz(a, b) => {
                if self.read(a) != 0 {
                    self.ip += self.read(b) - 1;
                }
            }
            Tgl(a) => {
                let target = self.ip + self.read(a);
                if target >= 0 && (target as usize) < self.code.len() {
                    self.code[target as usize] = self.code[target as usize].toggle();
                }
            }
        }

        self.ip += 1;
        if self.ip as usize >= self.code.len() {
            self.is_halt = true;
        }
    }

    fn read(&self, id: Identifier) -> i32 {
        match id {
            Const(i) => i,
            Reg(i) => self.regs[i],
        }
    }
}