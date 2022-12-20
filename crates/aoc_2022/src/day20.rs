
// I decided to tackle this one in C#, as it is heavily LinkedList based and rusts built in LinkedList is garbage.
// Rusts ownership rules also make this diffuclt to do the way I wanted:
//  - keep a vector of LinkedListNode in the original order
//  - insert these in the original order into a LinkedList
//  - now I can iterate over the nodes in the original order but still operate on them in the list, 
//      the removals and insertions are constant and and I never have to spend time "finding" something
//
// I'd like to get around to trying this in rust, but it feeeeeels like it's headed towards Rc<RefCell> territory and I'd need to roll my own LinkedList.
// Maybe someday...
//
// C# Solution: https://github.com/chuckries/AdventOfCode/blob/main/AdventOfCode.2022/Day20.cs


// Here I'll try a naive approach just using a Vec, pay for the find and buffer copies, and see how it goes
//
// Afterthoughts: 
// I'm surprised (and a little annoyed) that just using Vecs and dealing with all the memcpy from remove/insert is
// still better than LinkedList, but I suppose the cost of iterating through list items one at a time is just too slow
// 
// Working on it this way did expose the big "trap" in this question though, which was intersting. The same number
// appears multiple times in the input list, but they must maintain identity. I struggled with this for a long time by
// finding and moving the first instance in the mix list but this wasn't always the correct one to move.
// I solved this by storing the number with it's original order idx so I could track identity. I think it's really
// funny that I managed to dodge this trap entirely in .NET by working with LinkedListNodes which already have identity.
// The numbers themselves were nearly meaningless in the C# approach.

use aoc_common::{file_lines, IteratorExt};

fn input(multiplier: i64) -> Vec<i64> {
    file_lines("inputs/day20.txt").map(|l| l.parse::<i64>().unwrap() * multiplier).to_vec()
}

#[inline]
fn safe_mod(lhs: i64, rhs: i64) -> i64 {
    ((lhs % rhs) + rhs) % rhs
}

fn mix(order: &[i64], times: usize) -> Vec<i64> {
    let mut list = order.iter().copied().enumerate().to_vec();

    for _ in 0..times {
        for item in order.iter().copied().enumerate() {
            if item.1 != 0 {
                let idx = list.iter().enumerate().filter_map(|(idx, cand)| {
                    if item == *cand {
                        Some(idx)
                    } else {
                        None
                    }
                }).next().unwrap();

                list.remove(idx);
                let new_idx = safe_mod(idx as i64 + item.1, list.len() as i64) as usize;
                list.insert(new_idx, item);
            }
        }
    }

    list.into_iter().map(|(_, val)| val).to_vec()
}

fn sum(list: &Vec<i64>) -> i64 {
    let mut idx = list.iter().enumerate().filter_map(|(idx, val)| if *val == 0 { Some(idx) } else { None }).next().unwrap();
    let mut sum = 0;
    for _ in 0..3 {
        idx = (idx + 1000) % list.len();
        sum += list[idx];
    }
    sum
}

#[test]
fn part1() {
    let input = input(1);

    let list = mix(&input, 1);
    let answer = sum(&list);

    assert_eq!(answer, 988);
}

#[test]
fn part2() {
    let input = input(811589153);

    let list = mix(&input, 10);
    let answer = sum(&list);

    assert_eq!(answer, 7768531372516);
}