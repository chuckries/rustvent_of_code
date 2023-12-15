use aoc_common::{file_lines, IteratorExt, Vec2us, Vec2i32};

fn input() -> Vec<Vec<char>> {
    file_lines("inputs/day14.txt").map(|l| l.chars().to_vec()).to_vec()
}

fn roll<I>(map: &mut Vec<Vec<char>>, start_iter: I, increment: Vec2i32)
    where I: Iterator<Item = Vec2us>
{
    for p in start_iter {
        let mut open: Vec2i32 = p.cast();
        let mut current: Vec2i32 = p.cast();

        while current.x >= 0 && current.x < map[0].len() as i32 && current.y >= 0 && current.y < map.len() as i32 {
            match map[current.y as usize][current.x as usize] {
                'O' => {
                    map[open.y as usize][open.x as usize] = 'O';
                    if current != open {
                        map[current.y as usize][current.x as usize] = '.'
                    }
                    open += increment;
                }
                '#' => {
                    open = current + increment;
                }
                _ => ()
            }

            current += increment;
        }
    }
}

fn roll_north(map: &mut Vec<Vec<char>>)
{
    let start_iter = Vec2us::new(0, 0).area(Vec2us::new(map[0].len() - 1, 0));
    roll(map, start_iter, Vec2i32::unit_y());
}

fn roll_south(map: &mut Vec<Vec<char>>)
{
    let start_iter = Vec2us::new(0, map.len() - 1).area(Vec2us::new(map[0].len() - 1, map.len() - 1));
    roll(map, start_iter, -Vec2i32::unit_y());
}

fn roll_east(map: &mut Vec<Vec<char>>)
{
    let start_iter = Vec2us::new(map[0].len() - 1, 0).area(Vec2us::new(map[0].len() - 1, map.len() - 1));
    roll(map, start_iter, -Vec2i32::unit_x());
}

fn roll_west(map: &mut Vec<Vec<char>>)
{
    let start_iter = Vec2us::new(0, 0).area(Vec2us::new(0, map.len() - 1));
    roll(map, start_iter, Vec2i32::unit_x());
}

fn calc_load(map: &Vec<Vec<char>>) -> usize {
    map.iter().rev().enumerate().map(|(idx, row)| (idx + 1) * row.iter().filter(|c| **c == 'O').count()).sum()
}

#[test]
fn part1() {
    let mut map = input();
    roll_north(&mut map);
    let answer = calc_load(&map);
    assert_eq!(111979, answer);
}

#[test]
fn part2()
{
    let mut map = input();
    let mut states: Vec<Vec<Vec<char>>> = Vec::new();

    states.push(map.clone());

    let cycle_start;
    'outer: loop {
        roll_north(&mut map);
        roll_west(&mut map);
        roll_south(&mut map);
        roll_east(&mut map);

        for i in 0 .. states.len() {
            if states[i] == map {
                cycle_start = i;
                break 'outer;
            }
        }

        states.push(map.clone());
    }

    let cycle_length = states.len() - cycle_start;
    let mut steps = 1000000000;
    steps -= cycle_start;
    steps %= cycle_length;
    steps += cycle_start;

    let answer = calc_load(&states[steps]);
    assert_eq!(102055, answer);
}