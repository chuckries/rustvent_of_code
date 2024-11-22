use aoc_common::{file_lines, IteratorExt};

use Instr::*;

enum Instr {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(i32),
    Jie(usize, i32),
    Jio(usize, i32),
}

fn input() -> Vec<Instr> {
    file_lines("inputs/day23.txt").map(|l| {
        let split = l.split(' ').to_vec();

        let reg: &str;
        let offset: i32;

        if split.len() == 3 {
            reg = split[1].trim_end_matches(',');
            offset = split[2].trim_start_matches('+').parse().unwrap();
        } else if split.len() == 2 {
            let n = split[1].trim_end_matches('+');
            if let Ok(n) = n.parse::<i32>() {
                offset = n;
                reg = "a";
            } else {
                reg = split[1];
                offset = 0;
            }
        } else {
            panic!()
        }

        let reg: usize = match reg {
            "a" => 0,
            "b" => 1,
            _ => panic!()
        };

        match split[0] {
            "hlf" => Hlf(reg),
            "tpl" => Tpl(reg),
            "inc" => Inc(reg),
            "jmp" => Jmp(offset),
            "jie" => Jie(reg, offset),
            "jio" => Jio(reg, offset),
            _ => panic!()
        }
    }).collect()
}

fn run(code: &[Instr], regs: &mut[i32]) {
    fn add_offset(ip: usize, offset: i32) -> usize {
        (ip as i32 + (offset - 1)) as usize
    }

    let mut ip = 0;

    loop {
        if ip >= code.len() {
            break;
        }

        match code[ip] {
            Hlf(reg) => regs[reg] /= 2,
            Tpl(reg) => regs[reg] *= 3,
            Inc(reg) => regs[reg] += 1,
            Jmp(offset) => ip = add_offset(ip, offset),
            Jie(reg, offset) => if regs[reg] % 2 == 0 { ip = add_offset(ip, offset) },
            Jio(reg, offset) => if regs[reg] == 1 { ip = add_offset(ip, offset) },
        }

        ip += 1;
    }
}

#[test]
fn part1() {
    let code = input();
    let mut regs = [0, 0];
    run (&code, &mut regs);
    
    let answer = regs[1];
    assert_eq!(answer, 307);
}

#[test]
fn part2() {
    let code = input();
    let mut regs = [1, 0];
    run (&code, &mut regs);
    
    let answer = regs[1];
    assert_eq!(answer, 160);
}