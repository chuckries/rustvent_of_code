use core::num;
use std::{collections::VecDeque, f64, vec};

use aoc_common::{IteratorExt, PriorityQueue, Vec2us, file_lines, lcm};
use rustplex::core::{constraint::ConstraintSense, expression::{ExprVariable, LinearExpr}, model::{self, Model}, objective::ObjectiveSense, variable::Var};

fn input() -> Vec<(usize, Vec<usize>)> {
    file_lines("inputs/day10.txt").map(|l| {
        let mut split = l.split(' ');

        let target_string = split.next().unwrap().trim_matches(['[', ']']);
        let mut target = 0;
        for b in target_string.bytes() {
            target = (target << 1) | if b == b'#' { 1 } else { 0 };
        }

        let mut masks: Vec<usize> = Vec::new();

        let mut split = split.peekable();
        loop {
            let next = split.peek().unwrap();
            if next.starts_with('{') {
                break;
            }
            let mask_split = split.next().unwrap().trim_matches(['(', ')']).split(',');
            let mut mask = 0;
            for i in mask_split.map(|s| s.parse::<usize>().unwrap()) {
                mask |= 1 << (target_string.len() - i - 1);
            }
            masks.push(mask);
        }

        (target, masks)
    }).collect()
}

fn input2() -> Vec<(Vec<usize>, Vec<Vec<usize>>)> {
    file_lines("inputs/day10.txt").map(|l| {
        let mut split = l.split(' ').skip(1).peekable();
        let mut indices_list = Vec::new();
        loop {
            let next = split.peek().unwrap();
            if next.starts_with('{') {
                break;
            }
            let indices = split.next().unwrap().trim_matches(['(', ')']).split(',').map(|s| s.parse().unwrap()).to_vec();
            indices_list.push(indices);
        }

        let target = split.next().unwrap().trim_matches(['{', '}']).split(',').map(|s| s.parse().unwrap()).to_vec();

        (target, indices_list)
    }).collect()
}

#[test]
fn part1() {
    let input = input();

    let mut total = 0;
    for (target, masks) in input {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((0, 0));
        'outer: while let Some((state, count)) = queue.pop_front() {
            let count = count + 1;
            for mask in masks.iter() {
                let next = state ^ *mask;
                if next == target {
                    total += count;
                    break 'outer;
                } else {
                    queue.push_back((next, count));
                }
            }
        }
    }

    assert_eq!(475, total);
}

#[test]
fn part2() {
    let input = input2();
    let mut total = 0;
    for (target, indices_list) in input {
        let mut queue: PriorityQueue<(Vec<usize>, usize), usize> = PriorityQueue::new();
        queue.enqueue((vec![0; target.len()], 0), 0);
        'outer: while let Some((current, count)) = queue.dequeue() {
            if current == target {
                total += count;
                break;
            }

            let count = count + 1;
            for indices in indices_list.iter() {
                let mut next = current.clone();
                let mut dist = 0;
                for i in indices {
                    next[*i] += 1;
                    if next[*i] > target[*i] {
                        continue 'outer;
                    }
                    dist += target[*i] - next[*i];
                }
                queue.enqueue((next, count), count + dist);
            }
        }
    }

    assert_eq!(0, total);
}

#[test]
fn part2_simplex() {
    let input = input2();

    let answer: i64 = input.into_iter().map(|(target, indicies_list)| do_simplex(&target, &indicies_list)).sum();
    assert_eq!(0, answer);
}

fn add_rows(mat: &mut Vec<Vec<i64>>, src_idx: usize, dst_idx: usize, mul: i64) {
    let [src, dst] = mat.get_disjoint_mut([src_idx, dst_idx]).unwrap();

    for (src, dst) in src.iter_mut().zip(dst.iter_mut()) {
        *dst += *src * mul;
    }
}

fn mul_row(mat: &mut Vec<Vec<i64>>, idx: usize, mul: i64) {
    for n in mat[idx].iter_mut() {
        *n *= mul;
    }
}

fn print_mat(matrix: &Vec<Vec<i64>>) {
    for row in matrix.iter() {
        for n in row.iter() {
            print!("{}\t", n);
        }
        println!();
    }
    println!();
}

