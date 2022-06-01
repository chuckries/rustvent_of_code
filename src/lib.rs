#![cfg(test)]

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::FromStr;

fn file_string(path: &str) -> String {
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    buf
}

fn file_lines(path: &str) -> impl Iterator<Item = String> {
    let reader = BufReader::new(File::open(path).unwrap());
    return reader.lines().map(|l| l.unwrap());
}

fn file_lines_as<T>(path: &str) -> impl Iterator<Item = T> 
    where T: FromStr, <T as FromStr>::Err: Debug
{
    file_lines(path).map(|l| l.parse().expect("failed to parse line from file"))
}

fn adjacent(p: (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj: Vec<(usize, usize)> = Vec::with_capacity(4);

    if p.0 > 0 { adj.push((p.0 - 1, p.1)); }
    if p.0 < bounds.0 - 1 { adj.push((p.0 + 1, p.1)); }
    if p.1 > 0 { adj.push((p.0, p.1 - 1)); }
    if p.1 < bounds.1 - 1 { adj.push((p.0, p.1 + 1)); }
    adj
}

mod day1 {
    use crate::file_lines_as;

    fn input() -> Vec<i32> {
        file_lines_as("inputs/day1.txt").collect()
    }

    #[test]
    fn part1() {
        let input = input();

        let count = input.windows(2).filter(|w| w[0] < w[1]).count();

        assert_eq!(count, 1342);
    }

    #[test]
    fn part2() {
        let input = input();

        let first = input.windows(3);
        let second = input[1..].windows(3);

        let count = first.zip(second).filter(|(l, r)| l.iter().sum::<i32>() < r.iter().sum::<i32>()).count();
        assert_eq!(count, 1378);
    }
}

mod day2 {
    use crate::file_lines;

    fn input() -> Vec<(String, i32)> {
        file_lines("inputs/day2.txt").map(|l| {
            let mut tok = l.split_whitespace();
            let dir = tok.next().unwrap().to_owned();
            let num = tok.next().unwrap().parse().unwrap();
            (dir, num)
        }).collect()
    }

    #[test]
    fn part1() {
        let mut pos = 0;
        let mut depth = 0;
        for (dir, num) in input() {
            match dir.as_str() {
                "forward" => pos += num,
                "down" => depth += num,
                "up" => depth -= num,
                _ => panic!()
            }
        }

        assert_eq!(pos * depth, 2102357);
    }

    #[test]
    fn part2() {
        let mut pos = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (dir, num) in input() {
            match dir.as_str() {
                "down" => aim += num,
                "up" => aim -= num,
                "forward" => {
                    pos += num;
                    depth += aim * num;
                }
                _ => panic!()
            }
        }

        assert_eq!(pos * depth, 2101031224);
    }
}

mod day3 {
    use crate::file_lines;

    fn input() -> Vec<Vec<char>> {
        file_lines("inputs/day3.txt").map(|l| l.chars().collect()).collect()
    }

    #[test]
    fn part1() {
        let input: Vec<Vec<char>> = input();
        let length = input[0].len();

        let mut gamma = 0;
        for bit in 0..length {
            let count = input.iter().filter(|l| l[bit] == '1').count();
            if count > input.len() / 2 {
                gamma |= 1 << length - bit - 1;
            }
        }

        let epsilon = !gamma & ((1 << length) - 1);

        assert_eq!(gamma * epsilon, 1307354);
    }

    #[test]
    fn part2() {
        let input = input();
        let mut oxygen: Vec<&[char]> = input.iter().map(|l| l.as_slice()).collect();
        let mut co2 = oxygen.clone();

        let len = oxygen[0].len();

        for bit in 0..len {
            if oxygen.len() > 1 {
                let ones = oxygen.iter().filter(|l| l[bit] == '1').count();
                let target = if ones >= oxygen.len() - ones {
                    '1'
                } else {
                    '0'
                };
                oxygen = oxygen.iter().filter_map(|l| if l[bit] == target { Some(*l) } else { None }).collect();
            }

            if co2.len() > 1 {
                let zeroes = co2.iter().filter(|l| l[bit] == '0').count();
                let target = if zeroes <= co2.len() - zeroes {
                    '0'
                } else {
                    '1'
                };
                co2 = co2.iter().filter_map(|l| if l[bit] == target { Some(*l) } else { None }).collect();
            }
        }

        let oxygen = oxygen[0].iter().collect::<String>();
        let co2 = co2[0].iter().collect::<String>();
        let oxygen = i32::from_str_radix(&oxygen, 2).unwrap();
        let c02 = i32::from_str_radix(&co2, 2).unwrap();

        assert_eq!(oxygen * c02, 482500);
    }
}

mod day4 {
    use crate::file_lines;

    struct Board {
        numbers: Vec<Vec<i32>>,
        marked: Vec<Vec<bool>>
    }

    impl Board {
        fn new(lines: &mut impl Iterator<Item = String>) -> Board {
            let mut rows: Vec<Vec<i32>> = Vec::new();
            for _ in 0..5 {
                let row: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
                rows.push(row);
            }

            let marked = vec![vec![false; 5]; 5];

            Board { numbers: rows, marked: marked }
        }

        fn call(&mut self, called: i32) {
            for (j, row) in self.numbers.iter().enumerate() {
                for (i , num) in row.iter().enumerate() {
                    if *num == called {
                        self.marked[j][i] = true;
                        return;
                    }
                }
            }
        }

        fn is_win(&self) -> bool {
            for i in 0..5 {
                let mut win = true;
                for j in 0..5 {
                    win = win && self.marked[j][i];
                }
                if win { return true; }
            }

            for j in 0..5 {
                let mut win = true;
                for i in 0..5 {
                    win = win && self.marked[j][i];
                }
                if win { return true; }
            }

            false
        }

        fn sum_unmarked(&self) -> i32 {
            let mut sum = 0;
            for (row_marks, row) in self.numbers.iter().zip(&self.marked) {
                sum += row.iter().zip(row_marks).filter_map(|(mark, num)| if *mark { None } else { Some (*num) }).sum::<i32>();
            }

            sum
        }
    }

    fn input() -> (Vec<i32>, Vec<Board>) {
        let mut lines = file_lines("inputs/day4.txt");

        let numbers: Vec<i32> = lines.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();

        let mut boards: Vec<Board> = Vec::new();
        while let Some(_) = lines.next() {
            boards.push(Board::new(&mut lines));
        }

        (numbers, boards)
    }

    #[test]
    fn part1() {
        let (numbers, mut boards) = input();

        'outer: for num in numbers {
            for board in boards.iter_mut() {
                board.call(num);

                if board.is_win() {
                    let answer = num * board.sum_unmarked();
                    assert_eq!(answer, 54275);
                    break 'outer;
                }
            }
        }
    }

    #[test]
    fn part2() {
        let (numbers, mut boards) = input();

        let mut last_win: Option<Board> = None;
        let mut last_num: i32 = 0;

        for num in numbers {
            for b in boards.iter_mut() {
                b.call(num);
            }

            boards = boards.into_iter().filter_map(|b| {
                if b.is_win() {
                    last_num = num;
                    last_win = Some(b);
                    return None
                }
                Some(b)
            }).collect();
        }

        let answer = last_num * last_win.unwrap().sum_unmarked();
        assert_eq!(answer, 13158);
    }
}

