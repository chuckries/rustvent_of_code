use std::{cell::RefCell, collections::HashMap};

use aoc_common::{file_lines, IteratorExt};

type Wires = HashMap<String, Wire>;

enum Input {
    Signal(u16),
    Wire(String),
}

impl Input {
    fn parse(s: &str) -> Self {
        if let Ok(n) = s.parse::<u16>() {
            Input::Signal(n)
        } else {
            Input::Wire(s.to_string())
        }
    }

    fn resolve(&self, wires: &Wires) -> u16 {
        match self {
            Input::Signal(n) => *n,
            Input::Wire(w) => wires[w].resolve(wires),
        }
    }
}

enum UnaryOp {
    Nop,
    Not
}

enum BinaryOp {
    And,
    Or,
    LShift,
    RShift,
}

enum WireType {
    Unary(UnaryOp, Input),
    Binary(BinaryOp, Input, Input),
}

struct Wire {
    wire_type: WireType,
    cached: RefCell<Option<u16>>
}

impl Wire {
    fn new(wire_type: WireType) -> Self {
        Self {
            wire_type,
            cached: RefCell::new(None),
        }
    }

    fn reset(&self) {
        *self.cached.borrow_mut() = None;
    }

    fn resolve(&self, wires: &Wires) -> u16 {
        if let Some(cached) = *self.cached.borrow() {
            return cached;
        }

        let val: u16 = match &self.wire_type {
            WireType::Unary(op, input) => {
                let input = input.resolve(wires);
                match op {
                    UnaryOp::Nop => input,
                    UnaryOp::Not => !input,
                }
            },
            WireType::Binary(op, left, right) => {
                let left = left.resolve(wires);
                let right = right.resolve(wires);

                match op {
                    BinaryOp::And => left & right,
                    BinaryOp::Or => left | right,
                    BinaryOp::LShift => left << right,
                    BinaryOp::RShift => left >> right,
                }
            }
        };

        *self.cached.borrow_mut() = Some(val);
        val
    }
}

fn input() -> Wires {
    let mut wires: Wires = Wires::new();

    for s in file_lines("inputs/day07.txt") {
        let split = s.split(' ').to_vec();

        if split[0] == "NOT" {
            wires.insert(split[3].to_string(), Wire::new(WireType::Unary(UnaryOp::Not, Input::parse(split[1]))));
        } else if split[1] == "->" {
            wires.insert(split[2].to_string(), Wire::new(WireType::Unary(UnaryOp::Nop, Input::parse(split[0]))));
        } else {
            let op = match split[1] {
                "AND" => BinaryOp::And,
                "OR" => BinaryOp::Or,
                "LSHIFT" => BinaryOp::LShift,
                "RSHIFT" => BinaryOp::RShift,
                _ => panic!(),
            };

            wires.insert(split[4].to_string(), Wire::new(WireType::Binary(op, Input::parse(split[0]), Input::parse(split[2]))));
        }
    }

    wires
}

#[test]
fn part1() {
    let wires = input();
    let answer = wires["a"].resolve(&wires);

    assert_eq!(answer, 16076);
}

#[test]
fn part2() {
    let mut wires = input();
    let a = wires["a"].resolve(&wires);

    for w in wires.values() {
        w.reset();
    }

    wires.get_mut("b").unwrap().wire_type = WireType::Unary(UnaryOp::Nop, Input::Signal(a));
    let answer = wires["a"].resolve(&wires);

    assert_eq!(answer, 2797);
}