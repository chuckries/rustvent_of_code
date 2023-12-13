use aoc_common::{file_lines, IteratorExt, Vec2us};

fn input() -> Vec<Vec<char>> {
    file_lines("inputs/day11.txt").map(|l| l.chars().to_vec()).to_vec()
}

fn run(multiplier: usize) -> usize {
    let map = input();

    let mut rows = vec![false; map.len()];
    let mut cols = vec![false; map[0].len()];

    let mut points: Vec<Vec2us> = Vec::new();

    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == '#' {
                points.push((i, j).into());
                rows[j] = true;
                cols[i] = true;
            }
        }
    }

    fn create_transform(bools: Vec<bool>, multiplier: usize) -> Vec<usize> {
        let mut transform: Vec<usize> = Vec::with_capacity(bools.len());

        let mut idx = 0;
        for b in bools {
            transform.push(idx);
            idx += if b { 1 } else { multiplier };
        }

        transform
    }

    let row_transform = create_transform(rows, multiplier);
    let col_transform = create_transform(cols, multiplier);

    for p in points.iter_mut() {
        *p = (col_transform[p.x], row_transform[p.y]).into();
    }

    let mut total = 0;
    for i in 0 .. points.len() - 1 {
        for j in i + 1 .. points.len() {
            total += points[i].manhattan_from(points[j]);
        }
    }

    total
}

#[test]
fn part1() {
    let answer = run(2);
    assert_eq!(10494813, answer);
}

#[test]
fn part2() {
    let answer = run(1000000);
    assert_eq!(840988812853, answer);
}