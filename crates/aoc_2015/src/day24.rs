use aoc_common::{file_lines_as, IteratorExt};

fn input() -> Vec<i32> {
    file_lines_as("inputs/day24.txt").collect()
}

fn combinations(nums: &[i32], count: usize) -> Vec<Vec<i32>> {
    fn make_combo(indices: &[usize], nums: &[i32]) -> Vec<i32> {
        indices.iter().map(|i| nums[*i]).to_vec()
    }
    
    let mut answers = Vec::new();
    let mut indices: Vec<usize> = (0..count).to_vec();

    answers.push(make_combo(&indices, nums));

    loop {
        let mut i: i32 = (count - 1) as i32;
        loop {
            if i < 0 {
                break;
            }

            if indices[i as usize] != i as usize + nums.len() - count {
                break;
            }

            i -= 1;
        }
        if i < 0 {
            break;
        }

        indices[i as usize] += 1;
        for j in i as usize + 1 .. count {
            indices[j as usize] = indices[j as usize - 1] + 1;
        }

        answers.push(make_combo(&indices, nums));
    }

    answers
}

fn run(groups: i32) -> i64 {
    let numbers = input();
    let sum = numbers.iter().sum::<i32>();
    let target = sum / groups;

    let mut count = 1; 
    let mut answers: Vec<Vec<i32>> = Vec::new();
    loop {
        answers.clear();
        for combo in combinations(&numbers, count) {
            if combo.iter().sum::<i32>() == target {
                answers.push(combo)
            }
        }

        if answers.len() > 0 {
            break;
        }

        count += 1;
    }

    answers.into_iter().map(|v| v.into_iter().map(|n| n as i64).product()).min().unwrap()
}

#[test]
fn part1() {
    let answer = run(3);
    assert_eq!(answer, 11846773891);
}

#[test]
fn part2() {
    let answer = run(4);
    assert_eq!(answer, 80393059);
}