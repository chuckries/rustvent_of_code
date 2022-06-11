use std::collections::{HashMap, VecDeque, HashSet, BinaryHeap};
use std::fmt::{Write, Display};
use aoc_common::{Vec2us, file_lines, ToVec, SearchNode};

use Cell::*;

#[derive(Copy, Clone, Debug)]
enum Cell {
    Hall(usize),
    Home(usize),
}

#[derive(Copy, Clone, Debug)]
enum PodType {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, Debug)]
struct Pod {
    p_type: PodType,
    has_moved: bool,
    is_home: bool,
}

impl Pod {
    fn new(p_type: PodType) -> Self {
        Self {
            p_type,
            has_moved: false,
            is_home: false,
        }
    }

    fn is_home_index(&self, idx: usize) -> bool {
        match self.p_type {
            PodType::A if idx == 0 || idx == 1 => true,
            PodType::B if idx == 2 || idx == 3 => true,
            PodType::C if idx == 4 || idx == 5 => true,
            PodType::D if idx == 6 || idx == 7 => true,
            _ => false
        }
    }

    fn weight(&self) -> usize {
        match self.p_type {
            PodType::A => 1,
            PodType::B => 10,
            PodType::C => 100,
            PodType::D => 1000,
        }
    }
}

impl From<char> for Pod {
    fn from(c: char) -> Self {
        let p_type = match c {
            'A' => PodType::A,
            'B' => PodType::B,
            'C' => PodType::C,
            'D' => PodType::D,
            _ => panic!()
        };
        Pod::new(p_type)
    }
}

impl Display for Pod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.p_type {
            PodType::A => 'A',
            PodType::B => 'B',
            PodType::C => 'C',
            PodType::D => 'D',
        };

        write!(f, "{}", c)
    }
}

type Map = Vec<Vec<char>>;
type IndexMap = HashMap<Vec2us, usize>;

type Child = (Cell, usize, Tree);

#[derive(Debug, Clone)]
struct Tree {
    children: Vec<Child>,
}

struct InputBuilder {
    map: Map,
    home_indices: IndexMap,
    hall_indices: IndexMap,
}

impl InputBuilder {
    fn new() -> Self {
        let home_indices: IndexMap = [
            (Vec2us::new(3, 2), 0),
            (Vec2us::new(3, 3), 1),
            (Vec2us::new(5, 2), 2),
            (Vec2us::new(5, 3), 3),
            (Vec2us::new(7, 2), 4),
            (Vec2us::new(7, 3), 5),
            (Vec2us::new(9, 2), 6),
            (Vec2us::new(9, 3), 7),
        ].into_iter().collect();
    
        let hall_indices: IndexMap = [
            (Vec2us::new(1, 1), 0),
            (Vec2us::new(2, 1), 1),
            (Vec2us::new(4, 1), 2),
            (Vec2us::new(6, 1), 3),
            (Vec2us::new(8, 1), 4),
            (Vec2us::new(10, 1), 5),
            (Vec2us::new(11, 1), 6),
        ].into_iter().collect();

        let map = file_lines("inputs/day23.txt").map(|l| l.chars().to_vec()).to_vec();

        Self {
            map,
            home_indices,
            hall_indices
        }
    }

    fn explore_recurse(&self, pos: Vec2us, dist: usize, mut parent: &mut Tree, visited: &mut HashSet<Vec2us>) {
        if !visited.insert(pos) {
            return;
        }

        if let Some(home) = self.home_indices.get(&pos) {
            let mut new_parent = Tree { children: Vec::new() };
            for adj in pos.adjacent() {
                if self.map[adj.y][adj.x] == '#' {
                    continue;
                }

                if let Some(adj_home) = self.home_indices.get(&adj) {
                    if adj_home & 1 == 1 {
                        continue;
                    }
                }

                self.explore_recurse(adj, dist + 1, &mut new_parent, visited);
            }
            parent.children.push((Home(*home), dist, new_parent));
        } else if let Some(hall) = self.hall_indices.get(&pos) {
            let mut new_parent = Tree { children: Vec::new() };
            for adj in pos.adjacent() {
                if self.map[adj.y][adj.x] == '#' {
                    continue;
                }

                self.explore_recurse(adj, dist + 1, &mut new_parent, visited);
            }
            parent.children.push((Hall(*hall), dist, new_parent));
        } else {
            for adj in pos.adjacent() {
                if self.map[adj.y][adj.x] == '#' {
                    continue;
                }

                self.explore_recurse(adj, dist + 1, parent, visited);
            }
        };
    }

