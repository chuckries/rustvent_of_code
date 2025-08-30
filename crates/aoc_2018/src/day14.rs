use std::{collections::VecDeque, ops::Index};

use aoc_common::IteratorExt;

const INPUT: usize = 793031;

struct Recipes {
    recipes: Vec<usize>,
    a: usize,
    b: usize,
}

impl Recipes {
    fn new() -> Self {
        Recipes { recipes: vec![3, 7], a: 0, b: 1 }
    }

    fn tick(&mut self) {
        let a_score = self.recipes[self.a];
        let b_score = self.recipes[self.b];
        
        let sum = a_score + b_score;
        
        if sum < 10 {
            self.recipes.push(sum);
        } else {
            let ones = sum % 10;
            let tens = sum / 10;
            self.recipes.push(tens);
            self.recipes.push(ones);
        }

        self.a = (self.a + a_score + 1) % self.recipes.len();
        self.b = (self.b + b_score + 1) % self.recipes.len();
    }

    fn len(&self) -> usize {
        self.recipes.len()
    }

    fn iter(&self) -> impl Iterator<Item = &usize> {
        self.recipes.iter()
    }
}

impl Index<usize> for Recipes {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.recipes[index]
    }
}

#[test]
fn part1() {
    let mut recipes = Recipes::new();

    while recipes.len() < INPUT + 10 {
        recipes.tick();
    }

    let num = recipes.iter().skip(INPUT).take(10).fold(0, |acc, n| acc * 10 + n);

    assert_eq!(4910101614, num);
}

#[test]
fn part2() {
    let mut recipes = Recipes::new();

    let mut target = VecDeque::new();
    let mut num = INPUT;
    while num > 0 {
        target.push_front(num % 10);
        num /= 10;
    }
    let target = target.into_iter().to_vec();

    let mut current_idx = 0;
    'outer: loop {
        for _ in 0 .. 1000 {
            recipes.tick();
        }

        while current_idx < recipes.len() - target.len() {
            if recipes.recipes[current_idx .. current_idx + target.len()] == target {
                break 'outer;
            }
            current_idx += 1;
        }
    }

    assert_eq!(20253137, current_idx);
}