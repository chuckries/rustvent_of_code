use core::num;
use std::{collections::VecDeque, f64, ops::MulAssign, usize, vec};

use aoc_common::{IteratorExt, PriorityQueue, Vec2us, file_lines, lcm};
use z3::{Optimize, SatResult, Solver, ast::{Ast, Int, Real}};

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

// #[test]
// fn part2() {
//     let input = input2();
//     let mut total = 0;
//     for (target, indices_list) in input {
//         let mut queue: PriorityQueue<(Vec<usize>, usize), usifze> = PriorityQueue::new();
//         queue.enqueue((vec![0; target.len()], 0), 0);
//         'outer: while let Some((current, count)) = queue.dequeue() {
//             if current == target {
//                 total += count;
//                 break;
//             }

//             let count = count + 1;
//             for indices in indices_list.iter() {
//                 let mut next = current.clone();
//                 let mut dist = 0;
//                 for i in indices {
//                     next[*i] += 1;
//                     if next[*i] > target[*i] {
//                         continue 'outer;
//                     }
//                     dist += target[*i] - next[*i];
//                 }
//                 queue.enqueue((next, count), count + dist);
//             }
//         }
//     }

//     assert_eq!(0, total);
// }

// #[test]
// fn part2_simplex() {
//     let input = input2();

//     let answer: i64 = input.into_iter().map(|(target, indicies_list)| do_simplex(&target, &indicies_list)).sum();
//     assert_eq!(0, answer);
// }

// fn add_rows(mat: &mut Vec<Vec<i64>>, src_idx: usize, dst_idx: usize, mul: i64) {
//     let [src, dst] = mat.get_disjoint_mut([src_idx, dst_idx]).unwrap();

//     for (src, dst) in src.iter_mut().zip(dst.iter_mut()) {
//         *dst += *src * mul;
//     }
// }

// fn mul_row(mat: &mut Vec<Vec<i64>>, idx: usize, mul: i64) {
//     for n in mat[idx].iter_mut() {
//         *n *= mul;
//     }
// }

// fn print_mat(matrix: &Vec<Vec<i64>>) {
//     for row in matrix.iter() {
//         for n in row.iter() {
//             print!("{}\t", n);
//         }
//         println!();
//     }
//     println!();
// }

// fn do_simplex(target: &[usize], indices_list: &[Vec<usize>]) -> i64 {
//     // form system vectors and dedup
//     let mut system_vecs = vec![vec![0; indices_list.len() + 1]; target.len()];
//     for (i, n) in target.iter().enumerate() {
//         *system_vecs[i].last_mut().unwrap() = *n as i64;
//     }

//     for (col, indices) in indices_list.iter().enumerate() {
//         for idx in indices {
//             system_vecs[*idx][col] = 1;
//         }
//     }

//     let system_vecs = system_vecs.into_iter().to_set().into_iter().to_vec();
//     let num_unknowns = indices_list.len();
//     let num_equations = system_vecs.len();


//     println!("{:?}", system_vecs);


//     let columns = 2 + num_unknowns + num_equations + 1;
//     let rows = 2 + num_equations;
//     let mut matrix = vec![vec![0; columns]; rows];

//     matrix[0][0] = 1;
//     for i in 0 .. num_equations {
//         matrix[0][2 + num_unknowns + i] = -1;
//     }

//     matrix[1][1] = 1;
//     for i in 0 .. num_unknowns {
//         matrix[1][2 + i] = -1;
//     }

//     for (j, v) in system_vecs.into_iter().enumerate() {
//         for i in 0 .. v.len() - 1 {
//             matrix[2 + j][2 + i] = v[i];
//         }
//         matrix[2 + j][2 + num_unknowns + j] = 1;
//         *matrix[2 + j].last_mut().unwrap() = *v.last().unwrap();    
//     }

//     print_mat(&matrix);

//     for i in 0..num_equations {
//         add_rows(&mut matrix, 2 + i, 0, 1);
//     }

