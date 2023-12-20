use std::{collections::{VecDeque, HashMap}, fmt::Debug};

use aoc_common::{file_lines, IteratorExt};

struct Signal {
    src: String,
    dst: String,
    val: bool,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.src, if self.val { "high" } else { "low" }, self.dst)?;
        Ok(())
    }
}

enum ModuleType {
    Broadcaster(),
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct Module {
    name: String,
    dsts: Vec<String>,
    mod_type: ModuleType,
}

impl Module {
    fn receive_signal(&mut self, signal: Signal, queue: &mut VecDeque<Signal>) {
        let val = match &mut self.mod_type {
            ModuleType::Broadcaster() => Some(signal.val),
            ModuleType::FlipFlop(val) => {
                if !signal.val {
                    *val = !*val;
                    Some(*val)
                } else {
                    None
                }
            }
            ModuleType::Conjunction(map) => {
                map.insert(signal.src, signal.val).unwrap();
                if map.values().all(|f| *f) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
        };

        if let Some(val) = val {
            for dst in self.dsts.iter() {
                queue.push_back(Signal { src: self.name.clone(), dst: dst.clone(), val });
            }
        }
    }
}

fn input() -> HashMap<String, Module> {
    let lines: Vec<(String, String, Vec<String>)>  = file_lines("inputs/day20.txt").map(|l| {
        let mut split = l.split(" -> ");
        let type_name = split.next().unwrap();
        let dsts = split.next().unwrap();

        let mod_type;
        let mod_name;
        if type_name == "broadcaster" {
            mod_type = String::new();
            mod_name = type_name.to_string();
        } else {
            mod_type = type_name[0..1].to_string();
            mod_name = type_name[1..].to_string();
        }

        let dsts = dsts.split(", ").map(|s| s.to_string()).to_vec();

        (mod_type, mod_name, dsts)
    }).to_vec();

    let mut map: HashMap<String, Module> = HashMap::new();
    for (mod_type, mod_name, dsts) in lines.iter() {
        let module_type = match mod_type.as_str() {
            "" if mod_name == "broadcaster" => ModuleType::Broadcaster(),
            "%" => ModuleType::FlipFlop(false),
            "&" => ModuleType::Conjunction(HashMap::new()),
            _ => panic!(),
        };

        map.insert(mod_name.clone(), Module { name: mod_name.clone(), dsts: dsts.clone(), mod_type: module_type });
    }

    for (_, mod_name, dsts) in lines.iter() {
        for dst in dsts {
            if let Some(module) = map.get_mut(dst.as_str()) {
                if let ModuleType::Conjunction(map) = &mut module.mod_type {
                    map.insert(mod_name.clone(), false);
                }
            }
        }
    }

    map
}

#[test]
fn part1() {
    let mut map = input();
    let mut queue: VecDeque<Signal> = VecDeque::new();

    const TIMES: i32 = 1000;

    let mut lows: usize = 0;
    let mut highs: usize = 0;

    for _ in 0..TIMES {
        queue.push_back(Signal { src: "button".to_string(), dst: "broadcaster".to_string(), val: false });
        while let Some(signal) = queue.pop_front() {
            if signal.val {
                highs += 1;
            } else {
                lows += 1;
            }

            if let Some(dst) = map.get_mut(signal.dst.as_str()) {
                dst.receive_signal(signal, &mut queue);
            }
        }
    }

    let answer = highs * lows;
    assert_eq!(866435264, answer);
}

#[test]
fn part2() {
    let mut map = input();
    let mut queue: VecDeque<Signal> = VecDeque::new();

    // assumes all inputs are similar to this
    let rx_source = map.values().filter(|m| m.dsts.contains(&"rx".to_string())).next().unwrap();
    let mut sources: HashMap<String, Option<usize>> = if let ModuleType::Conjunction(sources) = &rx_source.mod_type {
        sources.keys().map(|k| (k.clone(), None)).collect()
    } else {
        panic!();
    };

    let rx_source = rx_source.name.clone();

    let mut presses = 0;
    'outer: loop {
        presses += 1;
        queue.push_back(Signal { src: "button".to_string(), dst: "broadcaster".to_string(), val: false });
        while let Some(signal) = queue.pop_front() {
            if signal.val && signal.dst == rx_source {
                let root_source = sources.get_mut(&signal.src).unwrap();
                if root_source.is_none() {
                    *root_source = Some(presses);
                }

                if sources.values().all(|v| v.is_some()) {
                    break 'outer;
                }
            }

            if let Some(dst) = map.get_mut(signal.dst.as_str()) {
                dst.receive_signal(signal, &mut queue);
            }
        }
    }

    let answer: usize = sources.values().map(|v| v.unwrap()).product();
    assert_eq!(229215609826339, answer);
}