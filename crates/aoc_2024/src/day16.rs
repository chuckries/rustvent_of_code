use std::{collections::{HashMap, HashSet, VecDeque}, i32};

use aoc_common::{file_lines, Grid, PriorityQueue, Vec2i32};

fn input() -> (Grid<u8>, Vec2i32, Vec2i32) {
    let mut start = Vec2i32::default();
    let mut end = Vec2i32::default();
    let mut map: Vec<Vec<u8>> = Vec::new();
    for (j, line) in file_lines("inputs/day16.txt").enumerate() {
        for (i, c) in line.bytes().enumerate() {
            if c == b'S' {
                start = (i as i32, j as i32).into();
            } else if c == b'E' {
                end = (i as i32, j as i32).into();
            }
        }
        map.push(line.into_bytes());
    }
    (Grid::new(map), start, end)
}

const DIRS: [Vec2i32; 4] = [
    Vec2i32::new(1, 0),
    Vec2i32::new(0, 1),
    Vec2i32::new(-1, 0),
    Vec2i32::new(0, -1),
];


const fn is_opposite(dir_0: usize, dir_1: usize) -> bool {
    match (dir_0, dir_1) {
        (0, 2) | (2, 0) | (1, 3) | (3, 1) => true,
        _ => false,
    }
}

type Graph = Vec<[Option<(usize, usize, i32, i32)>; 4]>;

fn reduce_map() -> Graph {
    let (map, start, end) = input();

    let mut intersections: Vec<Vec2i32> = vec![start, end];
    for (p, c) in map.enumerate() {
        if *c == b'.' {
            if p.adjacent().filter(|adj| map[*adj] == b'.').count() >= 3 {
                intersections.push(p.cast());
            }
        }
    }

    let lookup: HashMap<Vec2i32, usize> = intersections.iter().enumerate().map(|(idx, p)| (*p, idx)).collect();
    let mut graph: Graph = vec![[None; 4]; intersections.len()];

    for i in 0..intersections.len() {
        for (adj_dir_idx, adj_dir) in DIRS.iter().cloned().enumerate() {
            let mut pos = intersections[i];
            let mut dir = adj_dir;
            let mut dir_idx = adj_dir_idx;

            pos += dir;
            if map[pos] == b'#' {
                continue;
            }

            let mut count = 1;
            let mut tile_count = 1;

            loop {
                let next = pos + dir;
                if let Some(sink) = lookup.get(&next) {
                    count += 1;
                    tile_count += 1;
                    graph[i][adj_dir_idx] = Some((*sink, dir_idx, count, tile_count));
                    break;
                }

                if map[next] == b'#' {
                    let left_dir_idx = if dir_idx == 0 { 
                        3
                    } else {
                        dir_idx - 1
                    };
                    let left_dir = DIRS[left_dir_idx];
                    let left = pos + left_dir;
                    if map[left] != b'#' {
                        dir_idx = left_dir_idx;
                        dir = left_dir;
                        count += 1000;
                        continue;
                    }

                    let right_dir_idx = (dir_idx + 1) % 4;
                    let right_dir = DIRS[right_dir_idx];
                    let right = pos + right_dir;
                    if map[right] != b'#' {
                        dir_idx = right_dir_idx;
                        dir = right_dir;
                        count += 1000;
                        continue;
                    }

                    break;
                } else {
                    count += 1;
                    tile_count += 1;
                    pos = next;
                }
            }
        }
    }

    graph
}

