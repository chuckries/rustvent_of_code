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

    fn is_match<'a>(&self, other: &mut impl Iterator<Item = &'a Point>) -> Option<Point> {
        let mut counts: HashMap<Point, i32> = HashMap::new();

        for a in other {
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
            if let Some(offset) = rotated.is_match(&mut space.iter()) {
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