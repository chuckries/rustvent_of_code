#![cfg(test)]

use intcode::*;

mod day1 {
    use aoc_common::file_lines_as;

    fn input() -> Vec<i32> {
        file_lines_as("inputs/day01.txt").collect()
    }

    #[test]
    fn part1() {
        let answer = input().into_iter().map(|l| l / 3 - 2).sum::<i32>();
        assert_eq!(answer, 3363929);
    }

    #[test]
    fn part2() {
        let answer = input().into_iter().map(|f| {

            let mut next = f / 3 - 2;
            let mut total = 0;
            while next > 0 {
                total += next;
                next = next / 3 - 2;
            }

            total
        }).sum::<i32>();

        assert_eq!(answer, 5043026);
    }
}

mod day2 {
    use crate::IntCode;

    #[test]
    fn part1() {
        let mut int_code = IntCode::from_file("inputs/day02.txt");

        let mem = int_code.mem_mut();
        mem[1] = 12;
        mem[2] = 2;
        int_code.run_to_halt().unwrap();

        let answer = int_code.mem()[0];
        assert_eq!(answer, 3931283);
    }

    const TARGET: i64 = 19690720;
    #[test]
    fn part2() {
        let mut int_code = IntCode::from_file("inputs/day02.txt");

        let mut answer = 0;
        'outer: for i in 0..100 {
            for j in 0..100 {
                int_code.reset();
                let mem = int_code.mem_mut();
                mem[1] = i;
                mem[2] = j;
                int_code.run_to_halt().unwrap();
                if int_code.mem()[0] == TARGET {
                    answer = 100 * i + j;
                    break 'outer;
                }
            }
        }

        assert_eq!(answer, 6979);
    }
}

mod day3 {
    use std::{collections::HashMap};

    use aoc_common::{Vec2i32, file_lines};

    struct Turn(Vec2i32, i32);

    fn input() -> [Vec<Turn>; 2] {
        let mut iter = file_lines("inputs/day03.txt").map(|l| {
            l.split(',').map(|t| {
                let (dir, num) = t.split_at(1);
                let dir = match dir {
                    "U" => -Vec2i32::unit_y(),
                    "D" => Vec2i32::unit_y(),
                    "L" => -Vec2i32::unit_x(),
                    "R" => Vec2i32::unit_x(),
                    _ => panic!()
                };

                let num = num.parse::<i32>().unwrap();
                Turn(dir, num)
            }).collect::<Vec<_>>()
        });

        [iter.next().unwrap(), iter.next().unwrap()]
    }

    fn map() -> HashMap<Vec2i32, (i32, [i32; 2])> {
        let mut map: HashMap<aoc_common::Vec2<i32>, (i32, [i32; 2])> = HashMap::new();

        for (id, turns) in input().into_iter().enumerate() {
            let mut pos = Vec2i32::zero();
            let mut count_steps = 0;
            for turn in turns {
                for _ in 0..turn.1 {
                    pos += turn.0;
                    count_steps += 1;
                    let entry = map.entry(pos).or_default();
                    let steps = &mut entry.1[id];
                    if *steps == 0 {
                        *steps = count_steps;
                    }
                    entry.0 |= (id + 1) as i32;
                }
            }
        }

        map
    }

    #[test]
    fn part1() {
        let answer = map().iter().filter_map(|p| {
            if p.1.0 == 3 {
                Some(i32::abs(p.0.x) + i32::abs(p.0.y))
            } else {
                None
            }
        }).min().unwrap();

        assert_eq!(answer, 248);
    }

    #[test]
    fn part2() {
        let answer = map().values().filter_map(|p| {
            if p.0 == 3 {
                Some(p.1[0] + p.1[1])
            } else {
                None
            }
        }).min().unwrap();

        assert_eq!(answer, 28580);
    }
}

mod day4 {
    fn increasing_digits() -> impl Iterator<Item = Vec<i32>> {
        (158126..=624574)
        .map(|n| {
            let mut digits = Vec::new();
            let mut current = n;
            while current > 0 {
                digits.push(current % 10);
                current /= 10;
            }
            digits
        })
        .filter(|digits| {
            digits.windows(2).all(|w| w[0] >= w[1])
        })
    }

    #[test]
    fn part1() {
        let answer = increasing_digits()
            .filter(|digits| {
                digits.windows(2).any(|w| w[0] == w[1])
            })
            .count();

        assert_eq!(answer, 1665);
    }

    #[test]
    fn part2(){
        let answer = increasing_digits()
            .filter(|digits| {
                let mut count = 1;
                let mut current = digits[0];
                for next in digits[1..].iter() {
                    if *next == current {
                        count += 1;
                    } else {
                        if count == 2 {
                            return true;
                        }
                        count = 1;
                        current = *next;
                    }
                }
                return count == 2;
            })
            .count();

        assert_eq!(answer, 1131);
    }
}

