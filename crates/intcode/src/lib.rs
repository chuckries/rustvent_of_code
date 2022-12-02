use std::{collections::VecDeque, rc::Rc};

use aoc_common::{file_string};

#[derive(Debug)]
pub enum IntCodeResult {
    Halt,
    Input,
    Output(i64)
}

impl IntCodeResult {
    pub fn unwrap(self) -> i64 {
        match self {
            IntCodeResult::Output(val) => val,
            _ => panic!()
        }
    }
}

#[derive(Clone)]
pub struct IntCode {
    pc: usize,
    relative_base: i64,
    code: Rc<Vec<i64>>,
    mem: Vec<i64>,
    is_halt: bool,
    input: VecDeque<i64>
}

impl IntCode {
    pub fn new(code: Vec<i64>) -> Self {
        IntCode {
            pc: 0,
            relative_base: 0,
            code: Rc::new(code.clone()),
            mem: code,
            is_halt: false,
            input: VecDeque::new(),
        }
    }

    pub fn from_string(code: &str) -> Self {
        Self::new(code.split(',').map(|s| s.parse::<i64>().unwrap()).collect())
    }

    pub fn from_file(path: &str) -> Self {
        Self::from_string(&file_string(path))
    }

    pub fn is_halt(&self) -> bool {
        self.is_halt
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.relative_base = 0;
        self.is_halt = false;

        self.mem[..self.code.len()].copy_from_slice(&self.code);
        self.mem[self.code.len()..].fill(0);
    }

    pub fn run(&mut self) -> IntCodeResult {
        loop {
            if let Some(result) = self.step() {
                return result;
            }
        }
    }

    pub fn run_input(&mut self, input: &[i64]) -> IntCodeResult {
        self.input.extend(input.iter());
        self.run()
    }

    pub fn run_to_halt(&mut self) -> Result<Vec<i64>, IntCodeResult> {
        let mut outputs = Vec::new();
        loop {
            match self.run() {
                IntCodeResult::Output(i) => outputs.push(i),
                IntCodeResult::Halt => return Ok(outputs),
                input @ IntCodeResult::Input => return Err(input),
            }
        }
    }

    pub fn run_input_to_halt(&mut self, input: &[i64]) -> Result<Vec<i64>, IntCodeResult> {
        self.input.extend(input.iter());
        self.run_to_halt()
    }

    pub fn step(&mut self) -> Option<IntCodeResult> {
        if self.is_halt {
            return Some(IntCodeResult::Halt);
        }

        let (op, mode0, mode1, mode2) = self.decode();

        if op == 99 {
            self.is_halt = true;
            return Some(IntCodeResult::Halt);
        } else { 
            match op {
                3 => {
                    if let Some(val) = self.input.pop_front() {
                        *self.read_mode_mut(mode0) = val;
                    }
                    else {
                        self.pc -=1;
                        return Some(IntCodeResult::Input);
                    }
                }
                4 => {
                    let val = self.read_mode(mode0);
                    return Some(IntCodeResult::Output(val));
                }
                5 | 6 => {
                    let cond = self.read_mode(mode0);
                    let new_pc = self.read_mode(mode1);
                    if (cond != 0 && op == 5) || (cond == 0 && op == 6) {
                        self.pc = new_pc as usize;
                    }
                }
                9 => {
                    self.relative_base += self.read_mode(mode0);
                }
                _ => {
                    let a = self.read_mode(mode0);
                    let b = self.read_mode(mode1);
                    let c = self.read_mode_mut(mode2);

                    *c = match op {
                        1 => a + b,
                        2 => a * b,
                        7 => if a < b { 1 } else { 0 },
                        8 => if a == b { 1 } else { 0 },
                        _ => panic!()
                    };
                }
            }
        }

        None
    }

    fn decode(&mut self) -> (i64, i64, i64, i64) {
        let mut val = self.read_pc();
        let op = val % 100;
        val /= 100;
        let mode0 = val % 10;
        val /= 10;
        let mode1 = val % 10;
        val /= 10;
        (op, mode0, mode1, val)
    }

    fn read_mode(&mut self, mode: i64) -> i64 {
        let val = self.read_pc();
        match mode {
            0 => self.read(val as usize),
            1 => val,
            2 => self.read((self.relative_base + val) as usize),
            _ => panic!()
        }
    }

    fn read_mode_mut(&mut self, mode: i64) -> &mut i64 {
        let val = self.read_pc();
        match mode {
            0 => self.read_mut(val as usize),
            2 => self.read_mut((self.relative_base + val) as usize),
            _ => panic!()
        }
    }

    fn read_pc(&mut self) -> i64 {
        self.pc += 1;
        self.read(self.pc - 1)
    }

    fn read(&mut self, addr: usize) -> i64 {
        self.ensure_memory(addr);
        self.mem[addr]
    }

    fn read_mut(&mut self, addr: usize) -> &mut i64 {
        self.ensure_memory(addr);
        &mut self.mem[addr]
    }

    fn ensure_memory(&mut self, addr: usize) {
        if addr >= self.mem.len() {
            let size = usize::max(addr + 1, self.mem.len() * 2);
            self.mem.resize(size, 0);
        }
    }

    pub fn mem(&self) -> &[i64] {
        &self.mem
    }

    pub fn mem_mut(&mut self) -> &mut [i64] {
        &mut self.mem
    }

    pub fn push_input_back(&mut self, val: i64) {
        self.input.push_back(val);
    }

    pub fn add_input(&mut self, input: &[i64]) {
        self.input.extend(input.iter());
    }
}

pub trait IntCodeAscii {
    fn write_line(&mut self, line: &str);
    fn read_line(&mut self) -> Result<String, IntCodeResult>;
}

impl IntCodeAscii for IntCode {
    fn write_line(&mut self, line: &str) {
        for b in line.bytes() {
            self.push_input_back(b as i64);
        }
        self.push_input_back(b'\n' as i64);
    }

    fn read_line(&mut self) -> Result<String, IntCodeResult> {
        let mut output = String::new();
        loop {
            match self.run() {
                IntCodeResult::Output(o) => {
                    if o <= 0x7F {
                        let o = o as u8 as char;
                        if o == '\n' {
                            return Ok(output);
                        } else {
                            output.push(o);
                        }
                    } else {
                        return Err(IntCodeResult::Output(o));
                    }
                }
                o => {
                    return Err(o);
                }
            }
        }
    }
}