//     print_mat(&matrix);

//     let mut avail_colums = vec![false; matrix[0].len()];
//     for i in 0..num_unknowns {
//         avail_colums[2 + i] = true;
//     }

//     for phase in 0 ..= 1 {
//         while let Some(pivot) = select_pivot(&matrix, 2 - phase, &mut avail_colums) {
//             avail_colums[pivot.x] = false;
//             println!("pivot column: {}", pivot.x);
//             println!("pivot row: {}", pivot.y);

//             let pivot_val = matrix[pivot.y][pivot.x];
//             println!("pivot value: {}", pivot_val);

//             for j in 0..matrix.len() {
//                 if j == pivot.y { continue; }
//                 if matrix[j][pivot.x] == 0 { continue; }

//                 mul_row(&mut matrix, j, pivot_val);
//                 let mul = matrix[j][pivot.x] / pivot_val;
//                 add_rows(&mut matrix, pivot.y, j, -mul);
//             }
            
//             print_mat(&matrix);

//             if matrix[0].last().unwrap().eq(&0) {
//                 break;
//             }
//         }

//         return matrix.iter().skip(2).map(|v| *v.last().unwrap()).sum();

//         if phase == 0 {
//             if matrix[0].last().unwrap().ne(&0) {
//                 panic!("phase 1 failed???");
//             }

//             matrix.remove(0);
//             for j in 0 .. matrix.len() {
//                 matrix[j].remove(0);
//                 for _ in 0 .. num_equations {
//                     matrix[j].remove(1 + num_unknowns);
//                 }
//             }

//             print_mat(&matrix);

//             avail_colums = vec![false; matrix[0].len()];
//             for i in 0 .. num_unknowns {
//                 avail_colums[1 + i] = true;
//             }
//         }
//     }

//     *matrix[0].last().unwrap() / matrix[0][0]
// }

// fn select_pivot(mat: &Vec<Vec<i64>>, skip_row: usize, avail_columns: &mut [bool]) -> Option<Vec2us> {
//     for i in skip_row .. mat[0].len() - 1 {
//         if avail_columns[i] && mat[0][i] > 0 {
//             let mut min: f64 = f64::MAX;
//             let mut min_idx = 0;
//             for j in skip_row .. mat.len() {
//                 if mat[j][i] > 0 && *mat[j].last().unwrap() > 0 {
//                     let ratio = *mat[j].last().unwrap() as f64 / mat[j][i] as f64;
//                     if ratio < min {
//                         min = ratio;
//                         min_idx = j;
//                     }
//                 }
//             }

//             if min_idx != 0 {
//                 return Some(Vec2us::new(i, min_idx));
//             }
//         }
//     }

//     None
// }

#[test]
fn part2_simplex_lib() {
    for (target, indices_list) in input2() {


        // let mut model = Model::new();

        // let variables = (0..indices_list.len()).map(|i| {
        //     model.add_variable().name(format!("x{}", i)).lower_bound(0f64)
        // }).to_vec();

        // let mut expr: LinearExpr<Var> = LinearExpr::new();
        // for var in variables.iter() {
        //     expr.add_term(var.clone(), 1f64);
        // }

        // model.set_objective(ObjectiveSense::Minimize, expr);

        // let mut exprs = vec![LinearExpr::new(); target.len()];

        // for (x, indices) in indices_list.iter().enumerate() {
        //     for idx in indices {
        //         exprs[*idx].add_term(variables[x].clone(), 1f64);
        //     }
        // }

        // for (i, expr) in exprs.into_iter().enumerate() {
        //     model.add_constraint(expr, ConstraintSense::Equal, target[i] as f64);
        // }

        // model.solve();

        // println!("{}", model.get_solution());
    }
}

