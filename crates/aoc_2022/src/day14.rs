use aoc_common::{file_lines, IteratorExt, Vec2us};

fn input() -> (Vec<Vec<char>>, Vec2us) {
    let lines = file_lines("inputs/day14.txt").map(|l| {
        l.split(" -> ").map(|s| {
            let split = s.split(',').map(|i| i.parse::<usize>().unwrap()).to_vec();
            Vec2us::new(split[0], split[1])
        }).to_vec()
    }).to_vec();

    let bounds = Vec2us::bounds(lines.iter().flatten().copied()) + (10000, 1).into();

    let mut map = vec![vec!['.'; bounds.x]; bounds.y];

    for line in lines {
        for w in line.windows(2) {
            let mut p0 = w[0];
            let mut p1 = w[1];

            if p0.x == p1.x {
                if p0.y > p1.y {
                    (p0, p1) = (p1, p0);
                }

                for v in p0.y..=p1.y {
                    map[v][p0.x] = '#';
                }
            } else if p0.y == p1.y {
                if p0.x > p1.x {
                    (p0, p1) = (p1, p0);
                }

                for u in p0.x..=p1.x {
                    map[p0.y][u] = '#';
                }
            } else {
                panic!()
            }
        }
    }

    (map, bounds)
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

#[test]
fn part1() {
    let (mut map, bounds) = input();

    let origin: Vec2us = (500, 0).into();
    map[origin.y][origin.x] = '+';

    // print_map(&map);

    'outer: loop {
        let mut current = origin;

        loop {
            if current.y + 1 == bounds.y {
                break 'outer;
            }

            if map[current.y + 1][current.x] == '.' {
                current.y += 1;
                continue;
            }

            if map[current.y + 1][current.x - 1] == '.' {
                current.x -= 1;
                current.y += 1;
                continue;
            }

            if map[current.y + 1][current.x + 1] == '.' {
                current.x += 1;
                current.y += 1;
                continue;
            }

            map[current.y][current.x] = 'o';
            continue 'outer;
        }
    }

    //print_map(&map);

    let answer = map.iter().flatten().filter(|c| **c == 'o').count();
    assert_eq!(answer, 638);
}

#[test]
fn part2() {
    let (mut map, bounds) = input();

    map.push(vec!['#'; map[0].len()]);

    let origin: Vec2us = (500, 0).into();
    map[origin.y][origin.x] = '+';

    // print_map(&map);

    'outer: loop {
        let mut current = origin;

        loop {
            if map[current.y + 1][current.x] == '.' {
                current.y += 1;
                continue;
            }

            if map[current.y + 1][current.x - 1] == '.' {
                current.x -= 1;
                current.y += 1;
                continue;
            }

            if map[current.y + 1][current.x + 1] == '.' {
                current.x += 1;
                current.y += 1;
                continue;
            }

            map[current.y][current.x] = 'o';

            if current == origin {
                break 'outer;
            }

            continue 'outer;
        }
    }

    //print_map(&map);

    let answer = map.iter().flatten().filter(|c| **c == 'o').count();
    assert_eq!(answer, 31722);
}