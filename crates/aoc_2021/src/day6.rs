use aoc_common::file_string;

fn run(iterations: usize) -> usize {
    let input: Vec<usize> = file_string("inputs/day6.txt").split(',').map(|s| s.parse().unwrap()).collect();

    let mut fish = [0; 9];
    let mut next = fish.clone();

    for i in input {
        fish[i] += 1;
    }

    for _ in 0..iterations {
        for i in 0..8 {
            next[i] = fish[i + 1];
        }

        next[6] += fish[0];
        next[8] = fish[0];

        std::mem::swap(&mut fish, &mut next);
    }

    fish.iter().sum()
}

#[test]
fn part1() {
    assert_eq!(run(80), 386755);
}

#[test]
fn part2() {
    assert_eq!(run(256), 1732731810807);
}