#[test]
fn part2_z3() {
    let mut total = 0;
    for (target, indices_list) in input2() {
        // form system vectors and dedup

        let num_variables = indices_list.len();
        let mut constraints = vec![vec![0; indices_list.len()]; target.len()];
        for (col, indices) in indices_list.iter().enumerate() {
            for idx in indices {
                constraints[*idx][col] = 1;
            }
        }

        let variables = (0..num_variables).map(|n| {
            Int::fresh_const(&format!("x{}", n))
        }).to_vec();

        let optimize = Optimize::new();
        for var in variables.iter() {
            optimize.assert(&var.ge(0));
        }

        for (eq_idx, b) in target.iter().enumerate() {
            let expr: Int = indices_list.iter().enumerate().filter_map(|(var_idx, used_variables)| {
                if used_variables.contains(&eq_idx) {
                    Some(&variables[var_idx])
                } else { None }
            }).sum();

            optimize.assert(&expr.eq(*b as i64));
        }

        let sum: Int = variables.iter().sum();
        optimize.minimize(&sum);

        assert_eq!(optimize.check(&[]), SatResult::Sat);
        let model = optimize.get_model().unwrap();
        println!("{}", model);
        let minimum = model.eval(&sum, false).unwrap().as_i64().unwrap();
        total += minimum;
    }

    assert_eq!(0, total);
}

#[test]
fn part2_two_phase() {
    let mut total = 0f64;
    for (target, indices_list) in input2() {
        // form system vectors and dedup
        let mut system_vecs = vec![vec![0f64; indices_list.len() + 1]; target.len()];
        for (i, n) in target.iter().enumerate() {
            *system_vecs[i].last_mut().unwrap() = *n as f64;
        }

        for (col, indices) in indices_list.iter().enumerate() {
            for idx in indices {
                system_vecs[*idx][col] = 1f64;
            }
        }

        // let mut system_vecs = system_vecs.into_iter().to_set().into_iter().to_vec();
        // system_vecs.sort_by_cached_key(|v| *v.last().unwrap());

        let target: Vec<f64> = system_vecs.iter().map(|v| *v.last().unwrap() as f64).collect();
        let constraints: Vec<Vec<f64>> = system_vecs.iter().map(|v| v[..v.len() - 1].iter().cloned().collect()).collect();

        let solver = SimpleSimplex::new(target, constraints);
        let answer = solver.solve();
        println!("{}", answer);
        println!("");

        total += answer;
    }

    assert_eq!(0f64, total);
}

struct SimpleSimplex {
    target: Vec<f64>,
    constraints: Vec<Vec<f64>>,
    mat: Vec<Vec<f64>>,

    // the current basic variable index for each row of the tableauf
    // i.e. basics[0] is the idx of the variable correspodning to the firs equation, which is row 1 of the matrix
    basics: Vec<usize>,
    num_unknowns: usize,
    num_artificial: usize,
}

impl SimpleSimplex {
    fn new(target: Vec<f64>, constraint_expressions: Vec<Vec<f64>>) -> Self {
        println!("target: {:?}", target);
        println!("constraints: {:?}", constraint_expressions);

        // create the tableau

        let num_unknowns = constraint_expressions[0].len();
        let num_constraints = constraint_expressions.len();

        // we will have 1 row for the objective and 1 row each for the constraint expressions
        let rows = 1 + constraint_expressions.len();

        // we will have 1 col for Z, a column for each variable, a column for each artifical variable (equal to number of constraints), and a column for b
        let cols = 1 + constraint_expressions[0].len() + constraint_expressions.len() + 1;

        let mut matrix = vec![vec![0f64; cols]; rows];

        // we will always set up to solve Minimize Z = SUM(x0..x(n -1)) where n is the number of unknowns
        // our contraints are always = target[i] where i is the index of the constraint expression
        // we will do this by setting up a phase one problem Min Z = sum(xn .. xm) where m - n is the number of constraints and therefore the number of artifical variables
        matrix[0][0] = -1f64;
        for i in 0..num_constraints {
            matrix[0][1 + num_unknowns + i] = 1f64;
        }

        // fill out the constraint rows
        for (j, constraint) in constraint_expressions.iter().enumerate() {

            // constraint coefficients
            for (i, n) in constraint.iter().enumerate() {
                matrix[1 + j][1 + i] = *n as f64;
            }

            // artifical variable coefficient
            matrix[1 + j][1 + num_unknowns + j] = 1f64;
            
            // b value
            matrix[1 + j][1 + num_unknowns + num_constraints] = target[j];
        }

        // initial basic variables are all the artifical variables
        let mut basics = vec![0; num_constraints];
        for i in 0 .. num_constraints {
            basics[i] = num_unknowns + i;
        }

        let mut solver = Self {
            target,
            constraints: constraint_expressions,
            mat: matrix,
            basics,
            num_unknowns,
            num_artificial: num_constraints,
        };
        solver.print();

        solver.remove_basic_from_objective();

        solver.print();

        solver
    }

