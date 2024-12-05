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

#[test]
fn part1() {
    let (rules, updates) = input();

    let mut total = 0;
    'update: for update in updates {
        for i in 0..update.len() - 1 {
            for j in i + 1 .. update.len() {
                if rules.contains(&(update[j], update[i]).into()) {
                    continue 'update;
                }
            }
        }
        total += update[update.len() / 2];
    }

    assert_eq!(total, 4662);
}

#[test]
fn part2() {
    let (rules, updates) = input();

    let mut total = 0;
    for update in updates {
        let sorted = update.iter().copied().sorted_by(|lhs, rhs| {
            if rules.contains(&(*lhs, *rhs).into()) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).to_vec();
        if sorted != update {
            total += sorted[sorted.len() / 2];
        }
    }

    assert_eq!(total, 5900);
}