use aoc_common::{Vec3i32, file_lines};

struct Game {
    id: i32,
    sets: Vec<Vec3i32>
}

fn input() -> Vec<Game> {
    file_lines("inputs/day02.txt").map(|l| {
        let mut split = l.split(": ");
        let id = split.next().unwrap().split(' ').skip(1).next().unwrap().parse::<i32>().unwrap();
        let sets = split.next().unwrap().split("; ").map(|s| {
            let mut cubes = s.split(", ").map(|c| c.split(' ')).flatten();

            let mut v = Vec3i32::zero();
            while let Some(num) = cubes.next() {
                let num = num.parse::<i32>().unwrap();
                let color = cubes.next().unwrap();

                match color {
                    "red" => v.x = num,
                    "green" => v.y = num,
                    "blue" => v.z = num,
                    _ => panic!(),
                };
            }

            v
        }).collect();

        Game {
            id,
            sets
        }
    }).collect()
}

#[test]
fn part1() {
    let answer = input().iter().filter_map(|p| {
        if p.sets.iter().all(|p| p.x <= 12 && p.y <= 13 && p.z <= 14) {
            Some(p.id)
        } else {
            None
        }}).sum();

    assert_eq!(2377, answer);
}

#[test]
fn part2() {
    let answer: i32 = input().iter().map(|g| {
        let mut min = Vec3i32::zero();
        for s in g.sets.iter() {
            if s.x > min.x { min.x = s.x; }
            if s.y > min.y { min.y = s.y; }
            if s.z > min.z { min.z = s.z; }
        }
        min.x * min.y * min.z
    }).sum();

    assert_eq!(71220, answer);
}