mod day5 {
    use crate::{IntCode};

    fn run(id: i64) -> i64 {
        let mut int_code = IntCode::from_file("inputs/day05.txt");
        let outputs = int_code.run_input_to_halt(&[id]).unwrap();

        *outputs.last().unwrap()
    }

    #[test]
    fn part1() {
        let answer = run(1);
        assert_eq!(answer, 8332629);
    }

    #[test]
    fn part2() {
        let answer = run(5);
        assert_eq!(answer, 8805067);
    }
}

mod day6 {
    use std::{collections::HashMap};

    use aoc_common::file_lines;

    type Map = HashMap<String, (String, Vec<String>)>;

    fn input() -> Map {
        let mut map = Map::new();

        for s in file_lines("inputs/day06.txt") {
            let tok = s.split(')').collect::<Vec<_>>();

            map.entry(tok[0].to_string()).or_default().1.push(tok[1].to_string());
            map.entry(tok[1].to_string()).or_default().0 = tok[0].to_string();
        }

        map
    }

    fn count(key: &str, map: &Map, depth: usize) -> usize {
        depth + if let Some((_, children)) = map.get(key) {
            children.iter().map(|c| count(c, map, depth + 1)).sum::<usize>()
        } else {
            0
        }
    }

    #[test]
    fn part1() {
        let answer = count("COM", &input(), 0);
        assert_eq!(answer, 417916);
    }

    #[test]
    fn part2() {
        let map = input();

        let mut distances: HashMap<&str, i32> = HashMap::new();
        let mut current = "YOU";
        let mut steps = 0;
        while let Some((parent, _)) = map.get(current) {
            distances.insert(parent, steps);
            current = parent;
            steps += 1;
        }

        current = &map.get("SAN").unwrap().0;
        steps = 0;
        loop {
            if let Some(dist) = distances.get(current) {
                steps += *dist;
                break;
            } else {
                current = &map.get(current).unwrap().0;
                steps += 1;
            }
        }

        assert_eq!(steps, 523);
    }

}

mod day7 {
    use aoc_common::full_permutations;

    use crate::{IntCode, IntCodeResult};

    fn run_once(phases: &[i64]) -> i64 {
        let mut computers = vec![IntCode::from_file("inputs/day07.txt"); 5];
        for pair in computers.iter_mut().zip(phases) {
            pair.0.push_input_back(*pair.1);
        }

        let mut result = 0;
        for c in computers.iter_mut() {
            result = c.run_input(&[result]).unwrap();
        }

        result
    }

    fn run_multiple(phases: &[i64]) -> i64 {
        let mut computers = vec![IntCode::from_file("inputs/day07.txt"); 5];
        for pair in computers.iter_mut().zip(phases) {
            pair.0.push_input_back(*pair.1);
        }

        let mut result = 0;
        while computers.iter().any(|c| !c.is_halt()) {
            for c in computers.iter_mut() {
                if let IntCodeResult::Output(o) = c.run_input(&[result]) {
                    result = o;
                }
            }
        }

        result
    }

    #[test]
    fn part1() {
        let permutations = full_permutations(&[0, 1, 2, 3, 4]);

        let mut max = 0;
        for current in permutations {
            let answer = run_once(&current);
            if answer > max {
                max = answer;
            }
        }

        assert_eq!(max, 17440);
    }

    #[test]
    fn part2() {
        let permutations = full_permutations(&[5, 6, 7, 8, 9]);

        let mut max = 0;
        for current in permutations {
            let answer = run_multiple(&current);
            if answer > max {
                max = answer;
            }
        }

        assert_eq!(max, 27561242);
    }
}

mod day8 {
    use aoc_common::{file_string, Vec2us, map_points_to_string};

    const DIMENSIONS: Vec2us = Vec2us::new(25, 6);
    const AREA: usize = DIMENSIONS.x * DIMENSIONS.y;

    fn input() -> Vec<u8> {
        file_string("inputs/day08.txt").into_bytes()
    }

    #[test]
    fn part1() {
        let frames = input();

        let min_frame = frames
            .chunks(AREA)
            .min_by_key(|frame| {
                frame.iter().filter(|c| **c == b'0').count()
            })
            .unwrap();

        let mut ones = 0;
        let mut twos = 0;
        for c in min_frame {
            match *c {
                b'1' => ones += 1,
                b'2' => twos += 1,
                _ => ()
            }
        }

        let answer = ones * twos;
        assert_eq!(answer, 2032);
    }

