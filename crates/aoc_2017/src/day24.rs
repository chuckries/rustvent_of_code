use std::{str::FromStr};

use aoc_common::{IteratorExt, file_lines_as};

#[derive(Debug)]
struct Connector {
    a: usize,
    b: usize,
}

impl Connector {
    fn try_connect(&self, to: usize) -> Option<usize> {
        if to == self.a {
            Some(self.b)
        } else if to == self.b {
            Some(self.a)
        } else {
            None
        }
    }
}

impl FromStr for Connector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides = s.split('/').map(|n| n.parse::<usize>().unwrap()).to_vec();
        Ok(Connector
        {
            a: sides[0],
            b: sides[1],
        })
    }
}

#[derive(Debug)]
struct Bridge {
    current: usize,
    strength: usize,
    length: usize
}

struct BridgeBuilder {
    connectors: Vec<Connector>,
}

impl BridgeBuilder {
    fn build(&self, f: &mut impl FnMut(&Bridge)) {
        let mut bridge = Bridge {
            current: 0,
            strength: 0,
            length: 0
        };
        let mut used = vec![false; self.connectors.len()];
        self.build_recurse(&mut bridge, &mut used, f);
    }

    fn build_recurse(&self, bridge: &mut Bridge, used: &mut [bool], f: &mut impl FnMut(&Bridge)) {
        let mut has_connection = false;

        for (idx, connector) in self.connectors.iter().enumerate() {
            if !used[idx] {
                if let Some(next) = connector.try_connect(bridge.current) {
                    let previous = bridge.current;

                    bridge.length += 1;
                    bridge.strength += connector.a + connector.b;
                    bridge.current = next;
                    used[idx] = true;
                    has_connection = true;

                    self.build_recurse(bridge, used, f);

                    bridge.length -=1;
                    bridge.strength -= connector.a + connector.b;
                    bridge.current = previous;
                    used[idx] = false;
                }
            }
        }

        if !has_connection {
            f(&bridge);
        }
    }
}

fn input() -> BridgeBuilder {
    BridgeBuilder {
        connectors: file_lines_as("inputs/day24.txt").to_vec()
    }
}

#[test]
fn part1() {
    let builder = input();

    let mut max = 0;
    let mut f = |b: &Bridge| {
        if b.strength > max {
            max = b.strength;
        }
    };
    builder.build(&mut f);

    assert_eq!(max, 1695);
}

#[test]
fn part2() {
    let builder = input();

    let mut max_len = 0;
    let mut max_strength = 0;
    let mut f = |b: &Bridge| {
        if b.length >= max_len {
            max_len = b.length;
            if b.strength > max_strength {
                max_strength = b.strength;
            }
        }
    };
    builder.build(&mut f);

    assert_eq!(max_strength, 1673);
}