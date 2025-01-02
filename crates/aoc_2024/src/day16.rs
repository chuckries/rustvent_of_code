use std::collections::HashMap;

use aoc_common::{file_lines, PriorityQueue, Vec2i32};

fn input() -> (Vec<Vec<u8>>, Vec2i32, Vec2i32) {
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
    (map, start, end)
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

type Graph = Vec<[Option<(usize, usize, i32)>; 4]>;

fn reduce_map() -> Graph {
    let (map, start, end) = input();

    let mut intersections: Vec<Vec2i32> = vec![start, end];
    for j in 1..map.len() - 1 {
        for i in 1..map[0].len() - 1 {
            if map[j][i] == b'.' {
                if Vec2i32::new(i as i32, j as i32).adjacent().filter(|adj| map[adj.y as usize][adj.x as usize] == b'.').count() >= 3 {
                    intersections.push((i as i32, j as i32).into());
                }
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
            if map[pos.y as usize][pos.x as usize] == b'#' {
                continue;
            }

            let mut count = 1;

            loop {
                let next = pos + dir;
                if let Some(sink) = lookup.get(&next) {
                    count += 1;
                    graph[i][adj_dir_idx] = Some((*sink, dir_idx, count));
                    break;
                }

                if map[next.y as usize][next.x as usize] == b'#' {
                    let left_dir_idx = if dir_idx == 0 { 
                        3
                    } else {
                        dir_idx - 1
                    };
                    let left_dir = DIRS[left_dir_idx];
                    let left = pos + left_dir;
                    if map[left.y as usize][left.x as usize] != b'#' {
                        dir_idx = left_dir_idx;
                        dir = left_dir;
                        count += 1000;
                        continue;
                    }

                    let right_dir_idx = (dir_idx + 1) % 4;
                    let right_dir = DIRS[right_dir_idx];
                    let right = pos + right_dir;
                    if map[right.y as usize][right.x as usize] != b'#' {
                        dir_idx = right_dir_idx;
                        dir = right_dir;
                        count += 1000;
                        continue;
                    }

                    break;
                } else {
                    count += 1;
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

            if let Some((adj_idx, adj_dir_idx, mut adj_len)) = graph[current_idx][src_dir_idx] {
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

// fn get_mulit_min(graph: &Graph, start_idx: usize, start_dir_idx: usize, end_idx: usize) -> i32 {
//     let mut queue: PriorityQueue<(usize, usize, i32, Vec<bool>), i32> = PriorityQueue::new();
//     queue.enqueue((start_idx, start_dir_idx, 0, vec![false; graph.len()]), 0);

//     let mut min = i32::MAX;
//     while let Some((current_idx, current_dir_idx, current_len, mut visited)) = queue.dequeue() {
//         if current_idx == end_idx {
//             min = current_len;
//             continue;
//         }

//         if current_len > min {
//             break;
//         }

//         if visited[current_idx] {
//             continue;
//         }
//         visited[current_idx] = true;

//         for adj_dir_idx in 0..4 {
//             if is_opposite(current_dir_idx, adj_dir_idx) {
//                 continue;
//             }

//             if let Some((adj_idx, mut adj_len)) = graph[current_idx][adj_dir_idx] {
//                 if visited[adj_idx] {
//                     continue;
//                 }

//                 if adj_dir_idx != current_dir_idx {
//                     adj_len += 1000;
//                 }

//                 queue.enqueue((adj_idx, adj_dir_idx, current_len + adj_len, visited.clone()), current_len + adj_len);
//             }
//         }
//     }

//     panic!();
// }

#[test]
fn part1() {
    let graph = reduce_map();
    let answer = get_min(&graph, 0, 0, 1);
    assert_eq!(answer, 106512);
}

// #[test]
// fn scratch() {
//     let graph = reduce_map();
//     let answer = get_mulit_min(&graph, 0, 1, 1);
// }

// #[test]
// fn part2() {
//     let graph = reduce_map();

//     let mut queue: PriorityQueue<(usize, i32, usize), i32> = PriorityQueue::new();
//     queue.enqueue((0, 0, 1), 0);

//     fn dfs(graph: &Graph, current_idx: usize, current_dir_idx: usize, len: i32, visited: &mut [bool]) {
//         if len > 106512 {
//             return;
//         }
        
//         if current_idx == 1 {
//             println!("found end: {}", len);
//             return;
//         }

//         visited[current_idx] = true;
//         for adj_dir_idx in 0..4 {
//             // don't go backwards
//             if is_opposite(adj_dir_idx, current_dir_idx) {
//                 continue;
//             }

//             if let Some((adj_idx, mut adj_len)) = graph[current_idx][adj_dir_idx] {
//                 if !visited[adj_idx] {

//                     if adj_dir_idx != current_dir_idx {
//                         adj_len += 1000;
//                     }

//                     let new_len = len + adj_len;

//                     dfs(graph, adj_idx, adj_dir_idx, new_len, visited);

//                     visited[adj_idx] = false;
//                 }
//             }
//         }
//         visited[current_idx] = false;
//     }

//     dfs(&graph, 0, 1, 0, &mut vec![false; graph.len()]);

//     //assert_eq!(answer, 106512);
// }