    #[test]
    fn part2() {
        let mut frames = input();

        let points = frames
            .chunks_mut(AREA)
            .reduce(|accum, frame| {
                accum.iter_mut().zip(frame).for_each(|(a, b)| { 
                    if *a == b'2' {
                        *a = *b
                    }
                 });
                accum
            })
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if *p == b'1' {
                    Some(Vec2us::new(i % DIMENSIONS.x, i / DIMENSIONS.x))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            ;

        let answer = map_points_to_string(points.into_iter());

        let known = "
 ██  ████  ██  █  █  ██ 
█  █ █    █  █ █  █ █  █
█    ███  █    █  █ █   
█    █    █    █  █ █ ██
█  █ █    █  █ █  █ █  █
 ██  █     ██   ██   ███";

         assert_eq!(answer, known);
    }
}

mod day9 {
    use crate::IntCode;

    fn run(val: i64) -> i64 {
        IntCode::from_file("inputs/day09.txt").run_input_to_halt(&[val]).unwrap()[0]
    }

    #[test]
    fn part1() {
        let answer = run(1);
        assert_eq!(answer, 2518058886);
    }

    #[test]
    fn part2() {
        let answer = run(2);
        assert_eq!(answer, 44292);
    }
}

mod day10 {
    use std::collections::HashSet;

    use aoc_common::{file_lines, Vec2i32, gcf};

    type Points = HashSet<Vec2i32>;

    fn input() -> Points {
        file_lines("inputs/day10.txt")
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Vec2i32::new(x as i32, y as i32))
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
            }).collect()
    }

    fn reduce(p: Vec2i32) -> Vec2i32 {
        let reduced = match (p.x, p.y) {
            (0, 0) => panic!(),
            (x, y) if x == 0 || y == 0 => (i32::signum(x), i32::signum(y)),
            (x, y) => {
                let gcf = gcf(i32::abs(x), i32::abs(y));
                (p / gcf).into()
            }
        };

        reduced.into()
    }

    fn station() -> (Vec2i32, Points) {
        let points = input();
        points.iter()
            .map(|cand| {
                let points = points.iter().filter(|p| **p != *cand).map(|p| {
                    reduce(*p - *cand)
                }).collect::<Points>();
                (*cand, points)
            })
            .max_by_key(|(_origin, in_sight)| in_sight.len())
            .unwrap()
    }

    #[test]
    fn part1() {
        let answer = station().1.len();
        assert_eq!(answer, 340);
    }

    #[test]
    fn part2() {
        let (station, in_sight) = station();
        assert!(in_sight.len() >= 200);

        let in_sight: Vec<Vec2i32> = in_sight.into_iter().collect();

        let mut in_sight = in_sight.into_iter().map(|p| {
            let quad = match (i32::signum(p.x), i32::signum(p.y)) {
                ( 0, -1) => 0,
                ( 1, -1) => 1,
                ( 1,  0) => 2,
                ( 1,  1) => 3,
                ( 0,  1) => 4,
                (-1,  1) => 5,
                (-1,  0) => 6,
                (-1, -1) => 7,
                _ => unreachable!()
            };

            (p, quad)
        }).collect::<Vec<_>>();

        in_sight.sort_by(|(a, a_quad), (b, b_quad)| {
            let mut sort_value = *a_quad - *b_quad;
            if sort_value == 0 {
                let left_top = i32::abs(a.y * b.x);
                let right_top = i32::abs(b.y * a.x);

                sort_value = if i32::signum(a.x) == i32::signum(a.y) {
                    left_top - right_top
                } else {
                    right_top - left_top
                }
            }
            match sort_value {
                0 => std::cmp::Ordering::Equal,
                i if i < 0 => std::cmp::Ordering::Less,
                i if i > 0 => std::cmp::Ordering::Greater,
                _ => unreachable!(),
            }
        });

        let answer = station + in_sight[199].0;
        let answer = answer.x * 100 + answer.y;
        assert_eq!(answer, 2628);
    }
}

mod day11 {
    use std::collections::{HashMap};

    use aoc_common::{Vec2i32, map_points_to_string};

    use crate::{IntCode, IntCodeResult};

    fn run(start: bool) -> HashMap<Vec2i32, bool> {
        let mut robot = IntCode::from_file("inputs/day11.txt");
        let mut map: HashMap<Vec2i32, bool> = HashMap::new();
        let mut pos = Vec2i32::zero();
        let mut dir = -Vec2i32::unit_y();

        map.insert(pos, start);

        loop {
            let panel = map.entry(pos).or_default();
            robot.push_input_back(if *panel { 1 } else { 0 });

            if let IntCodeResult::Output(output) = robot.run() {
                if output == 1 {
                    *panel = true;
                } else {
                    *panel = false;
                }

                let turn = robot.run().unwrap();
                dir = if turn == 0 {
                    dir.rotated_left()
                } else {
                    dir.rotated_right()
                };
                pos += dir;
            } else {
                break;
            }
        }

        map
    }

