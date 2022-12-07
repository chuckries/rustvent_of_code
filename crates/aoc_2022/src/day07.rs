use std::{collections::HashMap, iter::Peekable};
use aoc_common::{file_lines, IteratorExt};
use Node::*;

type Directory = HashMap<String, Node>;

enum Node {
    Dir(Directory),
    File(usize),
}

impl Node {
    fn as_dir(&mut self) -> &mut Directory {
        match self {
            Dir(map) => map,
            _ => panic!()
        }
    }
}

fn input() -> Node {
    fn explore<T>(lines: &mut Peekable<T>, cd: &mut Directory) 
        where T: Iterator<Item = String>
    {
        while let Some(line) = lines.next() {
            let split = line.split(' ').to_vec();

            if split[0] != "$" { panic!(); }

            match split[1] {
                "cd" => match split[2] {
                    ".." => break,
                    next @ _ => explore(lines, cd.get_mut(next).unwrap().as_dir()),
                }
                "ls" => {
                    while let Some(line) = lines.peek() {
                        let split = line.split(' ').to_vec();
                        match split[0] {
                            "$" => break,
                            "dir" => cd.insert(split[1].to_string(), Dir(Directory::new())),
                            size @ _ => cd.insert(split[1].to_string(), File(size.parse().unwrap())),
                        };
    
                        lines.next();
                    }
                }
                _ => panic!()
            }
        }
    }

    let lines = file_lines("inputs/day07.txt");
    let mut root = Directory::new();
    root.insert("/".to_string(), Dir(Directory::new()));
    explore(&mut lines.peekable(), &mut root);
    root.into_values().next().unwrap()
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
        if s >= min_to_delete && s < answer {
            answer = s;
        }
    });

    assert_eq!(answer, 8474158);
}