    fn solve(mut self) -> f64 {
        // first phase
        //println!("phase one");
        self.solve_impl(1);

        // // check if artifical varialbes remain in basis
        // let artificials_in_basis = self.basics.iter().copied().filter(|b| *b >= self.num_unknowns).to_vec();
        // if artificials_in_basis.len() > 0 {
        //     for a in artificials_in_basis

        //     println!("artifical variables in basis, assume we are optimal as idk wtf else to do");
        //     return self.mat[1..].iter().map(|v| v.last().unwrap()).sum();
        // }
        let mut idx = 0;
        while idx < self.basics.len() {
            let basic = self.basics[idx];
            if basic >= self.num_unknowns {
                // basic is artifical
                if self.mat[idx + 1][1 .. self.num_unknowns + 1].iter().all(|n| *n == 0f64) {
                    // basic is redundant
                    println!("removing redundant contraint from basis: {}", self.get_var_name(basic));
                    self.basics.remove(idx);
                    self.mat.remove(idx + 1);
                    continue;
                }
                else {
                    println!("pivoting out artificial");
                    let mut pivot_idx = 0;
                    for i in 1 .. 1 + self.num_unknowns {
                        if self.mat[idx + 1][i] != 0f64 {
                            pivot_idx = i;
                            break;
                        }
                    }

                    if pivot_idx == 0 {
                        panic!("unable to pivot artifical out")
                    }

                    for j in 0..self.mat.len() {
                        if j == idx + 1 { continue; }
                        if self.mat[j][pivot_idx] == 0f64 { continue; }

                        self.reduce_row(j, idx + 1, pivot_idx);
                    }
                    self.basics[idx] = pivot_idx - 1;

                }
            }
            idx += 1;
        }

        //println!("phase two");

        // remove artificals
        for v in self.mat.iter_mut() {
            for _ in 0 .. self.num_artificial {
                v.remove(1 + self.num_unknowns);
            }
        }
        self.num_artificial = 0;

        // restore original objective function
        for i in 0..self.num_unknowns {
            self.mat[0][1 + i] = 1f64;
        }

        self.print();
        self.remove_basic_from_objective();
        self.print();

        self.solve_impl(2);
        
        *self.mat[0].last().unwrap() / self.mat[0][0]
    }

    fn solve_impl(&mut self, phase: usize) {
        while let Some(pivot) = self.find_pivot() {
            if phase == 2 {
                println!("found pivot in phase 2");
            }

            // println!("pivot column: {}", pivot.x);
            // println!("pivot row: {}", pivot.y);

            let pivot_val = self.mat[pivot.y][pivot.x];
            //println!("pivot val: {}", pivot_val);

            let entering_variable = pivot.x - 1;
            //println!("entering variable: {}", self.get_var_name(entering_variable));
            
            let leaving = self.basics[pivot.y - 1];
            //println!("leaving variable: {}", self.get_var_name(leaving));

            self.basics[pivot.y - 1] = pivot.x - 1;

            for j in 0 .. self.mat.len() {
                if j == pivot.y { continue; }

                if self.mat[j][pivot.x] != 0f64 {
                    self.reduce_row(j, pivot.y, pivot.x);
                }
            }

            self.print();
        }
    }

