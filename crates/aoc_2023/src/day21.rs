use std::{collections::HashSet, fs::File};

use aoc_common::{file_lines, Vec2us, IteratorExt, Vec2i32};

fn input() -> (Vec<Vec<char>>, Vec2us) {
    let mut map = file_lines("inputs/day21.txt").map(|l| {
        l.chars().to_vec()
    }).to_vec();

    let mut start = Vec2us::zero();
    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (i, j).into();
                break;
            }
        }
    }

    map[start.y][start.x] = '.';
    (map, start)
}

fn print_map<W: std::io::Write>(map: &Vec<Vec<char>>, steps: &HashSet<Vec2us>, w: &mut W) {
    let mut map = map.clone();
    for p in steps {
        map[p.y][p.x] = 'O';
    }

    for row in map {
        for c in row {
            write!(w, "{}", c).unwrap();
        }
        writeln!(w).unwrap();
    }
    writeln!(w).unwrap();
}

fn print_space<W: std::io::Write, F>(space: &Vec<Vec<Vec<Vec<char>>>>, steps: &HashSet<Vec2i32>, w: &mut W, f: F)
    where F: Fn(Vec2i32) -> Option<(Vec2us, Vec2us)>
{
    let mut space = space.clone();
    for p in steps.iter() {
        let (uv, p) = f(*p).unwrap();
        space[uv.y][uv.x][p.y][p.x] = 'O';
    }

    // space[0][0][0][0] = '0';
    // space[0][1][0][0] = '1';
    // space[0][2][0][0] = '2';
    // space[1][0][0][0] = '3';

    for v in 0..5 {
        for y in 0..space[0][0].len() {
            for u in 0..5 {
                for x in 0..space[0][0][0].len() {
                    write!(w, "{}", space[v][u][y][x]).unwrap();
                }
                write!(w, " ").unwrap();
            }
            writeln!(w).unwrap();
        }
        writeln!(w).unwrap();
    }
}

#[test]
fn part1() {
    let (map, start) = input();
    let bounds = Vec2us::new(map[0].len(), map.len());
    let mut positions: HashSet<Vec2us> = HashSet::new();
    positions.insert(start);

    for _ in 0..64 {
        let mut next: HashSet<Vec2us> = HashSet::new();
        for p in positions {
            for adj in p.adjacent_bounded(&bounds) {
                if map[adj.y][adj.x] == '.' {
                    next.insert(adj);
                }
            }
        }
        positions = next;
    }

    let answer = positions.len();
    assert_eq!(3642, answer);
}

#[test]
fn part2() {
    /*

 ---- ---- ---- ---- ----
|    |D   |C/\ |N   |    |
|    |    |/  \|    |    |
|    |   /|    |\   |    |
|    |  / |    | \  |    |
 ---- ---- ---- ---- ----
|D   |E/  |B   |M \ |N   |
|    |/   |    |   \|    |
|   /|    |    |    |\   |
|  / |    |    |    | \  |
 ---- ---- ---- ---- ----
|F/  |B   |A   |B   |L \ |
|/   |    |    |    |   \|
|\   |    |    |    |   /|
| \  |    |    |    |  / |
 ---- ---- ---- ---- ----
|G \ |H   |B   |K   |J/  |
|   \|    |    |    |/   |
|    |\   |    |   /|    |
|    | \  |    |  / |    |
 ---- ---- ---- ---- ----
|    |G \ |I   |J/  |    |
|    |   \|    |/   |    |
|    |    |\  /|    |    |
|    |    | \/ |    |    |
 ---- ---- ---- ---- ----
       
     */

    const STEPS: usize = 26501365;

    let (map, start) = input();
    let bounds = Vec2us::new(map[0].len(), map.len());

    let space = vec![vec![map; 5]; 5];

    let p_to_space = |mut p: Vec2i32| -> Option<(Vec2us, Vec2us)> {
        let mut uv = Vec2i32::new(2, 2);
        while p.x < 0 {
            p.x += bounds.x as i32;
            uv.x -= 1;
        }
        while p.x >= bounds.x as i32 {
            p.x -= bounds.x as i32;
            uv.x += 1;
        }
        while p.y < 0 {
            p.y += bounds.y as i32;
            uv.y -= 1;
        }
        while p.y >= bounds.y as i32 {
            p.y -= bounds.y as i32;
            uv.y += 1;
        }

        if uv.x < 0 || uv.x >= 5 || uv.y < 0 || uv.y >= 5 {
            return None;
        }

        Some((uv.cast(), p.cast()))
    };

    let mut file = File::create("steps.txt").unwrap();

    let mut positions: HashSet<Vec2i32> = HashSet::new();
    let start = Vec2us::new(0, 0);
    positions.insert(start.cast());

    for _ in 0..(64 + 131) {
        let mut next: HashSet<Vec2i32> = HashSet::new();
        for p in positions {
            for adj in p.adjacent() {
                if let Some((uv, p)) = p_to_space(adj) {
                    if space[uv.y][uv.x][p.y][p.x] == '.' {
                        next.insert(adj);
                    }
                }
            }
        }
        println!("{}", next.len());
        positions = next;
    }

    print_space(&space, &positions, &mut file, p_to_space);

    let answer = positions.len();
    assert_eq!(3642, answer);
}