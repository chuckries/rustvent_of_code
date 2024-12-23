use std::collections::HashMap;
use rand::Rng;

use aoc_common::{file_lines, IdMap, IteratorExt};

fn input() -> (usize, Vec<(usize, usize)>) {
    let mut id_map = IdMap::new();
    let mut tranformed: Vec<(usize, Vec<usize>)> = Vec::new();
    for l in file_lines("inputs/day25.txt") {
        let mut split = l.split(": ");
        let src = id_map.get_or_insert(split.next().unwrap());
        let dsts = split.next().unwrap().split(' ').map(|dst| id_map.get_or_insert(dst)).to_vec();
        tranformed.push((src, dsts));
    }

    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (src, dsts) in tranformed {
        for dst in dsts {
            let mut a = src;
            let mut b = dst;
            if b < a {
                (a, b) = (b, a);
            }
            edges.push((a, b));
        }
    }

    (id_map.len(), edges)
}

#[test]
fn part1() {
    let (vertex_count, edges) = input();
    let mut rnd = rand::thread_rng();
    let answer: usize;
    loop {
        let mut edges = edges.clone();
        let mut vertices: HashMap<usize, usize> = (0..vertex_count).into_iter().map(|idx| (idx, 1)).collect();

        while vertices.len() > 2 {
            let edge_idx = rnd.gen_range(0..edges.len());

            let (a, b) = edges[edge_idx];

            let b_size = vertices.remove(&b).unwrap();
            *vertices.get_mut(&a).unwrap() += b_size;
    
            edges = edges.into_iter().filter_map(|mut edge| {
                if edge.0 == a && edge.1 == b {
                    None
                } else {
                    if edge.0 == b {
                        if a > edge.1 {
                            edge.0 = edge.1;
                            edge.1 = a;
                        } else {
                            edge.0 = a;
                        }
                    } else if edge.1 == b {
                        if a < edge.0 {
                            edge.1 = edge.0;
                            edge.0 = a;
                        } else {
                            edge.1 = a;
                        }
                    }

                    Some(edge)
                }

            }).to_vec();
        }

        if edges.len() == 3 {
            answer = vertices.values().copied().product();
            break;
        }
    }

    assert_eq!(555702, answer);
}