use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt};

#[derive(Default)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Copy, Clone)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Clone)]
enum RuleResult {
    None,
    Accept,
    Reject,
    Workflow(String),
}

enum Op {
    Lt,
    Gt,
}

enum Rule {
    Conditional(Category, Op, i32, RuleResult),
    Unconditional(RuleResult),
}

impl Rule {
    fn evaluate(&self, part: &Part) -> RuleResult {
        match self {
            Self::Conditional(category, op, constant, result) => {
                let property = match *category {
                    Category::X => part.x,
                    Category::M => part.m,
                    Category::A => part.a,
                    Category::S => part.s,
                };

                let meets_condition = match *op {
                    Op::Lt => property < *constant,
                    Op::Gt => property > *constant,
                };

                if meets_condition {
                    result.clone()
                } else {
                    RuleResult::None
                }
            }
            Self::Unconditional(result) => result.clone(),
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> RuleResult {
        for rule in self.rules.iter() {
            let result = rule.evaluate(part);
            if !matches!(result, RuleResult::None) {
                return result;
            }
        }

        panic!();
    }
}

fn input() -> (Vec<Workflow>, Vec<Part>) {
    let mut lines = file_lines("inputs/day19.txt");

    let mut workflows: Vec<Workflow> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split(|c| matches!(c, '{' | '}'));
        let name = split.next().unwrap().to_string();
        let rules = split.next().unwrap();
        let rule_strings = rules.split(',');

        let mut rules: Vec<Rule> = Vec::new();
        for rule in rule_strings {
            let parts = rule.split(':').to_vec();
            if parts.len() == 1 {
                let result = match parts[0] {
                    "A" => RuleResult::Accept,
                    "R" => RuleResult::Reject,
                    _ => RuleResult::Workflow(parts[0].to_string()),
                };
                rules.push(Rule::Unconditional(result));
            } else {
                let cmp = parts[0];
                let result = parts[1];

                let op_idx = cmp.find(|c| matches!(c, '<' | '>')).unwrap();
                let prop = &cmp[0..op_idx];
                let op = &cmp[op_idx..op_idx + 1];
                let constant: i32 = cmp[op_idx + 1 ..].parse().unwrap();

                let prop = match prop {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => panic!(),
                };

                let op =  match op {
                    "<" => Op::Lt,
                    ">" => Op::Gt,
                    _ => panic!(),
                };

                let result = match result {
                    "A" => RuleResult::Accept,
                    "R" => RuleResult::Reject,
                    _ => RuleResult::Workflow(result.to_string()),
                };

                rules.push(Rule::Conditional(prop, op, constant, result));
            }
        }

        workflows.push(Workflow { name, rules });
    }

    let mut parts: Vec<Part> = Vec::new();
    while let Some(line) = lines.next() {
        let split = line.trim_matches(|c| matches!(c, '{' | '}')).split(',');
        let mut part = Part::default();
        for s in split {
            let mut split = s.split('=');
            let prop = split.next().unwrap();
            let val: i32 = split.next().unwrap().parse().unwrap();
            *match prop {
                "x" => &mut part.x,
                "m" => &mut part.m,
                "a" => &mut part.a,
                "s" => &mut part.s,
                _ => panic!(),
            } = val;
        }
        parts.push(part);
    }

    (workflows, parts)
}

#[test]
fn part1() {
    let (rules, parts) = input();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for w in rules {
        workflows.insert(w.name.clone(), w);
    }

    let answer: i32 = parts.iter().filter_map(|p| {
        let mut current = &workflows["in"];
        loop {
            match current.evaluate(p) {
                RuleResult::Accept => return Some(p.sum()),
                RuleResult::Reject => return None,
                RuleResult::Workflow(next) => current = &workflows[&next],
                _ => panic!()
            };
        }
    }).sum();

    assert_eq!(509597, answer);
}