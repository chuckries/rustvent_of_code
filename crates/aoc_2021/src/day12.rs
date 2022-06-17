use std::{collections::{HashMap, HashSet}};

use aoc_common::file_lines;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Node<'a> {
    Start,
    End,
    Big(&'a str),
    Small(&'a str),
}

impl<'a> Node<'a> {
    fn new(name: &'a str) -> Node<'a> {
        if name == "start" {
            Node::Start
        } else if name == "end" {
            Node::End
        } else if name.chars().next().unwrap().is_uppercase() {
            Node::Big(name)
        } else {
            Node::Small(name)
        }
    }
}

fn backtrack<'a>(current: Node, visited: &mut HashSet<Node<'a>>, map: &'a HashMap<Node<'a>, Vec<Node<'a>>>) -> usize {
    let candidates = map.get(&current).unwrap();

    let mut total = 0;
    for cand in candidates {
        match cand {
            Node::End => total += 1,
            Node::Big(_) => total += backtrack(*cand, visited, map),
            Node::Small(_) if !visited.contains(cand) => {
                visited.insert(*cand);
                total += backtrack(*cand, visited, map);
                visited.remove(cand);
            }
            _ => ()
        }
    }

    total
}

fn backtrack_multiple<'a>(current: Node, visited: &mut HashSet<Node<'a>>, map: &'a HashMap<Node<'a>, Vec<Node<'a>>>, double: Option<Node<'a>>) -> usize {
    let candidates = map.get(&current).unwrap();

    let mut total = 0;
    for cand in candidates {
        match cand {
            Node::End => total += 1,
            Node::Big(_) => total += backtrack_multiple(*cand, visited, map, double),
            Node::Small(_) => {
                if double.is_some() {
                    if !visited.contains(cand) {
                        visited.insert(*cand);
                        total += backtrack_multiple(*cand, visited, map, double);
                        visited.remove(cand);
                    }
                } else if visited.contains(cand) {
                    total += backtrack_multiple(*cand, visited, map, Some(*cand));
                } else {
                    visited.insert(*cand);
                    total += backtrack_multiple(*cand, visited, map, None);
                    visited.remove(cand);
                }
            }
            _ => ()
        }
    }

    total
}

fn input() -> Vec<String> {
    file_lines("inputs/day12.txt").collect()
}

fn mapify<'a>(input: &'a Vec<String>) -> HashMap<Node<'a>, Vec<Node<'a>>> {
    let mut map: HashMap<Node, Vec<Node>> = HashMap::new();
    for line in input.iter() {
        let tok: Vec<&str> = line.split('-').collect();

        let left = Node::new(tok[0]);
        let right = Node::new(tok[1]);

        map.entry(left).or_default().push(right);
        map.entry(right).or_default().push(left);
    }

    map
}

#[test]
fn part1() {
    let input = input();
    let map = mapify(&input);
    let mut visited: HashSet<Node> = HashSet::new();
    let answer = backtrack(Node::Start, &mut visited, &map);

    assert_eq!(answer, 3292);
}

#[test]
fn part2() {
    let input = input();
    let map = mapify(&input);
    let mut visited: HashSet<Node> = HashSet::new();
    let answer = backtrack_multiple(Node::Start, &mut visited, &map, None);

    assert_eq!(answer, 89592);
}