use aoc_common::{file_string, IteratorExt};


struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn from_nums_iter<T: Iterator<Item = i32>>(iter: &mut T) -> Self {
        let count_children = iter.next().unwrap();
        let count_metadata = iter.next().unwrap();

        let mut children = Vec::with_capacity(count_children as usize);
        for _ in 0..count_children {
            children.push(Self::from_nums_iter(iter));
        }

        let metadata = iter.take(count_metadata as usize).to_vec();

        Node {
            children,
            metadata
        }
    }

    fn total_metadata(&self) -> i32 {
        self.metadata.iter().sum::<i32>() + self.children.iter().map(|c| c.total_metadata()).sum::<i32>()
    }

    fn value(&self) -> i32 {
        if self.children.len() == 0 {
            self.metadata.iter().sum()
        } else {
            let mut value = 0;
            for i in self.metadata.iter().copied() {
                let i = i as usize;
                if i > 0 && i <= self.children.len() {
                    value += self.children[i - 1].value()
                }
            }
            value
        }
    }
}


fn input() -> Node {
    let input = file_string("inputs/day08.txt");
    let mut nums_iter = input.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap());
    Node::from_nums_iter(&mut nums_iter)
}

#[test]
fn part1() {
    let answer = input().total_metadata();
    assert_eq!(43825, answer);
}

#[test]
fn part2() {
    let answer  = input().value();
    assert_eq!(19276, answer);
}