mod day5 {
    use std::collections::HashMap;

    use crate::file_lines;

    type Point = (i32, i32);

    fn input() -> Vec<(Point, Point)> {
        file_lines("inputs/day5.txt").map(|l| {
            let mut split = l.split(" -> ");
            let mut left = split.next().unwrap().split(',');
            let left = (left.next().unwrap().parse::<i32>().unwrap(), left.next().unwrap().parse::<i32>().unwrap());
            let mut right = split.next().unwrap().split(',');
            let right = (right.next().unwrap().parse::<i32>().unwrap(), right.next().unwrap().parse::<i32>().unwrap());
            (left, right)
        }).collect()
    }

    fn non_diagonals(lines: &Vec<(Point, Point)>, map: &mut HashMap<Point, i32>) {
        for ((x0, y0), (x1, y1)) in lines.iter().cloned() {
            if x0 == x1 {
                let range = if y0 < y1 { y0..=y1 } else { y1..=y0 };

                for i in range {
                    *map.entry((x0, i)).or_default() += 1;
                }
            } else if y0 == y1 {
                let range = if x0 < x1 { x0..=x1 } else { x1..= x0 };

                for i in range {
                    *map.entry((i, y0)).or_default() += 1;
                }
            }
        }
    }

    fn diagonals(lines: &Vec<(Point, Point)>, map: &mut HashMap<Point, i32>) {
        for ((x0, y0), (x1, y1)) in lines.iter().filter(|(p0, p1)| p0.0 != p1.0 && p0.1 != p1.1).cloned() {
            let (left, right) = if x0 < x1 { ((x0, y0), (x1, y1)) } else { ((x1, y1), (x0, y0)) };

            if left.1 < right.1 {
                for p in (left.0..=right.0).zip(left.1..=right.1) {
                    *map.entry(p).or_default() += 1;
                }
            } else {
                for p in (left.0..=right.0).zip((right.1..=left.1).rev()) {
                    *map.entry(p).or_default() += 1;
                }
            }

        }
    }

