use aoc_common::{file_string, IteratorExt};

type Map = Vec<Vec<char>>;

fn input() -> Vec<Map> {
    let input = file_string("inputs/day13.txt");
    input.split("\r\n\r\n").map(|chunk| {
        chunk.split("\r\n").map(|l| l.chars().to_vec()).to_vec()
    }).to_vec()
}

fn cmp_rows(map: &Map, ja: usize, jb: usize) -> bool {
    for i in 0 .. map[0].len() {
        if map[ja][i] != map[jb][i] {
            return false;
        }
    }

    true
}

fn cmp_cols(map: &Map, ia: usize, ib: usize) -> bool {
    for j in 0 .. map.len() {
        if map[j][ia] != map[j][ib] {
            return false;
        }
    }

    return true;
}

fn find_reflection(map: &Vec<Vec<char>>) -> usize {
    for j in 0 .. map.len() - 1 {
        let mut ja = j as i32;
        let mut jb = ja + 1;

        while cmp_rows(map, ja as usize, jb as usize) {
            ja -= 1;
            jb += 1;

            if ja < 0 || jb == map.len() as i32 {
                return (j + 1) * 100;
            }
        }
    }

    for i in 0 .. map[0].len() - 1 {
        let mut ia = i as i32;
        let mut ib = ia + 1;

        while cmp_cols(map, ia as usize, ib as usize) {
            ia -= 1;
            ib += 1;

            if ia < 0 || ib == map[0].len() as i32 {
                return i + 1;
            }
        }
    }

    panic!();
}

fn delta_rows(map: &Map, ja: usize, jb: usize) -> usize {
    let mut total = 0;
    for i in 0 .. map[0].len() {
        if map[ja][i] != map[jb][i] {
            total += 1;
        }
    }

    total
}

fn delta_cols(map: &Map, ia: usize, ib: usize) -> usize {
    let mut total = 0;
    for j in 0 .. map.len() {
        if map[j][ia] != map[j][ib] {
            total += 1;
        }
    }

    total
}


fn find_off_by_one_reflection(map: &Map) -> usize {
    for j in 0 .. map.len() - 1 {
        let mut ja = j as i32;
        let mut jb = ja + 1;

        let mut delta = 0;
        loop {
            delta += delta_rows(map, ja as usize, jb as usize);
            if delta > 1 {
                break;
            }

            ja -= 1;
            jb += 1;

            if ja < 0 || jb == map.len() as i32 {
                if delta == 1 {
                    return (j + 1) * 100;
                }

                break;
            }
        }
    }

    for i in 0 .. map[0].len() - 1 {
        let mut ia = i as i32;
        let mut ib = ia + 1;

        let mut delta = 0;
        loop {
            delta += delta_cols(map, ia as usize, ib as usize);
            if delta > 1 {
                break;
            }

            ia -= 1;
            ib += 1;

            if ia < 0 || ib == map[0].len() as i32 {
                if delta == 1 {
                    return i + 1;
                }

                break;
            }
        }
    }

    panic!();
}

#[test]
fn part1() {
    let input = input();

    let answer: usize = input.iter().map(|m| find_reflection(m)).sum();
    assert_eq!(43614, answer);
}

#[test]
fn part2() {
    let input = input();

    let answer: usize = input.iter().map(|m| find_off_by_one_reflection(m)).sum();
    assert_eq!(36771, answer);
}