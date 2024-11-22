use aoc_common::{file_lines, Vec2i32, Vec2us};

fn input() -> Vec<Vec<Vec2i32>> {
    file_lines("inputs/day02.txt").map(|l| {
        l.bytes().map(|b| {
            match b {
                b'L' => -Vec2i32::unit_x(),
                b'R' =>  Vec2i32::unit_x(),
                b'U' => -Vec2i32::unit_y(),
                b'D' =>  Vec2i32::unit_y(),
                _ => panic!(),
            }
        }).collect()
    }).collect()
}

fn run<const N: usize>(pad: &[[char; N]; N], start: Vec2us) -> String {
    let mut p = start.cast();
    let mut answer = String::new();

    for line in input() {
        for dir in line {
            let mut next = p + dir;
            if next.y < 0 {
                next.y = 0;
            }
            if next.y >= pad.len() as i32 { 
                next.y = pad.len() as i32 - 1;
            }
            if next.x < 0 {
                next.x = 0;
            }
            if next.x >= pad[0].len() as i32 {
                next.x = pad[0].len() as i32 - 1;
            }
            if pad[next.y as usize][next.x as usize] != '\0' {
                p = next;
            }
        }

        answer.push(pad[p.y as usize][p.x as usize]);
    }

    answer
}

#[test]
fn part1() {
    const PAD: [[char; 3]; 3] = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9']
    ];

    let answer = run(&PAD, Vec2us::new(1, 1));
    assert_eq!(answer, "12578");
}

#[test]
fn part2() {
    const PAD: [[char; 5]; 5] = [
        ['\0', '\0', '1', '\0', '\0'],
        ['\0',  '2', '3',  '4', '\0'],
        [ '5',  '6', '7',  '8',  '9'],
        ['\0',  'A', 'B',  'C', '\0'],
        ['\0', '\0', 'D', '\0', '\0'],
    ];

    let answer = run(&PAD, Vec2us::new(0, 2));
    assert_eq!(answer, "516DD");
}