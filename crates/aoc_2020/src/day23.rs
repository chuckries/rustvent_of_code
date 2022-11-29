use aoc_common::ToVec;

const INPUT: &'static str = "137826495";

fn input() -> (usize, Vec<usize>) {
    let mut list = vec![0; INPUT.len() + 1];

    let nums = INPUT.bytes().map(|b| (b - b'0') as usize).to_vec();
    for i in 0..nums.len() - 1 {
        list[nums[i]] = nums[i + 1];
    }
    list[nums[nums.len() - 1]] = nums[0];

    (nums[0], list)
}

fn run(list: &mut [usize], start: usize, iterations: usize) {
    let mut current = start;
    let max = list.len() - 1;

    for _ in 0..iterations {
        let a = list[current];
        let b = list[a];
        let c = list[b];

        let mut destination = current;
        loop {
            destination -= 1;
            if destination == 0 {
                destination = max
            }

            if !(destination == a || destination == b || destination == c) {
                break;
            }
        }

        list[current] = list[c];
        list[c] = list[destination];
        list[destination] = a;
        current = list[current];
    }
}

#[test]
fn part1 () {
    let (current, mut list) = input();

    run(&mut list, current, 100);

    let mut answer = String::new();
    let mut current = list[1];
    while current != 1 {
        answer.push((current as u8 + b'0') as char);
        current = list[current];
    }

    assert_eq!(answer, "59374826");
}

#[test]
fn part2() {
    let (current, mut list) = input();

    let mut tmp = current;
    while list[tmp] != current {
        tmp = list[tmp];
    }
    list[tmp] = list.len();

    let range = list.len() + 1..=1000000;
    list.extend(range.chain(std::iter::once(current)));

    run(&mut list, current, 10000000);
    let a = list[1];
    let b = list[a];

    let answer = a * b;
    assert_eq!(answer, 66878091588);
}