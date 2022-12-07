use aoc_common::{file_lines, IteratorExt};

use Kind::*;

struct Node {
    name: String,
    kind: Kind,
}

enum Kind {
    Dir(Vec<Node>),
    File(usize),
}

fn explore(idx: &mut usize, lines: &[String], cd: &mut Vec<Node>) {
    while *idx < lines.len() {
        let line = &lines[*idx];
        *idx += 1;

        if line.starts_with('$') {
            let split = line.split(' ').to_vec();

            match split[1] {
                "cd" => {
                    let name = split[2];
                    if name == ".." {
                        return;
                    } else {
                        let dir = cd.iter_mut().find(|d| d.name == name).unwrap();
                        if let Dir(dir) = &mut dir.kind {
                            explore(idx, lines, dir);
                        } else {
                            panic!()
                        }
                    }
                }
                "ls" => {
                    while *idx < lines.len() {
                        let split = lines[*idx].split(' ').to_vec();
                        match split[0] {
                            "$" => break,
                            "dir" => {
                                cd.push(Node { name: split[1].to_string(), kind: Dir(Vec::new()) });
                            }
                            _ => {
                                let size = split[0].parse().unwrap();
                                let name = split[1].to_string();
                                cd.push(Node { name: name, kind: File(size) });
                            }
                        }
                        *idx += 1;
                    }
                }
                _ => panic!()
            }
        } else {
            panic!();
        }
    }
}

fn sum(node: &Node, running_total: &mut usize) -> usize {
    match &node.kind {
        File(size) => *size,
        Dir(nodes) => {
            let sum = nodes.iter().map(|n| sum(n, running_total)).sum();

            if sum <= 100000 {
                *running_total += sum;
            }

            sum
        }
    }
}

fn find_to_delete(node: &Node, min_required: usize, min: &mut usize) -> usize {
    match &node.kind {
        File(size) => *size,
        Dir(nodes) => {
            let sum = nodes.iter().map(|n| find_to_delete(n, min_required, min)).sum();

            if sum >= min_required && sum < *min {
                *min = sum;
            }

            sum
        }
    }
}

#[test]
fn part1() {
    let lines = file_lines("inputs/day07.txt").to_vec();
    let mut root: Vec<Node> = Vec::new();
    root.push(Node { name: "/".to_string(), kind: Dir(Vec::new()) });

    let mut idx: usize = 0;
    explore(&mut idx, &lines, &mut root);

    let mut answer = 0;
    sum(&root[0], &mut answer);

    assert_eq!(answer, 1723892);
}

const AVAILABLE: usize = 70000000;
const UNUSED_NEEDED: usize = 30000000;

#[test]
fn part2() {
    let lines = file_lines("inputs/day07.txt").to_vec();
    let mut root: Vec<Node> = Vec::new();
    root.push(Node { name: "/".to_string(), kind: Dir(Vec::new()) });

    let mut idx: usize = 0;
    explore(&mut idx, &lines, &mut root);

    let mut whatever = 0;
    let total = sum(&root[0], &mut whatever);

    let min_to_delete = UNUSED_NEEDED - (AVAILABLE - total);

    let mut answer = usize::MAX;
    find_to_delete(&root[0], min_to_delete, &mut answer);

    assert_eq!(answer, 8474158);
}