    #[test]
    fn part1() {
        let answer = run(false).len();
        assert_eq!(answer, 2018);
    }

    #[test]
    fn part2() {
        let map = run(true);

        let answer = map_points_to_string(map.iter().filter_map(|p| {
            if *p.1 {
                Some(p.0)
            } else {
                None
            }
        }).copied());

        let known = "
 ██  ███  ████ █  █ ███  █  █ ███  ███ 
█  █ █  █ █    █ █  █  █ █ █  █  █ █  █
█  █ █  █ ███  ██   █  █ ██   ███  █  █
████ ███  █    █ █  ███  █ █  █  █ ███ 
█  █ █    █    █ █  █ █  █ █  █  █ █ █ 
█  █ █    █    █  █ █  █ █  █ ███  █  █";

        assert_eq!(answer, known);
    }
}

mod day12 {
    use aoc_common::{Vec3i32, file_string, lcm, IteratorExt, Selector, SelectorMut};
    use lazy_static::lazy_static;
    use regex::Regex;

    const DIMS: [(Selector<i32>, SelectorMut<i32>); 3] = [
        (Vec3i32::x, Vec3i32::x_mut),
        (Vec3i32::y, Vec3i32::y_mut),
        (Vec3i32::z, Vec3i32::z_mut),
    ];

    fn input() -> Vec<Vec3i32> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
        }

        RE.captures_iter(&file_string("inputs/day12.txt")).map(|capture| {
            let mut num_iter = capture.iter().skip(1).map(|c| {
                c.unwrap().as_str().parse::<i32>().unwrap()
            });

            Vec3i32::new(num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap())
        }).collect()
    }

    fn run(iterations: usize, mut stopper: impl FnMut(&[Vec3i32], &[Vec3i32]) -> bool) -> (Vec<Vec3i32>, Vec<Vec3i32>) {
        let mut moons = input();
        let mut vels = vec![Vec3i32::zero(); moons.len()];

        #[inline]
        fn update_gravity(idx_a: usize, idx_b: usize, moons: &[Vec3i32], vels: &mut [Vec3i32], moon_selector: Selector<i32>, vel_selector: SelectorMut<i32>) {
            let a = moon_selector(&moons[idx_a]);
            let b = moon_selector(&moons[idx_b]);

            if a < b {
                *vel_selector(&mut vels[idx_a]) += 1;
                *vel_selector(&mut vels[idx_b]) -= 1;
            } else if a > b {
                *vel_selector(&mut vels[idx_a]) -= 1;
                *vel_selector(&mut vels[idx_b]) += 1;
            }
        }

        for _ in 0..iterations {
            for i in 0..moons.len() - 1 {
                for j in i..moons.len() {
                    for dim in DIMS {
                        update_gravity(i, j, &moons, &mut vels, dim.0, dim.1);
                    }
                }
            }

            moons.iter_mut().zip(vels.iter()).for_each(|(m, v)| { *m += *v });

            if stopper(&moons, &vels) {
                break;
            }
        }

        (moons, vels)
    }

    #[test]
    fn part1() {
        fn stopper(_: &[Vec3i32], _: &[Vec3i32]) -> bool { false }
        let (moons, vels) = run(1000, stopper);

        let answer = moons.into_iter().zip(vels).map(|(m, v)| {
            (i32::abs(m.x) + i32::abs(m.y) + i32::abs(m.z)) * (i32::abs(v.x) + i32::abs(v.y) + i32::abs(v.z))
        }).sum::<i32>();

        assert_eq!(answer, 7687);
    }

    struct Dim {
        selector: fn(&Vec3i32) -> i32,
        initial: Vec<(i32, i32)>,
        cycle: usize,
        found: bool
    }

    impl Dim {
        fn new(selector: fn(&Vec3i32) -> i32, input: &[Vec3i32]) -> Dim {
            let initial = input.iter().map(|p| (selector(p), 0)).collect();

            Dim {
                selector,
                initial,
                cycle: 0,
                found: false
            }
        }
    }

    #[test]
    fn part2() {
        let input = input();

        let mut dims = [
            Dim::new(Vec3i32::x, &input),
            Dim::new(Vec3i32::y, &input),
            Dim::new(Vec3i32::z, &input),
        ];

        let stopper = |moons: &[Vec3i32], vels: &[Vec3i32]| {
            dims.iter_mut().for_each(|dim| {
                if !dim.found {
                    dim.cycle += 1;

                    let state: Vec<(i32, i32)> = moons.iter().zip(vels)
                        .map(|(m, v)| {
                            ((dim.selector)(m), (dim.selector)(v))
                        }).collect();

                    if state == dim.initial {
                        dim.found = true;
                    }
                }
            });

            dims.iter().all(|d| d.found)
        };

        run(usize::MAX, stopper);

        let answer = lcm(&dims.into_iter().map(|d| d.cycle).to_vec());
        assert_eq!(answer, 334945516288044);
    }
}

