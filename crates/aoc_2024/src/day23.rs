use std::collections::HashSet;

use aoc_common::{file_lines, IdMap, IteratorExt};

fn input() -> (IdMap<String>, Vec<Vec<usize>>, Vec<Vec<bool>>) {
    let mut id_map = IdMap::new();
    let mut adj_list: Vec<Vec<usize>> = Vec::new();
    let mut adj_matrix: Vec<Vec<bool>> = Vec::new();

    for line in file_lines("inputs/day23.txt") {
        let mut split = line.split('-');

        let a = id_map.get_or_insert(split.next().unwrap());
        let b = id_map.get_or_insert(split.next().unwrap());

        if a == adj_list.len() {
            adj_list.push(Vec::new());
        }

        adj_list[a].push(b);

        if b == adj_list.len() {
            adj_list.push(Vec::new());
        }

        adj_list[b].push(a);
    }

    for _ in 0..id_map.len() {
        adj_matrix.push(vec![false; id_map.len()]);
    }

    for (a, list) in adj_list.iter().enumerate() {
        for b in list {
            adj_matrix[a][*b] = true;
        }
    }

    (id_map, adj_list, adj_matrix)
}

#[test]
fn part1() {
    let (id_map, adj_list, adj_matrix) = input();

    let mut found: HashSet<(usize, usize, usize)> = HashSet::new();
    for node in id_map.iter().filter(|(k, _)| k.starts_with('t')).map(|(_, v)| *v) {
        let adjacent = &adj_list[node];
        for i in 0..adjacent.len() - 1 {
            for j in i + 1 .. adjacent.len() {
                let a = adjacent[i];
                let b = adjacent[j];

                if adj_matrix[a][b] {
                    let mut x = node;
                    let mut y = a;
                    let mut z = b;
                    if y < x {
                        std::mem::swap(&mut x, &mut y);
                    }
                    if z < x {
                        std::mem::swap(&mut x, &mut z);
                    }
                    if z < y {
                        std::mem::swap(&mut y, &mut z);
                    }
                    found.insert((x, y, z));
                }
            }
        }
    }
    let answer = found.len();
    assert_eq!(answer, 1348);
}

#[test]
fn part2() {
    let (id_map, adj_list, adj_matrix) = input();

    // this is just an implementation of this, I didn't know this off hand
    // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    fn recurse(r: HashSet<usize>, mut p: HashSet<usize>, mut x: HashSet<usize>, adj_list: &Vec<Vec<usize>>, adj_matrix: &Vec<Vec<bool>>, max: &mut Vec<usize>) {
        if p.len() == 0 && x.len() == 0 {
            if r.len() > max.len() {
                *max = r.iter().cloned().to_vec();
            }
        } else {
            let u = if p.len() > 0  {
                *p.iter().next().unwrap()
            } else {
                *x.iter().next().unwrap()
            };

            for v in p.iter().cloned().to_vec() {
                if adj_matrix[v][u] {
                    continue;
                }

                let mut next_r = r.clone();
                next_r.insert(v);

                let mut next_p = HashSet::with_capacity(p.len());
                let mut next_x = HashSet::with_capacity(x.len());

                for adj in adj_list[v].iter() {
                    if p.contains(adj) {
                        next_p.insert(*adj);
                    }
                    if x.contains(adj) {
                        next_x.insert(*adj);
                    }
                }

                recurse(next_r, next_p, next_x, adj_list, adj_matrix, max);

                p.remove(&v);
                x.insert(v);
            }
        }
    }

    let mut max: Vec<usize> = Vec::new();
    recurse(HashSet::new(), (0..adj_list.len()).to_set(), HashSet::new(), &adj_list, &adj_matrix, &mut max);

    let mut answer = max.into_iter().map(|idx| id_map.get_key(idx).to_string()).to_vec();
    answer.sort();
    let answer = answer.join(",");
    assert_eq!(answer, "am,bv,ea,gh,is,iy,ml,nj,nl,no,om,tj,yv");
}