use std::collections::HashMap;

use aoc_common::{file_lines, IteratorExt, lcm};

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

struct GraphData(Vec<String>);

impl GraphData {
    fn graph(&self) -> Graph<'_> {
        self.0.iter().map(|l| {
            let source = &l[0..3];
            let left = &l[7..10];
            let right = &l[12..15];
    
            (source, (left, right))
        }).collect()
    }
}

fn input() -> (Vec<char>, GraphData) {
    let mut lines = file_lines("inputs/day08.txt");
    let dirs = lines.next().unwrap().chars().to_vec();

    lines.next().unwrap();
    let graph = GraphData(lines.to_vec());

    (dirs, graph)
}

fn count_steps_to<F>(start: &str, is_end: F, graph: &Graph, dirs: &[char]) -> i64 
    where F: Fn(&str) -> bool
{
    let mut dirs = dirs.iter().cycle();
    let mut current = start;

    let mut steps = 0;
    loop {
        let c = dirs.next().unwrap();
        current = match *c {
            'L' => graph[current].0,
            'R' => graph[current].1, 
            _ => panic!()
        };

        steps += 1;

        if is_end(current) {
            break;
        }
    }

    steps
}

#[test]
fn part1() {
    let (dirs, graph_data) = input();
    let graph = graph_data.graph();

    let answer = count_steps_to("AAA", |s| s == "ZZZ", &graph, &dirs);

    assert_eq!(22411, answer);
}

#[test]
fn part2() {
    let (dirs, graph_data) = input();
    let graph = graph_data.graph();

    let starts = graph.keys().filter(|s| s.ends_with('A')).copied().to_vec();

    let cycles = starts.into_iter().map(|s| {
        count_steps_to(s, |s| s.ends_with('Z'), &graph, &dirs)
    }).to_vec();

    let answer = lcm(&cycles);

    assert_eq!(11188774513823, answer);
}