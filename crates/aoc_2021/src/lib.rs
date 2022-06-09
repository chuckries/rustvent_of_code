
#![cfg(test)]

mod day1 {
    use aoc_common::file_lines_as;

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
    use aoc_common::file_lines;

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
    use aoc_common::file_lines;

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
    use aoc_common::file_lines;

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

    use aoc_common::file_lines;

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
    use aoc_common::file_string;

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
    use aoc_common::file_string;

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

    fn search(fuel: fn(i32, &[i32]) -> i32) -> i32 {
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

    use aoc_common::{file_lines, Vec2us};

    fn input() -> Vec<Vec<i32>> {
        file_lines("inputs/day9.txt")
            .map(|l| {
                l.bytes().map(|c| (c - b'0') as i32).collect()
            }).collect()
    }

    fn basins(map: &Vec<Vec<i32>>) -> Vec<Vec2us> {
        let mut basins = Vec::new();
        let bounds: Vec2us = (map[0].len(), map.len()).into();

        for p in bounds.iter() {
            if p.adjacent_bounded(&bounds).all(|adj| {
                map[adj.y][adj.x] > map[p.y][p.x]
            }) {
                basins.push(p);
            }
        }

        basins
    }

    #[test]
    fn part1() {
        let map = input();
        let risk: i32 = basins(&map).iter().map(|b| map[b.y][b.x] + 1).sum();

        assert_eq!(risk, 486);
    }

    #[test]
    fn part2() {
        let map = input();
        let mut visited: HashSet<Vec2us> = HashSet::new();
        let mut to_visit: VecDeque<Vec2us> = VecDeque::new();

        let mut sizes: Vec<usize> = Vec::new();

        let bounds = Vec2us::new(map[0].len(), map.len());

        for basin in basins(&map) {
            visited.clear();
            to_visit.clear();

            to_visit.push_back(basin);
            while let Some(current) = to_visit.pop_front() {
                if visited.contains(&current) {
                    continue;
                }

                visited.insert(current.clone());

                for adj in current.adjacent_bounded(&bounds).filter(|adj| map[adj.y][adj.x] != 9) {
                    to_visit.push_back(adj);
                }
            }

            sizes.push(visited.len());
        }

        sizes.sort_by(|a, b| b.cmp(a));
        let answer = sizes
            .into_iter()
            .take(3)
            .reduce(|accum, item| accum * item)
            .unwrap();

        assert_eq!(answer, 1059300);
    }
}

mod day10 {
    use aoc_common::file_lines;

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
    use aoc_common::{file_lines, Vec2us};

    fn input() -> Vec<Vec<i32>> {
        file_lines("inputs/day11.txt")
            .map(|l| l.bytes().map(|b| (b - b'0') as i32).collect())
            .collect()
    }

