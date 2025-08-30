use std::{fs::File, io::Write};

use aoc_common::{file_string, IteratorExt, Vec2us};
use regex::RegexBuilder;

fn input() -> (Vec<Vec<u8>>, Vec2us) {
    let input = file_string("inputs/day17.txt");

    let regex = RegexBuilder::new(r"^([xy])=(\d+), [xy]=(\d+)\.\.(\d+)$")
        .multi_line(true)
        .crlf(true)
        .build()
        .unwrap();

    let points = regex.captures_iter(&input).map(|c| {
        let xy = &c[1];
        let a = c[2].parse::<usize>().unwrap();
        let b = c[3].parse::<usize>().unwrap();
        let c = c[4].parse::<usize>().unwrap();

        match xy {
            "x" => (Vec2us::new(a, b), Vec2us::new(a, c)),
            "y" => (Vec2us::new(b, a), Vec2us::new(c, a)),
            _ => panic!(),
        }
    }).to_vec();

    let mut min = Vec2us::max_value();
    let mut max = Vec2us::min_value();

    // find our bounding rect coords
    for (a, b) in points.iter() {
        if a.x < min.x { min.x = a.x; }
        if a.y < min.y { min.y = a.y; }
        if b.x > max.x { max.x = b.x; }
        if b.y > max.y { max.y = b.y; }
    }

    // offset x such that 0 becomes min.x - 2, this allows water to flow over the left edge of the minimum wall, 
    // and makes our bounds checking easier
    let x_offset = min.x - 2;

    // offset y such that 0 becomes the first cell position with wall
    let y_offset = min.y;

    let offset = Vec2us::new(x_offset, y_offset);

    // determine the width of the map, which is the width of the walls + 4, 2 cells on either side for water and border
    let width = max.x - min.x + 1 + 4;

    // height is enough to capture all walls vertically with no buffer
    let height = max.y - min.y + 1;

    let mut map = vec![vec![b'.'; width]; height];
    for (a, b) in points.into_iter().map(|(a, b)| (a - offset, b - offset)) {
        for y in a.y ..= b.y {
            for x in a.x ..= b.x {
                map[y][x] = b'#';
            }
        }
    }

    let source = Vec2us::new(500 - x_offset, 0);
    map[source.y][source.x] = b'+';

    (map, source)
}

fn _print_map(map: &Vec<Vec<u8>>) -> std::io::Result<()> {
    let mut f = File::create("map.txt")?;

    for l in map.iter() {
        writeln!(&mut f, "{}", str::from_utf8(l).unwrap())?;
    }

    f.flush()?;

    Ok(())
}

fn explore(map: &mut Vec<Vec<u8>>, pos: Vec2us) -> bool {
    if pos.y >= map.len() {
        return false;
    }

    match map[pos.y][pos.x] {
        b'#' | b'~' => return true,
        b'|' => return false,
        _ => (),
    };

    if explore(map, pos + Vec2us::unit_y()) {
        // solid obstacle, check bounds and stuff
        let (l, l_bounded) = explore_bound(map, pos, |p| p - Vec2us::unit_x());
        let (r, r_bounded) = explore_bound(map, pos, |p| p + Vec2us::unit_x());

        let bounded = l_bounded && r_bounded;

        let symbol = if bounded {
            b'~'
        } else {
            b'|'
        };

        for x in l ..= r {
            map[pos.y][x] = symbol;
        }

        bounded
    } else {
        // not solid
        map[pos.y][pos.x] = b'|';
        false
    }
}

// returns the bound value and true if it is bounded, false if it is unbounded
fn explore_bound(map: &mut Vec<Vec<u8>>, mut pos: Vec2us, dir_func: fn(Vec2us) -> Vec2us) -> (usize, bool) {
    loop {
        let cand = dir_func(pos);
        if map[cand.y][cand.x] == b'#' {
            return (pos.x, true);
        }

        pos = cand;
        if map[cand.y + 1][cand.x] == b'.' {
            if !explore(map, (pos.x, pos.y + 1).into()) {
                return (pos.x, false);
            }
        }
    }
}

fn explore_map() -> Vec<Vec<u8>> {
    let (mut map, source) = input();
    explore(&mut map, source);
    map
}

fn count_filtered_map<F: Fn(&&u8) -> bool>(f: F) -> usize {
    explore_map().iter().map(|v| v.iter()).flatten().filter(f).count()
}

#[test]
fn part1() {
    let answer = count_filtered_map(|c| matches!(c, b'|' | b'~'));
    assert_eq!(32552, answer);
}

#[test]
fn part2() {
    let answer = count_filtered_map(|c| matches!(c, b'~'));
    assert_eq!(26405, answer);
}