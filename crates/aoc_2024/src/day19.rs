use aoc_common::file_lines;

fn input() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut lines = file_lines("inputs/day19.txt");
    let towels = lines.next().unwrap().split(", ").map(|s| s.bytes().collect()).collect();
    lines.next().unwrap();
    let patterns = lines.map(|l| l.into_bytes()).collect();
    (towels, patterns)
}

#[test]
fn part1() {
    let (words, patterns) = input();

    fn can_build_recursive(remaining: &[u8], words: &[Vec<u8>]) -> bool {
        if remaining.len() == 0 {
            return true;
        }

        for word in words {
            if word.len() <= remaining.len() && &remaining[..word.len()] == word.as_slice() {
                if can_build_recursive(&remaining[word.len()..], words) {
                    return true;
                }
            }
        }

        false
    }

    let total = patterns.into_iter().filter(|p| can_build_recursive(&p, &words)).count();
    assert_eq!(total, 0);
}

#[test]
fn part2() {
    let (words, patterns) = input();

    fn can_build_recursive(remaining: &[u8], words: &[Vec<u8>], total: &mut i32) {
        if remaining.len() == 0 {
            *total += 1;
            return;
        }

        for word in words {
            if word.len() <= remaining.len() && &remaining[..word.len()] == word.as_slice() {
                can_build_recursive(&remaining[word.len()..], words, total);
            }
        }
    }

    let mut total = 0;
    for pattern in patterns {
        can_build_recursive(&pattern, &words, &mut total);
    }
    assert_eq!(total, 0);
}