use std::collections::HashMap;

use aoc_common::{file_lines, IdMap, IteratorExt};

#[derive(Clone, Default)]
struct Disc {
    name: String,
    weight: i32,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

fn input() -> Vec<Disc> {
    let mut id_map = IdMap::new();

    let lines = file_lines("inputs/day07.txt").to_vec();
    let mut graph = vec![Disc::default(); lines.len()];

    for l in lines {
        let mut split = l.split(" -> ");

        let mut lhs = split.next().unwrap().split(' ');
        let name = lhs.next().unwrap().to_string();
        let weight = lhs.next().unwrap().trim_start_matches('(').trim_end_matches(')').parse::<i32>().unwrap();

        let idx = id_map.get_or_insert(&name);
        graph[idx].name = name;
        graph[idx].weight = weight;

        if let Some(rhs) = split.next() {
            for s in rhs.split(", ") {
                let adj = id_map.get_or_insert(s);
                graph[idx].supports.push(adj);
                graph[adj].supported_by.push(idx);
            }
        }
    }

    graph
}

#[test]
fn part1() {
    let graph = input();
    let answer = graph.iter().find(|d| d.supported_by.len() == 0).map(|d| &d.name).unwrap();
    assert_eq!(answer, "gynfwly");
}

#[test]
fn part2() {
    let graph = input();
    let root = graph.iter().position(|d| d.supported_by.len() == 0).unwrap();
    let mut weights = vec![0; graph.len()];

    fn calc_weight(idx: usize, graph: &[Disc], weights: &mut[i32]) -> i32 {
        let d = &graph[idx];
        let weight = d.weight + d.supports.iter().map(|idx| calc_weight(*idx, graph, weights)).sum::<i32>();
        weights[idx] = weight;
        weight
    }

    calc_weight(root, &graph, &mut weights);

    fn find_wrong_weight(idx: usize, graph: &[Disc], weights: &[i32], target_weight: i32) -> i32 {
        if graph[idx].supports.len() == 0 {
            return target_weight;
        }

        let mut child_weights: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in graph[idx].supports.iter() {
            child_weights.entry(weights[*i]).or_default().push(*i);
        }

        if child_weights.len() == 1 {
            return target_weight - child_weights.keys().next().unwrap() * graph[idx].supports.len() as i32;
        } else {
            let incorrect_tower_idx = child_weights.values().filter(|v| v.len() == 1).next().unwrap()[0];
            let target_weight = *child_weights.iter().filter(|(_, value)| value.len() > 1).next().unwrap().0;

            return find_wrong_weight(incorrect_tower_idx, graph, weights, target_weight);
        }
    }
    
    let answer = find_wrong_weight(root, &graph, &weights, 0);
    assert_eq!(answer, 1526);

}