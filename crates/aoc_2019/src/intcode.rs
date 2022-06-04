use aoc_common::{file_string};

// pub trait IntCodeRead {
//     fn read(&mut self) -> i64;
// }

// pub trait IntCodeWrite {
//     fn write(&mut self, _: i64);
// }

// pub struct Empty;

// impl IntCodeRead for Empty {
//     fn read(&mut self) -> i64 {
//         panic!();
//     }
// }

// impl IntCodeWrite for Empty {
//     fn write(&mut self, _: i64) {
//         panic!();
//     }
// }

pub trait IntCodeIo {
    fn read(&mut self) -> i64;
    fn write(&mut self, val: i64);
}

struct EmtpyIo;

impl IntCodeIo for EmtpyIo {
    fn read(&mut self) -> i64 { 
        0
    }
    fn write(&mut self, val: i64)
    {

    }
}

static mut EMPTY: EmtpyIo = EmtpyIo;

pub struct FnIo<'io, R, W>
    where R: FnMut() -> i64, W: FnMut(i64)
{
    r: &'io mut R,
    w: &'io mut W,
}

impl<'io, R: FnMut() -> i64, W: FnMut(i64)> FnIo<'io, R, W> {
    pub fn new(r: &'io mut R, w: &'io mut W) -> FnIo<'io, R, W> {
        FnIo { r: r, w: w }
    }
}

impl<'io, R: FnMut() -> i64, W: FnMut(i64)> IntCodeIo for FnIo<'io, R, W> {
    fn read(&mut self) -> i64 {
        (self.r)()
    }

    fn write(&mut self, val: i64) {
        (self.w)(val)
    }
}

pub struct IntCode<'a> {
    pc: usize,
    code: Vec<i64>,
    mem: Vec<i64>,
    is_halt: bool,
    io: &'a mut dyn IntCodeIo
}

impl<'a> IntCode<'a> {
    pub fn new(code: Vec<i64>) -> Self {
        IntCode {
            pc: 0,
            code: code.clone(),
            mem: code,
            is_halt: false,
            io: unsafe { &mut EMPTY as &mut dyn IntCodeIo },
        }
    }

    pub fn from_string(code: &str) -> Self {
        Self::new(code.split(',').map(|s| s.parse::<i64>().unwrap()).collect())
    }

    pub fn from_file(path: &str) -> Self {
        Self::from_string(&file_string(path))
    }

    pub fn with_io<'b>(self, io: &'b mut dyn IntCodeIo) -> IntCode<'b> {
        IntCode {
            pc: self.pc,
            code: self.code,
            mem: self.mem,
            is_halt: self.is_halt,
            io
        }
    }

    // pub fn with_reader<T: FnMut() -> i64 + 'static>(mut self, reader: T) -> Self {
    //     self.reader = Box::new(reader);
    //     self
    // }

    // pub fn with_write<T: IntCodeWrite + 'static>(mut self, writer: T) -> Self {
    //     self.writer = Box::new(writer);
    //     self
    // }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.is_halt = false;

        self.mem[0..self.code.len()].copy_from_slice(&self.code);
        self.mem[self.code.len()..].fill(0);
    }

    pub fn run(&mut self) {
        while !self.is_halt {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if self.is_halt {
            return;
        }

        let (op, mode0, mode1, mode2) = self.decode();

        if op == 99 {
            self.is_halt = true;
        } else { 
            match op {
                3 => {
                    *self.read_mode_mut(mode0) = self.io.read();
                }
                4 => {
                    let val = self.read_mode(mode0);
                    self.io.write(val);
                }
                5 | 6 => {
                    let cond = self.read_mode(mode0);
                    let new_pc = self.read_mode(mode1);
                    if (cond != 0 && op == 5) || (cond == 0 && op == 6) {
                        self.pc = new_pc as usize;
                    }
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
        let mut val = self.read_pc();
        if mode == 0 {
            val = self.read(val as usize)
        }
        val
    }

    fn read_mode_mut(&mut self, mode: i64) -> &mut i64 {
        if mode != 0 { panic!(); }
        let val = self.read_pc();
        self.read_mut(val as usize)
    }

    fn read_pc(&mut self) -> i64 {
        self.pc += 1;
        self.read(self.pc - 1)
    }

    fn read_pc_mut(&mut self) -> &mut i64 {
        self.pc += 1;
        self.read_mut(self.pc - 1)
    }

    fn read(&mut self, addr: usize) -> i64 {
        self.mem[addr]
    }

    fn read_mut(&mut self, addr: usize) -> &mut i64 {
        &mut self.mem[addr]
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn pc_mut(&mut self) -> &mut usize {
        &mut self.pc
    }

    pub fn mem(&self) -> &[i64] {
        &self.mem
    }

    pub fn mem_mut(&mut self) -> &mut [i64] {
        &mut self.mem
    }
}