    #[test]
    fn part1() {
        let input = input();
        let mut map: HashMap<Point, i32>  = HashMap::new();

        non_diagonals(&input, &mut map);

        let answer = map.values().filter(|v| **v > 1).count();
        assert_eq!(answer, 5576)
    }

    #[test]
    fn part2() {
        let input = input();
        let mut map: HashMap<Point, i32> = HashMap::new();

        non_diagonals(&input, &mut map);
        diagonals(&input, &mut map);

        let answer = map.values().filter(|v| **v > 1).count();
        assert_eq!(answer, 18144);
    }
}

mod day6 {
    use crate::file_string;

    fn run(iterations: usize) -> usize {
        let input: Vec<usize> = file_string("inputs/day6.txt").split(',').map(|s| s.parse().unwrap()).collect();
    
        let mut fish = [0; 9];
        let mut next = fish.clone();

        for i in input {
            fish[i] += 1;
        }

        for _ in 0..iterations {
            for i in 0..8 {
                next[i] = fish[i + 1];
            }

            next[6] += fish[0];
            next[8] = fish[0];

            std::mem::swap(&mut fish, &mut next);
        }

        fish.iter().sum()
    }

    #[test]
    fn part1() {
        assert_eq!(run(80), 386755);
    }

    #[test]
    fn part2() {
        assert_eq!(run(256), 1732731810807);
    }
}

mod day7 {
    use crate::file_string;

    fn input() -> (Vec<i32>, i32, i32) {
        let input: Vec<i32> = file_string("inputs/day7.txt").split(',').map(|s| s.parse().unwrap()).collect();
    
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for i in input.iter() {
            if *i < min {
                min = *i;
            }
            if *i > max {
                max = *i;
            }
        }
        (input, min, max)
    }

    fn fuel_const(pos: i32, crabs: &[i32]) -> i32 {
        crabs.iter().map(|c| i32::abs(*c - pos)).sum()
    }

    fn fuel_variable(pos: i32, crabs: &[i32]) -> i32 {
        crabs.iter().map(|c| {
            let dist = i32::abs(*c - pos);
            (dist * (dist + 1)) / 2
        }).sum()
    }

    fn search<T>(fuel: T) -> i32 
        where T: Fn(i32, &[i32]) -> i32
    {
        let (input, min, max) = input();
        let mut min_fuel = i32::MAX;

        for i in min..max {
            let cost = fuel(i, &input);
            if cost < min_fuel {
                min_fuel = cost;
            }
        }

        min_fuel
    }

    #[test]
    fn part1() {
        assert_eq!(search(fuel_const), 329389);
    }

    #[test]
    fn part2() {
        assert_eq!(search(fuel_variable), 86397080);
    }
}

mod day8 {

}

mod day9 {
    use std::{collections::{HashSet, VecDeque}};

    use crate::{file_lines, adjacent};

    fn input() -> Vec<Vec<i32>> {
        file_lines("inputs/day9.txt")
            .map(|l| {
                l.bytes().map(|c| (c - b'0') as i32).collect()
            }).collect()
    }

    fn basins(map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
        let mut basins = Vec::new();
        let bounds = (map[0].len(), map.len());
        for j in 0..map.len() {
            for i in 0..map[0].len() {
                if adjacent((i, j), bounds).iter().all(|adj| {
                    map[adj.1][adj.0] > map[j][i]
                }) {
                    basins.push((i, j));
                }
            }
        }
        basins
    }

    #[test]
    fn part1() {
        let map = input();
        let risk: i32 = basins(&map).iter().map(|b| map[b.1][b.0] + 1).sum();

        assert_eq!(risk, 486);
    }

    #[test]
    fn part2() {
        let map = input();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();

        let mut sizes: Vec<usize> = Vec::new();

        let bounds = (map[0].len(), map.len());

        for basin in basins(&map) {
            visited.clear();
            to_visit.clear();

            to_visit.push_back(basin);
            while let Some(current) = to_visit.pop_front() {
                if visited.contains(&current) {
                    continue;
                }

                visited.insert(current.clone());

                for adj in adjacent(current, bounds).into_iter().filter(|adj| map[adj.1][adj.0] != 9) {
                    to_visit.push_back(adj);
                }
            }

            sizes.push(visited.len());
        }

        sizes.sort();
        let answer = sizes
            .iter()
            .rev()
            .take(3)
            .copied()
            .reduce(|accum, item| accum * item)
            .unwrap();

        assert_eq!(answer, 1059300);
    }
}

mod day10 {
    use crate::file_lines;

    fn input() -> Vec<String> {
        file_lines("inputs/day10.txt").collect()
    }

