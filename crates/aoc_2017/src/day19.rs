use aoc_common::{Grid, Vec2i32};

type Map = Grid<u8>;

fn input() -> (Map, Vec2i32, Vec2i32) {
    let map = Map::file_as_grid("inputs/day19.txt", &mut |b, _| b);

    let mut pos = Vec2i32::zero();
    let mut dir = Vec2i32::zero();

    for (col, b) in map.row(0).iter().enumerate() {
        if *b == b'|' {
            pos = Vec2i32::new(col as i32, 0);
            dir = Vec2i32::unit_y();
            break;
        }
    }

    // i'm tired

    (map, pos, dir)
}

fn run() -> (String, i32) {
    let (map, start, mut dir) = input();
    let mut letters: Vec<u8> = Vec::new();
    let mut count= 0;

    let mut pos = start;
    loop {
        let next = pos + dir;
        count += 1;

        match map[next] {
            b'-' | b'|' => (),
            c @ b'A'..=b'Z' => letters.push(c),
            b'+' => {
                for (adj_p, adj) in map.adjacent_enumerate(next.cast()) {
                    let adj_p = adj_p.cast::<i32>();
                    if adj_p == pos {
                        continue;
                    }

                    if matches!(adj, b'+' | b' ') {
                        continue;
                    }

                    dir = adj_p - next;
                    break;
                }
            }
            b' ' => break,
            _ => panic!(),
        }

        pos = next;
    }

    (String::from_utf8(letters).unwrap(), count)
}

#[test]
fn part1() {
    let (answer, _) = run();
    assert_eq!(answer, "VEBTPXCHLI");
}

#[test]
fn part2() {
    let (_, answer) = run();
    assert_eq!(answer, 18702);
}