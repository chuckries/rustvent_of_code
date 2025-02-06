use std::{cell::Cell, default};

use aoc_common::{file_lines, IdMap, IteratorExt};

#[derive(Clone, Copy, Default)]
enum Op {
    #[default]
    And,
    Or,
    Xor,
}

#[derive(Default)]
struct Gate {
    cached: Cell<Option<bool>>,
    a: usize,
    b: usize,
    op: Op,
}

impl Gate {
    fn from_value(val: bool) -> Self {
        Self {
            cached: Cell::new(Some(val)),
            a: 0,
            b: 0,
            op: Op::And
        }
    }

    fn from_gate(a: usize, b: usize, op: Op) -> Self {
        Self {
            cached: Cell::new(None),
            a,
            b,
            op
        }
    }

    fn resolve(&self, gates: &[Gate]) -> bool {
        if let Some(cached) = self.cached.get() {
            return cached;
        }

        let a = gates[self.a].resolve(gates);
        let b = gates[self.b].resolve(gates);
        let val = match self.op {
            Op::And => a && b,
            Op::Or => a || b,
            Op::Xor => a ^ b,
        };
        //self.cached.set(Some(val));
        val
    }
}

fn input() -> (IdMap, Vec<Gate>) {
    let mut lines = file_lines("inputs/day24.txt");

    let mut id_map = IdMap::new();
    let mut gates: Vec<Gate> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() { break; }

        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let val = split.next().unwrap();

        id_map.get_or_insert(name);
        gates.push(Gate::from_value(val == "1"));
    }

    for line in lines {
        let mut split = line.split(' ');
        let a = split.next().unwrap();
        let op = split.next().unwrap();
        let b = split.next().unwrap();
        split.next().unwrap();
        let c = split.next().unwrap();

        let a = id_map.get_or_insert(a);
        if a == gates.len() {
            gates.push(Gate::default());
        }

        let b = id_map.get_or_insert(b);
        if b == gates.len() {
            gates.push(Gate::default());
        }

        let c = id_map.get_or_insert(c);
        if c == gates.len() {
            gates.push(Gate::default());
        }

        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!()
        };

        gates[c] = Gate::from_gate(a, b, op);
    }

    (id_map, gates)
}

#[test]
fn part1() {
    let (id_map, gates) = input();

    let zs = id_map.iter().filter(|(name, _)| name.starts_with("z")).sorted_by_cached_key(|(name, _)| *name).map(|(_, idx)| {
        gates[*idx].resolve(&gates)
    }).to_vec();

    let mut num: u64 = 0;
    let mut bit = 1;
    for z in zs {
        if z {
            num |= bit;
        }
        bit <<= 1;
    }

    assert_eq!(num, 55730288838374);
}