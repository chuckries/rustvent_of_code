use aoc_common::file_string;

fn input() -> String {
    file_string("inputs/day09.txt")
}

fn parse_garbage(s: &[u8], idx: &mut usize) -> i32 {
    if s[*idx] != b'<' {
        panic!();
    }
    *idx += 1;

    let mut count = 0;
    loop {
        let c = s[*idx];
        *idx += 1;
        match c {
            b'>' => return count,
            b'!' => *idx += 1,
            _ => count += 1,
        };
    }
}

fn parse_group(s: &[u8], idx: &mut usize, level: i32) -> (i32, i32) {
    let mut score = level;
    let mut garbage = 0;

    if s[*idx] != b'{' {
        panic!();
    }
    *idx += 1;

    loop {
        match s[*idx] {
            b'{' => {
                let (diff_score, diff_garbage) = parse_group(s, idx, level + 1);
                score += diff_score;
                garbage += diff_garbage;
            }
            b'<' => garbage += parse_garbage(s, idx),
            b',' => *idx += 1,
            b'}' => {
                *idx += 1;
                return (score, garbage);
            }
            _ => panic!()
        }
    }
}

fn run() -> (i32, i32) {
    let input = input();
    let mut idx = 0;
    parse_group(input.as_bytes(), &mut idx, 1)
}

#[test]
fn part1() {
    let (score, _) = run();
    assert_eq!(score, 8337);
}

#[test]
fn part2() {
    let (_, garbage) = run();
    assert_eq!(garbage, 4330);
}