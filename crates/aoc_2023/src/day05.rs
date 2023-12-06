use aoc_common::{file_string, IteratorExt};

#[derive(Copy, Clone, Debug)]
struct Range {
    src_begin: i64,
    src_end: i64,
    dst: i64,
}

impl Range {
    fn new(src: i64, dst: i64, len: i64) -> Range {
        Range {
            src_begin: src,
            src_end: src + len - 1,
            dst: dst
        }
    }

    fn lookup(&self, i64: i64) -> Option<i64> {
        if i64 >= self.src_begin && i64 <= self.src_end {
            Some(self.dst + (i64 - self.src_begin))
        } else {
            None
        }
    }

    fn lookup_range(&self, mut r: Self) -> (Vec<Self>, Vec<Self>) {
        let mut mapped: Vec<Self> = Vec::new();
        let mut unmapped: Vec<Self> = Vec::new();

        if r.src_end < self.src_begin || self.src_end < r.src_begin {
            unmapped.push(r.clone());
        } else {
            if r.src_begin < self.src_begin {
                let unmapped_range = Range {
                    src_begin: r.src_begin,
                    src_end: self.src_begin - 1,
                    dst: 0
                };

                r.src_begin = self.src_begin;
                unmapped.push(unmapped_range);
            }

            if r.src_end > self.src_end {
                let unmapped_range = Range {
                    src_begin: self.src_end + 1,
                    src_end: r.src_end,
                    dst: 0,
                };

                r.src_end = self.src_end;
                unmapped.push(unmapped_range);
            }

            r.src_begin = self.lookup(r.src_begin).unwrap();
            r.src_end = self.lookup(r.src_end).unwrap();
            mapped.push(r);
        }

        (unmapped, mapped)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn lookup(&self, i64: i64) -> i64 {
        for r in self.ranges.iter() {
            if let Some(n) = r.lookup(i64) {
                return n;
            }
        }

        i64
    }

    fn lookup_range(&self, r: Range) -> Vec<Range> {
        let mut mapped: Vec<Range> = Vec::new();
        let mut unmapped: Vec<Range> = Vec::new();

        unmapped.push(r.clone());

        for range in self.ranges.iter() {
            let mut next_unmapped: Vec<Range> = Vec::new();
            for unampped_range in unmapped.iter() {
                let (mut new_unmapped, mut new_mapped) = range.lookup_range(unampped_range.clone());
                mapped.append(&mut new_mapped);
                next_unmapped.append(&mut new_unmapped);
            }

            unmapped = next_unmapped;
        }

        mapped.append(&mut unmapped);

        mapped
    }
}

fn input() -> (Vec<i64>, Vec<Map>) {
    let input = file_string("inputs/day05.txt");
    let mut chunks = input.split("\r\n\r\n");

    let i64s = chunks.next().unwrap().split(": ").skip(1).next().unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).to_vec();

    let mut maps: Vec<Map> = Vec::new();
    for mut lines in chunks.map(|c| c.split("\r\n")) {
        let _desc = lines.next().unwrap().split_whitespace().to_vec()[0].split('-').to_vec();

        let mut ranges: Vec<Range> = Vec::new();
        for l in lines {
            let mut nums = l.split_whitespace().map(|s| s.parse::<i64>().unwrap());
            let dst = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();

            ranges.push(Range::new(src, dst, len));
        }

        maps.push(Map { ranges });
    }

    (i64s, maps)
}

#[test]
fn part1() {
    let (i64s, maps) = input();

    let answer = i64s.iter().copied().map(|mut s| {
        for m in maps.iter() {
            s = m.lookup(s);
        }
        s
    }).min().unwrap();

    assert_eq!(340994526, answer);
}

#[test]
fn part2() {
    let (i64s, maps) = input();

    let mut seed_ranges = i64s.chunks_exact(2).map(|c| {
        Range::new(c[0], 0, c[1])
    }).to_vec();

    for m in maps.iter() {
        let mut next_seed_ranges: Vec<Range> = Vec::new();
        for seed_range in seed_ranges.iter() {
            let mut next_seed_range = m.lookup_range(seed_range.clone());
            next_seed_ranges.append(&mut next_seed_range);
        }
        seed_ranges = next_seed_ranges;
    }

    let answer = seed_ranges.iter().map(|s| s.src_begin).min().unwrap();

    assert_eq!(52210644, answer);
}