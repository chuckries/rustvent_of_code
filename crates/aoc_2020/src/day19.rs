use std::collections::HashMap;

use aoc_common::{file_lines, ToVec};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Rule {
    Term(char),
    NonTerm(Vec<Vec<usize>>)
}

fn input() -> (Tester, Vec<Vec<char>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^(?P<id>\d+): ("(?P<term>[ab])"|(?P<left>\d+( \d+)*)( \| (?P<right>\d+( \d+)*))?)$"#).unwrap();
    }

    let mut lines = file_lines("inputs/day19.txt");

    let mut rules: HashMap<usize, Rule> = HashMap::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let cap = RE.captures(&line).unwrap();

        let id = cap.name("id").unwrap().as_str().parse::<usize>().unwrap();
        let rule = if let Some(term) = cap.name("term") {
            Rule::Term(term.as_str().chars().next().unwrap())
        } else {
            let left = cap.name("left").unwrap().as_str().split(' ').map(|i| i.parse::<usize>().unwrap()).to_vec();
            let mut rules = vec![left];
            if let Some(right) = cap.name("right") {
                let right = right.as_str().split(' ').map(|i| i.parse::<usize>().unwrap()).to_vec();
                rules.push(right);
            }
            Rule::NonTerm(rules)
        };
        rules.insert(id, rule);
    }

    let mut sorted = rules.into_iter().to_vec();
    sorted.sort_by_key(|pair| pair.0);
    let (_, rules): (Vec<_>, Vec<_>) = sorted.into_iter().unzip();
    let messages = lines.map(|l| l.chars().to_vec()).to_vec();

    (Tester { rules }, messages)
}

struct Tester {
    rules: Vec<Rule>,
}

impl Tester {
    fn test(&self, message: &[char]) -> bool {
        match self.test_recurse(message, 0, 0) {
            Some(len) if len == message.len() => true,
            _ => false
        }
    }

    fn test_recurse(&self, message: &[char], idx: usize, rule: usize) -> Option<usize> {
        match &self.rules[rule] {
            Rule::Term(c) => {
                if message[idx] == *c {
                    Some(1)
                } else {
                    None
                }
            }
            Rule::NonTerm(parts) => {
                'outer: for side in parts {
                    let mut current = idx;
                    for rule in side {
                        if let Some(delta) = self.test_recurse(message, current, *rule) {
                            current += delta;
                        } else {
                            continue 'outer;
                        }
                    }
                    return Some(current - idx);
                }

                None
            }
        }
    }

    fn test_loops(&self, message: &[char]) -> bool {
        self.test_loops_recurse(message, 0, 0, 0)
    }

    fn test_loops_recurse(&self, message: &[char], idx: usize, count_42: usize, count_31: usize) -> bool {
        if idx == message.len() {
            return count_31 > 0 && count_42 > count_31
        }

        if count_31 == 0 {
            if let Some(delta) = self.test_recurse(message, idx, 42) {
                if self.test_loops_recurse(message, idx + delta, count_42 + 1, count_31) {
                    return true;
                }
            }
        }

        if count_42 >= 2 {
            if let Some(delta) = self.test_recurse(message, idx, 31) {
                if self.test_loops_recurse(message, idx + delta, count_42, count_31 + 1) {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn part1() {
    let (tester, messages) = input();
    let answer = messages.into_iter().filter(|m| tester.test(&m)).count();
    assert_eq!(answer, 269);
}

#[test]
fn part2() {
    let (tester, messages) = input();
    let answer = messages.into_iter().filter(|m| tester.test_loops(m)).count();
    assert_eq!(answer, 403);
}