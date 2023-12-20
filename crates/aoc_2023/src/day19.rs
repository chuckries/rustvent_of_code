use std::collections::{HashMap, VecDeque};

use aoc_common::{file_lines, IteratorExt};

#[derive(Default, Clone, Copy, Debug)]
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

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    fn m_mut(&mut self) -> &mut i32 {
        &mut self.m
    }

    fn a_mut(&mut self) -> &mut i32 {
        &mut self.a
    }

    fn s_mut(&mut self) -> &mut i32 {
        &mut self.s
    }

    fn volume(min: Self, max: Self) -> usize {
        (max.x - min.x + 1) as usize * (max.m - min.m + 1) as usize * (max.a - min.a + 1) as usize * (max.s - min.s + 1) as usize
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
    fn evaluate(&self, part: &Part) -> Option<RuleResult> {
        match self {
            Self::Conditional(category, op, constant, result) => {
                let category = match *category {
                    Category::X => part.x,
                    Category::M => part.m,
                    Category::A => part.a,
                    Category::S => part.s,
                };

                let meets_condition = match *op {
                    Op::Lt => category < *constant,
                    Op::Gt => category > *constant,
                };

                if meets_condition {
                    Some(result.clone())
                } else {
                    None
                }
            }
            Self::Unconditional(result) => Some(result.clone()),
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
            if let Some(result) = rule.evaluate(part) {
                return result;
            }
        }

        panic!();
    }
}

fn input() -> (HashMap<String, Workflow>, Vec<Part>) {
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
                let category = &cmp[0..op_idx];
                let op = &cmp[op_idx..op_idx + 1];
                let constant: i32 = cmp[op_idx + 1 ..].parse().unwrap();

                let category = match category {
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

                rules.push(Rule::Conditional(category, op, constant, result));
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

    let workflows: HashMap<String, Workflow> = workflows.into_iter().map(|w| (w.name.clone(), w)).collect();

    (workflows, parts)
}

#[test]
fn part1() {
    let (workflows, parts) = input();

    let answer: i32 = parts.iter().filter_map(|p| {
        let mut current = &workflows["in"];
        loop {
            match current.evaluate(p) {
                RuleResult::Accept => return Some(p.sum()),
                RuleResult::Reject => return None,
                RuleResult::Workflow(next) => current = &workflows[&next],
            };
        }
    }).sum();

    assert_eq!(509597, answer);
}

struct State {
    workflow: String,
    min: Part,
    max: Part,
}

#[test]
fn part2() {
    let (workflows, _) = input();

    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(State {
        workflow: "in".to_string(),
        min: Part { x: 1, m: 1, a: 1, s: 1 },
        max: Part { x: 4000, m: 4000, a: 4000, s: 4000 },
    });

    let mut accepted: Vec<(Part, Part)> = Vec::new();

    while let Some(State { workflow, mut min, mut max }) = queue.pop_front() {
        let workflow = &workflows[&workflow];

        for rule in workflow.rules.iter() {
            match rule {
                Rule::Unconditional(result) => {
                    match result {
                        RuleResult::Accept => accepted.push((min, max)),
                        RuleResult::Reject => (),
                        RuleResult::Workflow(workflow) => {
                            queue.push_back(State {
                                workflow: workflow.clone(),
                                min,
                                max
                            });
                        }
                    }
                },
                Rule::Conditional(category, op, constant, result) => {
                    let selector = match category {
                        Category::X => Part::x_mut,
                        Category::M => Part::m_mut,
                        Category::A => Part::a_mut,
                        Category::S => Part::s_mut,
                    };

                    let min_prop = *selector(&mut min);
                    let max_prop = *selector(&mut max);

                    let (range_in, range_out): (Option<(Part,Part)>, Option<(Part, Part)>) = match op {
                        Op::Lt => {
                            let mut range_in = None;
                            let mut range_out = None;

                            if min_prop < *constant {
                                let min_in = min;
                                let mut max_in = max;
                                *selector(&mut max_in) = max_prop.min(*constant - 1);
                                range_in = Some((min_in, max_in));
                            }

                            if max_prop >= *constant {
                                let mut min_out = min;
                                let max_out = max;
                                *selector(&mut min_out) = min_prop.max(*constant);
                                range_out = Some((min_out, max_out));
                            }

                            (range_in, range_out)
                        },
                        Op:: Gt => {
                            let mut range_in = None;
                            let mut range_out = None;

                            if max_prop > *constant {
                                let mut min_in = min;
                                let max_in = max;
                                *selector(&mut min_in) = min_prop.max(*constant + 1);
                                range_in = Some((min_in, max_in));
                            }

                            if min_prop <= *constant {
                                let min_out = min;
                                let mut max_out = max;
                                *selector(&mut max_out) = max_prop.min(*constant);
                                range_out = Some((min_out, max_out));
                            }

                            (range_in, range_out)
                        }
                    };

                    if let Some((min_in, max_in)) = range_in {
                        match result {
                            RuleResult::Accept => accepted.push((min_in, max_in)),
                            RuleResult::Reject => (),
                            RuleResult::Workflow(next_workflow) => {
                                queue.push_back(State {
                                    workflow: next_workflow.clone(),
                                    min: min_in,
                                    max: max_in,
                                });
                            }
                        }
                    }

                    if let Some((min_out, max_out)) = range_out {
                        min = min_out;
                        max = max_out;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let mut total_volume: usize = 0;
    let mut added: Vec<(Part, Part)> = Vec::new();
    for (min, max) in accepted {
        total_volume += Part::volume(min, max);

        for (existing_min, existing_max) in added.iter() {
            if min.x > existing_max.x || max.x < existing_min.x ||
               min.m > existing_max.m || max.m < existing_min.m ||
               min.a > existing_max.a || max.a < existing_min.a ||
               min.s > existing_max.s || max.s < existing_min.s {
                continue;
            }

            let overlapped_min = Part {
                x: min.x.max(existing_min.x),
                m: min.m.max(existing_min.m),
                a: min.a.max(existing_min.a),
                s: min.s.max(existing_min.s),
            };

            let overlapped_max = Part {
                x: max.x.min(existing_max.x),
                m: max.m.min(existing_max.m),
                a: max.a.min(existing_max.a),
                s: max.s.min(existing_max.s),
            };

            let overlapped_volume = Part::volume(overlapped_min, overlapped_max);
            total_volume -= overlapped_volume;
        }

        added.push((min, max));
    }

    assert_eq!(143219569011526, total_volume);
}