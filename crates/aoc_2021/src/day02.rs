use aoc_common::file_lines;

fn input() -> Vec<(String, i32)> {
    file_lines("inputs/day02.txt").map(|l| {
        let mut tok = l.split_whitespace();
        let dir = tok.next().unwrap().to_owned();
        let num = tok.next().unwrap().parse().unwrap();
        (dir, num)
    }).collect()
}

#[test]
fn part1() {
    let mut pos = 0;
    let mut depth = 0;
    for (dir, num) in input() {
        match dir.as_str() {
            "forward" => pos += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => panic!()
        }
    }

    assert_eq!(pos * depth, 2102357);
}

#[test]
fn part2() {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, num) in input() {
        match dir.as_str() {
            "down" => aim += num,
            "up" => aim -= num,
            "forward" => {
                pos += num;
                depth += aim * num;
            }
            _ => panic!()
        }
    }

    assert_eq!(pos * depth, 2101031224);
}