    fn build(&self) -> (Vec<Vec<Child>>, Vec<Vec<Child>>, Vec<Option<Pod>>, Vec<Option<Pod>>) {
        let hall_state: Vec<Option<Pod>> = vec![None; 7];
        let mut home_state: Vec<Option<Pod>> = vec![None; 8];

        let mut home_paths = Vec::new();
        let mut hall_paths = Vec::new();

        for j in 0..self.map.len() {
            for i in 0..self.map[j].len() {
                if let Some(home) = self.home_indices.get(&(i, j).into()) {
                    home_state[*home] = Some(self.map[j][i].into());

                    if *home >= home_paths.len() {
                        home_paths.resize(*home + 1, vec![]);
                    }
                    let mut parent = Tree { children: vec![] };
                    let mut visited = HashSet::new();
                    self.explore_recurse((i, j).into(), 0, &mut parent, &mut visited);
                    home_paths[*home] = parent.children;
                } else if let Some(hall) = self.hall_indices.get(&(i, j).into()) {
                    if *hall >= hall_paths.len() {
                        hall_paths.resize(*hall + 1, vec![]);
                    }
                    let mut parent = Tree { children: vec![] };
                    let mut visited = HashSet::new();
                    self.explore_recurse((i, j).into(), 0, &mut parent, &mut visited);
                    hall_paths[*hall] = parent.children;
                }
            }
        }

        (home_paths, hall_paths, home_state, hall_state)
    }

    fn print_state(&self, home_state: &Vec<Option<Pod>>, hall_state: &Vec<Option<Pod>>) {
        let mut string = String::new();
        for j in 0..5 {
            for i in 0..13 {
                let pod = if let Some(idx) = self.home_indices.get(&(i, j).into()) {
                    home_state[*idx]
                } else if let Some(idx) = self.hall_indices.get(&(i, j).into()) {
                    hall_state[*idx]
                } else {
                    None
                };

                let c = pod.map_or('.', |p| {
                    match p.p_type {
                        PodType::A => 'A',
                        PodType::B => 'B',
                        PodType::C => 'C',
                        PodType::D => 'D',
                    }
                });

                string.push(c);
            }
            string.push('\n');
        }

        println!("{}", string);
    }
}

struct Move {
    from: Cell,
    to: Cell,
    cost: usize,
}

