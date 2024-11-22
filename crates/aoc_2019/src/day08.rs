use aoc_common::{file_string, Vec2us, map_points_to_string};

const DIMENSIONS: Vec2us = Vec2us::new(25, 6);
const AREA: usize = DIMENSIONS.x * DIMENSIONS.y;

fn input() -> Vec<u8> {
    file_string("inputs/day08.txt").into_bytes()
}

#[test]
fn part1() {
    let frames = input();

    let min_frame = frames
        .chunks(AREA)
        .min_by_key(|frame| {
            frame.iter().filter(|c| **c == b'0').count()
        })
        .unwrap();

    let mut ones = 0;
    let mut twos = 0;
    for c in min_frame {
        match *c {
            b'1' => ones += 1,
            b'2' => twos += 1,
            _ => ()
        }
    }

    let answer = ones * twos;
    assert_eq!(answer, 2032);
}

#[test]
fn part2() {
    let mut frames = input();

    let points = frames
        .chunks_mut(AREA)
        .reduce(|accum, frame| {
            accum.iter_mut().zip(frame).for_each(|(a, b)| { 
                if *a == b'2' {
                    *a = *b
                }
             });
            accum
        })
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if *p == b'1' {
                Some(Vec2us::new(i % DIMENSIONS.x, i / DIMENSIONS.x))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        ;

    let answer = map_points_to_string(points.into_iter());

    let known = "
 ██  ████  ██  █  █  ██ 
█  █ █    █  █ █  █ █  █
█    ███  █    █  █ █   
█    █    █    █  █ █ ██
█  █ █    █  █ █  █ █  █
 ██  █     ██   ██   ███";

     assert_eq!(answer, known);
}