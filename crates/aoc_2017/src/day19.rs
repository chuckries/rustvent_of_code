use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> (Vec<Vec<u8>>, Vec2i32, Vec2i32) {
    let map = file_lines("inputs/day19.txt").map(|l| l.as_bytes().to_vec()).to_vec();

    let mut pos = Vec2i32::zero();
    let mut dir = Vec2i32::zero();

    for i in 0..map[0].len() {
        if map[0][i] == b'|' {
            pos = (i as i32, 0).into();
            dir = (0, 1).into();
            break;
        }
    }

    // i'm tired

    (map, pos, dir)
}

fn run() -> (String, i32) {
    let (map, start, mut dir) = input();
    let bounds = Vec2i32::new(map[0].len() as i32, map.len() as i32);
    let mut letters: Vec<u8> = Vec::new();
    let mut count= 0;

    let mut pos = start;
    loop {
        let next = pos + dir;
        count += 1;

        match map[next.y as usize][next.x as usize] {
            b'-' | b'|' => (),
            c @ b'A'..=b'Z' => letters.push(c),
            b'+' => {
                for adj in next.adjacent_bounded(&bounds) {
                    if adj == pos {
                        continue;
                    }

                    if matches!(map[adj.y as usize][adj.x as usize], b'+' | b' ') {
                        continue;
                    }

                    dir = adj - next;
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