    fn process<C, I>(line: &str, corrupted: &mut C, incomplete: &mut I)
        where C: FnMut(char), I: FnMut(Vec<char>)
    {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => if *stack.last().unwrap() == '(' { stack.pop(); } else { corrupted(c); return; }
                ']' => if *stack.last().unwrap() == '[' { stack.pop(); } else { corrupted(c); return; }
                '}' => if *stack.last().unwrap() == '{' { stack.pop(); } else { corrupted(c); return; }
                '>' => if *stack.last().unwrap() == '<' { stack.pop(); } else { corrupted(c); return; }
                _ => panic!()
            }
        }

        incomplete(stack);
    }

    #[test]
    fn part1() {
        let lines = input();
        
        let mut total = 0;
        let mut corrupted = |c| { 
            total += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!()
            };
        };
        for line in lines {
            process(&line, &mut corrupted, &mut |_| { });
        }

        assert_eq!(total, 343863);
    }

    #[test]
    fn part2() {
        let lines = input();

        let mut scores: Vec<usize> = Vec::new();
        let mut incomplete = |stack: Vec<char>| {
            let mut current = 0;
            for c in stack.iter().rev() {
                let score = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!()
                };
                current = current * 5 + score;
            }
            scores.push(current);
        };

        for line in lines {
            process(&line, &mut |_| { }, &mut incomplete);
        }

        scores.sort();
        assert_eq!(scores[scores.len() / 2], 2924734236);
    }
}

mod day11 {
    use crate::file_lines;

    fn input() -> Vec<Vec<i32>> {
        file_lines("inputs/day11.txt")
            .map(|l| l.bytes().map(|b| (b - b'0') as i32).collect())
            .collect()
    }

    fn run(map: &mut Vec<Vec<i32>>) -> usize {
        let mut to_flash: Vec<(usize, usize)> = Vec::new();
            
        for j in 0..map.len() {
            for i in 0..map[0].len() {
                map[j][i] += 1;
                if map[j][i] == 10 {
                    to_flash.push((i, j));
                }
            }
        }

        let mut i = 0;
        while i < to_flash.len() {
            let (x, y) = to_flash[i];
            let mut adj: Vec<(usize, usize)> = Vec::new();
            if x > 0 && y > 0 {
                adj.push((x - 1, y - 1));
            }
            if y > 0 {
                adj.push((x, y - 1));
            }
            if x < map[0].len() - 1 && y > 0 {
                adj.push((x + 1, y - 1));
            }
            if x > 0 { 
                adj.push((x - 1, y));
            }
            if x < map[0].len() - 1 {
                adj.push((x + 1, y));
            }
            if x > 0 && y < map.len() - 1 {
                adj.push((x - 1, y + 1));
            }
            if y < map.len() - 1 {
                adj.push((x, y + 1));
            }
            if x < map[0].len() - 1 && y < map.len() - 1 {
                adj.push((x + 1, y + 1));
            }

            for (u, v) in adj {
                map[v][u] += 1;
                if map[v][u] == 10 {
                    to_flash.push((u, v));
                }
            }
            i += 1;
        }

        for (u, v) in &to_flash {
            map[*v][*u] = 0;
        }

        to_flash.len()
    }

    #[test]
    fn part1() {
        let mut map = input();

        let mut flashes = 0;
        for _ in 0..100 {
            flashes += run(&mut map);
        }

        assert_eq!(flashes, 1591);
    }

    #[test]
    fn part2() {
        let mut map = input();
        let target = map[0].len() * map.len();
        let mut iteration = 0;

        loop {
            iteration += 1;
            if run(&mut map) == target {
                break;
            }
        }

        assert_eq!(iteration, 314);
    }
}

mod day12 {
    use std::{collections::{HashMap, HashSet}};

    use crate::file_lines;

    #[derive(Hash, PartialEq, Eq, Clone, Copy)]
    enum Node<'a> {
        Start,
        End,
        Big(&'a str),
        Small(&'a str),
    }

    impl<'a> Node<'a> {
        fn new(name: &'a str) -> Node<'a> {
            if name == "start" {
                Node::Start
            } else if name == "end" {
                Node::End
            } else if name.chars().next().unwrap().is_uppercase() {
                Node::Big(name)
            } else {
                Node::Small(name)
            }
        }
    }

    fn backtrack<'a>(current: Node, visited: &mut HashSet<Node<'a>>, map: &'a HashMap<Node<'a>, Vec<Node<'a>>>) -> usize {
        let candidates = map.get(&current).unwrap();

        let mut total = 0;
        for cand in candidates {
            match cand {
                Node::End => total += 1,
                Node::Big(_) => total += backtrack(*cand, visited, map),
                Node::Small(_) if !visited.contains(cand) => {
                    visited.insert(*cand);
                    total += backtrack(*cand, visited, map);
                    visited.remove(cand);
                }
                _ => ()
            }
        }

        total
    }

    fn backtrack_multiple<'a>(current: Node, visited: &mut HashSet<Node<'a>>, map: &'a HashMap<Node<'a>, Vec<Node<'a>>>, double: Option<Node<'a>>) -> usize {
        let candidates = map.get(&current).unwrap();

        let mut total = 0;
        for cand in candidates {
            match cand {
                Node::End => total += 1,
                Node::Big(_) => total += backtrack_multiple(*cand, visited, map, double),
                Node::Small(_) => {
                    if double.is_some() {
                        if !visited.contains(cand) {
                            visited.insert(*cand);
                            total += backtrack_multiple(*cand, visited, map, double);
                            visited.remove(cand);
                        }
                    } else if visited.contains(cand) {
                        total += backtrack_multiple(*cand, visited, map, Some(*cand));
                    } else {
                        visited.insert(*cand);
                        total += backtrack_multiple(*cand, visited, map, None);
                        visited.remove(cand);
                    }
                }
                _ => ()
            }
        }

        total
    }