fn generate_next_moves(home_paths: &Vec<Vec<Child>>, hall_paths: &Vec<Vec<Child>>, home_start_state: &Vec<Option<Pod>>, hall_start_state: &Vec<Option<Pod>>) -> Vec<Move> {
    let mut next_moves: Vec<Move> = Vec::new();

    for (pod_idx, pod) in home_start_state.iter().enumerate().filter_map(|(idx, h)| h.and_then(|h| Some((idx, h))) ) {
        if pod.is_home {
            continue;
        }

        let mut to_visit: VecDeque<&Vec<Child>> = VecDeque::new();
        for children in home_paths[pod_idx].iter() {
            to_visit.push_back(&children.2.children);
        }

        while let Some(next_step) = to_visit.pop_front() {
            for (next_cell, mut delta, next_children) in next_step.iter() {
                if let Home(mut home_idx) = next_cell {
                    if (pod_idx & 1) == 1 {
                        if (home_idx | 1) == pod_idx {
                            if home_start_state[home_idx].is_none() {
                                to_visit.push_back(&next_children.children);
                            }
                            continue;
                        }
                    }

                    if !pod.is_home_index(home_idx) {
                        continue;
                    }

                    if home_start_state[home_idx].is_none() {
                        if let Some(lower_pod) = home_start_state[home_idx | 1] {
                            if !lower_pod.is_home {
                                continue;
                            }
                        } else {
                            home_idx |= 1;
                            delta += 1;
                        }

                        next_moves.push(Move { from: Home(pod_idx), to: Home(home_idx), cost: delta * pod.weight() });

                        // let mut next_home_state = home_start_state.clone();
                        // let mut pod = pod;
                        // pod.is_home = true;
                        // pod.has_moved = true;
                        // next_home_state[pod_idx] = None;
                        // next_home_state[home_idx] = Some(pod);
                        // next_states.push((delta * pod.weight(), next_home_state, hall_start_state.clone()));
                    }

                } else if let Hall(hall_idx) = next_cell {
                    if hall_start_state[*hall_idx].is_none() {
                        next_moves.push(Move { from: Home(pod_idx), to: Hall(*hall_idx), cost: delta * pod.weight() });
                        // let mut home_next_state = home_start_state.clone();
                        // let mut hall_next_state = hall_start_state.clone();
                        // let mut pod = pod;
                        // pod.has_moved = true;
                        // home_next_state[pod_idx] = None;
                        // hall_next_state[*hall_idx] = Some(pod);
                        // next_states.push((delta * pod.weight(), home_next_state, hall_next_state));
                        to_visit.push_back(&next_children.children)
                    }
                } else {
                    panic!()
                }
            }
        }
    }

    for (pod_idx, pod) in hall_start_state.iter().enumerate().filter_map(|(idx, h)| h.and_then(|h| Some((idx, h))) ) {
        let mut to_visit: VecDeque<&Vec<Child>> = VecDeque::new();
        for children in hall_paths[pod_idx].iter() {
            to_visit.push_back(&children.2.children);
        }

        while let Some(next_step) = to_visit.pop_front() {
            for (next_cell, mut delta, next_children) in next_step.iter() {
                if let Home(mut home_idx) = next_cell {
                    if !pod.is_home_index(home_idx) {
                        continue;
                    }

                    if home_start_state[home_idx].is_none() {
                        if let Some(lower_pod) = home_start_state[home_idx | 1] {
                            if !lower_pod.is_home {
                                continue;
                            }
                        } else {
                            home_idx |= 1;
                            delta += 1;
                        }

                        next_moves.push(Move { from: Hall(pod_idx), to: Home(home_idx), cost: delta * pod.weight()});

                        // let mut next_home_state = home_start_state.clone();
                        // let mut next_hall_state = hall_start_state.clone();

                        // let mut pod = pod;
                        // pod.is_home = true;
                        // next_hall_state[pod_idx] = None;
                        // next_home_state[home_idx] = Some(pod);
                        // next_states.push((delta * pod.weight(), next_home_state, next_hall_state));
                    }
                } else if let Hall(hall_idx) = next_cell {
                    if hall_start_state[*hall_idx].is_none() {
                        to_visit.push_back(&next_children.children);
                    }
                } else {
                    panic!()
                }
            }
        }
    }

    next_moves
}

// fn generate_next_states(home_paths: &Vec<Vec<Child>>, hall_paths: &Vec<Vec<Child>>, home_start_state: Vec<Option<Pod>>, hall_start_state: Vec<Option<Pod>>) -> Vec<(usize, Vec<Option<Pod>>, Vec<Option<Pod>>)> {
//     let mut next_states = Vec::new();

//     for (pod_idx, pod) in home_start_state.iter().enumerate().filter_map(|(idx, h)| h.and_then(|h| Some((idx, h))) ) {
//         if pod.is_home {
//             continue;
//         }