fn do_simplex(target: &[usize], indices_list: &[Vec<usize>]) -> i64 {
    // form system vectors and dedup
    let mut system_vecs = vec![vec![0; indices_list.len() + 1]; target.len()];
    for (i, n) in target.iter().enumerate() {
        *system_vecs[i].last_mut().unwrap() = *n as i64;
    }

    for (col, indices) in indices_list.iter().enumerate() {
        for idx in indices {
            system_vecs[*idx][col] = 1;
        }
    }

    let system_vecs = system_vecs.into_iter().to_set().into_iter().to_vec();
    let num_unknowns = indices_list.len();
    let num_equations = system_vecs.len();


    println!("{:?}", system_vecs);


    let columns = 2 + num_unknowns + num_equations + 1;
    let rows = 2 + num_equations;
    let mut matrix = vec![vec![0; columns]; rows];

    matrix[0][0] = 1;
    for i in 0 .. num_equations {
        matrix[0][2 + num_unknowns + i] = -1;
    }

    matrix[1][1] = 1;
    for i in 0 .. num_unknowns {
        matrix[1][2 + i] = -1;
    }

    for (j, v) in system_vecs.into_iter().enumerate() {
        for i in 0 .. v.len() - 1 {
            matrix[2 + j][2 + i] = v[i];
        }
        matrix[2 + j][2 + num_unknowns + j] = 1;
        *matrix[2 + j].last_mut().unwrap() = *v.last().unwrap();    
    }

    print_mat(&matrix);

    for i in 0..num_equations {
        add_rows(&mut matrix, 2 + i, 0, 1);
    }

    print_mat(&matrix);

    let mut avail_colums = vec![false; matrix[0].len()];
    for i in 0..num_unknowns {
        avail_colums[2 + i] = true;
    }

    for phase in 0 ..= 1 {
        while let Some(pivot) = select_pivot(&matrix, 2 - phase, &mut avail_colums) {
            avail_colums[pivot.x] = false;
            println!("pivot column: {}", pivot.x);
            println!("pivot row: {}", pivot.y);

            let pivot_val = matrix[pivot.y][pivot.x];
            println!("pivot value: {}", pivot_val);

            for j in 0..matrix.len() {
                if j == pivot.y { continue; }
                if matrix[j][pivot.x] == 0 { continue; }

                mul_row(&mut matrix, j, pivot_val);
                let mul = matrix[j][pivot.x] / pivot_val;
                add_rows(&mut matrix, pivot.y, j, -mul);
            }
            
            print_mat(&matrix);

            if matrix[0].last().unwrap().eq(&0) {
                break;
            }
        }

        return matrix.iter().skip(2).map(|v| *v.last().unwrap()).sum();

        if phase == 0 {
            if matrix[0].last().unwrap().ne(&0) {
                panic!("phase 1 failed???");
            }

            matrix.remove(0);
            for j in 0 .. matrix.len() {
                matrix[j].remove(0);
                for _ in 0 .. num_equations {
                    matrix[j].remove(1 + num_unknowns);
                }
            }

            print_mat(&matrix);

            avail_colums = vec![false; matrix[0].len()];
            for i in 0 .. num_unknowns {
                avail_colums[1 + i] = true;
            }
        }
    }

    *matrix[0].last().unwrap() / matrix[0][0]
}

fn select_pivot(mat: &Vec<Vec<i64>>, skip_row: usize, avail_columns: &mut [bool]) -> Option<Vec2us> {
    for i in skip_row .. mat[0].len() - 1 {
        if avail_columns[i] && mat[0][i] > 0 {
            let mut min: f64 = f64::MAX;
            let mut min_idx = 0;
            for j in skip_row .. mat.len() {
                if mat[j][i] > 0 && *mat[j].last().unwrap() > 0 {
                    let ratio = *mat[j].last().unwrap() as f64 / mat[j][i] as f64;
                    if ratio < min {
                        min = ratio;
                        min_idx = j;
                    }
                }
            }

            if min_idx != 0 {
                return Some(Vec2us::new(i, min_idx));
            }
        }
    }

    None
}

#[test]
fn part2_simplex_lib() {
    for (target, indices_list) in input2() {
        let mut model = Model::new();

        let variables = (0..indices_list.len()).map(|i| {
            model.add_variable().name(format!("x{}", i)).lower_bound(0f64)
        }).to_vec();

        let mut expr: LinearExpr<Var> = LinearExpr::new();
        for var in variables.iter() {
            expr.add_term(var.clone(), 1f64);
        }

        model.set_objective(ObjectiveSense::Minimize, expr);

        let mut exprs = vec![LinearExpr::new(); target.len()];

        for (x, indices) in indices_list.iter().enumerate() {
            for idx in indices {
                exprs[*idx].add_term(variables[x].clone(), 1f64);
            }
        }

        for (i, expr) in exprs.into_iter().enumerate() {
            model.add_constraint(expr, ConstraintSense::Equal, target[i] as f64);
        }

        model.solve();

        println!("{}", model.get_solution());
    }
}