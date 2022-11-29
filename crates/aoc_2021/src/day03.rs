use aoc_common::file_lines;

fn input() -> Vec<Vec<char>> {
    file_lines("inputs/day03.txt").map(|l| l.chars().collect()).collect()
}

#[test]
fn part1() {
    let input: Vec<Vec<char>> = input();
    let length = input[0].len();

    let mut gamma = 0;
    for bit in 0..length {
        let count = input.iter().filter(|l| l[bit] == '1').count();
        if count > input.len() / 2 {
            gamma |= 1 << length - bit - 1;
        }
    }

    let epsilon = !gamma & ((1 << length) - 1);

    assert_eq!(gamma * epsilon, 1307354);
}

#[test]
fn part2() {
    let input = input();
    let mut oxygen: Vec<&[char]> = input.iter().map(|l| l.as_slice()).collect();
    let mut co2 = oxygen.clone();

    let len = oxygen[0].len();

    for bit in 0..len {
        if oxygen.len() > 1 {
            let ones = oxygen.iter().filter(|l| l[bit] == '1').count();
            let target = if ones >= oxygen.len() - ones {
                '1'
            } else {
                '0'
            };
            oxygen = oxygen.iter().filter_map(|l| if l[bit] == target { Some(*l) } else { None }).collect();
        }

        if co2.len() > 1 {
            let zeroes = co2.iter().filter(|l| l[bit] == '0').count();
            let target = if zeroes <= co2.len() - zeroes {
                '0'
            } else {
                '1'
            };
            co2 = co2.iter().filter_map(|l| if l[bit] == target { Some(*l) } else { None }).collect();
        }
    }

    let oxygen = oxygen[0].iter().collect::<String>();
    let co2 = co2[0].iter().collect::<String>();
    let oxygen = i32::from_str_radix(&oxygen, 2).unwrap();
    let c02 = i32::from_str_radix(&co2, 2).unwrap();

    assert_eq!(oxygen * c02, 482500);
}