mod day13 {
    use crate::{IntCode, IntCodeResult};

    #[test]
    fn part1() {
        let mut computer = IntCode::from_file("inputs/day13.txt");
        let blocks = computer
            .run_to_halt()
            .unwrap()
            .chunks(3)
            .filter(|i| { i[2] == 2 })
            .count();

        assert_eq!(blocks, 173);
    }

    #[test]
    fn part2() {
        let mut computer = IntCode::from_file("inputs/day13.txt");
        computer.mem_mut()[0] = 2;

        let mut ball = 0;
        let mut paddle = 0;
        let mut score = 0;

        loop {
            match computer.run() {
                IntCodeResult::Output(x) => {
                    let y = computer.run().unwrap();
                    let id = computer.run().unwrap();

                    if id == 4 {
                        ball = x;
                    } else if id == 3 {
                        paddle = x;
                    } else if x == -1 && y == 0 {
                        score = id;
                    }
                }
                IntCodeResult::Input => computer.push_input_back(i64::signum(ball - paddle)),
                IntCodeResult::Halt => break
            }
        }

        assert_eq!(score, 8942);
    }
}

mod day14 {
    use std::collections::HashMap;

    use aoc_common::{file_lines, IteratorExt};

    type Map = HashMap<String, Vec<(i64, i64, String)>>;

    struct Graph {
        map: Map
    }

    impl Graph {
        fn new() -> Self {
            let mut map = Map::new();

            for (left, right) in file_lines("inputs/day14.txt").map(|l| {
                let mut sides = l.split(" => ");
                let left = sides.next().unwrap().split(',').flat_map(|tok| tok.trim().split(' ')).map(|s| s.to_string()).to_vec();
                let right = sides.next().unwrap().split(' ').map(|s| s.to_string()).to_vec();
                (left, right)
            }) {
                let right_num = right[0].parse::<i64>().unwrap();
                let right = right[1].clone();
    
                for chunk in left.chunks(2) {
                    let left_num = chunk[0].parse::<i64>().unwrap();
                    let left = chunk[1].clone();
                    map.entry(left).or_default().push((left_num, right_num, right.clone()));
                }
            }

            Self {
                map
            }
        }

        fn get_total_required(&self, name: &str, required: i64) -> i64 {
            self.map.get(name).map_or(required, |deps| {
                deps.iter().map(|dep| {
                    let downstream = self.get_total_required(&dep.2, required);
                    let multiplier = downstream / dep.1 + i64::signum(downstream % dep.1);
                    dep.0 * multiplier
                }).sum()
            })
        }
    }

    #[test]
    fn part1() {
        let answer = Graph::new().get_total_required("ORE", 1);
        assert_eq!(answer, 443537);
    }

    #[test]
    fn part2() {
        let graph = Graph::new();

        let ore_per_fuel = graph.get_total_required("ORE", 1);
        let input_ore: i64 = 1000000000000;

        let mut lower_bound = input_ore / ore_per_fuel;
        let mut upper_bound = lower_bound * 2;

        let answer;
        loop {
            if lower_bound >= upper_bound {
                answer = lower_bound;
                break;
            }

            let half = lower_bound + ((upper_bound - lower_bound + 1) / 2);
            if graph.get_total_required("ORE", half) <= input_ore {
                lower_bound = half;
            } else {
                upper_bound = half - 1;
            }
        }

        assert_eq!(answer, 2910558);
    }
}

mod day15 {
    use std::collections::{HashSet};

    use aoc_common::{Vec2i64, PriorityQueue};

    use crate::IntCode;

    const DIRS: [Vec2i64; 4] = [
        Vec2i64::new( 0, -1),
        Vec2i64::new( 0,  1),
        Vec2i64::new(-1,  0),
        Vec2i64::new( 1,  0),
    ];
    const OPPOSITES: [i64; 4] = [2, 1, 4, 3];

    fn explore() -> (HashSet<Vec2i64>, Vec2i64) {
        let mut robot = IntCode::from_file("inputs/day15.txt");
        let mut map: HashSet<Vec2i64> = HashSet::new();
        let start = Vec2i64::zero();
        map.insert(start);
        let mut target = Vec2i64::zero();
        explore_recurse(&mut robot, &mut map, start, &mut target);
        (map, target)
    }

