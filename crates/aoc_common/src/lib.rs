use std::fmt::{Debug, Write};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use num_traits::PrimInt;

pub use vec2::*;
pub use vec3::*;

mod vec2;
mod vec3;

pub fn file_string(path: &str) -> String {
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    buf
}

pub fn file_lines(path: &str) -> impl Iterator<Item = String> {
    let reader = BufReader::new(File::open(path).unwrap());
    return reader.lines().map(|l| l.unwrap());
}

pub fn file_lines_as<T>(path: &str) -> impl Iterator<Item = T> 
    where T: FromStr, <T as FromStr>::Err: Debug
{
    file_lines(path).map(|l| l.parse().expect("failed to parse line from file"))
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

pub fn full_permutations<T: Copy + Eq>(items: &[T]) -> Vec<Vec<T>> {
    let mut permutations: Vec<Vec<T>> = Vec::new();

    let mut stack: Vec<(Vec<T>, Vec<T>)> = Vec::new();
    stack.push((Vec::new(), items.to_vec()));

    while let Some((solution, candidates)) = stack.pop() {
        if candidates.len() == 0 {
            permutations.push(solution);
        } else {
            for cand in candidates.iter() {
                let next_solution = solution.iter().chain(std::iter::once(cand)).copied().collect();
                let next_candidates = candidates.iter().filter(|c| *c != cand).copied().collect();
                stack.push((next_solution, next_candidates));
            }
        }
    }

    permutations
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
        buff[(p.y - min.y).to_usize().unwrap()][(p.x - min.x).to_usize().unwrap()] = '???';
    }

    let mut s = String::new();
    for line in buff {
        writeln!(&mut s).unwrap();
        write!(&mut s, "{}", line.into_iter().collect::<String>()).unwrap();
    }

    s
}

pub struct SearchNode<T, U> {
    pub dist: T,
    pub data: U
}

impl<T, U> Deref for SearchNode<T, U> {
    type Target = U;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, U> DerefMut for SearchNode<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: PartialEq, U> PartialEq for SearchNode<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl<T: Eq, U> Eq for SearchNode<T, U> { }

impl<T: PartialOrd, U> PartialOrd for SearchNode<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl<T: Ord, U> Ord for SearchNode<T, U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

pub trait ToVec: Iterator
{
    fn to_vec(self) -> Vec<Self::Item>
        where Self: Sized
    {
        self.collect()
    }
}

impl<T: ?Sized> ToVec for T where T: Iterator { }