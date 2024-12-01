use std::collections::VecDeque;

use aoc_common::{file_lines, IteratorExt};

fn input() -> Vec<Vec<usize>> {
    let lines = file_lines("inputs/day12.txt").to_vec();
    let mut graph = vec![vec![]; lines.len()];

    for l in lines {
        let mut split = l.split(" <-> ");
        let source: usize = split.next().unwrap().parse().unwrap();
        let sinks: Vec<usize> = split.next().unwrap().split(", ").map(|s| s.parse().unwrap()).to_vec();

        graph[source].extend_from_slice(&sinks);
        for sink in sinks {
            graph[sink].push(source);
        }
    }

    graph
}

fn explore_group(start: usize, graph: &[Vec<usize>], visited: &mut[bool]) {
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if visited[current] {
            continue;
        }

        visited[current] = true;

        for adj in graph[current].iter() {
            if !visited[*adj] {
                queue.push_back(*adj);
            }
        }
    }
}

#[test]
fn part1() {
    let graph = input();
    let mut visited = vec![false; graph.len()];
    explore_group(0, &graph, &mut visited);

    let answer = visited.into_iter().filter(|v| *v).count();
    assert_eq!(answer, 380);
}

#[test]
fn part2() {
    let graph = input();
    let mut visited = vec![false; graph.len()];

    let mut count = 0;
    for i in 0..graph.len() {
        if !visited[i] {
            count += 1;
            explore_group(i, &graph, &mut visited);
        }
    }

    assert_eq!(count, 181);
}