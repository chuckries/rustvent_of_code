use aoc_common::file_lines;

fn input() -> Vec<String> {
    file_lines("inputs/day10.txt").collect()
}

fn process<C, I>(line: &str, corrupted: &mut C, incomplete: &mut I)
    where C: FnMut(char), I: FnMut(Vec<char>)
{
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => if *stack.last().unwrap() == '(' { stack.pop(); } else { corrupted(c); return; }
            ']' => if *stack.last().unwrap() == '[' { stack.pop(); } else { corrupted(c); return; }
            '}' => if *stack.last().unwrap() == '{' { stack.pop(); } else { corrupted(c); return; }
            '>' => if *stack.last().unwrap() == '<' { stack.pop(); } else { corrupted(c); return; }
            _ => panic!()
        }
    }

    incomplete(stack);
}

#[test]
fn part1() {
    let lines = input();
    
    let mut total = 0;
    let mut corrupted = |c| { 
        total += match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!()
        };
    };
    for line in lines {
        process(&line, &mut corrupted, &mut |_| { });
    }

    assert_eq!(total, 343863);
}

#[test]
fn part2() {
    let lines = input();

    let mut scores: Vec<usize> = Vec::new();
    let mut incomplete = |stack: Vec<char>| {
        let mut current = 0;
        for c in stack.iter().rev() {
            let score = match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!()
            };
            current = current * 5 + score;
        }
        scores.push(current);
    };

    for line in lines {
        process(&line, &mut |_| { }, &mut incomplete);
    }

    scores.sort();
    assert_eq!(scores[scores.len() / 2], 2924734236);
}