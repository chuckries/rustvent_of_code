use std::{collections::HashMap};

use aoc_common::{file_string, IteratorExt};


fn input() -> String {
    file_string("inputs/day06.txt")
}

// too good not to keep
// fn find(n: usize) -> usize {
//     input()
//         .chars()
//         .enumerate()
//         .to_vec()
//         .windows(n)
//         .filter(|w| 
//             w.iter()
//                 .map(|p| p.1)
//                 .to_set()
//                 .len() == n
//             )
//         .next()
//         .unwrap()[n - 1].0 + 1
// }

fn find(n: usize) -> usize {
    let chars = input().chars().to_vec();
    let mut map: HashMap<char, usize> = HashMap::new();

    for i in 0..n {
        *map.entry(chars[i]).or_default() += 1;
    }

    let mut front = 0;
    let mut back = n;

    loop {
        if map.values().all(|v| *v == 0 || *v == 1) {
            break;
        }

        *map.get_mut(&chars[front]).unwrap() -=1 ;
        *map.entry(chars[back]).or_default() += 1;

        front += 1;
        back += 1;
    }

    back
}

#[test]
fn part1() {
    let answer = find(4);

    assert_eq!(answer, 1542);
}

#[test]
fn part2() {
    let answer = find(14);

    assert_eq!(answer, 3153);
}