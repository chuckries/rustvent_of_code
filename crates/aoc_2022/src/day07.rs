use std::collections::HashMap;
use aoc_common::{file_lines, IteratorExt};
use Node::*;

enum Node {
    Dir(HashMap<String, Node>),
    File(usize),
}

impl Node {
    fn as_dir(&mut self) -> &mut HashMap<String, Node> {
        match self {
            Dir(map) => map,
            _ => panic!()
        }
    }
}

fn input() -> Node {
    fn explore(mut idx: usize, lines: &[String], cd: &mut HashMap<String, Node>) -> usize {
        while idx < lines.len() {
            let split = lines[idx].split(' ').to_vec();
            idx += 1;
    
            if split[0] != "$" { panic!(); }
    
            match split[1] {
                "cd" => match split[2] {
                    ".." => break,
                    next @ _ => idx = explore(idx, lines, cd.get_mut(next).unwrap().as_dir()),
                }
                "ls" => {
                    while idx < lines.len() {
                        let split = lines[idx].split(' ').to_vec();
                        match split[0] {
                            "$" => break,
                            "dir" => cd.insert(split[1].to_string(), Dir(HashMap::new())),
                            size @ _ => cd.insert(split[1].to_string(), File(size.parse().unwrap())),
                        };
    
                        idx += 1;
                    }
                }
                _ => panic!()
            }
        }
    
        idx
    }

    let mut lines = file_lines("inputs/day07.txt").to_vec();
    let mut root: HashMap<String, Node> = HashMap::new();
    root.insert("/".to_string(), Dir(HashMap::new()));
    _ = explore(0, &mut lines, &mut root);
    root.into_iter().next().unwrap().1
}

fn sum_visit<F>(node: &Node, f: &mut F) -> usize
    where F: FnMut(usize)
{
    match node {
        File(size) => *size,
        Dir(nodes) => {
            let sum = nodes.iter().map(|(_, n)| sum_visit(n, f)).sum();
            f(sum);
            sum
        }
    }
}

#[test]
fn part1() {
    let mut answer = 0;
    sum_visit(&input(), &mut |s| {
        if s <= 100000 {
            answer += s;
        }
    });

    assert_eq!(answer, 1723892);
}

const AVAILABLE: usize = 70000000;
const UNUSED_NEEDED: usize = 30000000;

#[test]
fn part2() {
    let mut answer = usize::MAX;
    let min_to_delete = UNUSED_NEEDED - (AVAILABLE - sum_visit(&input(), &mut |_| { }));
    sum_visit(&input(), &mut |s| {
        if s > min_to_delete && s < answer {
            answer = s;
        }
    });

    assert_eq!(answer, 8474158);
}