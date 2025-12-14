use std::fmt::{Debug, Write};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::FromStr;

use num_traits::PrimInt;

pub use aabb::*;
pub use grid::*;
pub use grid2::*;
pub use id_map::*;
pub use iterator_ext::*;
pub use linear::*;
pub use ord_wrapper::*;
pub use priority_queue::*;
pub use rect::*;
pub use vec2::*;
pub use vec3::*;
pub use vecn::*;
pub use virtual_grid::*;

mod aabb;
mod grid;
mod grid2;
mod id_map;
mod iterator_ext;
mod linear;
mod ord_wrapper;
mod priority_queue;
mod rect;
mod vec2;
mod vec3;
mod vecn;
mod virtual_grid;

pub fn file_string(path: &str) -> String {
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    buf
}

pub fn file_lines(path: &str) -> impl Iterator<Item = String> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines().map(|l| l.unwrap())
}

pub fn file_lines_as<T>(path: &str) -> impl Iterator<Item = T> 
    where T: FromStr, <T as FromStr>::Err: Debug
{
    file_lines(path).map(|l| l.parse().expect("failed to parse line from file"))
}

pub fn file_line_blocks(path: &str) -> Vec<Vec<String>> {
    let mut iter = file_lines(path);
    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            blocks.push(current.clone());
            current.clear();
        } else {
            current.push(line);
        }
    }
    blocks.push(current);

    blocks
}

pub fn gcf<T: PrimInt>(mut a: T, mut b: T) -> T {
    loop {
        let remainder = a % b;
        if remainder == T::zero() {
            return b;
        }
        a = b;
        b = remainder;
    }
}

pub fn lcm<T: PrimInt>(nums: &[T]) -> T {
    nums.iter().copied().reduce(|accum, next| {
        if accum % next == T::zero() { 
            accum
        } else if next % accum == T::zero() {
            next
        } else {
            (accum * next) / gcf(accum, next)
        }
    }).unwrap()
}

pub fn full_permutations<'a, T>(items: &'a [T]) -> Vec<Vec<&'a T>> {
    fn recurse<'a, T>(solution: &mut Vec<&'a T>, candidates: &mut Vec<&'a T>, solutions: &mut Vec<Vec<&'a T>>, used: &mut [bool]) {

        let mut found = false;
        for i in 0..used.len() {
            if !used[i] {
                found = true;
                used[i] = true;
                solution.push(candidates[i]);

                recurse(solution, candidates, solutions, used);

                solution.pop();
                used[i] = false;
            }
        }

        if !found {
            solutions.push(solution.clone());
        }
    }

    let mut candidates: Vec<&T> = items.iter().collect();
    let mut solution: Vec<&T> = Vec::with_capacity(candidates.len());
    let mut solutions: Vec<Vec<&T>> = Vec::with_capacity(candidates.len() * candidates.len());
    let mut used = vec![false; candidates.len()];

    recurse(&mut solution, &mut candidates, &mut solutions, &mut used);

    solutions
}

pub fn map_points_to_string<T, U>(points: T) -> String 
    where T: Clone + Iterator<Item = Vec2<U>>, U: PrimInt
{
    let mut min = Vec2::new(U::max_value(), U::max_value());
    let mut max = Vec2::new(U::min_value(), U::min_value());

    for p in points.clone() {
        if p.x < min.x {
            min.x = p.x;
        }
        if p.x > max.x {
            max.x = p.x;
        }
        if p.y < min.y {
            min.y = p.y;
        }
        if p.y > max.y {
            max.y = p.y;
        }
    }

    let mut buff = vec![vec![' '; (max.x - min.x).to_usize().unwrap() + 1]; (max.y - min.y).to_usize().unwrap() + 1];
    for p in points {
        buff[(p.y - min.y).to_usize().unwrap()][(p.x - min.x).to_usize().unwrap()] = '█';
    }

    let mut s = String::new();
    for line in buff {
        writeln!(&mut s).unwrap();
        write!(&mut s, "{}", line.into_iter().collect::<String>()).unwrap();
    }

    s
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}