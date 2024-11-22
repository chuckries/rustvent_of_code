use aoc_common::{file_lines, full_permutations, IdMap, IteratorExt};

type Graph = Vec<Vec<i32>>;

fn input(add_me: bool) -> Graph {
    let mut id_map = IdMap::new();
    let mut edges: Vec<(usize, usize, i32)> = Vec::new();

    for l in file_lines("inputs/day13.txt") {
        let split = l.split(' ').to_vec();

        let src = split[0];
        let sink = split[10].trim_end_matches('.');
        let mut num = split[3].parse::<i32>().unwrap();
        if split[2] == "lose" {
            num = -num;
        }

        edges.push((id_map.get_or_insert(src), id_map.get_or_insert(sink), num));
    }

    if add_me {
        let me = id_map.get_or_insert("me");

        for i in 0..me {
            edges.push((i, me, 0));
            edges.push((me, i, 0));
        }
    }

    let mut graph = vec![vec![i32::MIN; id_map.len()]; id_map.len()];

    for e in edges {
        graph[e.0][e.1] = e.2;
    }

    graph
}

fn run(add_me: bool) -> i32 {
    let graph: Graph = input(add_me);
    let ids = (0..graph.len()).to_vec();

    full_permutations(&ids).iter().map(|perm| {
        let mut total = 0;
        for i in 0..perm.len() {
            let left = if i == 0 { perm.len() - 1 } else { i - 1 };
            let right = if i == perm.len() - 1 { 0 } else { i + 1 };

            let left = *perm[left];
            let right = *perm[right];
            let center = *perm[i];

            total += graph[center][left] + graph[center][right];
        }
        total
    }).max().unwrap()
}

#[test]
fn part1() {
    let answer = run(false);
    assert_eq!(answer, 664);
}

#[test]
fn part2() {
    let answer = run(true);
    assert_eq!(answer, 640);
}