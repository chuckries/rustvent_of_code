use aoc_common::{file_lines, IteratorExt};

type Seed = i64;

#[derive(Debug)]
struct Range {
    src: Seed,
    dst: Seed,
    len: Seed,
}

impl Range {
    fn lookup(&self, seed: Seed) -> Option<Seed> {
        if seed >= self.src && seed <= self.src + self.len {
            Some(self.dst + (seed - self.src))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

impl Map {
    fn lookup(&self, seed: Seed) -> Seed {
        for r in self.ranges.iter() {
            if let Some(n) = r.lookup(seed) {
                return n;
            }
        }

        seed
    }
}

fn input() -> (Vec<Seed>, Vec<Map>) {
    let lines = file_lines("inputs/day05.txt").to_vec();

    let seeds = lines[0].split(": ").skip(1).next().unwrap().split_whitespace().map(|s| s.parse::<Seed>().unwrap()).to_vec();

    let mut idx = 2;
    let mut maps: Vec<Map> = Vec::new();
    while idx < lines.len() {
        let desc = lines[idx].split_whitespace().to_vec()[0].split('-').to_vec();
        let src = desc[0].to_string();
        let dst = desc[2].to_string();
        idx += 1;

        let mut ranges: Vec<Range> = Vec::new();
        loop {
            if idx >= lines.len() || lines[idx].is_empty() {
                maps.push(Map { src, dst, ranges });
                break;
            }

            let mut nums = lines[idx].split_whitespace().map(|s| s.parse::<Seed>().unwrap());
            let dst = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();

            ranges.push(Range { dst, src, len });

            idx += 1;
        }

        idx += 1;
    }

    (seeds, maps)
}

#[test]
fn part1() {
    let (seeds, maps) = input();

    let answer = seeds.iter().copied().map(|mut s| {
        for m in maps.iter() {
            s = m.lookup(s);
        }
        s
    }).min().unwrap();

    assert_eq!(340994526, answer);
}