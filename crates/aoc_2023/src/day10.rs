use std::collections::{HashSet, VecDeque};

use aoc_common::{Vec2us, file_lines, IteratorExt, Vec2i32};

fn input() -> (Vec<Vec<char>>, Vec2us) {
    let map = file_lines("inputs/day10.txt").map(|l| l.chars().to_vec()).to_vec();
    let start = map.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(move |(x, c)| {
            (x, y, *c)
        })
    }).flatten().find_map(|(x, y, c)| {
        if c == 'S' {
            Some(Vec2us::new(x, y))
        } else {
            None
        }
     }).unwrap();

    (map, start)
}

fn adjacent(p: Vec2us, c: char) -> [Vec2us; 2] {
    match c {
        '|' => [Vec2us::new(p.x, p.y - 1), Vec2us::new(p.x, p.y + 1)],
        '-' => [Vec2us::new(p.x - 1, p.y), Vec2us::new(p.x + 1, p.y)],
        'L' => [Vec2us::new(p.x, p.y - 1), Vec2us::new(p.x + 1, p.y)],
        'J' => [Vec2us::new(p.x, p.y - 1), Vec2us::new(p.x - 1, p.y)],
        '7' => [Vec2us::new(p.x - 1, p.y), Vec2us::new(p.x, p.y + 1)],
        'F' => [Vec2us::new(p.x + 1, p.y), Vec2us::new(p.x, p.y + 1)],
        _ => panic!()
    }
}

#[test]
fn part1() {
    let (map, start) = input();

    let mut visited: HashSet<Vec2us> = HashSet::new();
    let mut queue: VecDeque<(Vec2us, i32)> = VecDeque::new();

    match map[start.y - 1][start.x] {
        '|' | '7' | 'F' => queue.push_back((start - Vec2us::unit_y(), 1)),
        _ => ()
    };

    match map[start.y + 1][start.x] {
        '|' | 'J' | 'L' => queue.push_back((start + Vec2us::unit_y(), 1)),
        _ => ()
    };

    match map[start.y][start.x - 1] {
        '-' | 'F' | 'L' => queue.push_back((start - Vec2us::unit_x(), 1)),
        _ => ()
    };

    match map[start.y][start.x + 1] {
        '-' | '7' | 'J' => queue.push_back((start + Vec2us::unit_x(), 1)),
        _ => ()
    };

    visited.insert(start);

    let mut max = 0;
    while let Some((p, steps)) = queue.pop_front() {  
        max = steps;
        visited.insert(p);
        for s in adjacent(p, map[p.y][p.x]) {
            if !visited.contains(&s) {
                queue.push_back((s, steps + 1));
            }
        }
    }

    assert_eq!(6757, max);
}

type Map = Vec<Vec<char>>;

fn get_start_dir(map: &Map, start: Vec2i32) -> Vec2i32 {
    let (n, s, e, w) = (
        start.north_of(),
        start.south_of(),
        start.east_of(),
        start.west_of()
    );

    match map[n.y as usize][n.x as usize] {
        '|' | 'F' | '7' => -Vec2i32::unit_y(),
        _ => match map[s.y as usize][s.x as usize] {
            '|' | 'J' | 'L' => Vec2i32::unit_y(),
            _ => match map[e.y as usize][e.x as usize] {
                '-' | 'J' | '7' => Vec2i32::unit_x(),
                _ => match map[w.y as usize][w.x as usize] {
                    '-' | 'F' | 'L' => -Vec2i32::unit_x(),
                    _ => panic!()
                }
            }
        }
    }
}

fn move_forward(p: Vec2i32, dir: Vec2i32, map: &Map) -> (Vec2i32, Vec2i32) {
    let next = p + dir;

    let c = map[next.y as usize][next.x as usize];
    let dir = match (c, dir.x, dir.y) {
        ('-', _, _) | ('|', _, _) | ('S', _, _) => dir,
        ('F', -1,  0) =>  Vec2i32::unit_y(),
        ('F',  0, -1) =>  Vec2i32::unit_x(),
        ('7',  1,  0) =>  Vec2i32::unit_y(),
        ('7',  0, -1) => -Vec2i32::unit_x(),
        ('J',  1,  0) => -Vec2i32::unit_y(),
        ('J',  0,  1) => -Vec2i32::unit_x(),
        ('L', -1,  0) => -Vec2i32::unit_y(),
        ('L',  0,  1) =>  Vec2i32::unit_x(),
        _ => panic!(),
    };

    (next, dir)
}

