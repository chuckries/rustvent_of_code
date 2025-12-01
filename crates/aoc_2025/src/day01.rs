use aoc_common::file_lines;

fn input() -> Vec<(i32, i32)> {
    file_lines("inputs/day01.txt").map(|s| {
        let (dir, len) = s.split_at(1);
        let dir = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!(),
        };

        let len = len.parse().unwrap();

        (dir, len)
    }).collect()
}

#[test]
fn part1() {
    const MOD: i32 = 100;
    let mut current = 50;
    let mut count = 0;

    for (dir, len) in input() {
        current += dir * len;
        current = (current + MOD) % MOD;
        if current == 0 {
            count += 1;
        }
    }

    assert_eq!(1078, count);
}

#[test]
fn part2() {
    let mut current = 50;
    let mut count = 0;

    for (dir, len) in input() {
        for _ in 0..len {
            current += dir;
            if current == 100 {
                current = 0;
            } else if current == -1 {
                current = 99;
            }

            if current == 0 {
                count += 1;
            }
        }
    }

    assert_eq!(6412, count);
}