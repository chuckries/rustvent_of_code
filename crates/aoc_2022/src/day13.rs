use std::{str::FromStr, cmp::Ordering};
use aoc_common::{file_lines};

#[derive(Clone)]
enum Node {
    List(Vec<Node>),
    Int(i32),
}

impl Node {
    fn as_list_mut(&mut self) -> &mut Vec<Node> {
        match self {
            Node::List(v) => v,
            _ => panic!()
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Node { }

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Node::Int(a), Node::Int(b)) => {
                return a.cmp(b);
            }
            (Node::List(a), Node::List(b)) => {
                let mut i = 0;
                loop {
                    if i == a.len() && i == b.len() {
                        return Ordering::Equal
                    } else if i == a.len() {
                        return Ordering::Less;
                    } else if i == b.len() {
                        return Ordering::Greater;
                    } else {
                        let cmp = a[i].cmp(&b[i]);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                        i += 1;
                    }
                }
            }
            (a @ Node::List(_), Node::Int(b)) => {
                let right = Node::List(vec![Node::Int(*b)]);
                return a.cmp(&right);
            }
            (Node::Int(a), b @ Node::List(_)) => {
                let left = Node::List(vec![Node::Int(*a)]);
                return left.cmp(b);
            }
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Node> = Vec::new();
        stack.push(Node::List(Vec::new()));

        let mut chars = s.bytes().peekable();

        while let Some(c) = chars.next() {
            match c {
                b'[' => stack.push(Node::List(Vec::new())),
                b']' => {
                    let top = stack.pop().unwrap();
                    stack.last_mut().unwrap().as_list_mut().push(top);
                }
                c if c.is_ascii_digit() => {
                    let mut bytes: Vec<u8> = vec![c];
                    while let Some(c) = chars.peek() {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        bytes.push(*c);
                        chars.next();
                    }

                    stack.last_mut().unwrap().as_list_mut().push(Node::Int(String::from_utf8(bytes).unwrap().parse().unwrap()));
                }
                b',' => (),
                _ => panic!()
            }
        }

        Ok(stack.into_iter().next().unwrap())
    }
}

fn input() -> Vec<Node>{
    let mut packets = Vec::new();
    for line in file_lines("inputs/day13.txt") {
        if !line.is_empty() {
            packets.push(line.parse().unwrap());
        }
    }

    packets
}

#[test]
fn part1() {
    let packets = input();

    let answer: usize = packets.chunks(2).enumerate().filter_map(|(idx, packets)| {
        if packets[0] < packets[1] {
            Some(idx + 1)
        } else {
            None
        }
    }).sum();

    assert_eq!(answer, 5292);
}

#[test]
fn part2() {
    let mut packets = input();

    let divider2: Node = "[[2]]".parse().unwrap();
    let divider6: Node = "[[6]]".parse().unwrap();

    packets.push(divider2.clone());
    packets.push(divider6.clone());

    packets.sort();

    let mut answer = 1;
    for (idx, n) in packets.iter().enumerate() {
        if *n == divider2 || *n == divider6 {
            answer *= idx + 1;
        }
    }

    assert_eq!(answer, 23868);
}