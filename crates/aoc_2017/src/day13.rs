use aoc_common::file_lines;

struct Scanner {
    depth: usize,
    range: usize,
    modulo: usize,
}

impl Scanner {
    fn new(depth: usize, range: usize) -> Self {
        Self {
            depth,
            range,
            modulo: (range - 1) * 2,
        }
    }

    fn is_at_pos_zero_from_initial_time(&self, t: usize) -> bool {
        (t + self.depth) % self.modulo == 0
    }
}

fn input() -> Vec<Scanner> {
    file_lines("inputs/day13.txt").map(|l| {
        let mut split = l.split(": ");
        let depth = split.next().unwrap().parse::<usize>().unwrap();
        let range = split.next().unwrap().parse::<usize>().unwrap();
        Scanner::new(depth, range)
    }).collect()
}

#[test]
fn part1() {
    let scanners = input();
    let answer = scanners.iter().filter(|s| s.is_at_pos_zero_from_initial_time(0)).map(|s| s.depth * s.range).sum::<usize>();
    assert_eq!(answer, 1704);
}

#[test]
fn part2() {
    let scanners = input();
    let mut t = 0;

    loop {
        if scanners.iter().any(|s| s.is_at_pos_zero_from_initial_time(t)) {
            t += 1;
            continue;
        }

        break;
    }

    assert_eq!(t, 3970918);
}