//         let mut to_visit: VecDeque<&Vec<Child>> = VecDeque::new();
//         for children in home_paths[pod_idx].iter() {
//             to_visit.push_back(&children.2.children);
//         }

//         while let Some(next_step) = to_visit.pop_front() {
//             for (next_cell, mut delta, next_children) in next_step.iter() {
//                 if let Home(mut home_idx) = next_cell {
//                     if (pod_idx & 1) == 1 {
//                         if (home_idx | 1) == pod_idx {
//                             if home_start_state[home_idx].is_none() {
//                                 to_visit.push_back(&next_children.children);
//                             }
//                             continue;
//                         }
//                     }

//                     if !pod.is_home_index(home_idx) {
//                         continue;
//                     }

//                     if home_start_state[home_idx].is_none() {
//                         if let Some(lower_pod) = home_start_state[home_idx | 1] {
//                             if !lower_pod.is_home {
//                                 continue;
//                             }
//                         } else {
//                             home_idx |= 1;
//                             delta += 1;
//                         }

//                         let mut next_home_state = home_start_state.clone();
//                         let mut pod = pod;
//                         pod.is_home = true;
//                         pod.has_moved = true;
//                         next_home_state[pod_idx] = None;
//                         next_home_state[home_idx] = Some(pod);
//                         next_states.push((delta * pod.weight(), next_home_state, hall_start_state.clone()));
//                     }

//                 } else if let Hall(hall_idx) = next_cell {
//                     if hall_start_state[*hall_idx].is_none() {
//                         let mut home_next_state = home_start_state.clone();
//                         let mut hall_next_state = hall_start_state.clone();
//                         let mut pod = pod;
//                         pod.has_moved = true;
//                         home_next_state[pod_idx] = None;
//                         hall_next_state[*hall_idx] = Some(pod);
//                         next_states.push((delta * pod.weight(), home_next_state, hall_next_state));
//                         to_visit.push_back(&next_children.children)
//                     }
//                 } else {
//                     panic!()
//                 }
//             }
//         }
//     }

//     for (pod_idx, pod) in hall_start_state.iter().enumerate().filter_map(|(idx, h)| h.and_then(|h| Some((idx, h))) ) {
//         let mut to_visit: VecDeque<&Vec<Child>> = VecDeque::new();
//         for children in hall_paths[pod_idx].iter() {
//             to_visit.push_back(&children.2.children);
//         }

//         while let Some(next_step) = to_visit.pop_front() {
//             for (next_cell, mut delta, next_children) in next_step.iter() {
//                 if let Home(mut home_idx) = next_cell {
//                     if !pod.is_home_index(home_idx) {
//                         continue;
//                     }

//                     if home_start_state[home_idx].is_none() {
//                         if let Some(lower_pod) = home_start_state[home_idx | 1] {
//                             if !lower_pod.is_home {
//                                 continue;
//                             }
//                         } else {
//                             home_idx |= 1;
//                             delta += 1;
//                         }

//                         let mut next_home_state = home_start_state.clone();
//                         let mut next_hall_state = hall_start_state.clone();

//                         let mut pod = pod;
//                         pod.is_home = true;
//                         next_hall_state[pod_idx] = None;
//                         next_home_state[home_idx] = Some(pod);
//                         next_states.push((delta * pod.weight(), next_home_state, next_hall_state));
//                     }
//                 } else if let Hall(hall_idx) = next_cell {
//                     if hall_start_state[*hall_idx].is_none() {
//                         to_visit.push_back(&next_children.children);
//                     }
//                 } else {
//                     panic!()
//                 }
//             }
//         }
//     }

//     next_states
// }

