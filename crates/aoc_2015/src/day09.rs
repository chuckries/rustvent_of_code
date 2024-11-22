use aoc_common::{file_lines, full_permutations, IdMap, IteratorExt};

fn input() -> Vec<Vec<i32>> {
    let lines = file_lines("inputs/day09.txt").to_vec();

    let mut edges: Vec<(usize, usize, i32)> = Vec::new();
    let mut nodes = IdMap::new();

    for line in lines {
        let split = line.split(' ').to_vec();
        edges.push((nodes.get_or_insert(split[0]), nodes.get_or_insert(split[2]), split[4].parse().unwrap()));
    }

    let mut graph: Vec<Vec<i32>> = vec![vec![-1; nodes.len()]; nodes.len()];

    for edge in edges {
        graph[edge.0][edge.1] = edge.2;
        graph[edge.1][edge.0] = edge.2;
    }

    graph
}

fn run(min: bool) -> i32 {
    let graph = input();

    let nums = (0..graph.len()).to_vec();
    let full_permuatations = full_permutations(&nums);
    let totals = full_permuatations.iter().map(|perm| {
        perm.windows(2).map(|w| graph[*w[0]][*w[1]]).sum::<i32>()
    });

    let extreme = if min {
        totals.min()
    } else {
        totals.max()
    };

    extreme.unwrap()
}

#[test]
fn part1() {
    let answer = run(true);
    assert_eq!(answer, 251);
}

#[test]
fn part2() {
    let answer = run(false);
    assert_eq!(answer, 898);
}