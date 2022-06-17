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