use std::array;

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
            if word.len() <= remaining.len() && &remaining[..word.len()] == word {
                if can_build_recursive(&remaining[word.len()..], words) {
                    return true;
                }
            }
        }

        false
    }

    let total = patterns.into_iter().filter(|p| can_build_recursive(&p, &words)).count();
    assert_eq!(total, 342);
}

#[derive(Default, Clone)]
struct TrieNode {
    is_terminal: bool,
    children: Option<Box<[TrieNode; 26]>>
}

struct Trie {
    root: TrieNode
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    fn add(&mut self, word: &[u8]) {
        let mut current = &mut self.root;
        for c in word {
            current = &mut current.children.get_or_insert_with(|| Box::new(array::from_fn(|_| TrieNode::default())))[(*c - b'a') as usize];
        }
        current.is_terminal = true;
    }

    fn lookup(&self, word: &[u8]) -> Vec<usize> {
        let mut lengths = Vec::new();

        let mut i = 0;
        let mut current = &self.root;
        loop {
            if current.is_terminal {
                lengths.push(i);
            }
            if i >= word.len() {
                break;
            }
            if let Some(children) = &current.children {
                current = &children[(word[i] - b'a') as usize];
            } else {
                break;
            }
            i += 1;
        }

        lengths
    }
}

#[test]
fn part2() {
    let (words, patterns) = input();
    let mut trie = Trie::new();
    for word in words {
        trie.add(&word);
    }

    let mut total: i64 = 0;
    for pattern in patterns {
        let mut counts = vec![0; pattern.len() + 1];
        counts[0] = 1;

        for start in 0..pattern.len() {
            if counts[start] == 0 {
                continue;
            }

            for length in trie.lookup(&pattern[start..]) {
                counts[start + length] += counts[start];
            }
        }

        total += counts[pattern.len()];
    }

    assert_eq!(total, 891192814474630);
}