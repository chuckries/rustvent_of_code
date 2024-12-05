use std::{cmp::Ordering, collections::HashSet};

use aoc_common::{file_lines, IteratorExt, Vec2us};

fn input() -> (HashSet<Vec2us>, Vec<Vec<usize>>) {
    let mut lines = file_lines("inputs/day05.txt");

    let mut rules: HashSet<Vec2us> = HashSet::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let pages = line.split('|').map(|s| s.parse::<usize>().unwrap()).to_vec();
        rules.insert((pages[0], pages[1]).into());
    }

    let updates = lines.map(|l| l.split(',').map(|s| s.parse::<usize>().unwrap()).to_vec()).to_vec();

    (rules, updates)
}

fn is_sorted(list: &[usize], rules: &HashSet<Vec2us>) -> bool {
    list.is_sorted_by(|lhs, rhs| {
        rules.contains(&(*lhs, *rhs).into())
    })
}

#[test]
fn part1() {
    let (rules, updates) = input();
    let answer: usize = updates.into_iter()
        .filter(|l| is_sorted(&l, &rules))
        .map(|l| l[l.len() / 2])
        .sum();
    assert_eq!(answer, 4662);
}

#[test]
fn part2() {
    let (rules, updates) = input();

    let answer: usize = updates.into_iter()
        .filter(|l| !is_sorted(&l, &rules))
        .map(|mut l| {
            l.sort_by(|lhs, rhs| {
                if rules.contains(&(*lhs, *rhs).into()) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            l[l.len() / 2]
        }).sum();

    assert_eq!(answer, 5900);
}