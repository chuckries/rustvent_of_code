use std::{collections::HashMap, ops::{AddAssign, SubAssign}};

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<String> {
    file_lines("inputs/day08.txt").collect()
}

fn run<F>(regs: &mut HashMap<String, i32>, mut f: F) 
    where F: FnMut(i32)
{
    let input = input();
    for l in input {
        let split = l.split(' ').to_vec();

        let cond_reg = *regs.entry(split[4].to_string()).or_default();
        let cond_op = match split[5] {
            ">" => i32::gt,
            "<" => i32::lt,
            ">=" => i32::ge,
            "<=" => i32::le,
            "==" => i32::eq,
            "!=" => i32::ne,
            _ => panic!()
        };
        let cond_val = split[6].parse::<i32>().unwrap();

        if !cond_op(&cond_reg, &cond_val) {
            continue;
        }

        let mod_reg = regs.entry(split[0].to_string()).or_default();
        let mod_op = match split[1] {
            "inc" => <i32 as AddAssign>::add_assign,
            "dec" => <i32 as SubAssign>::sub_assign,
            _ => panic!()
        };
        let mod_val = split[2].parse::<i32>().unwrap();

        mod_op(mod_reg, mod_val);
        f(*mod_reg);
    }
}

#[test]
fn part1() {
    let mut regs: HashMap<String, i32> = HashMap::new();
    run(&mut regs, |_|());

    let answer = *regs.values().max().unwrap();
    assert_eq!(answer, 4416);
}

#[test]
fn part2() {
    let mut regs: HashMap<String, i32> = HashMap::new();
    let mut max = 0;
    run(&mut regs, |reg| {
        if reg > max {
            max = reg;
        }
    });

    assert_eq!(max, 5199);
}