use std::{cell::RefCell, collections::HashMap};

use aoc_common::{file_lines, IteratorExt};

type InputGraph = HashMap<usize, Robot>;
type OutputGraph = HashMap<usize, Input>;

enum Input {
    Constant(usize),
    Low(usize),
    High(usize),
}

impl Input {
    fn resolve(&self, graph: &InputGraph) -> usize {
        match self {
            Input::Constant(c) => *c,
            Input::Low(id) => graph[id].low(graph),
            Input::High(id) => graph[id].high(graph),
        }
    }
}

#[derive(Default)]
struct Robot {
    inputs: Vec<Input>,
    cached: RefCell<Option<[usize; 2]>>
}

impl Robot {
    fn low(&self, graph: &InputGraph) -> usize {
        self.resolve(graph)[0]
    }

    fn high(&self, graph: &InputGraph) -> usize {
        self.resolve(graph)[1]
    }

    fn resolve(&self, graph: &InputGraph) -> [usize; 2] {
        if self.cached.borrow().is_none() {
            let values = self.inputs.iter().map(|i| i.resolve(graph)).to_vec();
            *self.cached.borrow_mut() = Some(if values[0] < values[1] {
                [values[0], values[1]]
            } else {
                [values[1], values[0]]
            });
        }
        self.cached.borrow().unwrap()
    }
}

fn input() -> (InputGraph, OutputGraph) {
    let mut graph = InputGraph::new();
    let mut outputs = OutputGraph::new();

    for l in file_lines("inputs/day10.txt") {
        let split = l.split(' ').to_vec();
        if split[0] == "value" {
            let value = split[1].parse().unwrap();
            let id = split[5].parse().unwrap();
            graph.entry(id).or_default().inputs.push(Input::Constant(value));
        } else if split[0] == "bot" {
            let src = split[1].parse().unwrap();
            let low = split[6].parse().unwrap();
            let high = split[11].parse().unwrap();

            if split[5] == "bot" {
                graph.entry(low).or_default().inputs.push(Input::Low(src));
            } else {
                outputs.insert(low, Input::Low(src));
            }
            if split[10] == "bot" {
                graph.entry(high).or_default().inputs.push(Input::High(src));
            } else {
                outputs.insert(high, Input::High(src));
            }
        } else {
            panic!();
        }
    }

    (graph, outputs)
}

#[test]
fn part1() {
    let (graph, _) = input();
    let mut answer = 0;
    for (idx, bot) in graph.iter() {
        if bot.resolve(&graph) == [17, 61] {
            answer = *idx;
            break;
        }
    }

    assert_eq!(answer, 161);
}

#[test]
fn part2() {
    let (graph, outputs) = input();

    let answer = outputs[&0].resolve(&graph) * outputs[&1].resolve(&graph) * outputs[&2].resolve(&graph);
    assert_eq!(answer, 133163);
}