//#[test]
fn part1() {

    fn print_recurse(string: String, children: &Vec<Child> ) {
        if children.is_empty() {
            println!("{}", string);
        }

        for child in children.iter() {
            let mut new_string = string.clone();
            write!(new_string, " -> ({:?}, {}", child.0, child.1).unwrap();
            print_recurse(new_string, &child.2.children);
        }
    }

    fn search_recurse(dist: usize, home_state: &mut Vec<Option<Pod>>, hall_state: &mut Vec<Option<Pod>>, home_paths: &Vec<Vec<Child>>, hall_paths: &Vec<Vec<Child>>, min: &mut usize) {
        // if dist >= *min {
        //     return;
        // }
        // println!("dist: {}", dist);
        //InputBuilder::new().print_state(home_state, hall_state);

        if home_state.iter().all(|pod| matches!(pod, Some(p) if p.is_home)) {
            if dist < *min {
                *min = dist;
                println!("min: {}", min);
            }

            //std::process::exit(0);
            return;
        }
        
        for next_move in generate_next_moves(&home_paths, &hall_paths, home_state, hall_state) {
            if (dist + next_move.cost) >= *min {
                continue;
            }

            //InputBuilder::new().print_state(home_state, hall_state);

            let from = match next_move.from {
                Home(home_idx) => home_state[home_idx].take(),
                Hall(hall_idx) => hall_state[hall_idx].take(),
            };

            match next_move.to {
                Home(home_idx) => home_state[home_idx] = from.and_then(|mut p| { p.is_home = true; Some(p) }),
                Hall(hall_idx) => hall_state[hall_idx] = from,
            }

            //InputBuilder::new().print_state(home_state, hall_state);

            search_recurse(dist + next_move.cost, home_state, hall_state, home_paths, hall_paths, min);

            //InputBuilder::new().print_state(home_state, hall_state);

            match next_move.to {
                Home(home_idx) => home_state[home_idx] = None,
                Hall(hall_idx) => hall_state[hall_idx] = None,
            }

             match next_move.from {
                Home(home_idx) => home_state[home_idx] = from,
                Hall(hall_idx) => hall_state[hall_idx] = from,
            };

            //InputBuilder::new().print_state(home_state, hall_state);
        }
    }

    let builder = InputBuilder::new();
    let (home_paths, hall_paths, mut home_state, mut hall_state) = builder.build();

    let mut answer = usize::MAX;
    search_recurse(0, &mut home_state, &mut hall_state, &home_paths, &hall_paths, &mut answer);

    // builder.print_state(&home_state, &hall_state);

    // let mut to_visit: BinaryHeap<SearchNode<usize, (Vec<Option<Pod>>, Vec<Option<Pod>>)>> = BinaryHeap::new();

    // to_visit.push(SearchNode { dist:0, data: (home_state, hall_state) });

    // let mut answer = 0;
    // let mut threshold = 100;
    // while let Some(current) = to_visit.pop() {
    //     // println!("{}", current.dist);
    //     if to_visit.len() > threshold {
    //         println!("{}", threshold);
    //         threshold *= 10;
    //     }

    //     //builder.print_state(&current.data.0, &current.data.1);

    //     if current.0.iter().all(|pod| matches!(pod, Some(p) if p.is_home)) {
    //         answer = current.dist;
    //         println!("break");
    //         break;
    //     }

    //     for next_states in generate_next_states(&home_paths, &hall_paths, current.data.0, current.data.1) {
    //         to_visit.push(SearchNode { dist: current.dist + next_states.0, data: (next_states.1, next_states.2) });
    //     }
    // }

    assert_eq!(answer, 17400);

    // // for path in home_paths {
    // //     print_recurse("".to_string(), &path);
    // // }

    // let next_states = generate_next_states(&home_paths, &hall_paths, home_state, hall_state);
    // for state in next_states {
    //     println!("{:?}", state);
    // }

    // for path in home_paths {
    //     print_recurse("".to_string(), &path);
    // }

    // for path in hall_paths {
    //     print_recurse("".to_string(), &path);
    // }
}