use aoc_common::{file_lines, IteratorExt, Vec2i32};

enum Dir {
    Walk(i32),
    Turn(char),
}

fn input() -> (Vec<Vec<char>>, Vec<Dir>, Vec2i32, Vec2i32) {
    let mut lines = file_lines("inputs/day22.txt");

    let mut map: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        map.push(line.chars().to_vec());
    }

    let start_x = map[0].iter().enumerate().find(|(_, c)| **c == '.').unwrap().0;
    let start_pos = Vec2i32::new(start_x as i32, 0);
    let start_dir = Vec2i32::unit_x();

    let mut dirs: Vec<Dir> = Vec::new();
    let line = lines.next().unwrap().chars().to_vec();
    let mut start = 0;
    let mut current = 0;

    while start < line.len() {
        if current == line.len() || matches!(line[current], 'L' | 'R') {
            dirs.push(Dir::Walk(line[start..current].iter().collect::<String>().parse().unwrap()));
            if current < line.len() {
                dirs.push(Dir::Turn(line[current]));
            }
            current += 1;
            start = current;
        } else {
            current += 1;
        }
    }

    (map, dirs, start_pos, start_dir)
}

#[test]
fn part1() {
    let (map, dirs, mut pos, mut dir) = input();

    for d in dirs {
        match d {
            Dir::Turn(c) => {
                dir = match c {
                    'L' => dir.rotate_left(),
                    'R' => dir.rotate_right(),
                    _ => panic!(),
                };
            },
            Dir::Walk(steps) => {
                for _ in 0..steps {
                    let mut next = pos + dir;

                    if next.y < 0 || next.y >= map.len() as i32 || next.x < 0 || next.x >= map[next.y as usize].len() as i32 || map[next.y as usize][next.x as usize] == ' ' {
                        let mut rev = pos;
                        loop {
                            let next_rev = rev - dir;
                            if next_rev.y < 0 || next_rev.y >= map.len() as i32 || next_rev.x < 0 || next_rev.x >= map[next_rev.y as usize].len() as i32 || map[next_rev.y as usize][next_rev.x as usize] == ' ' {
                                break;
                            }
                            rev = next_rev;
                        }
                        next = rev;
                    }

                    if map[next.y as usize][next.x as usize] == '#' {
                        break;
                    }

                    pos = next;
                }
            }
        }
    }

    let answer = ((pos.y + 1) * 1000) + ((pos.x + 1) * 4) + match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };

    assert_eq!(answer, 36518);
}