    fn input() -> Vec<String> {
        file_lines("inputs/day12.txt").collect()
    }

    fn mapify<'a>(input: &'a Vec<String>) -> HashMap<Node<'a>, Vec<Node<'a>>> {
        let mut map: HashMap<Node, Vec<Node>> = HashMap::new();
        for line in input.iter() {
            let tok: Vec<&str> = line.split('-').collect();

            let left = Node::new(tok[0]);
            let right = Node::new(tok[1]);

            map.entry(left).or_default().push(right);
            map.entry(right).or_default().push(left);
        }

        map
    }

    #[test]
    fn part1() {
        let input = input();
        let map = mapify(&input);
        let mut visited: HashSet<Node> = HashSet::new();
        let answer = backtrack(Node::Start, &mut visited, &map);

        assert_eq!(answer, 3292);
    }

    #[test]
    fn part2() {
        let input = input();
        let map = mapify(&input);
        let mut visited: HashSet<Node> = HashSet::new();
        let answer = backtrack_multiple(Node::Start, &mut visited, &map, None);

        assert_eq!(answer, 89592);
    }
}

mod day13 {
    use std::{collections::HashSet, fmt::Write};

    use crate::file_lines;

    #[derive(Copy, Clone, PartialEq)]
    enum Fold {
        X(usize),
        Y(usize),
    }

    fn input() -> (HashSet<(usize, usize)>, Vec<Fold>) {
        let mut lines = file_lines("inputs/day13.txt");

        let mut paper: HashSet<(usize, usize)> = HashSet::new();
        let mut folds: Vec<Fold> = Vec::new();
        
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() { break; }

            let mut nums = line.split(',').map(|n| n.parse::<usize>().unwrap());
            paper.insert((nums.next().unwrap(), nums.next().unwrap()));
        }

        while let Some(line) = lines.next() {
            let mut tok = line.split(' ').skip(2).next().unwrap().split('=');
            let dim = tok.next().unwrap();
            let num = tok.next().unwrap().parse::<usize>().unwrap();
            let fold = match dim {
                "x" => Fold::X(num),
                "y" => Fold::Y(num),
                _ => panic!()
            };
            folds.push(fold);
        }

        (paper, folds)
    }

    fn fold_paper(paper: &mut HashSet<(usize, usize)>, fold: &Fold) {
        let points: Vec<(usize, usize)> = paper.drain().collect();

        match fold {
            Fold::X(n) => {
                for p in points {
                    if p.0 > *n {
                        let folded = (n - (p.0 - n), p.1);
                        paper.insert(folded);
                    } else {
                        paper.insert(p);
                    }
                }
            }
            Fold::Y(n) => {
                for p in points {
                    if p.1 > *n {
                        let folded = (p.0, n - (p.1 - n));
                        paper.insert(folded);
                    } else {
                        paper.insert(p);
                    }
                }
            }
        }
    }

    #[test]
    fn part1() {
        let (mut paper, folds) = input();

        fold_paper(&mut paper, &folds[0]);
        assert_eq!(paper.len(), 759);
    }

    #[test]
    fn part2() {
        let (mut paper, folds) = input();

        for fold in folds {
            fold_paper(&mut paper, &fold);
        }

        let mut min = (usize::MAX, usize::MAX);
        let mut max = (usize::MIN, usize::MIN);
        for p in paper.iter() {
            if p.0 < min.0 {
                min.0 = p.0;
            }
            if p.0 > max.0 {
                max.0 = p.0;
            }
            if p.1 < min.1 {
                min.1 = p.1;
            }
            if p.1 > max.1 {
                max.1 = p.1;
            }
        }

        let mut buff = vec![vec![' '; max.0 - min.0 + 1]; max.1 - min.1 + 1];
        for (x, y) in paper {
            buff[y - min.1][x - min.0] = '█';
        }

        let mut s = String::new();
        for line in buff {
            writeln!(&mut s).unwrap();
            write!(&mut s, "{}", line.into_iter().collect::<String>()).unwrap();
        }

        let answer = "
█  █ ████  ██  ███  ████ █  █ ███  ███ 
█  █ █    █  █ █  █    █ █ █  █  █ █  █
████ ███  █    █  █   █  ██   █  █ █  █
█  █ █    █    ███   █   █ █  ███  ███ 
█  █ █    █  █ █ █  █    █ █  █    █ █ 
█  █ ████  ██  █  █ ████ █  █ █    █  █";

        assert_eq!(s, answer);
    }
}