    fn explore_recurse(robot: &mut IntCode, map: &mut HashSet<Vec2i64>, pos: Vec2i64, target: &mut Vec2i64) {
        for cand in 0..4 {
            let new_pos = pos + DIRS[cand as usize];
            if map.contains(&new_pos) {
                continue;
            }

            match robot.run_input(&[cand + 1]).unwrap() {
                result @ (1 | 2) => {
                    if result == 2 {
                        *target = new_pos;
                    }

                    map.insert(new_pos);
                    explore_recurse(robot, map, new_pos, target);
                    robot.run_input(&[OPPOSITES[cand as usize]]).unwrap();
                }
                _ => ()
            }
        }
    }

    #[test]
    fn part1() {
        let (map, target) = explore();

        let mut to_visit: PriorityQueue<Vec2i64, usize> = PriorityQueue::new();
        to_visit.enqueue(Vec2i64::zero(), 0);

        let mut visited: HashSet<Vec2i64> = HashSet::new();

        let mut answer = 0;
        while let Some((current, dist)) = to_visit.dequeue_with_priority() {
            if current == target {
                answer = dist;
                break;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for adj in current.adjacent().filter(|adj| map.contains(adj)) {
                to_visit.enqueue(adj, dist + 1);
            }
        }

        assert_eq!(answer, 380);
    }

    #[test]
    fn part2() {
        let (map, target) = explore();
        let mut current = vec![target];
        let mut visited: HashSet<Vec2i64> = HashSet::new();

        let mut count = 0;
        loop {
            for p in current.drain(..).collect::<Vec<_>>() {
                visited.insert(p);
                for adj in p.adjacent().filter(|p|map.contains(p) && !visited.contains(p)) {
                    current.push(adj);
                }
            }

            if current.len() > 0 {
                count += 1;
            } else {
                break;
            }
        }

        assert_eq!(count, 410);
    }
}

mod day16 {
    use aoc_common::{file_string, IteratorExt};

    const BASE_SEQUENCE: [i32; 4] = [0, 1, 0, -1];

    struct Sequence {
        idx: usize,
        count: usize,
        current: usize
    }

    impl Sequence {
        fn new(count: usize) -> Self {
            Self {
                idx: 0,
                count,
                current: 0
            }
        }
    }

    impl Iterator for Sequence {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let result = BASE_SEQUENCE[self.idx];
            self.current += 1;
            if self.current == self.count {
                self.current = 0;
                self.idx += 1;
                if self.idx == BASE_SEQUENCE.len() {
                    self.idx = 0;
                }
            }

            Some(result)
        }
    }

    fn input() -> Vec<i32> {
        file_string("inputs/day16.txt").bytes().map(|b| (b - b'0') as i32).to_vec()
    }

    #[test]
    fn part1() {
        let mut current = input();

        println!("{}", current.len() * 10000);

        for _ in 0..100 {
            current = (0..current.len()).map(|i| {
                let seq = Sequence::new(i + 1).skip(1);
                i32::abs(current.iter().zip(seq).map(|(i, s)| i * s).sum::<i32>()) % 10
            }).to_vec();
        }

        let answer = current.into_iter().take(8).reduce(|accum, next| accum * 10 + next).unwrap();
        assert_eq!(answer, 84487724);
    }

    #[test]
    fn part2() {
        let input = input().into_iter().map(|i| i as u64).to_vec();
        let len = input.len();
        let pos = input.iter().take(7).copied().reduce(|accum, next| accum * 10 + next).unwrap() as usize;
        let mut current = input.into_iter().cycle().skip(pos).take(len * 10000 - pos).to_vec();

        for _ in 0..100 {
            let mut partial = current.iter().sum::<u64>();
            for c in current.iter_mut() {
                let previous = *c;
                *c = partial % 10;
                partial -= previous
            }
        }

        let answer = current.into_iter().take(8).reduce(|accum, next| accum * 10 + next).unwrap();
        assert_eq!(answer, 84692524);
    }
}

mod day17 {
    use aoc_common::{Vec2us, Vec2i64};

    use crate::{IntCode, IntCodeAscii, IntCodeResult};

    fn input() -> (IntCode, Vec<Vec<u8>>, Vec2us) {
        let mut computer = IntCode::from_file("inputs/day17.txt");
        let output = computer.run_to_halt().unwrap();

        let mut map: Vec<Vec<u8>> = vec![Vec::new()];
        let mut line = 0;
        let mut start = Vec2us::zero();
        for c in output {
            if c == b'\n'.into() {
                map.push(Vec::new());
                line += 1;
            } else {
                if c == b'^'.into() {
                    start = (map[line].len(), line).into();
                }
                map[line].push(c as u8);
            }
        }
        map.retain(|l| !l.is_empty());

        (computer, map, start)
    }

    #[test]
    fn part1() {
        let (_, map, _) = input();

        let mut total = 0;
        for j in 1..map.len() - 1 {
            for i in 1..map[0].len() - 1 {
                if map[j][i] == b'#' && Vec2us::new(i, j).adjacent().all(|p| {
                    map[p.y][p.x] == b'#'
                }) {
                    total += i * j;
                }
            }
        }

        assert_eq!(total, 3888);
    }

