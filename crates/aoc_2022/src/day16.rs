use std::{collections::{HashMap, HashSet, VecDeque}, cmp::Ordering};
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

fn input2() -> (usize, Vec<Vec<usize>>, Vec<Vec<i32>>, Vec<i32>) {
    let lines = file_lines("inputs/day16.txt").to_vec();

    // sort the input nodes such that non-zer0 rates are first and AA is the first 0 node
    let nodes: Vec<(&str, i32, Vec<&str>)> = lines.iter().map(|l| {
        let split = l.split([' ', '=', ';']).to_vec();

        let name = split[1];
        let rate = split[5].parse::<i32>().unwrap();
        let adj = split[11..].iter().map(|s| s.trim_end_matches(',')).to_vec();

        (name, rate, adj)
    }).sorted_by(|lhs, rhs| {
        let mut cmp = rhs.1.cmp(&lhs.1);
        if cmp == Ordering::Equal {
            cmp = lhs.0.cmp(rhs.0);
        }
        cmp
    }).to_vec();

    // collect rates for non-zero and start
    let rates: Vec<i32> = nodes.iter().filter_map(|n| if n.0 == "AA" || n.1 > 0 { Some(n.1) } else { None }).to_vec();
    let start: usize = rates.len() - 1;

    // create a map of node name to it's id
    let node_ids: HashMap<&str, usize> = nodes.iter().enumerate().map(|(idx, node)| (node.0, idx)).collect();

    // bfs the start and non-zero nodes to other non-zero nodes. Do not connect any node back to start
    let mut adjacencies: Vec<Vec<usize>> = vec![Vec::new(); rates.len()];
    let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX; rates.len()]; rates.len()];

    for i in 0..=start {
        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();
        queue.push_back((i, 0));
        let mut visited = vec![false; nodes.len()];
        visited[i] = true;

        while let Some((current, dist)) = queue.pop_front() {
            if current != i && current != start && current < rates.len() {
                distances[i][current] = dist;
                adjacencies[i].push(current);
            }

            for adj in nodes[current].2.iter().map(|s| node_ids[*s]) {
                if !visited[adj] {
                    visited[adj] = true;
                    queue.push_back((adj, dist + 1));
                }
            }
        }
    }

    (start, adjacencies, distances, rates)
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
    // let key = (current, steps, visited);
    // if let Some(cached) = cache.get(&key) {
    //     return *cached;
    // }

    let valid_adj = valid_adj(current, graph, min_graph, visited, steps);

    if visited == full_key || valid_adj.len() == 0 {
        //cache.insert(key, 0);
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

        //cache.insert(key, max);
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

fn recurse_max_diff(current: usize, steps: i32, adjacencies: &Vec<Vec<usize>>, distances: &Vec<Vec<i32>>, rates: &Vec<i32>, open_valves: i32) -> i32 {
    // let key = (current, steps, visited);
    // if let Some(cached) = cache.get(&key) {
    //     return *cached;
    // }

    //let valid_adj = valid_adj(current, graph, min_graph, visited, steps);

    let valid_adj = adjacencies[current].iter().filter_map(|adj| {
        if (open_valves & (1 << adj)) == 0 {
            let dist = distances[current][*adj];
            if dist + 1 < steps {
                return Some((*adj, dist));
            }
        }
        None
    }).to_vec();

    if valid_adj.len() == 0 {
        //cache.insert(key, 0);
        0
    } else {
        let mut max = 0;
        for (adj, dist) in valid_adj {
            let new_steps = steps - dist - 1;
            let delta = new_steps * rates[adj];

            max = max.max(delta + recurse_max_diff(adj, new_steps, adjacencies, distances, rates, open_valves | (1 << adj)));
        }

        //cache.insert(key, max);
        max
    }
}

#[test]
fn part3() {
    let (start, adjacencies, distances, rates) = input2();
    let answer = recurse_max_diff(start, 30, &adjacencies, &distances, &rates, 0);

    assert_eq!(answer, 1796);
}

fn recurse_states_diff(total: i32, current: usize, steps: i32, adjacencies: &Vec<Vec<usize>>, distances: &Vec<Vec<i32>>, rates: &Vec<i32>, open_valves: i32, end_states: &mut [i32]) {
    let valid_adj = adjacencies[current].iter().filter_map(|adj| {
        if (open_valves & (1 << adj)) == 0 {
            let dist = distances[current][*adj];
            if dist + 1 < steps {
                return Some((*adj, dist));
            }
        }
        None
    }).to_vec();

    if valid_adj.len() == 0 {
        end_states[open_valves as usize] = i32::max(end_states[open_valves as usize], total);
    } else {
        for (adj, dist) in valid_adj {
            let new_steps = steps - dist - 1;
            let delta = new_steps * rates[adj];

            recurse_states_diff(total + delta, adj, new_steps, adjacencies, distances, rates, open_valves | (1 << adj), end_states);
        }
    }
}

#[test]
fn part4() {
    let (start, adjacencies, distances, rates) = input2();

    let mut states = vec![0; (1 << (rates.len() - 1)) - 1];
    recurse_states_diff(0, start, 26, &adjacencies, &distances, &rates, 0, &mut states);

    let mut max = 0;
    // for x in 0..states.len() / 2 {
    //     let y = states.len() - x - 1;
    //     if x & y == 0 {
    //         max = max.max(states[x] + states[y]);
    //     }
    // }
    for i in 0..states.len() {
        for j in 0..states.len() {
            if i & j == 0 {
                max = max.max(states[i] + states[j]);
            }
        }
    }

    assert_eq!(max, 1999);
}