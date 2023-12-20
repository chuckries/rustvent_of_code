use aoc_common::{file_lines, IteratorExt, Vec2i32, PriorityQueue};

fn input() -> Vec<Vec<i32>> {
    file_lines("inputs/day17.txt").map(|l| l.bytes().map(|b| (b - b'0') as i32).to_vec()).to_vec()
}

#[derive(Clone)]
struct State {
    pos: Vec2i32,
    dir: Vec2i32,
    steps: i32,
    sum: i32,
}

fn dir_to_index(dir: Vec2i32) -> usize {
    match (dir.x, dir.y) {
        ( 1,  0) => 0,
        (-1,  0) => 1,
        ( 0,  1) => 2,
        ( 0, -1) => 3,
        _ => panic!(),
    }
}

fn run<const MIN: i32, const MAX: usize>() -> i32 {
    let map = input();
    let mut queue: PriorityQueue<State, i32> = PriorityQueue::new();
    let bounds = Vec2i32::new(map[0].len() as i32, map.len() as i32);
    let target = bounds - Vec2i32::new(1, 1);
    let mut nexts: Vec<(Vec2i32, Vec2i32, i32)> = Vec::with_capacity(3);

    let mut visisted: Vec<Vec<[[bool; MAX]; 4]>> = vec![vec![[[false; MAX]; 4]; map[0].len()]; map.len()];

    let start_right = State {
        pos: Vec2i32::new(1, 0),
        dir: Vec2i32::unit_x(),
        steps: 1,
        sum: map[0][1],
    };
    let weight = start_right.sum + start_right.pos.manhattan_from(target);
    queue.enqueue(start_right, weight);

    let start_left = State {
        pos: Vec2i32::new(0, 1),
        dir: Vec2i32::unit_y(),
        steps: 1,
        sum: map[1][0],
    };
    let weight = start_left.sum + start_left.pos.manhattan_from(target);
    queue.enqueue(start_left, weight);

    while let Some(state) = queue.dequeue() {
        if state.pos == target && state.steps >= MIN {
            return state.sum;
        }

        let visit_idx = dir_to_index(state.dir);
        let has_visited = &mut visisted[state.pos.y as usize][state.pos.x as usize][visit_idx][state.steps as usize - 1];
        if *has_visited {
            continue;
        }
        *has_visited = true;
        
        if state.steps < MAX as i32 {
            nexts.push((state.pos + state.dir, state.dir, state.steps + 1));
        }

        if state.steps >= MIN as i32 {
            let left = state.dir.rotated_left();
            let right = state.dir.rotated_right();
            nexts.push((state.pos + left, left, 1));
            nexts.push((state.pos + right, right, 1));
        }

        for (pos, dir, steps) in nexts.iter() {
            if pos.is_in_bounds(bounds) && !visisted[pos.y as usize][pos.x as usize][dir_to_index(*dir)][*steps as usize - 1] {
                let next = State {
                    pos: *pos,
                    dir: *dir,
                    steps: *steps,
                    sum: state.sum + map[pos.y as usize][pos.x as usize],
                };
                let weight = next.sum + next.pos.manhattan_from(target);
                queue.enqueue(next, weight);
            }
        }

        nexts.clear();
    }

    panic!();
}

#[test]
fn part1() {
    let answer = run::<1, 3>();
    assert_eq!(668, answer);
}

#[test]
fn part2() {
    let answer = run::<4, 10>();
    assert_eq!(788, answer);
}