    #[test]
    fn part2() {
        let (mut computer, map, pos) = input();

        let bounds: Vec2i64 = Vec2us::new(map[0].len(), map.len()).cast();
        let mut pos: Vec2i64 = pos.cast();
        let mut dir = Vec2i64::new(0, -1);
        let mut path: Vec<i32> = Vec::new();
        loop {
            let left = pos + dir.rotated_left();
            if left.is_in_bounds(bounds) && map[left.y as usize][left.x as usize] == b'#' {
                path.push(-1);
                dir = dir.rotated_left();
            } else {
                let right = pos + dir.rotated_right();
                if right.is_in_bounds(bounds) && map[right.y as usize][right.x as usize] == b'#' {
                    path.push(-2);
                    dir = dir.rotated_right();
                } else {
                    break;
                }
            }

            let mut count = 0;

            loop {
                let next = pos + dir;
                if next.is_in_bounds(bounds) && map[next.y as usize][next.x as usize] == b'#' {
                    count += 1;
                    pos = next;
                } else {
                    break;
                }
            }

            if count != 0 {
                path.push(count);
            }
        }

        let mut patterns: Vec<&[i32]> = Vec::new();
        let solution = recurse(&path, &mut patterns, Vec::new()).unwrap();

        computer.reset();
        computer.mem_mut()[0] = 2;

        let program = solution.iter().map(|c| String::from_utf8([*c as u8 + b'A'].into()).unwrap()).collect::<Vec<String>>().join(",");
        let routines: Vec<String> = patterns.iter().map(|p| to_string(*p)).collect();

        computer.write_line(&program);
        for routine in routines {
            computer.write_line(&routine);
        }
        computer.write_line("n");

        let mut answer = 0;
        loop {
            match computer.run() {
                IntCodeResult::Output(o) => answer = o,
                IntCodeResult::Halt => break,
                _ => panic!(),
            }
        }

        assert_eq!(answer, 927809);
    }

    fn recurse<'a>(mut remaining: &'a [i32], patterns: &mut Vec<&'a [i32]>, mut partial_solution: Vec<usize>) -> Option<Vec<usize>> {
        'outer: loop {
            for (i, pattern) in patterns.iter().enumerate() {
                if pattern.len() <= remaining.len() && *pattern == &remaining[0..pattern.len()] {
                    remaining = &remaining[pattern.len()..];
                    partial_solution.push(i);
                    continue 'outer;
                }
            }
            break;
        }

        if remaining.len() == 0 {
            return Some(partial_solution);
        }

        if patterns.len() == 3 {
            return None;
        }

        for i in 1..remaining.len() {
            let cand = &remaining[0..i];
            if to_string(cand).len() > 20 {
                break;
            }

            partial_solution.push(patterns.len());
            patterns.push(cand);
            if let Some(solution) = recurse(&remaining[cand.len()..], patterns, partial_solution.clone()) {
                return Some(solution);
            }
            partial_solution.pop();
            patterns.pop();
        }

        None
    }

    fn to_string(chars: &[i32]) -> String {
        chars.iter().map(|c| {
            if *c == -1 {
                "L".to_owned()
            } else if *c == -2 {
                "R".to_owned()
            } else {
                c.to_string()
            }
        }).collect::<Vec<String>>().join(",")
    }

}

mod day18;

mod day19 {
    use aoc_common::{Vec2i64};

    use crate::IntCode;

    struct Calc {
        computer: IntCode
    }

    impl Calc {
        fn new() -> Calc {
            Calc {
                computer: IntCode::from_file("inputs/day19.txt")
            }
        }

        fn get(&mut self, x: i64, y: i64) -> i64 {
            let result = self.computer.run_input(&[x, y]).unwrap();
            self.computer.reset();
            result
        }
    }

    const AREA: i64 = 50;

    #[test]
    fn part1() {
        let mut calc = Calc::new();

        let answer = Vec2i64::new(AREA, AREA)
            .iter()
            .filter(|p| {
                calc.get(p.x, p.y) == 1
            })
            .count();

        assert_eq!(answer, 112);
    }

    #[test]
    fn part2() {
        let mut calc = Calc::new();

        let mut y = 40;
        let mut x_begin = 0;
        let mut x_end;

        while calc.get(x_begin, y) == 0 {
            x_begin +=1;
        }

        x_end = x_begin;
        while calc.get(x_end, y) == 1 {
            x_end += 1;
        }

        let answer: Vec2i64;
        loop {
            if x_end - x_begin >= 100 {
                let cand_x = x_end - 100;
                let cand_y = y + 100 - 1;
                if calc.get(cand_x, cand_y) == 1 {
                    answer = (cand_x, y).into();
                    break;
                }
            }

            y += 1;
            while calc.get(x_begin, y) == 0 {
                x_begin += 1;
            }
            while calc.get(x_end, y) == 1 {
                x_end += 1;
            }
        }
        let answer = answer.x * 10000 + answer.y;

        assert_eq!(answer, 18261982);
    }
}

