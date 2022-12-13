use std::collections::VecDeque;
use aoc_common::{Vec2us, file_lines, IteratorExt};

fn input() -> (Vec<Vec<i32>>, Vec2us, Vec2us) {
    let mut start = Vec2us::zero();
    let mut end = Vec2us::zero();

    let lines = file_lines("inputs/day12.txt").map(|l| l.bytes().to_vec()).to_vec();

    let mut map = vec![vec![0; lines[0].len()]; lines.len()];
    for j in 0..lines.len() {
        for i in 0..lines[j].len() {
            map[j][i] = match lines[j][i] {
                b'S' => {
                    start = (i, j).into();
                    0
                }
                b'E' => {
                    end = (i, j).into();
                    25
                }
                c @ b'a'..=b'z' => {
                    (c - b'a') as i32
                }
                _ => panic!()
            }
        }
    }

    (map, start, end)
}

fn search<FFound, FTest>(map: &Vec<Vec<i32>>, start: Vec2us, f_found: FFound, f_test: FTest) -> i32
    where
        FFound: Fn(Vec2us, i32) -> bool,
        FTest: Fn(i32, i32) -> bool
{
    let bounds = Vec2us::new(map[0].len(), map.len());
    let mut queue: VecDeque<(Vec2us, i32)> = VecDeque::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[start.y][start.x] = true;

    queue.push_back((start, 0));

    while let Some((current, dist)) = queue.pop_front() {
        if f_found(current, map[current.y][current.x]) {
            return dist;
        }

        for adj in current.adjacent_bounded(&bounds) {
            if !visited[adj.y][adj.x] && f_test(map[current.y][current.x], map[adj.y][adj.x]) {
                visited[adj.y][adj.x] = true;
                queue.push_back((adj, dist + 1));
            }
        }
    }

    panic!()
}

#[test]
fn part1() {
    let (map, start, end) = input();

    let answer = search(
        &map,
        start,
        |current, _| current == end,
        |current, adj| adj <= current + 1
    );

    assert_eq!(answer, 412);
}

#[test]
fn part2() {
    let (map, _, end) = input();

    let answer = search(
        &map,
        end,
        |_, val| val == 0,
        |current, adj| adj + 1 >= current
    );

    assert_eq!(answer, 402);
}