use aoc_common::{file_lines, IteratorExt, Vec2i64, PriorityQueue, Rect, RectI64};

fn input() -> Vec<(Vec2i64, Vec2i64)> {
    file_lines("inputs/day15.txt").map(|l| {
        let split = l.split([' ', ',', '=', ':']).to_vec();

        ((split[3].parse().unwrap(), split[6].parse().unwrap()).into(), (split[13].parse().unwrap(), split[16].parse().unwrap()).into())
    }).to_vec()
}

fn range_within_manhattan_on_row(p: Vec2i64, row: i64, manhattan: i64) -> Option<(i64, i64)> {
    let vertical_delta = manhattan - i64::abs(p.y - row);
    if vertical_delta < 0 {
        None
    } else {
        Some((p.x - vertical_delta, p.x + vertical_delta))
    }
}

#[test]
fn part1() {
    let input = input();

    let row = 2000000;

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for (sensor, beacon) in input.iter() {
        let manhattan = sensor.manhattan_from(*beacon);
        
        if let Some(mut range) = range_within_manhattan_on_row(*sensor, row, manhattan) {
            for existing in ranges.drain(..).to_vec() {
                if range.0 > existing.1 + 1 || range.1 + 1 < existing.0 {
                    ranges.push(existing);
                } else {
                    range.0 = range.0.min(existing.0);
                    range.1 = range.1.max(existing.1);
                }
            }
            ranges.push(range);
        }
    }

    let range_on_row = ranges.iter().map(|r| r.1 - r.0 + 1).sum::<i64>() as usize;
    let beacons_on_row = input.iter().filter_map(|(_, b)| {
        if b.y == row {
            Some(b)
        } else {
            None
        }
    }).unique().count();

    let answer = range_on_row - beacons_on_row;
    assert_eq!(answer, 5181556);
}

#[test]
fn part2() {
    let bound = 4000000;

    let input = input();

    let manhattans = input.into_iter().map(|(s, b)| (s, s.manhattan_from(b))).to_vec();

    let mut queue: PriorityQueue<RectI64, i64> = PriorityQueue::new();
    let start = Rect::from_size((bound, bound).into());
    queue.enqueue(start, start.area());

    #[inline]
    fn fully_in_range_of(rect: &RectI64, p: Vec2i64, range: i64) -> bool {
        let max = rect.corners().iter().map(|c| c.manhattan_from(p)).max().unwrap();
        max <= range
    }

    let mut answer = 0;
    while let Some(rect) = queue.dequeue() {
        if rect.is_unit() {
            answer = rect.x() * 4000000 + rect.y();
            break;
        }

        if let Some(divisions) = rect.subdivide() {
            for div in divisions.rev() {
                if manhattans.iter().all(|p| !fully_in_range_of(&div, p.0, p.1)) {
                    queue.enqueue(div, div.area());
                }
            }
        }
    }

    assert_eq!(answer, 12817603219131);
}