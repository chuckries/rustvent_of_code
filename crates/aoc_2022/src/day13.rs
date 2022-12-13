use std::{str::FromStr, cmp::Ordering, rc::Rc, iter::Peekable};
use aoc_common::{file_lines, IteratorExt};

enum Node {
    List(Vec<Node>),
    Int(i32),
}

impl Node {
    fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Vec<Node> {
        if chars.next().unwrap() != '[' {
            panic!();
        }

        let mut list: Vec<Node> = Vec::new();
        loop {
            match chars.peek().unwrap() {
                ']' => {
                    chars.next().unwrap();
                    break;
                }
                '[' => list.push(Node::List(Self::parse(chars))),
                ',' => {
                    chars.next().unwrap();
                }
                c if c.is_digit(10) => {
                    let mut s = c.to_string();
                    chars.next().unwrap();
                    while chars.peek().unwrap().is_digit(10) {
                        s.push(chars.next().unwrap());
                    }
                    list.push(Node::Int(s.parse().unwrap()));
                }
                _ => panic!()
            }
        }

        list
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
                a.cmp(b)
            }
            (Node::List(a), Node::List(b)) => {
                a.cmp(b)
            }
            (a @ Node::List(_), Node::Int(b)) => {
                a.cmp(&Node::List(vec![Node::Int(*b)]))
            }
            (Node::Int(a), b @ Node::List(_)) => {
                Node::List(vec![Node::Int(*a)]).cmp(b)
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Packet(Rc<Vec<Node>>);

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Packet(Rc::new(Node::parse(&mut s.chars().peekable()))))
    }
}


fn input() -> Vec<Packet>{
    file_lines("inputs/day13.txt").filter(|l| !l.is_empty()).map(|l| l.parse().unwrap()).to_vec()
}

#[test]
fn part1() {
    let answer: usize = input().chunks(2).enumerate().filter_map(|(idx, packets)| {
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

    let divider2: Packet = "[[2]]".parse().unwrap();
    let divider6: Packet = "[[6]]".parse().unwrap();

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