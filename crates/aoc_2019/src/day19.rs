use aoc_common::{Vec2i64};

use intcode::IntCode;

struct Calc {
    computer: IntCode
}

impl Calc {
    fn new() -> Calc {
        Calc {
            computer: IntCode::from_file("inputs/day19.txt")
        }
    }

    fn get(&mut self, x: i64, y: i64) -> i64 {
        let result = self.computer.run_input(&[x, y]).unwrap();
        self.computer.reset();
        result
    }
}

const AREA: i64 = 50;

#[test]
fn part1() {
    let mut calc = Calc::new();

    let answer = Vec2i64::new(AREA, AREA)
        .iter()
        .filter(|p| {
            calc.get(p.x, p.y) == 1
        })
        .count();

    assert_eq!(answer, 112);
}

#[test]
fn part2() {
    let mut calc = Calc::new();

    let mut y = 40;
    let mut x_begin = 0;
    let mut x_end;

    while calc.get(x_begin, y) == 0 {
        x_begin +=1;
    }

    x_end = x_begin;
    while calc.get(x_end, y) == 1 {
        x_end += 1;
    }

    let answer: Vec2i64;
    loop {
        if x_end - x_begin >= 100 {
            let cand_x = x_end - 100;
            let cand_y = y + 100 - 1;
            if calc.get(cand_x, cand_y) == 1 {
                answer = (cand_x, y).into();
                break;
            }
        }

        y += 1;
        while calc.get(x_begin, y) == 0 {
            x_begin += 1;
        }
        while calc.get(x_end, y) == 1 {
            x_end += 1;
        }
    }
    let answer = answer.x * 10000 + answer.y;

    assert_eq!(answer, 18261982);
}