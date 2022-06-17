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