fn flood_fill(p: Vec2i32, map: &Map, visited: &mut Map, c: char) -> Option<i32> {
    if p.y < 0 || p.y as usize >= map.len() || p.x < 0 || p.x as usize >= map[0].len() {
        return None;
    }

    if visited[p.y as usize][p.x as usize] != ' ' || map[p.y as usize][p.x as usize] != ' ' {
        return Some(0);
    }

    let mut queue: VecDeque<Vec2i32> = VecDeque::new();
    let mut count = 0;
    queue.push_back(p);
    visited[p.y as usize][p.x as usize] = c;
    while let Some(current) = queue.pop_front() {
        count += 1;
        visited[current.y as usize][current.x as usize] = c;
        for adj in current.adjacent() {
            if adj.y < 0 || adj.y as usize >= map.len() || adj.x < 0 || adj.x as usize >= map[0].len() {
                return None;
            }

            if visited[adj.y as usize][adj.x as usize] == ' ' && map[adj.y as usize][adj.x as usize] == ' ' {
                visited[adj.y as usize][adj.x as usize] = c;
                queue.push_back(adj);
            }
        }
    }

    Some(count)
}

fn get_left_right(p: Vec2i32, dir: Vec2i32, map: &Map) -> (Vec<Vec2i32>, Vec<Vec2i32>) {
    let c = map[p.y as usize][p.x as usize];
    match (c, dir.x, dir.y) {
        ('-',  1,  0) => (vec![p - Vec2i32::unit_y()], vec![p + Vec2i32::unit_y()]),
        ('-', -1,  0) => (vec![p + Vec2i32::unit_y()], vec![p - Vec2i32::unit_y()]),
        ('|',  0,  1) => (vec![p + Vec2i32::unit_x()], vec![p - Vec2i32::unit_x()]),
        ('|',  0, -1) => (vec![p - Vec2i32::unit_x()], vec![p + Vec2i32::unit_x()]),

        ('F', -1,  0) => (vec![]                                            , vec![p - Vec2i32::unit_y(), p - Vec2i32::unit_x()]),
        ('F',  0, -1) => (vec![p - Vec2i32::unit_y(), p - Vec2i32::unit_x()], vec![]                                            ),
        ('7',  1,  0) => (vec![p - Vec2i32::unit_y(), p + Vec2i32::unit_x()], vec![]                                            ),
        ('7',  0, -1) => (vec![]                                            , vec![p - Vec2i32::unit_y(), p + Vec2i32::unit_x()]),
        ('J',  1,  0) => (vec![]                                            , vec![p + Vec2i32::unit_y(), p + Vec2i32::unit_x()]),
        ('J',  0,  1) => (vec![p + Vec2i32::unit_y(), p + Vec2i32::unit_x()], vec![]                                            ),
        ('L', -1,  0) => (vec![p + Vec2i32::unit_y(), p - Vec2i32::unit_x()], vec![]                                            ),
        ('L',  0,  1) => (vec![]                                            , vec![p + Vec2i32::unit_y(), p - Vec2i32::unit_x()]),
        ('S', _, _) => (Vec::new(), Vec::new()),
        _ => panic!(),
    }
}

#[test]
fn part2() {
    let (map, start) = input();
    
    let start: Vec2i32 = start.cast();
    let start_dir = get_start_dir(&map, start);

    let mut dir = start_dir;
    let mut loop_map = vec![vec![' '; map[0].len()]; map.len()];

    let mut current = start;
    while loop_map[current.y as usize][current.x as usize] == ' ' {
        loop_map[current.y as usize][current.x as usize] = map[current.y as usize][current.x as usize];
        (current, dir) = move_forward(current, dir, &map);
    }

    // for row in loop_map.iter() {
    //     for c in row.iter() {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    //panic!();

    let mut do_left = true;
    let mut do_right = true;
    let mut count_left = 0;
    let mut count_right = 0;

    let mut current = start;
    let mut dir = start_dir;

    let mut visited = vec![vec![' '; map[0].len()]; map.len()];

    loop {
        let (next, next_dir) = move_forward(current, dir, &loop_map);
        let (lefts, rights) = get_left_right(next, dir, &loop_map);

        if do_left {
            for l in lefts {
                if !do_left { break; }
                if let Some(n) = flood_fill(l, &loop_map, &mut visited, 'L') {
                    count_left += n;
                }
                else {
                    do_left = false;
                }
            }
        }

        if do_right {
            for r in rights {
                if !do_right { break; }
                if let Some(n) = flood_fill(r, &loop_map, &mut visited, 'R') {
                    count_right += n;
                }
                else {
                    do_right = false;
                }
            }
        }

        if !(do_left || do_right) {
            panic!();
        }



        current = next;
        dir = next_dir;

        if loop_map[current.y as usize][current.x as usize] == 'S' { break; }
    }

    // for row in visited.iter() {
    //     for c in row.iter() {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let answer = if do_left {
        count_left
    } else if do_right {
        count_right
    } else {
        panic!();
    };

    assert_eq!(523, answer);
}