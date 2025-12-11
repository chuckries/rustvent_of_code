use aoc_common::{IdMap, IteratorExt, file_lines};

fn input() -> (Vec<Vec<usize>>, IdMap) {
    let mut id_map: IdMap = IdMap::new();
    let mut adj_list: Vec<Vec<usize>> = Vec::new();

    let mut adj_list_push = |src: usize, dst: usize| {
        while adj_list.len() <= src {
            adj_list.push(vec![]);
        }
        adj_list[src].push(dst);
    };

    for line in file_lines("inputs/day11.txt").to_vec() {
        let mut split = line.split(' ');
        let src = split.next().unwrap().trim_end_matches(':');
        let src_id = id_map.get_or_insert(src);

        for dst in split {
            let dst_id = id_map.get_or_insert(dst);
            adj_list_push(src_id, dst_id);
        }
    }

    (adj_list, id_map)
}

#[test]
fn part1() {
    let (adj_list, id_map) = input();

    fn dfs(graph: &Vec<Vec<usize>>, node: usize, target: usize, count: &mut usize) {
        if node == target {
            *count += 1;
        } else {
            for adj in graph[node].iter() {
                dfs(graph, *adj, target, count);
            }
        }
    }

    let you = id_map.get("you").unwrap();
    let out = id_map.get("out").unwrap();
    let mut count = 0;
    dfs(&adj_list, you, out, &mut count);

    assert_eq!(566, count);
}

type Cache = Vec<Vec<Vec<Option<usize>>>>;

#[test]
fn part2() {
    let (adj_list, id_map) = input();

    fn dfs(graph: &Vec<Vec<usize>>, node: usize, target: usize, dac: usize, fft: usize, has_dac: bool, has_fft: bool, cache: &mut Cache, count: &mut usize) {
        if node == target {
            if has_dac && has_fft {
                *count += 1;
            }

            return;
        }

        let dac_state = if has_dac { 1 } else { 0 };
        let fft_state = if has_fft { 1 } else { 0 };
        if let Some(cached) = cache[dac_state][fft_state][node] {
            *count += cached;
            return;
        }

        let mut local = 0;
        for adj in graph[node].iter() {
            let is_dac = node == dac;
            let is_fft = node == fft;

            dfs(graph, *adj, target, dac, fft, has_dac || is_dac, has_fft || is_fft, cache, &mut local);
        }
        cache[dac_state][fft_state][node] = Some(local);
        *count += local;
    }

    let svr = id_map.get("svr").unwrap();
    let out = id_map.get("out").unwrap();
    let dac = id_map.get("dac").unwrap();
    let fft = id_map.get("fft").unwrap();
    let mut cache = vec![vec![vec![None; id_map.len()]; 2]; 2];
    let mut count = 0;
    dfs(&adj_list, svr, out, dac, fft, false, false, &mut cache, &mut count);

    assert_eq!(331837854931968, count);
}