mod day21 {
    use crate::{IntCode, IntCodeAscii, IntCodeResult};

    fn run_program(program: &[&str]) -> Result<i64, Vec<String>> {
        let mut droid = IntCode::from_file("inputs/day21.txt");

        for command in program {
            droid.write_line(command);
        }

        let mut output: Vec<String> = Vec::new();
        loop {
            match droid.read_line() {
                Ok(line) => output.push(line),
                Err(IntCodeResult::Output(o)) => return Ok(o),
                Err(IntCodeResult::Halt) => break,
                _ => panic!()
            }
        }

        return Err(output);
    }

    #[test]
    fn part1() {
        let answer = run_program(&[
            "NOT T T",
            "AND A T",
            "AND B T",
            "AND C T",
            "NOT T J",
            "AND D J",
            "WALK"
        ]).unwrap();

        assert_eq!(answer, 19358870);
    }

    #[test]
    fn part2() {
        let answer = run_program(&[
            "NOT T T",
            "AND A T",
            "AND B T",
            "AND C T",
            "NOT T J",
            "AND D J",
            "NOT E T",
            "NOT T T",
            "OR H T",
            "AND T J",
            "RUN"
        ]).unwrap();

        assert_eq!(answer, 1143356492);
    }
}

mod day23 {
    use std::collections::VecDeque;

    use aoc_common::Vec2i64;

    use crate::{IntCode, IntCodeResult};

    fn input() -> (Vec<IntCode>, Vec<VecDeque<Vec2i64>>) {
        let computer = IntCode::from_file("inputs/day23.txt");

        let mut network = vec![computer; 50];
        for (address, computer) in network.iter_mut().enumerate() {
            computer.push_input_back(address as i64);
        }

        let queues = vec![VecDeque::new(); network.len()];
        (network, queues)
    }

    #[test]
    fn part1() {
        let (mut network, mut queues) = input();
        let answer;
        'outer: loop {
            for (current, computer) in network.iter_mut().enumerate() {
                if let Some(next) = queues[current].pop_front() {
                    computer.push_input_back(next.x);
                    computer.push_input_back(next.y);
                } else {
                    computer.push_input_back(-1);
                }

                if let IntCodeResult::Output(dest) = computer.run() {
                    let x = computer.run().unwrap();
                    let y = computer.run().unwrap();
                    if dest == 255 {
                        answer = y;
                        break 'outer;
                    } else {
                        queues[dest as usize].push_back((x, y).into());
                    }
                }
            }
        }

        assert_eq!(answer, 22134);
    }

    #[test]
    fn part2() {
        let (mut network, mut queues) = input();
        let mut nat = Vec2i64::zero();
        let mut answer = 0;
        'outer: loop {
            for (current, computer) in network.iter_mut().enumerate() {
                if let Some(next) = queues[current].pop_front() {
                    computer.push_input_back(next.x);
                    computer.push_input_back(next.y);
                } else {
                    computer.push_input_back(-1);
                }

                if let IntCodeResult::Output(dest) = computer.run() {
                    let x = computer.run().unwrap();
                    let y = computer.run().unwrap();
                    if dest == 255 {
                        nat = (x, y).into();
                    } else {
                        queues[dest as usize].push_back((x, y).into());
                    }
                }
            }

            if queues.iter().all(|q| q.is_empty()) {
                if answer == nat.y {
                    break 'outer;
                }
                answer = nat.y;
                queues[0].push_back(nat);
            }
        }

        assert_eq!(answer, 16084);
    }
}

mod day25 {
    use crate::{IntCode, IntCodeAscii, IntCodeResult};

    #[test]
    fn part1() {
        let mut game = IntCode::from_file("inputs/day25.txt");

        for command in [
            "north",
            "take mutex",
            "south",
            "west",
            "take space law space brochure",
            "south",
            "take hologram",
            "west",
            "take manifold",
            "east",
            "north",
            "east",
            "south",
            "west",
            "south",
            "south",
            "south",
        ] {
            game.write_line(command);
        }

        let mut output = String::new();
        loop {
            match game.read_line() {
                Ok(line) => output = line,
                Err(IntCodeResult::Halt) => break,
                _ => panic!()
            }
        }

        let known = "\"Oh, hello! You should be able to get in by typing 262848 on the keypad at the main airlock.\"";
        assert_eq!(output, known);
    }
}