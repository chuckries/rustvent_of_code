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
    queue.push_back((start, 0));

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

type TspCache = HashMap<(usize, i64, i64), i64>;

fn recurse_tsp_1(current: usize, steps: i64, graph: &Graph, min_graph: &MinGraph, visited: i64, full_key: i64, cache: &mut TspCache) -> i64 {
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
            let new_steps = steps - (dist + 1);
            let delta = new_steps * graph[adj].rate;

            let pressure = delta + recurse_tsp_1(adj, new_steps, graph, min_graph, visited | (1 << adj), full_key, cache);
            if pressure > max {
                max = pressure;
            }
        }

        cache.insert(key, max);
        max
    }
}

fn dynamic_top_down(start: usize, steps: i64, graph: &Graph, min_graph: &MinGraph) -> HashMap<(usize, i64, i64), i64> {
    let mut queue: VecDeque<(usize, i64, i64, i64)> = VecDeque::new();
    let mut maxs: HashMap<(usize, i64, i64), i64> = HashMap::new();

    queue.push_back((start, steps, 0, 0));

    while let Some((current, steps, opened, total)) = queue.pop_front() {
        let current_max = maxs.entry((current, steps, opened)).or_insert(i64::MIN);
        if total > *current_max {
            *current_max = total;
            queue.push_back((current, steps, opened, total));
        }

        for (adj, dist) in valid_adj(current, graph, min_graph, opened, steps) {
            let next_steps = steps - (dist + 1);
            let delta = next_steps * graph[adj].rate;
            queue.push_back((adj, next_steps, opened | (1 << adj), total + delta));
        }
    }

    maxs
}

type TspCache2 = HashMap<(usize, i64, usize, i64, i64), i64>;

fn recurse_tsp_2(current_0: usize, steps_0: i64, current_1: usize, steps_1: i64, graph: &Graph, min_graph: &MinGraph, visited: i64, full_key: i64, cache: &mut TspCache2) -> i64 {
    let key = (current_0, steps_0, current_1, steps_1, visited);

    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let valid_adj_0 = valid_adj(current_0, graph, min_graph, visited, steps_0);
    let valid_adj_1 = valid_adj(current_1, graph, min_graph, visited, steps_1);

    if visited == full_key || (valid_adj_0.len() == 0 && valid_adj_1.len() == 0) {
        cache.insert(key, 0);
        0
    } else if valid_adj_1.len() == 0 {
        let pressure = recurse_tsp_1(current_0, steps_0, graph, min_graph, visited, full_key, &mut TspCache::new());
        cache.insert(key, pressure);
        pressure
    } else if valid_adj_0.len() == 0 {
        let pressure = recurse_tsp_1(current_1, steps_1, graph, min_graph, visited, full_key, &mut TspCache::new());
        cache.insert(key, pressure);
        pressure
    } else {

        let mut max = 0;

        for (adj_0, dist_0) in valid_adj_0.iter() {
            for (adj_1, dist_1) in valid_adj_1.iter() {
                let new_steps_0 = steps_0 - (*dist_0 + 1);
                let new_steps_1 = steps_1 - (*dist_1 + 1);
                let delta_0 = new_steps_0 * graph[*adj_0].rate;
                let delta_1 = new_steps_1 * graph[*adj_1].rate;

                if adj_0 == adj_1 {
                    let pressure = delta_0 + recurse_tsp_2(*adj_0, new_steps_0, current_1, steps_1, graph, min_graph, visited | (1 << adj_0), full_key, cache);
                    if pressure > max {
                        max = pressure;
                    }

                    let pressure = delta_1 + recurse_tsp_2(current_0, steps_0, *adj_1, new_steps_1, graph, min_graph, visited | (1 << adj_1), full_key, cache);
                    if pressure > max {
                        max = pressure;
                    }
                } else {
                    let mut visited = visited;
                    visited |= 1 << adj_0;
                    visited |= 1 << adj_1;
                    let pressure = delta_0 + delta_1 + recurse_tsp_2(*adj_0, new_steps_0, *adj_1, new_steps_1, graph, min_graph, visited, full_key, cache);
                    if pressure > max {
                        max = pressure;
                    }
                }
            }
        }

        cache.insert(key, max);
        max
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

    let mut cache = TspCache::new();
    let max = recurse_tsp_1(start, 30, &graph, &min_graph, 1 << start, full_key, &mut cache);

    assert_eq!(max, 1796);
}

#[test]
fn part1_top_down() {
    let (graph, start) = input();
    let mut min_graph = MinGraph::new();

    min_graph.resize(graph.len(), HashMap::new());

    for i in 0..graph.len() {
        if graph[i].rate != 0 || i == start {
            bfs(i, &graph, &mut min_graph);
        }
    }
    let maxs = dynamic_top_down(start, 30, &graph, &min_graph);
    let max = *maxs.values().max().unwrap();
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

    let mut cache = TspCache2::new();
    let max = recurse_tsp_2(start, 26, start, 26, &graph, &min_graph, 1 << start, full_key, &mut cache);

    assert_eq!(max, 1999);
}

#[test]
fn part2_top_down() {
    let (graph, start) = input();
    let mut min_graph = MinGraph::new();

    min_graph.resize(graph.len(), HashMap::new());

    for i in 0..graph.len() {
        if graph[i].rate != 0 || i == start {
            bfs(i, &graph, &mut min_graph);
        }
    }
    let maxs = dynamic_top_down(start, 26, &graph, &min_graph);

    let mut max = 0;
    for i in maxs.iter() {
        for j in maxs.iter() {
            if i.0.2 & j.0.2 == 0 && i.1 + j.1 > max {
                max = i.1 + j.1;
            }
        }
    }
    assert_eq!(max, 1999);
}