use aoc_common::{file_lines, IteratorExt, Vec2i64};

fn input() -> Vec<(Vec2i64, Vec2i64)> {
    file_lines("inputs/day14.txt").map(|l| {
        let split = l.split(['=', ',', ' ']).to_vec();
        let p = Vec2i64::parse(split[1], split[2]);
        let v = Vec2i64::parse(split[4], split[5]);
        (p, v)
    }).to_vec()
}

#[test]
fn part1() {
    let mut input = input();

    const BOUNDS: Vec2i64 = Vec2i64::new(101, 103);

    let counts = input.iter().map(|(p, _)| p).cloned().counts::<i64>();
    for j in 0..BOUNDS.y {
        for i in 0..BOUNDS.x {
            if let Some(count) = counts.get(&(i, j).into()) {
                print!("{}", count)
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    for _ in 0..100 {
        for (p, v) in input.iter_mut() {
            *p += *v;

            if p.x < 0 {
                p.x += BOUNDS.x;
            }
            if p.y < 0 {
                p.y += BOUNDS.y;
            }
            if p.x >= BOUNDS.x {
                p.x -= BOUNDS.x;
            }
            if p.y >= BOUNDS.y {
                p.y -= BOUNDS.y
            }
        }
    }

    let counts = input.iter().map(|(p, _)| p).cloned().counts::<i64>();
    for j in 0..BOUNDS.y {
        for i in 0..BOUNDS.x {
            if let Some(count) = counts.get(&(i, j).into()) {
                print!("{}", count)
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    let mut quads = [0; 4];
    for (p, _) in input {
        if p.x != BOUNDS.x / 2 && p.y != BOUNDS.y / 2 {
            let mut quad = 0;
            if p.x > BOUNDS.x / 2 {
                quad += 1;
            }
            if p.y > BOUNDS.y / 2 {
                quad += 2;
            }
            quads[quad] += 1;
        }
    }

    let answer = quads.into_iter().product::<i64>();
    assert_eq!(answer, 0);
}