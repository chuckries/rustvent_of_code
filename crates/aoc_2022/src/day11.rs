use std::{collections::VecDeque, str::FromStr};
use aoc_common::{file_lines, IteratorExt};

enum Identifier {
    Old,
    Const(i64),
}

impl Identifier {
    fn value(&self, old: i64) -> i64 {
        match self {
            Identifier::Old => old,
            Identifier::Const(c) => *c,
        }
    }
}

impl FromStr for Identifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Identifier::Old),
            _ => s.parse::<i64>().and_then(|c| Ok(Identifier::Const(c))).or(Err(()))
        }
    }
}

enum Op {
    Add,
    Mul,
    Div,
    Mod,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => return Err(()),
        };

        Ok(op)
    }
}

struct Expression(Identifier, Op, Identifier);

impl Expression {
    fn evaluate(&self, old: i64) -> i64 {
        let left = self.0.value(old);
        let right = self.2.value(old);
        match self.1 {
            Op::Add => left + right,
            Op::Mul => left * right,
            Op::Div => left / right,
            Op::Mod => left % right,
        }
    }
}

struct Monkey {
    items: VecDeque<i64>,
    expr: Expression,
    div_test: i64,
    next_true: usize,
    next_false: usize,
}

impl Monkey {
    fn inspect_next(&mut self, reduction: &Expression) -> Option<(i64, usize)> {
        self.items.pop_front().and_then(|i| {
            let mut value = self.expr.evaluate(i);
            value = reduction.evaluate(value);
            let next = if value % self.div_test == 0 {
                self.next_true
            } else {
                self.next_false
            };

            Some((value, next))
        })
    }
}

fn input() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut lines = file_lines("inputs/day11.txt");
    while let Some(_) = lines.next() {
        let line = lines.next().unwrap();
        let items = line.trim_start().split(' ').skip(2).map(|i| i.trim_end_matches(',').parse::<i64>().unwrap()).to_vec_deque();

        let line = lines.next().unwrap();
        let split = line.trim_start().split(' ').to_vec();
        let expr = Expression(split[3].parse().unwrap(), split[4].parse().unwrap(), split[5].parse().unwrap());

        let line = lines.next().unwrap();
        let split = line.trim_start().split(' ').to_vec();
        let div_test: i64 = split[3].parse().unwrap();

        let line = lines.next().unwrap();
        let split = line.trim_start().split(' ').to_vec();
        let next_true: usize = split[5].parse().unwrap();

        let line = lines.next().unwrap();
        let split = line.trim_start().split(' ').to_vec();
        let next_false: usize = split[5].parse().unwrap();

        monkeys.push(Monkey {
            items,
            expr,
            div_test,
            next_true,
            next_false,
        });

        lines.next();
    }

    monkeys
}

fn run(monkeys: &mut [Monkey], rounds: i32, reduction: &Expression) -> i64 {
    let mut counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((value, next)) = monkeys[i].inspect_next(reduction) {
                counts[i] += 1;
                monkeys[next].items.push_back(value);
            }
        }
    }

    counts.into_iter().sorted_by(|a, b| b.cmp(a)).take(2).product()
}

#[test]
fn part1() {
    let mut monkeys = input();
    let expr = Expression(Identifier::Old, Op::Div, Identifier::Const(3));
    let answer = run(&mut monkeys, 20, &expr);
    assert_eq!(answer, 90294);
}

#[test]
fn part2() {
    let mut monkeys = input();
    let divisor = monkeys.iter().map(|m| m.div_test).product();
    let expr = Expression(Identifier::Old, Op::Mod, Identifier::Const(divisor));
    let answer = run(&mut monkeys, 10000, &expr);
    assert_eq!(answer, 18170818354);
}