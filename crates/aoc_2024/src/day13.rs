use aoc_common::{file_lines, IteratorExt, Vec2i64};

fn input() -> Vec<(Vec2i64, Vec2i64, Vec2i64)> {
    let mut lines = file_lines("inputs/day13.txt");

    let mut inputs = Vec::new();
    while let Some(line) = lines.next() {
        let split = line.split([' ', '+']).to_vec();
        let x = split[3].trim_end_matches(',').parse().unwrap();
        let y = split[5].parse().unwrap();
        let a = (x, y).into();

        let line = lines.next().unwrap();
        let split = line.split([' ', '+']).to_vec();
        let x = split[3].trim_end_matches(',').parse().unwrap();
        let y = split[5].parse().unwrap();
        let b = (x, y).into();

        let line = lines.next().unwrap();
        let split = line.split([' ', '=']).to_vec();
        let x = split[2].trim_end_matches(',').parse().unwrap();
        let y = split[4].parse().unwrap();
        let target = (x, y).into();

        inputs.push((a, b, target));

        _ = lines.next();
    }

    inputs
}

#[allow(non_snake_case)]
fn run(input: &[(Vec2i64, Vec2i64, Vec2i64)]) -> i64 {
    // A * ax + B * bx = tx
    // A * ay + B * by = ty

    // A * ax = tx - B * bx
    // A = (tx - B * bx) / ax

    // ((tx - B * bx) / ax) * ay + B * by = ty
    // (ay / ax) * (tx - B * bx) + B * by = ty
    // tx * (ay / ax) - (B * bx) * (ay /ax) + B * by = ty
    // B * by - B * bx * (ay / ax) = ty - tx * (ay / ax)
    // B * (by - bx * (ay /ax)) = ty - tx * (ay / ax)
    // B = (ty - tx * (ay / ax)) / (by - bx * (ay / ax))

    let mut total = 0;
    for (a, b, t) in input.iter().cloned() {
        let ay_ax: f64 = a.y as f64 / a.x as f64;

        let B = (t.y as f64 - t.x as f64 * ay_ax) / (b.y as f64 - b.x as f64 * ay_ax);
        let A = (t.x as f64 - B * b.x as f64) / a.x as f64;
        
        let A = A.round() as i64;
        let B = B.round() as i64;
        if a * A + b * B != t {
            continue;
        }

        total += 3 * A as i64 + B as i64;
    }

    total
}

#[test]
fn part1() {
    let input = input();
    let answer = run(&input);
    assert_eq!(answer, 29388);
}

#[test]
fn part2() {
    const ADD: i64 = 10_000_000_000_000;
    let mut input = input();
    for (_, _, t) in input.iter_mut() {
        t.x += ADD;
        t.y += ADD;
    }
    let answer: i64 = run(&input);
    assert_eq!(answer, 99548032866004);
}