mod day14 {
    use std::{collections::HashMap};

    use crate::file_lines;

    fn input() -> (Vec<u8>, HashMap<[u8; 2], u8>) {
        let mut lines = file_lines("inputs/day14.txt");
        let start = lines.next().unwrap().into_bytes();

        let mut lines = lines.skip(1);

        let mut map: HashMap<[u8; 2], u8> = HashMap::new();
        while let Some(line) = lines.next() {
            let mut tok = line.split(" -> ");
            let key_bytes = tok.next().unwrap().as_bytes();
            let key = [key_bytes[0], key_bytes[1]];
            let value = tok.next().unwrap().as_bytes()[0];
            map.insert(key, value);
        }

        (start, map)
    }

    fn run(iterations: i32) -> usize {
        let (start, rules) = input();

        let mut counts: HashMap<u8, usize> = HashMap::new();
        let mut pairs: HashMap<[u8; 2], usize> = HashMap::new();

        for c in start.iter() {
            *counts.entry(*c).or_default() += 1;
        }

        for p in start.windows(2) {
            *pairs.entry([p[0], p[1]]).or_default() += 1;
        }

        for _ in 0..iterations {
            let current: Vec<_> = pairs.drain().collect();

            for (pair, count) in current {
                let next = rules.get(&pair).unwrap();
                *counts.entry(*next).or_default() += count;
                *pairs.entry([pair[0], *next]).or_default() += count;
                *pairs.entry([*next, pair[1]]).or_default() += count;
            }
        }

        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for (_, c) in counts {
            if c < min { min = c; }
            if c > max { max = c; }
        }

        max - min
    }

    #[test]
    fn part1() {
        let answer = run(10);
        assert_eq!(answer, 2587);
    }

    #[test]
    fn part2() {
        let answer = run(40);
        assert_eq!(answer, 3318837563123);
    }
}

mod day15 {
    use std::{collections::{BinaryHeap, HashSet}};

    use crate::{file_lines, adjacent};

    fn input() -> Vec<Vec<usize>> {
        file_lines("inputs/day15.txt").map(|l| {
            l.bytes().map(|b| (b - b'0' - 1) as usize).collect()
        }).collect()
    }

    struct Search {
        p: (usize, usize),
        d: usize
    }

    impl PartialEq for Search {
        fn eq(&self, other: &Self) -> bool {
            self.p.eq(&other.p)
        }
    }

    impl Eq for Search { }

    impl PartialOrd for Search {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.d.partial_cmp(&self.d)
        }
    }

    impl Ord for Search {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.d.cmp(&self.d)
        }
    }

    fn search(map: &Vec<Vec<usize>>) -> usize {
        let mut to_visit: BinaryHeap<Search> = BinaryHeap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let bounds = (map[0].len(), map.len());
        let target = (bounds.0 - 1, bounds.1 - 1);

        to_visit.push(Search { p: (0, 0), d: 0 });

        while !to_visit.is_empty() {
            let current = to_visit.pop().unwrap();

            if current.p == target {
                return current.d;
            }

            if visited.contains(&current.p) {
                continue;
            }
            visited.insert(current.p);

            for adj in adjacent(current.p, bounds) {
                if visited.contains(&adj) {
                    continue;
                }
                to_visit.push(Search { p: adj, d: current.d + map[adj.1][adj.0] + 1});
            }
        }

        panic!();
    }

    #[test]
    fn part1() {
        let map = input();
        let answer = search(&map);
        assert_eq!(answer, 595);
    }

    #[test]
    fn part2() {
        let start = input();
        let start_bounds = (start[0].len(), start.len());
        let bounds = (start_bounds.0 * 5, start_bounds.1 * 5);

        let mut map = vec![vec![0; bounds.0]; bounds.1];

        for x in 0usize..5 {
            for y in 0usize..5 {
                for u in 0..start[0].len() {
                    for v in 0..start.len() {
                        map[y * start_bounds.1 + v][x * start_bounds.0 + u] = (start[v][u] + x + y) % 9;
                    }
                }
            }
        }

        let answer = search(&map);
        assert_eq!(answer, 2914);
    }
}

