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

fn count_positions(map: &Vec<Vec<char>>, start: Vec2us, iterations: &[usize], print: bool) -> Vec<usize> {
    let bounds = Vec2us::new(map[0].len(), map.len());
    let mut positions: HashSet<Vec2us> = HashSet::new();
    let mut next = positions.clone();
    positions.insert(start);

    let mut counts: Vec<usize> = Vec::new();
    let mut loop_count = 0;
    for iteration in iterations {
        while loop_count < *iteration {
            for p in positions.iter() {
                for adj in p.adjacent_bounded(&bounds) {
                    if map[adj.y][adj.x] == '.' {
                        next.insert(adj);
                    }
                }
            }

            positions.clear();
            (positions, next) = (next, positions);

            loop_count += 1;
        }

        counts.push(positions.len());

        if print {
            print_map(map, &positions, &mut std::io::stdout());
        }
    }

    counts
}

#[test]
fn part1() {
    let (map, start) = input();
    let answer = count_positions(&map, start, &[64], false)[0];
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

    let (map, start) = input();
    let size = map.len();
    let half_size = map.len() / 2;

    let nw_edges = count_positions(&map, Vec2us::new(size - 1, size - 1), &[half_size - 1, size + half_size - 1], false);
    let nw_thin = nw_edges[0];
    let nw_thick = nw_edges[1];

    let sw_edges = count_positions(&map, Vec2us::new(size - 1, 0), &[half_size - 1, size + half_size - 1], false);
    let sw_thin = sw_edges[0];
    let sw_thick = sw_edges[1];

    let ne_edges = count_positions(&map, Vec2us::new(0, size - 1), &[half_size - 1, size + half_size - 1], false);
    let ne_thin = ne_edges[0];
    let ne_thick = ne_edges[1];

    let se_edges = count_positions(&map, Vec2us::new(0, 0), &[half_size - 1, size + half_size - 1], false);
    let se_thin = se_edges[0];
    let se_thick = se_edges[1];

    let n_point = count_positions(&map, Vec2us::new(half_size, size - 1), &[size - 1], false)[0];
    let s_point = count_positions(&map, Vec2us::new(half_size, 0), &[size - 1], false)[0];
    let w_point = count_positions(&map, Vec2us::new(size - 1, half_size), &[size - 1], false)[0];
    let e_point = count_positions(&map, Vec2us::new(0, half_size), &[size - 1], false)[0];

    let centers = count_positions(&map, start, &[half_size + size - 1, half_size + size], false);
    let center_0 = centers[0];
    let cetner_1 = centers[1];

    const ITERATIONS: usize = 26501365;

    // calculate height of map in "tiles", this number is from the center line to one boundary (inclusive)
    let height_in_one_dir_inclusive = (ITERATIONS - half_size) / size + 1;

    // this is the numbef of tiles between the center line and a boundary (exclusive)
    let interior_distance = height_in_one_dir_inclusive - 2;

    // calculate the total number of edge tiles on one edge of the diamond
    // every row between center and boundary (exlusive) will have 1 thin and 1 thick
    // the boundary row will have an additional thin
    let thicks = interior_distance;
    let thins = thicks + 1;

    // count the interior tiles, there is probably a forumla this but idk
    // this counts primary and alternate interior tiles between the center line and a boundary (exclusive)
    // first line inside the boundary will have one alt, next will have two alts and 1 primary, next will have 3 alts and 2 primary, etc.
    // 0: (0, 1)
    // 1: (1, 2)
    // 2: (2, 3)
    // n: (n, n + 1)

    let primaries = (interior_distance * (interior_distance - 1) ) / 2;
    let alternates = ((interior_distance + 1) * interior_distance) / 2;

    // double both of them
    let primaries = primaries * 2;
    let alternates = alternates * 2;

    // add the center line
    let interior_width_of_center_line = 2 * height_in_one_dir_inclusive - 3;
    let primaries = primaries + interior_width_of_center_line / 2;
    let alternates = alternates + interior_width_of_center_line / 2 + 1;

    let mut total: usize = 0;

    // add all the edges
    total += thins * (nw_thin + ne_thin + sw_thin + se_thin);
    total += thicks * (nw_thick + ne_thick + sw_thick + se_thick);

    // add the points
    total += n_point + s_point + w_point + e_point;

    // add the interiors
    total += primaries * center_0;
    total += alternates * cetner_1;

    assert_eq!(93270, total);
}