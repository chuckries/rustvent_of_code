use aoc_common::file_string;

fn input() -> String {
    file_string("inputs/day15.txt")
}

fn hash(s: &str) -> usize {
    let mut current = 0;
    for b in s.bytes() {
        current += b as usize;
        current *= 17;
        current %= 256;
    }

    current
}

#[test]
fn part1() {
    let answer: usize = input().split(',').map(|s| hash(s)).sum();

    assert_eq!(508552, answer);
}

#[test]
fn part2() {
    let input = input();
    let mut map: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    for s in input.split(',') {
        if s.ends_with('-') {
            let label = &s[.. s.len() - 1];
            let hash = hash(label);
            map[hash].retain(|kvp| kvp.0 != label);
        }
        else {
            let mut split = s.split('=');
            let label = split.next().unwrap();
            let value: usize = split.next().unwrap().parse().unwrap();
            let hash = hash(label);
            let bucket = &mut map[hash];

            if let Some(kvp) = bucket.iter_mut().find(|kvp| kvp.0 == label) {
                kvp.1 = value;
            } else {
                bucket.push((label, value));
            }
        }
    }

    let mut total = 0;
    for (bucket_idx, bucket) in map.iter().enumerate() {
        for (slot_idx, slot) in bucket.iter().enumerate() {
            total += (bucket_idx + 1) * (slot_idx + 1) * slot.1;
        }
    }

    assert_eq!(265462, total);
}