mod day16 {
    use crate::file_string;

    struct PacketReader {
        bytes: Vec<u8>,
        bit_idx: usize,
        data_idx: usize,
        position: usize,
    }

    impl PacketReader {
        fn new(input: &str) -> PacketReader {
            let mut bytes: Vec<u8> = Vec::with_capacity(input.len() / 2);

            for chunk in input.as_bytes().chunks(2) {
                let byte = (Self::hex_to_byte(chunk[0]) << 4) |
                    (Self::hex_to_byte(chunk[1]));
                bytes.push(byte);
            }

            PacketReader {
                bytes,
                bit_idx: 7,
                data_idx: 0,
                position: 0,
            }
        }

        fn read_packet(&mut self) -> Packet {
            let version = self.read_bits(3);
            let type_id = self.read_bits(3);

            let mut packet = Packet {
                version,
                type_id,
                literal: 0,
                subs: Vec::new()
            };

            if type_id == 4 {
                let literal = self.read_literal();
                packet.literal = literal;
            } else {
                self.read_subpackets(&mut packet);
            }

            packet
        }

        fn read_subpackets(&mut self, packet: &mut Packet) {
            let length_type_id = self.read_bits(1);
            if length_type_id == 0 {
                let bit_count = self.read_bits(15);
                let offset = self.position + bit_count;
                while self.position < offset {
                    packet.subs.push(self.read_packet());
                }
            } else {
                let count = self.read_bits(11);
                for _ in 0..count {
                    packet.subs.push(self.read_packet());
                }
            }
        }

        fn read_literal(&mut self) -> usize {
            let mut literal = 0;
            
            loop {
                let chunk = self.read_bits(5);
                literal = (literal << 4) | (chunk & 0x0F);
                if (chunk & 0x10) == 0 {
                    break;
                }
            }

            literal
        }

        fn read_bits(&mut self, count: usize) -> usize {
            let mut result: usize = 0;

            for _ in 0..count {
                result = (result << 1) | ((self.bytes[self.data_idx] as usize & (1 << self.bit_idx)) >> self.bit_idx);
                if self.bit_idx == 0 {
                    self.data_idx += 1;
                    self.bit_idx = 7;
                } else {
                    self.bit_idx -= 1;
                }
            }

            self.position += count;
            result
        }

        fn hex_to_byte(hex: u8) -> u8 {
            if hex <= b'9' {
                hex - b'0'
            } else {
                hex - b'A' + 10
            }
        }
    }

    struct Packet {
        version: usize,
        type_id: usize,
        literal: usize,
        subs: Vec<Packet>
    }

    impl Packet {
        fn sum_versions(&self) -> usize {
            self.version + self.subs.iter().map(|s| s.sum_versions()).sum::<usize>()
        }

        fn evaluate(&self) -> usize {
            match self.type_id {
                0 => self.subs.iter().map(|s| s.evaluate()).sum::<usize>(),
                1 => self.subs.iter().map(|s| s.evaluate()).reduce(|accum, item| accum * item).unwrap(),
                2 => self.subs.iter().map(|s| s.evaluate()).min().unwrap(),
                3 => self.subs.iter().map(|s| s.evaluate()).max().unwrap(),
                4 => self.literal,
                5 => if self.subs[0].evaluate() > self.subs[1].evaluate() { 1 } else { 0 },
                6 => if self.subs[0].evaluate() < self.subs[1].evaluate() { 1 } else { 0 },
                7 => if self.subs[0].evaluate() == self.subs[1].evaluate() { 1 } else { 0 },
                _ => panic!()
            }
        }
    }

    fn input() -> Packet {
        let mut reader = PacketReader::new(&file_string("inputs/day16.txt"));
        reader.read_packet()
    }

    #[test]
    fn part1() {
        let answer = input().sum_versions();
        assert_eq!(answer, 893);
    }

    #[test]
    fn part2() {
        let answer = input().evaluate();
        assert_eq!(answer, 4358595186090);
    }
}

mod day18 {
    use std::str::{Chars, FromStr};

    use crate::{file_lines_as};

    type Tree = Box<TreeNode>;

    #[derive(Debug, Clone)]
    enum TreeNode {
        Literal(usize),
        Pair(Tree, Tree)
    }

    enum ExplodeResult {
        Exploded,
        Imm(usize, usize),
        Left(usize),
        Right(usize),
    }

    enum SplitResult {
        Splitted,
        Imm(usize, usize)
    }

    impl TreeNode {
        fn as_literal(&self) -> Option<usize> {
            if let TreeNode::Literal(n) = self {
                Some(*n)
            } else {
                None
            }
        }

        fn magnitude(&self) -> usize {
            match self {
                TreeNode::Literal(n) => *n,
                TreeNode::Pair(l, r) => {
                    3 * l.magnitude() + 2 * r.magnitude()
                }
            }
        }

