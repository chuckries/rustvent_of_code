use std::{collections::{HashMap, VecDeque}};

use aoc_common::{file_lines, IteratorExt};

struct Blueprint {
    id: usize,
    ore_robot_ore: usize,
    clay_robot_ore: usize,
    obsidian_robot_ore: usize,
    obsidian_robot_clay: usize,
    geode_robot_ore: usize,
    geode_robot_obsidian: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
struct CacheKey {
    ore: usize,
    clay: usize,
    obsidian: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

type Cache = HashMap<CacheKey, usize>;

#[derive(Copy, Clone, Default)]
struct State {
    steps: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

impl State {
    fn tick_steps(&mut self, steps: usize) {
        self.ore += steps * self.ore_bots;
        self.clay += steps * self.clay_bots;
        self.obsidian += steps * self.obsidian_bots;
        self.geodes += steps * self.geode_bots;
    }
}

impl Blueprint {
    fn max(&self, steps: usize) -> usize {
        let mut table = vec![Cache::new(); steps + 1];
        let mut start_state = CacheKey::default();
        start_state.ore_bots = 1;
        table[0].insert(start_state, 0);

        for i in 0..steps {
            for (state, geodes) in table[i].iter().map(|(s, g)| (*s, *g)).to_vec() {
                let mut next_state = state;
                next_state.ore += next_state.ore_bots;
                next_state.clay += next_state.clay_bots;
                next_state.obsidian += next_state.obsidian_bots;

                if state.ore >= self.geode_robot_ore && state.obsidian >= self.geode_robot_obsidian {
                    let mut next_state = next_state;
                    next_state.ore -= self.geode_robot_ore;
                    next_state.obsidian -= self.geode_robot_obsidian;
                    next_state.geode_bots += 1;

                    let val = table[i + 1].entry(next_state).or_default();
                    if geodes + state.geode_bots > *val {
                        *val = geodes + state.geode_bots;
                    }
                } else if state.ore >= self.obsidian_robot_ore && state.clay >= self.obsidian_robot_clay {
                    let mut next_state = next_state;
                    next_state.ore -= self.obsidian_robot_ore;
                    next_state.clay -= self.obsidian_robot_clay;
                    next_state.obsidian_bots += 1;

                    let val = table[i + 1].entry(next_state).or_default();
                    if geodes + state.geode_bots > *val {
                        *val = geodes + state.geode_bots;
                    }
                } else {
                    if state.ore >= self.ore_robot_ore {
                        let mut next_state = next_state;
                        next_state.ore -= self.ore_robot_ore;
                        next_state.ore_bots += 1;

                        let val = table[i + 1].entry(next_state).or_default();
                        if geodes + state.geode_bots > *val {
                            *val = geodes + state.geode_bots;
                        }
                    }

                    if state.ore >= self.clay_robot_ore {
                        let mut next_state = next_state;
                        next_state.ore -= self.clay_robot_ore;
                        next_state.clay_bots += 1;

                        let val = table[i + 1].entry(next_state).or_default();
                        if geodes + state.geode_bots > *val {
                            *val = geodes + state.geode_bots;
                        }
                    }

                    let val = table[i + 1].entry(next_state).or_default();
                    if geodes + state.geode_bots > *val {
                        *val = geodes + state.geode_bots;
                    }
                }
            }
        }

        *table[steps].values().max().unwrap()
    }

