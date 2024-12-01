use aoc_common::file_lines;

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day02.txt").map(|l| {
        l.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
    }).collect()
}

#[test]
fn part1() {
    let input = input();

    let mut checksum = 0;

    for v in input.iter() {
        let mut max = v[0];
        let mut min = v[0];

        for i in v[1..].iter().cloned() {
            if i > max {
                max = i;
            }
            if i < min {
                min = i;
            }
        }

        checksum += max - min;
    }

    assert_eq!(checksum, 39126);
}

#[test]
fn part2() {
    let input = input();

    let mut checksum = 0;

    for v in input.iter() {
        for i in 0..v.len() - 1 {
            for j in i + 1 .. v.len() {
                let mut a = v[i];
                let mut b = v[j];
                if a < b {
                    (a, b) = (b, a);
                }

                if a % b == 0 {
                    checksum += a / b;
                }
            }
        }
    }

    assert_eq!(checksum, 258);
}