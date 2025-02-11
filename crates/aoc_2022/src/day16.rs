use std::{collections::{HashMap, HashSet, VecDeque}};
use aoc_common::{file_lines, IteratorExt};

struct Node {
    rate: i64,
    adj: Vec<usize>,
}
type Graph = Vec<Node>;
type MinGraph = Vec<HashMap<usize, i64>>;

fn input() -> (Graph, usize) {
    let mut graph = Graph::new();

    let mut ids: HashMap<String, usize> = HashMap::new();

    for line in file_lines("inputs/day16.txt") {
        let split = line.split([' ', '=', ';']).to_vec();

        let name = split[1].to_string();
        let rate = split[5].parse().unwrap();
        let adj = split[11..].iter().map(|s| s.trim_end_matches(',').to_string()).to_vec();

        let id = if let Some(id) = ids.get(&name) {
            *id
        } else {
            let id = ids.len();
            ids.insert(name, id);
            id
        };

        while graph.len() <= id {
            graph.push(Node { rate: 0, adj: Vec::new()});
        }
        graph[id].rate = rate;

        for adj in adj {
            let adj = if let Some(id) = ids.get(&adj) {
                *id
            } else {
                let id = ids.len();
                ids.insert(adj, id);
                id
            };

            graph[id].adj.push(adj);
        }
    }

    (graph, ids["AA"])
}

fn bfs(start: usize, graph: &Graph, min_graph: &mut MinGraph) {
    let mut queue: VecDeque<(usize, i64)> = VecDeque::new();
    queue.push_back((start, 1));

    let mut visisted: HashSet<usize> = HashSet::new();
    visisted.insert(start);

    while let Some((current, dist)) = queue.pop_front() {
        let n = &graph[current];

        if current != start && n.rate != 0 {
            *min_graph[start].entry(current).or_default() = dist;
        }

        for adj in n.adj.iter() {
            if !visisted.contains(adj) {
                visisted.insert(*adj);
                queue.push_back((*adj, dist + 1));
            }
        }
    }
}

fn valid_adj(current: usize, graph: &Graph, min_graph: &MinGraph, visited: i64, time_remaining: i64) -> Vec<(usize, i64)> {
    min_graph[current].iter().filter(|(adj, dist)| graph[**adj].rate != 0 && (visited & (1 << **adj)) == 0 && *dist + 1 < time_remaining).map(|(a, b)| (*a, *b)).to_vec()
}

type Cache = HashMap<(usize, i64, i64), i64>;

fn recurse_max(current: usize, steps: i64, graph: &Graph, min_graph: &MinGraph, visited: i64, full_key: i64, cache: &mut Cache) -> i64 {
    let key = (current, steps, visited);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let valid_adj = valid_adj(current, graph, min_graph, visited, steps);

    if visited == full_key || valid_adj.len() == 0 {
        cache.insert(key, 0);
        0
    } else {
        let mut max = 0;
        for (adj, dist) in valid_adj {
            let new_steps = steps - dist;
            let delta = new_steps * graph[adj].rate;

            let pressure = delta + recurse_max(adj, new_steps, graph, min_graph, visited | (1 << adj), full_key, cache);
            if pressure > max {
                max = pressure;
            }
        }

        cache.insert(key, max);
        max
    }
}

fn recurse_states(current: usize, steps: i64, total: i64, graph: &Graph, min_graph: &MinGraph, visited: i64, full_key: i64, best: &mut Cache) {
    let key = (current, steps, visited);

    let valid_adj = valid_adj(current, graph, min_graph, visited, steps);

    if visited == full_key || valid_adj.len() == 0 {
        let max = best.entry(key).or_default();
        *max = (*max).max(total);
    } else {
        for (adj, dist) in valid_adj {
            let new_steps = steps - dist;
            let delta = new_steps * graph[adj].rate;

            recurse_states(adj, new_steps, total + delta, graph, min_graph, visited | (1 << adj), full_key, best);
        }
    }
}

#[test]
fn part1() {
    let (graph, start) = input();
    let mut min_graph = MinGraph::new();

    min_graph.resize(graph.len(), HashMap::new());

    for i in 0..graph.len() {
        if graph[i].rate != 0 || i == start {
            bfs(i, &graph, &mut min_graph);
        }
    }

    let mut full_key = 1 << start;
    for i in 0..graph.len() {
        if graph[i].rate != 0 {
            full_key |= 1 << i;
        }
    }

    let mut cache = Cache::new();
    let max = recurse_max(start, 30, &graph, &min_graph, 0, full_key, &mut cache);

    assert_eq!(max, 1796);
}

#[test]
fn part2() {
    let (graph, start) = input();
    let mut min_graph = MinGraph::new();

    min_graph.resize(graph.len(), HashMap::new());

    for i in 0..graph.len() {
        if graph[i].rate != 0 || i == start {
            bfs(i, &graph, &mut min_graph);
        }
    }

    let mut full_key = 1 << start;
    for i in 0..graph.len() {
        if graph[i].rate != 0 {
            full_key |= 1 << i;
        }
    }

    let mut cache = Cache::new();
    recurse_states(start, 26, 0, &graph, &min_graph, 0, full_key, &mut cache);

    let mut max = 0;
    for i in cache.iter() {
        for j in cache.iter() {
            if i.0.2 & j.0.2 == 0 {
                max = max.max(i.1 + j.1);
            }
        }
    }

    assert_eq!(max, 1999);
}