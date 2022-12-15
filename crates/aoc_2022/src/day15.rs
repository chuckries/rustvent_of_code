use aoc_common::{file_lines, IteratorExt, Vec2i64};


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

    for (sensor, beacon) in input {
        let manhattan = sensor.manhattan_from(beacon);
        let range = range_within_manhattan_on_row(sensor, row, manhattan);

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

    let answer: i64 = ranges.iter().map(|r| r.1 - r.0).sum();
    assert_eq!(answer, 5181556);
}

#[test]
fn part2() {
    let input = input();

    let min_y = input.iter().map(|(s, b)| s.y - s.manhattan_from(*b)).min().unwrap();
    let max_y = input.iter().map(|(s, b)| s.y + s.manhattan_from(*b)).max().unwrap();

    let mut answer: i64 = 0;
    for y in min_y.max(0)..=max_y.min(4000000) {
        let mut ranges: Vec<(i64, i64)> = Vec::new();

        for (sensor, beacon) in input.iter().copied() {
            let manhattan = sensor.manhattan_from(beacon);
            let range = range_within_manhattan_on_row(sensor, y, manhattan);

            if let Some(mut range) = range {
                range.0 = range.0.max(0);
                range.1 = range.1.min(4000000);
                for mut existing in ranges.drain(..).to_vec() {
                    existing.0 = existing.0.max(0);
                    existing.1 = existing.1.min(4000000);

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

        if ranges.len() > 1 {
            let x = ranges.iter().map(|r| r.1).min().unwrap() + 1;
            answer = x * 4000000 + y;
            break;
        }
    }

    assert_eq!(answer, 12817603219131);
}