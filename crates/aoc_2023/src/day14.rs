use aoc_common::{Grid, Vec2i32, Vec2us };

type Map = Grid<char>;

fn input() -> Map {
    Grid::file_as_grid("inputs/day14.txt", &mut |b, _| b as char)
}

fn roll<I>(map: &mut Map, start_iter: I, increment: Vec2i32)
    where I: Iterator<Item = Vec2us>
{
    for p in start_iter {
        let mut open: Vec2i32 = p.cast();
        let mut current: Vec2i32 = p.cast();

        while current.is_in_bounds(map.bounds().cast()) {
            match map[current] {
                'O' => {
                    map[open] = 'O';
                    if current != open {
                        map[current] = '.'
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

fn roll_north(map: &mut Map)
{
    let start_iter = Vec2us::new(0, 0).area(Vec2us::new(map.width() - 1, 0));
    roll(map, start_iter, Vec2i32::unit_y());
}

fn roll_south(map: &mut Map)
{
    let start_iter = Vec2us::new(0, map.height() - 1).area(Vec2us::new(map.width() - 1, map.height() - 1));
    roll(map, start_iter, -Vec2i32::unit_y());
}

fn roll_east(map: &mut Map)
{
    let start_iter = Vec2us::new(map.width() - 1, 0).area(Vec2us::new(map.width() - 1, map.height() - 1));
    roll(map, start_iter, -Vec2i32::unit_x());
}

fn roll_west(map: &mut Map)
{
    let start_iter = Vec2us::new(0, 0).area(Vec2us::new(0, map.height() - 1));
    roll(map, start_iter, Vec2i32::unit_x());
}

fn calc_load(map: &Map) -> usize {
    let height = map.height();
    map.enumerate().filter_map(|(p, c)| {
        if *c == 'O' { Some(height - p.y) } else { None }
    }).sum()
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
    let mut states: Vec<Map> = Vec::new();

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