use std::{collections::{HashMap, HashSet, VecDeque}, usize};

use aoc_common::{file_lines, IteratorExt, Vec2i32};

fn input() -> Vec<Vec2i32> {
    file_lines("inputs/day06.txt").map(|l| {
        let split = l.split(", ").map(|s| s.parse::<i32>().unwrap()).to_vec();
        Vec2i32::new(split[0], split[1])
    }).to_vec()
}

#[test]
fn part1() {
    let input = input();

    let mut min = Vec2i32::max_value();
    let mut max = Vec2i32::min_value();
    for p in input.iter() {
        if p.x < min.x {
            min.x = p.x;
        }

        if p.y < min.y {
            min.y = p.y;
        }

        if p.x > max.x {
            max.x = p.x;
        }

        if p.y > max.y {
            max.y = p.y;
        }
    }

    // We're going to do a simulatenous flood fill from all originating points, up until the maximum
    // Any id that hits the maximum will be removed from contention
    // when regions start to overlap we will mark them and stop the fill from that point on

    let mut total_area = vec![0; input.len()];

    // map of position to (owned id, distance from owned id)
    let mut map: HashMap<Vec2i32, (usize, usize)> = HashMap::new();

    // seed the queue with all the starting positions
    let mut queue: VecDeque<(usize, Vec2i32, usize)> = VecDeque::new();
    for (idx, pos) in input.into_iter().enumerate() {
        queue.push_back((idx, pos, 0));
    }

    while let Some((id, pos, current_distance)) = queue.pop_front() {
        if pos.x < min.x || pos.x > max.x || pos.y < min.y || pos.y > max.y {
            // we have hit infinite zone for this region, mark it's total area as -1 and do not continue 
            // exploring from this spot
            total_area[id] = usize::MAX;
            continue;
        }

        // check the current position
        // - if it's unoccupied, we take ownership and area to this origin
        // - if it's occupied by another origin
        //   - if same disatnce we mark as shared and remove area from the pre-existing origin
        //   - if lower distance we are out of bounds and need to just stop
        //   - higher distance should be impossible
        if let Some((existing_id, existing_distance)) = map.get_mut(&pos) {
            if *existing_id == id || *existing_id == usize::MAX {
                continue;
            }

            if current_distance == *existing_distance {
                if total_area[*existing_id] != usize::MAX {
                    total_area[*existing_id] -= 1;
                }
                *existing_id = usize::MAX;
                continue;
            } else if current_distance > *existing_distance {
                // we're out of bounds, stop
                continue;
            } else {
                panic!("unexpectedly found a cell owned by another region but with higher distance")
            }
        } else {
            // map has nothing present here, add ourselves
            map.insert(pos, (id, current_distance));
            if total_area[id] != usize::MAX {
                total_area[id] += 1;
            }
        }

        let new_distance = current_distance + 1;
        // check the adjacent squares, adding only those that are unoccupied, or occupied by a different id with the same size
        for adj in pos.adjacent() {
            if let Some((adj_id, adj_dist)) = map.get(&adj) {
                // don't explore nodes already belonging to this region
                if *adj_id == id || *adj_id == usize::MAX || *adj_dist < new_distance {
                    continue;
                }
            }
            queue.push_back((id, adj, new_distance));
        }
    }

    let answer = total_area.into_iter().filter(|a| *a != usize::MAX).max().unwrap();
    assert_eq!(3722, answer);
}

#[test]
fn part2() {
    let input = input();
    let sum = input.iter().sum::<Vec2i32>();
    let origin = sum / input.len() as i32;

    let total_distance = |p0: Vec2i32| {
        input.iter().map(|p1| p0.manhattan_from(*p1)).sum::<i32>()
    };

    let mut visited: HashSet<Vec2i32> = HashSet::new();
    let mut queue: VecDeque<Vec2i32> = VecDeque::new();
    queue.push_back(origin);

    let mut total = 0;
    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        let total_manhattan = total_distance(current);
        if total_manhattan < 10000 {
            total += 1;

            for adj in current.adjacent() {
                if visited.contains(&adj) {
                    continue;
                }
                queue.push_back(adj);
            }
        }
    }

    assert_eq!(44634, total);
}