    fn max_different(&self, steps: usize) -> usize {
        let mut queue: VecDeque<State> = VecDeque::new();
        let mut start_state = State::default();
        start_state.ore_bots = 1;
        queue.push_back(start_state);

        fn get_steps_needed(current_count: usize, required_count: usize, bot_count: usize) -> usize {
            if current_count >= required_count {
                1
            } else {
                (required_count - current_count) / bot_count + 1
            }
        }

        let mut max = 0;
        while let Some(state) = queue.pop_front() {
            if state.geode_bots > 0 && state.steps + 1 < steps {
                // just find the max geodes we could get for this state
                max = max.max(state.geodes + (steps - state.steps - 2) * state.geode_bots);
                if max > 9 {
                    let i = 0;
                }
            }

            if state.obsidian_bots > 0 {
                // we can make geode_bots 
                let steps_needed = usize::max(
                    get_steps_needed(state.ore,self.geode_robot_ore, state.ore_bots),
                    get_steps_needed(state.obsidian, self.geode_robot_obsidian, state.obsidian_bots));
                if state.steps + steps_needed < steps {
                    let mut next_state = state.clone();
                    next_state.tick_steps(steps_needed);
                    next_state.ore -= self.geode_robot_ore;
                    next_state.obsidian -= self.geode_robot_obsidian;
                    next_state.geode_bots += 1;
                    next_state.steps += steps_needed;

                    if next_state.steps == 21 && next_state.ore == 3 && next_state.clay == 29 && next_state.obsidian == 2 && next_state.geodes == 3 {
                        let i = 0;
                    }

                    queue.push_back(next_state);
                }
            }

            if state.clay_bots > 0 {
                // we can make obsidian bots
                let steps_needed = usize::max(
                    get_steps_needed(state.ore,self.obsidian_robot_ore, state.ore_bots),
                    get_steps_needed(state.clay, self.obsidian_robot_clay, state.clay_bots));
                if state.steps + steps_needed < steps {
                    let mut next_state = state.clone();
                    next_state.tick_steps(steps_needed);
                    next_state.ore -= self.obsidian_robot_ore;
                    next_state.clay -= self.obsidian_robot_clay;
                    next_state.obsidian_bots += 1;
                    next_state.steps += steps_needed;
                    queue.push_back(next_state);
                }
            }

            {
                let steps_needed = get_steps_needed(state.ore, self.ore_robot_ore, state.ore_bots);
                if state.steps + steps_needed < steps {
                    let mut next_state = state.clone();
                    next_state.tick_steps(steps_needed);
                    next_state.ore -= self.ore_robot_ore;
                    next_state.ore_bots += 1;
                    next_state.steps += steps_needed;
                    queue.push_back(next_state);
                }
            }

            // clay_bot
            {
                let steps_needed = get_steps_needed(state.ore, self.clay_robot_ore, state.ore_bots);
                if state.steps + steps_needed < steps {
                    let mut next_state = state.clone();
                    next_state.tick_steps(steps_needed);
                    next_state.ore -= self.clay_robot_ore;
                    next_state.clay_bots += 1;
                    next_state.steps += steps_needed;
                    queue.push_back(next_state);
                }
            }
        }

        max
    }
}

fn input() -> Vec<Blueprint> {
    file_lines("inputs/day19.txt").map(|l| {
        let numbers = l.split([' ', ':', '.']).filter_map(|s| s.parse::<usize>().ok()).to_vec();

        let id = numbers[0];
        let ore_robot_ore = numbers[1];
        let clay_robot_ore = numbers[2];
        let obsidian_robot_ore = numbers[3];
        let obsidian_robot_clay = numbers[4];
        let geode_robot_ore = numbers[5];
        let geode_robot_obsidian = numbers[6];

        Blueprint {
            id,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
        }
    }).to_vec()
}

#[test]
fn part1() {
    let blueprints = input();
    let answer: usize = blueprints.into_iter().map(|bp| bp.id * bp.max(24)).sum();
    assert_eq!(answer, 1550);
}

#[test]
fn part2() {
    let blueprints = input();
    let answer: usize = blueprints[0..3].iter().map(|bp| bp.max(32)).product();
    assert_eq!(answer, 18630);
}

#[test]
fn part1_different() {
    let blueprints = input();

    let max = blueprints[1].max_different(24);


    assert_eq!(max, 0);
    // let answer: usize = blueprints.into_iter().map(|bp| bp.id * bp.max(24)).sum();
    // assert_eq!(answer, 1550);
}