        fn add(&self, rhs: &Self) -> Tree {
            let mut sum = Tree::new(TreeNode::Pair(Tree::new(self.clone()), Tree::new(rhs.clone())));
            sum.reduce();
            sum
        }

        fn reduce(&mut self) {
            loop {
                if self.try_explode() {
                    continue;
                }

                if self.try_split() {
                    continue;
                }

                break;
            }
        }

        fn try_explode(&mut self) -> bool {
            if self.try_explode_recurse(0).is_some() {
                true
            } else {
                false
            }
        }

        fn try_explode_recurse(&mut self, depth: usize) -> Option<ExplodeResult> {
            if let TreeNode::Pair(left, right) = self {
                if depth == 4 {
                    return Some(ExplodeResult::Imm(left.as_literal().unwrap(), right.as_literal().unwrap()));
                } else {
                    if let Some(result) = left.try_explode_recurse(depth + 1) {
                        return Some(match result {
                            ExplodeResult::Imm(l, r) => {
                                *left = Tree::new(TreeNode::Literal(0));
                                right.add_left(r);
                                ExplodeResult::Left(l)
                            }
                            ExplodeResult::Right(n) => {
                                right.add_left(n);
                                ExplodeResult::Exploded
                            }
                            _ => result
                        });
                    }

                    if let Some(result) = right.try_explode_recurse(depth + 1) {
                        return Some(match result {
                            ExplodeResult::Imm(l, r) => {
                                *right = Tree::new(TreeNode::Literal(0));
                                left.add_right(l);
                                ExplodeResult::Right(r)
                            }
                            ExplodeResult::Left(n) => {
                                left.add_right(n);
                                ExplodeResult::Exploded
                            }
                            _ => result
                        });
                    }
                }
            }

            None
        }

        fn add_left(&mut self, num: usize) {
            match self {
                TreeNode::Literal(n) => *n += num,
                TreeNode::Pair(l, _) => l.add_left(num)
            }
        }

        fn add_right(&mut self, num: usize) {
            match self {
                TreeNode::Literal(n) => *n += num,
                TreeNode::Pair(_, r) => r.add_right(num)
            }
        }

        fn try_split(&mut self) -> bool {
            if self.try_split_recurse().is_some() {
                true
            } else {
                false
            }
        }

        fn try_split_recurse(&mut self) -> Option<SplitResult> {
            match self {
                TreeNode::Literal(n) => {
                    if *n >= 10 {
                        let div = *n / 2;
                        let rem = *n % 2;
                        return Some(SplitResult::Imm(div, div + rem));
                    }
                }
                TreeNode::Pair(l, r) => {
                    if let Some(result) = l.try_split_recurse() {
                        return Some(if let SplitResult::Imm(l_split, r_split) = result {
                            *l = Tree::new(TreeNode::Pair(Tree::new(TreeNode::Literal(l_split)), Tree::new(TreeNode::Literal(r_split))));
                            SplitResult::Splitted
                        } else {
                            result
                        });
                    }

                    if let Some(result) = r.try_split_recurse() {
                        return Some(if let SplitResult::Imm(l_split, r_split) = result {
                            *r = Tree::new(TreeNode::Pair(Tree::new(TreeNode::Literal(l_split)), Tree::new(TreeNode::Literal(r_split))));
                            SplitResult::Splitted
                        } else {
                            result
                        });
                    }
                }
            }

            None
        }

        fn treeify(chars: &mut Chars<'_>) -> Tree {
            let c = chars.next().unwrap();
    
            if c == '[' {
                let left = Self::treeify(chars);
                chars.next().unwrap();
                let right = Self::treeify(chars);
                chars.next().unwrap();
                Tree::new(TreeNode::Pair(left, right))
            } else {
                Tree::new(TreeNode::Literal(c.to_digit(10).unwrap() as usize))
            }
        }
    }

    impl FromStr for Tree {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let tree = TreeNode::treeify(&mut s.chars());
            Ok(tree)
        }
    }

    fn input() -> Vec<Tree> {
        file_lines_as("inputs/day18.txt").collect()
    }

    #[test]
    fn part1() {
        let input = input();
        
        let sum = input.iter().skip(1).fold(input[0].clone(), |accum, i| accum.add(i));

        let answer = sum.magnitude();
        assert_eq!(answer, 3359);
    }

    #[test]
    fn part2() {
        let input = input();

        let mut max = 0;
        for i in 0..input.len() {
            for j in 0..input.len() {
                if i == j { continue; }

                let mag = input[i].add(&input[j]).magnitude();
                if mag > max {
                    max = mag;
                }
            }
        }

        assert_eq!(max, 4616);
    }

}