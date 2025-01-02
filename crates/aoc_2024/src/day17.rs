use aoc_common::{file_lines, IteratorExt};

fn input() -> ([i64; 3], Vec<i32>) {
    let mut lines = file_lines("inputs/day17.txt");

    let mut regs = [0; 3];
    for i in 0..3 {
        let reg = lines.next().unwrap().split(' ').skip(2).next().unwrap().parse::<i64>().unwrap();
        regs[i] = reg;
    }

    lines.next().unwrap();

    let code = lines.next().unwrap().split([' ', ',']).skip(1).map(|s| s.parse::<i32>().unwrap()).to_vec(); 

    (regs, code)
}

#[test]
fn part1() {
    let (mut regs, code) = input();
    let mut ip = 0;
    let mut output: Vec<i32> = Vec::new();

    while ip < code.len() {
        let op = code[ip];
        ip += 1;
        let operand = code[ip];
        ip += 1;

        let literal = operand as i64;
        let combo = match operand {
            0..=3 => operand as i64,
            4..=6 => regs[operand as usize - 4],
            _ => 0,
        };

        match op {
            0 => {
                regs[0] = regs[0] / i64::pow(2, combo as u32)
            }
            1 => {
                regs[1] = regs[1] ^ literal;
            }
            2 => {
                regs[1] = combo % 8;
            }
            3 => {
                if regs[0] != 0 {
                    ip = literal as usize;
                }
            }
            4 => {
                regs[1] = regs[1] ^ regs[2];
            }
            5 => {
                output.push((combo % 8) as i32);
            }
            6 => {
                regs[1] = regs[0] / i64::pow(2, combo as u32)
            }
            7 => {
                regs[2] = regs[0] / i64::pow(2, combo as u32)
            }
            _ => panic!(),
        }
    }

    let output = output.into_iter().map(|i| i.to_string()).to_vec();
    let output = output.join(",");
    assert_eq!(output, "1,5,0,1,7,4,1,0,3");
}

#[test]
fn part2() {
    // this is all just hard coded by hand for my input; this could be made to be generic but would require big assumptions such as all inputs are
    // hashing 3 bits at a time and shifting them in the specific way. This could be validated as part of the problem but whatever.
    // This worked.

    fn recurse(nums: &[i64], idx: usize, mut partial: i64) -> Option<i64> {
        if idx == nums.len() {
            return Some(partial);
        }

        partial <<= 3;
        let mut a;
        let mut b;
        let mut c;
        // find the first 3 bit number that produces the target output based on the current partial answer
        for i in 0..8 {
            a = partial | i;
            b = a & 7;
            b ^= 6;
            c = a / i64::pow(2, b as u32);
            b ^= c;
            b ^= 7;
            b &= 7;
            if b == nums[idx] {
                if let Some(r#final) = recurse(nums, idx + 1, partial | i) {
                    return Some(r#final);
                }
            }
        }
        
        None
    }

    let mut target = vec![2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0];
    target.reverse();

    let answer = recurse(&target, 0, 0).unwrap();
    assert_eq!(answer, 47910079998866);
}