fn get_min(graph: &Graph, start_idx: usize, start_dir_idx: usize, end_idx: usize) -> i32 {
    let mut visited = vec![false; graph.len()];
    let mut queue: PriorityQueue<(usize, usize, i32), i32> = PriorityQueue::new();
    queue.enqueue((start_idx, start_dir_idx, 0), 0);

    while let Some((current_idx, current_dir_idx, current_len)) = queue.dequeue() {
        if current_idx == end_idx {
            return current_len;
        }

        if visited[current_idx] {
            continue;
        }
        visited[current_idx] = true;

        for src_dir_idx in 0..4 {
            if is_opposite(current_dir_idx, src_dir_idx) {
                continue;
            }

            if let Some((adj_idx, adj_dir_idx, mut adj_len, _)) = graph[current_idx][src_dir_idx] {
                if visited[adj_idx] {
                    continue;
                }

                if src_dir_idx != current_dir_idx {
                    adj_len += 1000;
                }

                queue.enqueue((adj_idx, adj_dir_idx, current_len + adj_len), current_len + adj_len);
            }
        }
    }

    panic!();
}

#[derive(Default, Clone)]
struct NodeData
{
    dist: i32,
    previous: Vec<(usize, usize, usize)>,
    visisted: bool,
}

fn get_min2(graph: &Graph, start_idx: usize, start_dir_idx: usize, end_idx: usize) -> i32 {
    let mut node_data = vec![vec![NodeData::default(); 4]; graph.len()];
    for node in node_data.iter_mut().flatten() {
        node.dist = i32::MAX;
    }
    node_data[start_idx][start_dir_idx].dist = 0;

    let mut queue: PriorityQueue<(usize, usize, i32), i32> = PriorityQueue::new();
    queue.enqueue((start_idx, start_dir_idx, 0), 0);

    let mut min = i32::MAX;
    let mut end_dirs: Vec<usize> = Vec::new();
    while let Some((current_idx, current_dir_idx, current_len)) = queue.dequeue() {
         if current_len > min {
            break;
        }
        
        if current_idx == end_idx {
            end_dirs.push(current_dir_idx);
            min = current_len;
        }

        if node_data[current_idx][current_dir_idx].visisted {
            continue;
        }
        node_data[current_idx][current_dir_idx].visisted = true;

        for src_dir_idx in 0..4 {
            if is_opposite(current_dir_idx, src_dir_idx) {
                continue;
            }

            if let Some((adj_idx, adj_dir_idx, mut adj_len, _)) = graph[current_idx][src_dir_idx] {
                if src_dir_idx != current_dir_idx {
                    adj_len += 1000;
                }

                let new_len = current_len + adj_len;
                if new_len <= node_data[adj_idx][adj_dir_idx].dist {
                    if new_len < node_data[adj_idx][adj_dir_idx].dist {
                        node_data[adj_idx][adj_dir_idx].previous.clear();
                        node_data[adj_idx][adj_dir_idx].dist = new_len;
                        queue.enqueue((adj_idx, adj_dir_idx, new_len), new_len);
                    }
                    node_data[adj_idx][adj_dir_idx].previous.push((current_idx, current_dir_idx, src_dir_idx));
                }
            }
        }
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited = vec![vec![false; 4]; node_data.len()];
    for dir in end_dirs {
        queue.push_back((1, dir));
    }
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    while let Some((node_idx, dir_idx)) = queue.pop_front() {
        if visited[node_idx][dir_idx] {
            continue;
        }
        visited[node_idx][dir_idx] = true;

        for (prev_idx, prev_dir, taken_dir) in node_data[node_idx][dir_idx].previous.iter() {
            edges.insert((*prev_idx, *taken_dir));
            queue.push_back((*prev_idx, *prev_dir));
        }
    }

    let mut sunked_nodes = vec![false; node_data.len()];
    let mut total = 0;
    for (src, dir) in edges {
        let (adj_idx, _, _, mut adj_len) = graph[src][dir].unwrap();
        if sunked_nodes[adj_idx] {
            adj_len -= 1;
        }
        sunked_nodes[adj_idx] = true;
        total += adj_len;
    }

    total + 1
}

#[test]
fn part1() {
    let graph = reduce_map();
    let answer = get_min(&graph, 0, 0, 1);
    assert_eq!(answer, 106512);
}

#[test]
fn part2() {
    let graph = reduce_map();
    let answer = get_min2(&graph, 0, 0, 1);
    assert_eq!(answer, 563);
}