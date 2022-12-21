use std::collections::{HashMap};

use aoc_common::{file_lines, IteratorExt};

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn op(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Div => lhs / rhs,
            Op::Mul => lhs * rhs,
        }
    }
}

enum Node {
    Tree(Box<Node>, Op, Box<Node>),
    Leaf(i64),
    Humn(i64),
}

enum PartialOp {
    Lhs(Op, i64),
    Rhs(i64, Op),
}

enum Eval {
    Scalar(i64),
    Stack(Vec<PartialOp>)
}

impl Node {
    fn eval(&self) -> i64 {
        match self {
            Node::Leaf(val) => *val,
            Node::Humn(val) => *val,
            Node::Tree(lhs, op, rhs) => {
                let lhs = lhs.eval();
                let rhs = rhs.eval();
                op.op(lhs, rhs)
            }

        }
    }

    fn eval2(&self) -> Eval {
        match self {
            Node::Humn(_) => Eval::Stack(Vec::new()),
            Node::Leaf(val) => Eval::Scalar(*val),
            Node::Tree(lhs, op, rhs) => {
                let lhs = lhs.eval2();
                let rhs = rhs.eval2();

                match (lhs, rhs) {
                    (Eval::Scalar(lhs), Eval::Scalar(rhs)) => Eval::Scalar(op.op(lhs, rhs)),
                    (Eval::Stack(mut s), Eval::Scalar(rhs)) => {
                        s.push(PartialOp::Lhs(*op, rhs));
                        Eval::Stack(s)
                    }
                    (Eval::Scalar(lhs), Eval::Stack(mut s)) => {
                        s.push(PartialOp::Rhs(lhs, *op));
                        Eval::Stack(s)
                    }
                    _ => panic!()
                }
            }
        }
    }
}

fn input() -> Node {
    let lines = file_lines("inputs/day21.txt").to_vec();

    let map: HashMap<&str, Vec<&str>> = lines.iter().map(|l| {
        let split = l.split([' ']).to_vec();
        (split[0].trim_end_matches(':'), split)
    }).collect();

    fn get_node(name: &str, map: &HashMap<&str, Vec<&str>>) -> Node {
        let data = &map[name];

        if data.len() == 2 {
            let val = data[1].parse().unwrap();
            match name {
                "humn" => Node::Humn(val),
                _ => Node::Leaf(val),
            }
        } else {
            let op = match data[2] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "/" => Op::Div,
                "*" => Op::Mul,
                _ => panic!()
            };

            Node::Tree(Box::new(get_node(&data[1], map)), op, Box::new(get_node(&data[3], map)))
        }
    }

    get_node("root", &map)
}

#[test]
fn part1() {
    let root = input();
    let answer = root.eval();
    assert_eq!(answer, 21208142603224);
}

#[test]
fn part2() {
    if let Node::Tree(lhs, _, rhs) = input() {
        let lhs = lhs.eval2();
        let rhs = rhs.eval2();

        let (mut stack, mut total) = match (lhs, rhs) {
            (Eval::Scalar(val), Eval::Stack(s)) => (s, val),
            (Eval::Stack(s), Eval::Scalar(val), ) => (s, val),
            _ => panic!()
        };

        while let Some(partial) = stack.pop() {
            match partial {
                PartialOp::Lhs(Op::Add, val) => total -= val,
                PartialOp::Lhs(Op::Sub, val) => total += val,
                PartialOp::Lhs(Op::Div, val) => total *= val,
                PartialOp::Lhs(Op::Mul, val) => total /= val,
                PartialOp::Rhs(val, Op::Add) => total -= val,
                PartialOp::Rhs(val, Op::Sub) => total = val - total,
                PartialOp::Rhs(val, Op::Div) => total /= val,
                PartialOp::Rhs(val, Op::Mul) => total /= val,
            }
        }

        assert_eq!(total, 3882224466191);

    }
}