    fn find_pivot(&self) -> Option<Vec2us> {
        let mut min_coefficient = 0f64;
        let mut min_idx = 0;
        for (i, n) in self.mat[0].iter().enumerate() {
            if i == 0 || i == self.mat[0].len() - 1 {
                continue;
            }

            if self.basics.contains(&(i - 1)) { continue; }

            if *n < min_coefficient {
                min_coefficient = *n;
                min_idx = i;
            }
        }

        if min_coefficient < 0f64 {
            let mut min_ratio = f64::MAX;
            let mut min_row_idx = 0;

            for j in 1 .. self.mat.len() {
                if self.mat[j][min_idx] > 0f64 {
                    let ratio = *self.mat[j].last().unwrap() as f64 / self.mat[j][min_idx] as f64;
                    if ratio < min_ratio {
                        min_ratio = ratio;
                        min_row_idx = j;
                    }
                }
            }

            if min_row_idx == 0 {
                panic!("couldn't find pivot");
            }

            return Some(Vec2us::new(min_idx, min_row_idx));
        }

        None
    }

    fn remove_basic_from_objective(&mut self) {
        // juts assume this will work, panic on weird stuff, and come back later

        for b_idx in 0 .. self.basics.len() {
            // offset to get past z column
            let basic = self.basics[b_idx] + 1;

            // idk how to deal with artificals remaining in the basis
            if basic >= self.mat[0].len() {
                continue;
            }
            
            if self.mat[0][basic] != 0f64 {
                // find a row that is non-zero in the basic column, and reduce the objective row by it
                for j in 1..self.mat.len() {
                    if self.mat[j][basic] != 0f64 {
                        self.reduce_row(0, j, basic);
                        break;
                    }
                }
            }
        }

        for b in self.basics.iter().cloned() {
            let b = b + 1;

            // idk how to deal with artificals remaining in the basis
            if b >= self.mat[0].len() {
                continue;
            }

            if self.mat[0][b] != 0f64 {
                panic!("failed to reduce objective function");
            }
        }
    }

    fn reduce_row(&mut self, to_reduce_idx: usize, src_idx: usize, col_idx: usize) {
        let pivot_val = self.mat[src_idx][col_idx];

        self.mul_row(src_idx, 1f64 / pivot_val);

        let mul = self.mat[to_reduce_idx][col_idx];
        self.add_rows(src_idx, to_reduce_idx, -mul);
    }

    fn add_rows(&mut self, src_row: usize, dst_row: usize, multi: f64) {
        let [src, dst] = self.mat.get_disjoint_mut([src_row, dst_row]).unwrap();
        for (src_n, dst_n) in src.iter_mut().zip(dst.iter_mut()) {
            *dst_n += *src_n * multi;
        }
    }

    fn mul_row(&mut self, row: usize, mul: f64) {
        for n in self.mat[row].iter_mut() {
            n.mul_assign(mul);
        }
    }

    fn get_var_name(&self, idx: usize) -> String {
        const AUG: &str = "A";

        if idx >= self.num_unknowns {
            format!("{}x{}", AUG, idx)
        } else {
            format!("x{}", idx)
        }
    }

    fn print(&self) {
        //return;
        let w = 8;
        let p = 1;

        print!("{:w$}", "");
        print!("{:>w$}", "Z");

        for i in 0 .. self.num_unknowns {
            print!("{:>w$}", format!("x{}", i));
        }

        for i in 0 .. self.num_artificial {
            print!("{:>w$}", format!("Ax{}", self.num_unknowns + i));
        }

        print!("{:>w$}", "b");

        println!();
        for (j, row) in self.mat.iter().enumerate() {
            if j == 0 {
                print!("{:>w$}", "Z");
            } else {
                let basic = self.basics[j - 1];
                print!("{:>w$}", self.get_var_name(basic));
            }
            for n in row.iter() {
                print!("{:>w$.p$}", n);
            }
            println!();
        }
        println!();
    }
}