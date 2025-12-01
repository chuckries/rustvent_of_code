use aoc_common::file_lines;

fn input() -> Vec<Vec<u8>> {
    file_lines("inputs/day04.txt").map(|l| {
        l.into_bytes()
    }).collect()
}

#[test]
fn part1() {
    let map = input();

    let mut count = 0;
    for j in 0 .. map.len() {
        for i in 0 .. map[j].len() {
            if map[j][i] == b'X'{               
                let up = j >= 3;
                let down = j < map.len() - 3;
                let left = i >= 3;
                let right = i < map.len() - 3;

                if up {
                    if map[j - 1][i] == b'M' && map[j - 2][i] == b'A' && map[j - 3][i] == b'S' {
                        count += 1;
                    }
                }

                if down {
                    if map[j + 1][i] == b'M' && map[j + 2][i] == b'A' && map[j + 3][i] == b'S' {
                        count += 1;
                    }
                }

                if left {
                    if map[j][i - 1] == b'M' && map[j][i - 2] == b'A' && map[j][i - 3] == b'S' {
                        count += 1;
                    }
                }

                if right {
                    if map[j][i + 1] == b'M' && map[j][i + 2] == b'A' && map[j][i + 3] == b'S' {
                        count += 1;
                    }
                }

                if up && left {
                    if map[j - 1][i - 1] == b'M' && map[j - 2][i - 2] == b'A' && map[j - 3][i - 3] == b'S' {
                        count += 1;
                    }
                }

                if up && right {
                    if map[j - 1][i + 1] == b'M' && map[j - 2][i + 2] == b'A' && map[j - 3][i + 3] == b'S' {
                        count += 1;
                    }
                }

                if down && left {
                    if map[j + 1][i - 1] == b'M' && map[j + 2][i - 2] == b'A' && map[j + 3][i - 3] == b'S' {
                        count += 1;
                    }
                }

                if down && right {
                    if map[j + 1][i + 1] == b'M' && map[j + 2][i + 2] == b'A' && map[j + 3][i + 3] == b'S' {
                        count += 1;
                    }
                }
            }
        }
    }

    assert_eq!(2593, count);
}

#[test]
fn part2() {
    let map = input();

    let mut count = 0;
    for j in 1 .. map.len() - 1 {
        for i in 1 .. map[j].len() - 1 {
            if map[j][i] == b'A' {
                if ((map[j - 1][i - 1] == b'M' && map[j + 1][i + 1] == b'S') || (map[j - 1][i - 1] == b'S' && map[j + 1][i + 1] == b'M')) &&
                   ((map[j + 1][i - 1] == b'M' && map[j - 1][i + 1] == b'S') || (map[j + 1][i - 1] == b'S' && map[j - 1][i + 1] == b'M')) {
                    count += 1;
                }
            }
        }
    }

    assert_eq!(1950, count);
}