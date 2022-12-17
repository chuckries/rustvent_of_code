use std::{collections::{HashMap, HashSet, VecDeque}};
use aoc_common::{file_lines, IteratorExt};

struct Node {
    rate: i64,
    adj: Vec<String>
}

struct Node2 {
    rate: i64,
    adj: Vec<usize>,
}

type Graph = HashMap<String, Node>;
type MinGraph = HashMap<String, HashMap<String, i64>>;
type Graph2 = Vec<Node2>;
type MinGraph2 = Vec<HashMap<usize, i64>>;

fn input() -> Graph {
    let mut graph = Graph::new();

    for line in file_lines("inputs/day16.txt") {
        let split = line.split([' ', '=', ';']).to_vec();

        let name = split[1].to_string();
        let rate = split[5].parse().unwrap();
        let adj = split[11..].iter().map(|s| s.trim_end_matches(',').to_string()).to_vec();

        graph.insert(name, Node { rate, adj });
    }

    graph
}

fn input2() -> (Graph2, usize) {
    let mut graph = Graph2::new();

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
            graph.push(Node2 { rate: 0, adj: Vec::new()});
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

fn bfs(start: &str, graph: &Graph, min_graph: &mut MinGraph) {
    let mut queue: VecDeque<(&str, i64)> = VecDeque::new();
    queue.push_back((start, 0));

    let mut visisted: HashSet<String> = HashSet::new();
    visisted.insert(start.to_string());

    while let Some((current, dist)) = queue.pop_front() {
        let n = &graph[current];

        if current != start && n.rate != 0 {
            *min_graph.entry(start.to_string()).or_default().entry(current.to_string()).or_default() = dist;
        }

        for adj in n.adj.iter() {
            if !visisted.contains(adj) {
                visisted.insert(adj.to_string());
                queue.push_back((adj, dist + 1));
            }
        }
    }
}

fn bfs_idx(start: usize, graph: &Graph2, min_graph: &mut MinGraph2) {
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

fn valid_adj<'a>(current: &str, graph: &Graph, min_graph: &'a MinGraph, visited: &HashSet<String>, time_remaining: i64) -> Vec<(&'a String, &'a i64)> {
    min_graph[current].iter().filter(|(adj, dist)| graph[*adj].rate != 0 && !visited.contains(*adj) && *dist + 1 < time_remaining).to_vec()
}

fn valid_adj_idx(current: usize, graph: &Graph2, min_graph: &MinGraph2, visited: i64, time_remaining: i64) -> Vec<(usize, i64)> {
    min_graph[current].iter().filter(|(adj, dist)| graph[**adj].rate != 0 && (visited & (1 << **adj)) == 0 && *dist + 1 < time_remaining).map(|(a, b)| (*a, *b)).to_vec()
}

fn theoretical_1(graph: &Graph, mut time_remaining: i64, visited: &HashSet<String>) -> i64 {
    let sorted = graph.iter().filter_map(|(name, n)| {
        if !visited.contains(name) {
            Some(n.rate)
        } else {
            None
        }
    }).sorted_by(|a, b| b.cmp(a));

    let mut delta = 0;
    for val in sorted {
        time_remaining -= 2;
        if time_remaining <= 0 {
            break;
        }
        delta += time_remaining * val;
    }

    delta
}

fn theoretical_2(graph: &Graph, mut time_remaining_0: i64, mut time_remaining_1: i64, visited: &HashSet<String>) -> i64 {
    let sorted = graph.iter().filter_map(|(name, n)| {
        if !visited.contains(name) {
            Some(n.rate)
        } else {
            None
        }
    }).sorted_by(|a, b| b.cmp(a));

    let mut delta = 0;
    for val in sorted {
        if time_remaining_0 - 2 <= 0 || time_remaining_1 - 2 <= 0 {
            break;
        }

        let mut time = &mut time_remaining_0;
        if time_remaining_1 > *time {
            time = &mut time_remaining_1;
        }

        *time -= 2;
        delta += *time * val;
    }
    delta
}

fn recurse_1(current: &str, graph: &Graph, min_graph: &MinGraph, visited: &mut HashSet<String>, needed: usize, time_remaining: i64, total: i64, max: &mut i64) {
    let valid_adj = valid_adj(current, graph, min_graph, visited, time_remaining);

    if visited.len() == needed || valid_adj.len() == 0 {
        if total > *max {
            *max = total;
        }
    } else {
        let theoretical = theoretical_1(graph, time_remaining, visited);

        if total + theoretical > *max {
            for (adj, dist) in valid_adj {
                let steps = dist + 1;
                let new_time = time_remaining - steps;
                let delta = graph[adj].rate * new_time;
    
                visited.insert(adj.to_string());
                recurse_1(adj, graph, min_graph, visited, needed, new_time, total + delta, max);
                visited.remove(adj);
            }
        }
    }

}

type TspCache<'a> = HashMap<(String, i64, Vec<String>), i64>;
type TspCacheIdx = HashMap<(usize, i64, i64), i64>;

fn recurse_tsp_1(current: &str, steps: i64, graph: &Graph, min_graph: &MinGraph, visited: HashSet<String>, cache: &mut TspCache) -> i64 {
    let visisted_vec: Vec<String> = visited.iter().sorted().map(|s| s.to_string()).to_vec();

    let valid_adj = valid_adj(current, graph, min_graph, &visited, steps);

    if visited.len() == min_graph.len() || valid_adj.len() == 0 {
        cache.insert((current.to_string(), steps, visisted_vec), 0);
        0
    } else {

        let key = (current.to_string(), steps, visisted_vec);

        if let Some(cached) = cache.get(&key) {
            return *cached;
        }

        let mut max = 0;
        for (adj, dist) in valid_adj {
            let new_steps = steps - (dist + 1);
            let delta = new_steps * graph[adj].rate;

            let mut visited = visited.clone();
            visited.insert(adj.to_string());
            let pressure = delta + recurse_tsp_1(adj, new_steps, graph, min_graph, visited, cache);
            if pressure > max {
                max = pressure;
            }
        }

        cache.insert(key, max);
        max
    }
}

fn recurse_tsp_1_idx(current: usize, steps: i64, graph: &Graph2, min_graph: &MinGraph2, visited: i64, full_key: i64, cache: &mut TspCacheIdx) -> i64 {
    let key = (current, steps, visited);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let valid_adj = valid_adj_idx(current, graph, min_graph, visited, steps);

    if visited == full_key || valid_adj.len() == 0 {
        cache.insert(key, 0);
        0
    } else {
        let mut max = 0;
        for (adj, dist) in valid_adj {
            let new_steps = steps - (dist + 1);
            let delta = new_steps * graph[adj].rate;

            let pressure = delta + recurse_tsp_1_idx(adj, new_steps, graph, min_graph, visited | (1 << adj), full_key, cache);
            if pressure > max {
                max = pressure;
            }
        }

        cache.insert(key, max);
        max
    }
}

type TspCache2<'a> = HashMap<(String, i64, String, i64, Vec<String>), i64>;
type TspCache2Idx = HashMap<(usize, i64, usize, i64, i64), i64>;

fn recurse_tsp_2(current_0: &str, steps_0: i64, current_1: &str, steps_1: i64, graph: &Graph, min_graph: &MinGraph, visited: HashSet<String>, cache: &mut TspCache2) -> i64 {
    let visisted_vec: Vec<String> = visited.iter().sorted().map(|s| s.to_string()).to_vec();
    let key = (current_0.to_string(), steps_0, current_1.to_string(), steps_1, visisted_vec);

    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let valid_adj_0 = valid_adj(current_0, graph, min_graph, &visited, steps_0);
    let valid_adj_1 = valid_adj(current_1, graph, min_graph, &visited, steps_1);

    if visited.len() == min_graph.len() || (valid_adj_0.len() == 0 && valid_adj_1.len() == 0) {
        cache.insert(key, 0);
        0
    } else if valid_adj_1.len() == 0 {
        let pressure = recurse_tsp_1(current_0, steps_0, graph, min_graph, visited, &mut TspCache::new());
        cache.insert(key, pressure);
        pressure
    } else if valid_adj_0.len() == 0 {
        let pressure = recurse_tsp_1(current_1, steps_1, graph, min_graph, visited, &mut TspCache::new());
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
                    let mut next_visited = visited.clone();
                    next_visited.insert(adj_0.to_string());
                    let pressure = delta_0 + recurse_tsp_2(*adj_0, new_steps_0, current_1, steps_1, graph, min_graph, next_visited, cache);
                    if pressure > max {
                        max = pressure;
                    }

                    let mut next_visited = visited.clone();
                    next_visited.insert(adj_1.to_string());
                    let pressure = delta_1 + recurse_tsp_2(current_0, steps_0, *adj_1, new_steps_1, graph, min_graph, next_visited, cache);
                    if pressure > max {
                        max = pressure;
                    }
                } else {
                    let mut next_visited = visited.clone();
                    next_visited.insert(adj_0.to_string());
                    next_visited.insert(adj_1.to_string());
                    let pressure = delta_0 + delta_1 + recurse_tsp_2(*adj_0, new_steps_0, *adj_1, new_steps_1, graph, min_graph, next_visited, cache);
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

fn recurse_tsp_2_idx(current_0: usize, steps_0: i64, current_1: usize, steps_1: i64, graph: &Graph2, min_graph: &MinGraph2, visited: i64, full_key: i64, cache: &mut TspCache2Idx) -> i64 {
    let key = (current_0, steps_0, current_1, steps_1, visited);

    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let valid_adj_0 = valid_adj_idx(current_0, graph, min_graph, visited, steps_0);
    let valid_adj_1 = valid_adj_idx(current_1, graph, min_graph, visited, steps_1);

    if visited == full_key || (valid_adj_0.len() == 0 && valid_adj_1.len() == 0) {
        cache.insert(key, 0);
        0
    } else if valid_adj_1.len() == 0 {
        let pressure = recurse_tsp_1_idx(current_0, steps_0, graph, min_graph, visited, full_key, &mut TspCacheIdx::new());
        cache.insert(key, pressure);
        pressure
    } else if valid_adj_0.len() == 0 {
        let pressure = recurse_tsp_1_idx(current_1, steps_1, graph, min_graph, visited, full_key, &mut TspCacheIdx::new());
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
                    let pressure = delta_0 + recurse_tsp_2_idx(*adj_0, new_steps_0, current_1, steps_1, graph, min_graph, visited | (1 << adj_0), full_key, cache);
                    if pressure > max {
                        max = pressure;
                    }

                    let pressure = delta_1 + recurse_tsp_2_idx(current_0, steps_0, *adj_1, new_steps_1, graph, min_graph, visited | (1 << adj_1), full_key, cache);
                    if pressure > max {
                        max = pressure;
                    }
                } else {
                    let mut visited = visited;
                    visited |= 1 << adj_0;
                    visited |= 1 << adj_1;
                    let pressure = delta_0 + delta_1 + recurse_tsp_2_idx(*adj_0, new_steps_0, *adj_1, new_steps_1, graph, min_graph, visited, full_key, cache);
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
#[ignore]
fn part1() {
    let graph = input();
    let mut min_graph = MinGraph::new();

    let start = "AA";

    for (name, node) in graph.iter() {
        if node.rate != 0 || name == start {
            bfs(name, &graph, &mut min_graph);
        }
    }

    let mut max = i64::MIN;
    recurse_1(start, &graph, &min_graph, &mut [start.to_string()].into_iter().to_set(), min_graph.len(), 30, 0, &mut max);

    assert_eq!(max, 1796);
}

#[test]
#[ignore]
fn part1_tsp() {
    let graph = input();
    let mut min_graph = MinGraph::new();

    let start = "AA";

    for (name, node) in graph.iter() {
        if node.rate != 0 || name == start {
            bfs(name, &graph, &mut min_graph);
        }
    }

    let mut cache = TspCache::new();
    let mut visited = HashSet::new();
    visited.insert(start.to_string());
    let max = recurse_tsp_1(start, 30, &graph, &min_graph, visited, &mut cache);

    assert_eq!(max, 0);
}

#[test]
fn part1_tsp_idx() {
    let (graph, start) = input2();
    let mut min_graph = MinGraph2::new();

    min_graph.resize(graph.len(), HashMap::new());

    for i in 0..graph.len() {
        if graph[i].rate != 0 || i == start {
            bfs_idx(i, &graph, &mut min_graph);
        }
    }

    let mut full_key = 1 << start;
    for i in 0..graph.len() {
        if graph[i].rate != 0 {
            full_key |= 1 << i;
        }
    }

    let mut cache = TspCacheIdx::new();
    let max = recurse_tsp_1_idx(start, 30, &graph, &min_graph, 1 << start, full_key, &mut cache);

    assert_eq!(max, 1796);
}

fn recurse_2(current_0: &str, time_remaining_0: i64, current_1: &str, time_remaining_1: i64, graph: &Graph, min_graph: &MinGraph, visited: &mut HashSet<String>, needed: usize, total: i64, max: &mut i64) {
    let valid_adj_0 = valid_adj(current_0, graph, min_graph, visited, time_remaining_0);
    let valid_adj_1 = valid_adj(current_1, graph, min_graph, visited, time_remaining_1);

    if visited.len() == needed || (valid_adj_0.len() == 0 && valid_adj_1.len() == 0) {
        if total > *max {
            *max = total;
        }
    } else if valid_adj_0.len() == 0 {
        recurse_1(current_1, graph, min_graph, visited, needed, time_remaining_1, total, max)
    } else if valid_adj_1.len() == 0 {
        recurse_1(current_0, graph, min_graph, visited, needed, time_remaining_0, total, max)
    } else {
        let theoretical = theoretical_2(graph, time_remaining_0, time_remaining_1, visited);

        if total + theoretical > *max {
            for (adj_0, dist_0) in valid_adj_0.iter() {
                for (adj_1, dist_1) in valid_adj_1.iter() {
                    let steps_0 = **dist_0 + 1;
                    let steps_1 = **dist_1 + 1;
                    let new_time_0 = time_remaining_0 - steps_0;
                    let new_time_1 = time_remaining_1 - steps_1;
                    let delta_0 = graph[*adj_0].rate * new_time_0;
                    let delta_1 = graph[*adj_1].rate * new_time_1;

                    if adj_0 == adj_1 {
                        visited.insert(adj_0.to_string());
                        recurse_2(*adj_0, new_time_0, current_1, time_remaining_1, graph, min_graph, visited, needed, total + delta_0, max);
                        visited.remove(*adj_0);

                        visited.insert(adj_1.to_string());
                        recurse_2(current_0, time_remaining_0, *adj_1, new_time_1, graph, min_graph, visited, needed, total + delta_1, max);
                        visited.remove(*adj_1);
                    } else {
                        visited.insert(adj_0.to_string());
                        visited.insert(adj_1.to_string());
                        recurse_2(*adj_0, new_time_0, *adj_1, new_time_1, graph, min_graph, visited, needed, total + delta_0 + delta_1, max);
                        visited.remove(*adj_0);
                        visited.remove(*adj_1);
                    }
                }
            }
        }
    }
}

#[test]
#[ignore]
fn part2() {
    let graph = input();
    let mut min_graph = MinGraph::new();

    let start = "AA";

    for (name, node) in graph.iter() {
        if node.rate != 0 || name == start {
            bfs(name, &graph, &mut min_graph);
        }
    }

    let mut max = i64::MIN;
    recurse_2(start, 26, start, 26, &graph, &min_graph, &mut [start.to_string()].into_iter().to_set(), min_graph.len(), 0, &mut max);

    assert_eq!(max, 0);
}

#[test]
#[ignore]
fn part2_tsp() {
    let graph = input();
    let mut min_graph = MinGraph::new();

    let start = "AA";

    for (name, node) in graph.iter() {
        if node.rate != 0 || name == start {
            bfs(name, &graph, &mut min_graph);
        }
    }

    let mut cache = TspCache2::new();
    let mut visited = HashSet::new();
    visited.insert(start.to_string());
    let max = recurse_tsp_2(start, 26, start, 26, &graph, &min_graph, visited, &mut cache);

    assert_eq!(max, 0);
}

#[test]
fn part2_tsp_idx() {
    let (graph, start) = input2();
    let mut min_graph = MinGraph2::new();

    min_graph.resize(graph.len(), HashMap::new());

    for i in 0..graph.len() {
        if graph[i].rate != 0 || i == start {
            bfs_idx(i, &graph, &mut min_graph);
        }
    }

    let mut full_key = 1 << start;
    for i in 0..graph.len() {
        if graph[i].rate != 0 {
            full_key |= 1 << i;
        }
    }

    let mut cache = TspCache2Idx::new();
    let max = recurse_tsp_2_idx(start, 26, start, 26, &graph, &min_graph, 1 << start, full_key, &mut cache);

    assert_eq!(max, 1999);
}