    fn run(map: &mut Vec<Vec<i32>>) -> usize {
        let mut to_flash: Vec<Vec2us> = Vec::new();
        let bounds = Vec2us::new(map[0].len(), map.len());

        for p in bounds.iter() { 
            map[p.y][p.x] += 1;
            if map[p.y][p.x] == 10 {
                to_flash.push(p);
            }
        }

        let mut i = 0;
        while i < to_flash.len() {
            let p = to_flash[i];

            for adj in p.surrouding_bounded(&bounds) {
                map[adj.y][adj.x] += 1;
                if map[adj.y][adj.x] == 10 {
                    to_flash.push(adj);
                }
            }
            i += 1;
        }

        for p in &to_flash {
            map[p.y][p.x] = 0;
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

    use aoc_common::file_lines;

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
    use std::{collections::HashSet};

    use aoc_common::{file_lines, map_points_to_string, Vec2us};

    #[derive(Copy, Clone, PartialEq)]
    enum Fold {
        X(usize),
        Y(usize),
    }

    fn input() -> (HashSet<Vec2us>, Vec<Fold>) {
        let mut lines = file_lines("inputs/day13.txt");

        let mut paper: HashSet<Vec2us> = HashSet::new();
        let mut folds: Vec<Fold> = Vec::new();
        
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() { break; }

            let mut nums = line.split(',').map(|n| n.parse::<usize>().unwrap());
            paper.insert((nums.next().unwrap(), nums.next().unwrap()).into());
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

    fn fold_paper(paper: &mut HashSet<Vec2us>, fold: &Fold) {
        let points: Vec<_> = paper.drain().collect();

        match fold {
            Fold::X(n) => {
                for p in points {
                    if p.x > *n {
                        let folded = (n - (p.x - n), p.y).into();
                        paper.insert(folded);
                    } else {
                        paper.insert(p);
                    }
                }
            }
            Fold::Y(n) => {
                for p in points {
                    if p.y > *n {
                        let folded = (p.x, n - (p.y - n)).into();
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

        let answer = map_points_to_string(paper.iter().copied());

        let known = "
█  █ ████  ██  ███  ████ █  █ ███  ███ 
█  █ █    █  █ █  █    █ █ █  █  █ █  █
████ ███  █    █  █   █  ██   █  █ █  █
█  █ █    █    ███   █   █ █  ███  ███ 
█  █ █    █  █ █ █  █    █ █  █    █ █ 
█  █ ████  ██  █  █ ████ █  █ █    █  █";

        assert_eq!(answer, known);
    }
}

mod day14 {
    use std::{collections::HashMap};

    use aoc_common::file_lines;

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

    use aoc_common::{file_lines, Vec2us, SearchNode};

    fn input() -> Vec<Vec<usize>> {
        file_lines("inputs/day15.txt").map(|l| {
            l.bytes().map(|b| (b - b'0' - 1) as usize).collect()
        }).collect()
    }

    fn search(map: &Vec<Vec<usize>>) -> usize {
        let mut to_visit: BinaryHeap<SearchNode<usize, Vec2us>> = BinaryHeap::new();
        let mut visited: HashSet<Vec2us> = HashSet::new();

        let bounds = Vec2us::new(map[0].len(), map.len());
        let target = Vec2us::new(bounds.x - 1, bounds.y - 1);

        to_visit.push(SearchNode { dist: 0, data: Vec2us::zero() });

        while !to_visit.is_empty() {
            let current = to_visit.pop().unwrap();

            if current.data == target {
                return current.dist;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.data);

            for adj in current.adjacent_bounded(&bounds) {
                if visited.contains(&adj) {
                    continue;
                }
                to_visit.push(SearchNode { dist: current.dist + map[adj.y][adj.x] + 1, data: adj });
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
    use aoc_common::file_string;

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

    use aoc_common::{file_lines_as};

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

mod day19 {
    use std::{collections::{HashMap, HashSet, VecDeque}};

    use aoc_common::file_lines;

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
    struct Point(i32, i32, i32);

    impl Point {
        fn rotate(&self, rot: i32) -> Self {
            match rot {
                0 =>  Point( self.0,  self.1,  self.2),
                1 =>  Point(-self.2,  self.1,  self.0),
                2 =>  Point(-self.0,  self.1, -self.2),
                3 =>  Point( self.2,  self.1, -self.0),
 
                4 =>  Point( self.0, -self.2,  self.1),
                5 =>  Point(-self.1, -self.2,  self.0),
                6 =>  Point(-self.0, -self.2, -self.1),
                7 =>  Point( self.1, -self.2, -self.0),
 
                8 =>  Point( self.0,  self.2, -self.1),
                9 =>  Point( self.1,  self.2,  self.0),
                10 => Point(-self.0,  self.2,  self.1),
                11 => Point(-self.1,  self.2, -self.0),

                12 => Point(-self.0, -self.1,  self.2),
                13 => Point(-self.2, -self.1, -self.0),
                14 => Point( self.0, -self.1, -self.2),
                15 => Point( self.2, -self.1,  self.0),

                16 => Point(-self.1,  self.0,  self.2),
                17 => Point(-self.2,  self.0, -self.1),
                18 => Point( self.1,  self.0, -self.2),
                19 => Point( self.2,  self.0,  self.1),

                20 => Point( self.1, -self.0,  self.2),
                21 => Point(-self.2, -self.0,  self.1),
                22 => Point(-self.1, -self.0, -self.2),
                23 => Point( self.2, -self.0, -self.1),

                _ => panic!()
            }
        }
    }

    impl std::ops::Add for Point {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }

    impl std::ops::Add for &Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }

    impl std::ops::Sub for Point {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
        }
    }

    impl std::ops::Sub for &Point {
        type Output = Point;

        fn sub(self, rhs: Self) -> Self::Output {
            Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
        }
    }

    struct Scanner(Vec<Point>);

    impl Scanner {
        fn rotate(&self, rot: i32) -> Self {
            Scanner(self.0.iter().map(|p| p.rotate(rot)).collect())
        }

        fn is_match<'a>(&self, mut other: impl Iterator<Item = &'a Point>) -> Option<Point> {
            let mut counts: HashMap<Point, i32> = HashMap::new();

            for a in other.by_ref() {
                for b in self.0.iter() {
                    let offset = a - b;
                    let count = counts.entry(offset).or_default();
                    if *count == 11 {
                        return Some(offset);
                    } else {
                        *count += 1;
                    }
                }
            }

            None
        }
    }

    fn input() -> Vec<Scanner> {
        let mut lines = file_lines("inputs/day19.txt");

        let mut scanners: Vec<Scanner> = Vec::new();
        let mut points: Vec<Point> = Vec::new();

        lines.next().unwrap();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                scanners.push(Scanner(points.drain(..).collect()));
                lines.next().unwrap();
                continue;
            }

            let mut tok = line.split(',').map(|s| s.parse::<i32>().unwrap());
            points.push(Point(tok.next().unwrap(), tok.next().unwrap(), tok.next().unwrap()));
        }
        scanners.push(Scanner(points));

        scanners
    }

    fn run() -> (HashSet<Point>, Vec<Point>) {
        let mut input = input();

        let mut space: HashSet<Point> = HashSet::new();
        space.extend(input.pop().unwrap().0.into_iter());

        let mut candidates: VecDeque<Scanner> = input.into();
        let mut offsets: Vec<Point> = Vec::new();
        'outer: while !candidates.is_empty() {
            let current = candidates.pop_front().unwrap();

            for rot in 0..24 {
                let rotated = current.rotate(rot);
                if let Some(offset) = rotated.is_match(space.iter()) {
                    space.extend(rotated.0.into_iter().map(|p| p + offset));
                    offsets.push(offset);
                    continue 'outer;
                }
            }

            candidates.push_back(current);
        }

        (space, offsets)
    }

    #[test]
    fn part1() {
        let (space, _) = run();

        let answer = space.len();
        assert_eq!(answer, 440);
    }

    #[test]
    fn part2() {
        let (_, offsets) = run();

        let mut max = 0;

        for i in 0..offsets.len() - 1 {
            for j in i + 1..offsets.len() {
                let a = &offsets[i];
                let b = &offsets[j];

                let dist = i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1) + i32::abs(a.2 - b.2);
                if dist > max {
                    max = dist
                }
            }
        }

        assert_eq!(max, 13382);
    }
}

mod day20 {
    use std::collections::HashSet;

    use aoc_common::file_lines;

    fn input() -> (Vec<char>, Vec<Vec<char>>) {
        let mut lines = file_lines("inputs/day20.txt");

        let algo: Vec<char> = lines.next().unwrap().chars().collect();

        lines.next().unwrap();
        let start: Vec<Vec<char>> = lines.map(|l| {
            l.chars().collect()
        }).collect();

        (algo, start)
    }

    fn run(iterations: i32) -> usize {
        let (algo, start) = input();

        let mut bounds = ((0, 0), ((start[0].len() - 1) as i32, (start.len() - 1) as i32));

        let mut canvas: HashSet<(i32, i32)> = HashSet::new();

        for (j, cj) in start.into_iter().enumerate() {
            for (i, ci) in cj.into_iter().enumerate() {
                if ci == '#' {
                    canvas.insert((i as i32, j as i32));
                }
            }
        }

        let is_toggle = algo[0] == '#';
        let mut toggle = false;
        for _ in 0..iterations {
            let mut next: HashSet<(i32, i32)> = HashSet::new();

            for j in bounds.0.1 - 1..=bounds.1.1 + 1 {
                for i in bounds.0.0 - 1..=bounds.1.0 + 1 {
                    let mut idx = 0;
                    for v in j - 1..=j + 1 {
                        for u in i - 1..=i + 1 {
                            let is_set = if u < bounds.0.0 || u > bounds.1.0 || v < bounds.0.1 || v > bounds.1.1 {
                                toggle
                            } else {
                                canvas.contains(&(u, v))
                            };

                            idx = (idx << 1) | if is_set { 1 } else { 0 };
                        }
                    }
                    if algo[idx] == '#' {
                        next.insert((i, j));
                    }
                }
            }

            std::mem::swap(&mut next, &mut canvas);
            next.clear();

            if is_toggle {
                toggle = !toggle;
            }

            bounds = ((bounds.0.0 - 1, bounds.0.1 - 1), (bounds.1.0 + 1, bounds.1.1 + 1));
        }

        canvas.len()
    }

    #[test]
    fn part1() {
        let answer = run(2);

        assert_eq!(answer, 5663);
    }

    #[test]
    fn part2() {
        let answer = run(50);

        assert_eq!(answer, 19638);
    }
}

mod day21 {
    const A_START: usize = 8;
    const B_START: usize = 5;

    struct Dice {
        current: usize
    }

    impl Dice {
        fn new() ->Dice {
            Dice {
                current: 1
            }
        }
    }

    impl Iterator for Dice {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.current;
            self.current += 1;
            if self.current > 100 {
                self.current = 1;
            }

            Some(result)
        }
    }

    #[test] 
    fn part1() {
        let mut dice = Dice::new();
        let mut state = [(0usize, 0usize); 2];
        state[0] = (A_START - 1, 0);
        state[1] = (B_START - 1, 0);

        let mut rolls = 0;
        let answer;
        'outer: loop {
            for turn in 0..2 {
                let roll: usize = dice.by_ref().take(3).sum();
                rolls += 3;

                let (pos, score) = &mut state[turn];
                *pos += roll;
                *pos %= 10;
                *score += *pos + 1;

                if *score >= 1000 {
                    answer = state[turn ^ 1].1 * rolls;
                    break 'outer;
                }
            }
        }

        assert_eq!(answer, 597600);
    }

    #[test]
    fn part2() {
        let mut wins = [0usize; 2];
        let mut positions = [[[[[0usize; 2]; 10] ; 10]; 21]; 21];
        positions[0][0][A_START - 1][B_START - 1][0] = 1;

        for a_score in 0..21 {
            for b_score in 0..21 {
                for a_pos in 0..10 {
                    for b_pos in 0..10 {
                        for turn in 0..2 {
                            for i in 1..=3 {
                                for j in 1..=3 {
                                    for k in 1..=3 {
                                        let count = positions[a_score][b_score][a_pos][b_pos][turn];
                                        if count == 0 {
                                            continue;
                                        }

                                        let (mut pos, mut score) = if turn == 0 {
                                            (a_pos, a_score)
                                        } else {
                                            (b_pos, b_score)
                                        };

                                        pos += i + j + k;
                                        pos %= 10;

                                        score += pos + 1;
                                        if score >= 21 {
                                            wins[turn] += count;
                                        } else {
                                            if turn == 0 {
                                                positions[score][b_score][pos][b_pos][1] += count;
                                            } else {
                                                positions[a_score][score][a_pos][pos][0] += count;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        assert_eq!(usize::max(wins[0], wins[1]), 634769613696613);
    }
}

mod day22 {
    use std::str::FromStr;

    use aoc_common::file_lines_as;

    #[derive(Clone, Copy)]
    struct Point(i64, i64, i64);

    struct Cube {
        points: (Point, Point),
        status: bool
    }

    impl Cube {
        fn new(status: bool, lo: Point, hi: Point) -> Cube {
            Cube {
                points: (lo, hi),
                status
            }
        }

        fn volume(&self) -> i64 {
            (self.points.1.0 - self.points.0.0 + 1) *
            (self.points.1.1 - self.points.0.1 + 1) *
            (self.points.1.2 - self.points.0.2 + 1)
        }

        fn add_to_space(self, space: &mut Vec<Cube>) {

            let existing: Vec<Cube> = space.drain(..).collect();

            let (a_lo, a_hi) = self.points;
            for mut cube in existing {
                let (b_lo, b_hi) = &mut cube.points;

                if a_lo.0 <= b_lo.0 && a_lo.1 <= b_lo.1 && a_lo.2 <= b_lo.2 && 
                   a_hi.0 >= b_hi.0 && a_hi.1 >= b_hi.1 && a_hi.2 >= b_hi.2 {
                       continue;
                }

                if a_hi.0 < b_lo.0 || a_lo.0 > b_hi.0 ||
                   a_hi.1 < b_lo.1 || a_lo.1 > b_hi.1 ||
                   a_hi.2 < b_lo.2 || a_lo.2 > b_hi.2 {
                       space.push(cube);
                       continue;
                }

                if b_lo.0 < a_lo.0 {
                    space.push(Cube::new(cube.status, *b_lo, Point(a_lo.0 - 1, b_hi.1, b_hi.2)));
                    b_lo.0 = a_lo.0;
                }

                if b_hi.0 > a_hi.0 {
                    space.push(Cube::new(cube.status, Point(a_hi.0 + 1, b_lo.1, b_lo.2), *b_hi));
                    b_hi.0 = a_hi.0;
                }

                if b_lo.1 < a_lo.1 {
                    space.push(Cube::new(cube.status, *b_lo, Point(b_hi.0, a_lo.1 -1, b_hi.2)));
                    b_lo.1 = a_lo.1;
                }

                if b_hi.1 > a_hi.1 {
                    space.push(Cube::new(cube.status, Point(b_lo.0, a_hi.1 + 1, b_lo.2), *b_hi));
                    b_hi.1 = a_hi.1;
                }

                if b_lo.2 < a_lo.2 {
                    space.push(Cube::new(cube.status, *b_lo, Point(b_hi.0, b_hi.1, a_lo.2 - 1)));
                    b_lo.2 = a_lo.2;
                }

                if b_hi.2 > a_hi.2 {
                    space.push(Cube::new(cube.status, Point(b_lo.0, b_lo.1, a_hi.2 + 1), *b_hi));
                    b_hi.2 = a_hi.2;
                }
            }

            space.push(self);
        }
    }

    impl FromStr for Cube {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let tok: Vec<&str> = s.split(&[' ', '=', ',']).map(|s| s.split("..")).flatten().collect();

            let status = if tok[0] == "on" { true } else { false };

            let to_point = |a: &str, b: &str, c: &str| {
                Point(a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap(), c.parse::<i64>().unwrap())
            };

            let lo = to_point(tok[2], tok[5], tok[8]);
            let hi = to_point(tok[3], tok[6], tok[9]);

            Ok(Cube::new(status, lo, hi))
        }
    }

    fn input() -> Vec<Cube> {
        file_lines_as("inputs/day22.txt").collect()
    }

    #[test]
    fn part1() {
        let input = input();

        let mut space: Vec<Cube> = Vec::new();
        for cube in input.into_iter().filter(|c| {
            c.points.0.0 >= -50 && c.points.0.1 >= -50 && c.points.0.2 >= -50 &&
            c.points.1.0 <=  50 && c.points.1.1 <=  50 && c.points.1.2 <=  50
        }) {
            cube.add_to_space(&mut space);
        }

        let answer = space.into_iter().filter_map(|c| {
            if c.status == true {
                Some(c.volume())
            } else {
                None
            }
        }).sum::<i64>();

        assert_eq!(answer, 588120);
    }

    #[test]
    fn part2() {
        let input = input();

        let mut space: Vec<Cube> = Vec::new();
        for cube in input {
            cube.add_to_space(&mut space);
        }

        let answer = space.into_iter().filter_map(|c| {
            if c.status == true {
                Some(c.volume())
            } else {
                None
            }
        }).sum::<i64>();

        assert_eq!(answer, 1134088247046731);
    }
}