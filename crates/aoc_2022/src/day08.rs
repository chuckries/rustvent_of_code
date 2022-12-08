use aoc_common::file_lines;

fn input() -> Vec<Vec<u32>> {
    file_lines("inputs/day08.txt").map(|l| {
        l.bytes().map(|b| (b - b'0') as u32).collect()
    }).collect()
}

fn is_visible(map: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let current = map[j][i];
    let mut visible = true;

    for u in 0..i {
        if map[j][u] >= current {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }
    
    visible = true;
    for u in i + 1..map[0].len() {
        if map[j][u] >= current {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for v in 0..j {
        if map[v][i] >= current {
            visible = false;
            break;
        }
    }

    if visible {
        return true;
    }

    visible = true;
    for v in j + 1..map.len() {
        if map[v][i] >= current {
            visible = false;
            break;
        }
    }

    visible
}

#[test]
fn part1() {
    let map = input();

    let mut count = 0;
    for i in 0..map[0].len() {
        for j in 0..map.len() {

            if is_visible(&map, i, j) {
                count += 1;
            }
        }
    }

    assert_eq!(count, 1713);
}

#[test]
fn part2() {
    let map = input();

    let mut max = 0;

    for i in 0..map[0].len() {
        for j in 0..map.len() {
            let current = map[j][i];

            let mut left = 0;
            for u in (0..i).rev() {
                left += 1;
                if map[j][u] >= current {
                    break;
                }
            }

            let mut right = 0;
            for u in i + 1..map[0].len() {
                right += 1;
                if map[j][u] >= current {
                    break;
                }
            }

            let mut up = 0;
            for v in (0..j).rev() {
                up += 1;
                if map[v][i] >= current {
                    break;
                }
            }

            let mut down = 0;
            for v in j + 1..map.len() {
                down += 1;
                if map[v][i] >= current {
                    break;
                }
            }

            let cand = left as u64 * right * up * down;
            if cand > max {
                max = cand;
            }
        }
    }

    assert_eq!(max, 268464);
}