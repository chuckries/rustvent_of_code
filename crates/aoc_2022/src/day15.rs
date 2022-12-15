use aoc_common::{file_lines, IteratorExt, Vec2i64, PriorityQueue};

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
        let range = range_within_manhattan_on_row(*sensor, row, manhattan);

        if let Some(mut range) = range {
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
    }).to_set().len();

    let answer = range_on_row - beacons_on_row;
    assert_eq!(answer, 5181556);
}

#[derive(Copy, Clone)]
struct Rect(Vec2i64, Vec2i64);

impl Rect {
    fn fully_in_range_of(&self, p: Vec2i64, range: i64) -> bool {
        let max = [(self.0.x, self.0.y), (self.0.x, self.1.y), (self.1.x, self.0.y), (self.1.x, self.1.y)]
            .iter().map(|(x, y)| p.manhattan_from((*x, *y).into())).max().unwrap();
        
        max <= range
    }

    fn subdivide(&self) -> Option<impl Iterator<Item = Rect>> {
        if self.is_unit() {
            None
        } else {
            let mut divisions: Vec<Rect> = Vec::with_capacity(4);

            let split_x = self.0.x < self.1.x;
            let split_y = self.0.y < self.1.y;

            let mid_x = self.0.x + (self.1.x - self.0.x) / 2;
            let mid_y = self.0.y + (self.1.y - self.0.y) / 2;

            if split_x && split_y { divisions.push(Self((mid_x + 1, mid_y + 1).into(), (self.1.x, self.1.y).into())) }
            if split_y { divisions.push(Self((self.0.x, mid_y + 1).into(), (mid_x, self.1.y).into())) }
            if split_x { divisions.push(Self((mid_x + 1, self.0.y).into(), (self.1.x, mid_y).into())) }
            divisions.push(Self((self.0.x, self.0.y).into(), (mid_x, mid_y).into()));

            Some(divisions.into_iter())
        }
    }

    fn area(&self) -> i64 {
        (self.1.x - self.0.x + 1) * (self.1.y - self.0.y + 1)
    }

    fn is_unit(&self) -> bool {
        self.0.x == self.1.x && self.0.y == self.1.y
    }
}

#[test]
fn part2() {
    let bound = 4000000;

    let input = input();

    let manhattans = input.into_iter().map(|(s, b)| (s, s.manhattan_from(b))).to_vec();

    let mut queue: PriorityQueue<Rect, i64> = PriorityQueue::new();
    let start = Rect((0, 0).into(), (bound, bound).into());
    queue.enqueue(start, start.area());

    let mut answer = 0;
    while let Some(rect) = queue.dequeue() {
        if rect.is_unit() {
            answer = rect.0.x * 4000000 + rect.0.y;
            break;
        }

        if let Some(divisions) = rect.subdivide() {
            for div in divisions {
                if manhattans.iter().all(|p| !div.fully_in_range_of(p.0, p.1)) {
                    queue.enqueue(div, div.area());
                }
            }
        }
    }

    assert_eq!(answer, 12817603219131);
}