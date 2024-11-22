use std::{cmp::Reverse, collections::HashSet};

use aoc_common::PriorityQueue;

struct Item {
    cost: i32,
    damage: i32,
    armor: i32
}

impl Item {
    const fn new(cost: i32, damage: i32, armor: i32) -> Self {
        Item {
            cost, 
            damage,
            armor,
        }
    }

    const fn cost(&self) -> i32 {
        self.cost
    }

    const fn damage(&self) -> i32 {
        self.damage
    }

    const fn armor(&self) -> i32 {
        self.armor
    }
}

const WEAPONS: [Item; 5] = [
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8,0),
];

const ARMORS: [Item; 5] = [
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: [Item; 6] = [
    Item::new(20, 0, 1),
    Item::new(25, 1, 0),
    Item::new(40, 0, 2),
    Item::new(50, 2, 0),
    Item::new(80, 0, 3),
    Item::new(100, 3, 0),
];

struct Outfit {
    weapon: usize,
    armor: Option<usize>,
    ring1: Option<usize>,
    ring2: Option<usize>,
    cost: i32,
    damage: i32,
    armor_count: i32,
    id: usize,
}

impl Outfit {
    fn with_weapon(&self, weapon: usize) -> Self {
        Self::new(weapon, self.armor, self.ring1, self.ring2)
    }

    fn with_armor(&self, armor: Option<usize>) -> Self {
        Self::new(self.weapon, armor, self.ring1, self.ring2)
    }

    fn with_ring1(&self, ring1: Option<usize>) -> Self {
        Self::new(self.weapon, self.armor, ring1, self.ring2)
    }

    fn with_ring2(&self, ring2: Option<usize>) -> Self {
        Self::new(self.weapon, self.armor, self.ring1, ring2)
    }

    fn new(weapon: usize, armor: Option<usize>, ring1: Option<usize>, ring2: Option<usize>) -> Self {
        fn add_up(weapon: usize, armor: Option<usize>, ring1: Option<usize>, ring2: Option<usize>, selector: fn(&Item) -> i32) -> i32 {
            selector(&WEAPONS[weapon]) + 
            armor.map_or(0, |i| selector(&ARMORS[i])) +
            ring1.map_or(0, |i| selector(&RINGS[i])) +
            ring2.map_or(0, |i| selector(&RINGS[i]))
        }

        let cost = add_up(weapon, armor, ring1, ring2, Item::cost);
        let damage = add_up(weapon, armor, ring1, ring2, Item::damage);
        let armor_count = add_up(weapon, armor, ring1, ring2, Item::armor);

        let id = 
            (weapon + 1) * 1 +
            armor.map_or(0, |i| i + 1) * 10 +
            ring1.map_or(0, |i| i + 1) * 100 +
            ring2.map_or(0, |i| i + 1) * 1000;

        Self {
            weapon,
            armor,
            ring1,
            ring2,
            cost,
            damage,
            armor_count,
            id
        }
    }

    fn improve(&self, outfits: &mut Vec<Outfit>) {
        if self.weapon < WEAPONS.len() - 1 {
            outfits.push(self.with_weapon(self.weapon + 1));
        }

        if let Some(armor) = self.armor {
            if armor < ARMORS.len() - 1 {
                outfits.push(self.with_armor(Some(armor + 1)));
            }
        } else {
            outfits.push(self.with_armor(Some(0)));
        }

        if let Some(ring1) = self.ring1 {
            if ring1 < RINGS.len() - 1 {
                outfits.push(self.with_ring1(Some(ring1 + 1)))
            }

            if let Some(ring2) = self.ring2 {
                if ring2 < ring1 - 1 {
                    outfits.push(self.with_ring2(Some(ring2 + 1)));
                }
            } else if ring1 != 0 {
                outfits.push(self.with_ring2(Some(0)));
            }
        } else {
            outfits.push(self.with_ring1(Some(0)));
        }
    }

    fn diminish(&self, outfits: &mut Vec<Outfit>) {
        if self.weapon > 0 {
            outfits.push(self.with_weapon(self.weapon - 1));
        }

        if let Some(armor) = self.armor {
            if armor > 0 {
                outfits.push(self.with_armor(Some(armor - 1)));
            } else {
                outfits.push(self.with_armor(None));
            }
        }

        if let Some(ring1) = self.ring1 {
            if let Some(ring2) = self.ring2 {
                if ring1 > ring2 + 2 {
                    outfits.push(self.with_ring1(Some(ring1 - 1)));
                }

                if ring2 > 0 {
                    outfits.push(self.with_ring2(Some(ring2 - 1)));
                } else {
                    outfits.push(self.with_ring2(None));
                }
            } else {
                if ring1 > 0 {
                    outfits.push(self.with_ring1(Some(ring1 - 1)));
                } else {
                    outfits.push(self.with_ring1(None))
                }
            }
        }
    }
}

const HP: i32 = 100;
const DAMAGE: i32 = 8;
const ARMOR: i32 = 2;

fn test_win(outfit: &Outfit) -> bool {
    let my_attack = (outfit.damage - ARMOR).max(1);
    let opp_attack = (DAMAGE - outfit.armor_count).max(1);

    let my_turns = HP / my_attack + i32::signum(HP % my_attack);
    let opp_turns = HP / opp_attack + i32::signum(HP % opp_attack);

    my_turns <= opp_turns
}

#[test]
fn part1() {
    let mut queue: PriorityQueue<Outfit, i32> = PriorityQueue::new();
    let initial = Outfit::new(0, None, None, None);
    let initial_cost = initial.cost;
    queue.enqueue(initial, initial_cost);
    let mut visited: HashSet<usize> = HashSet::new();

    let mut next_outfits: Vec<Outfit> = Vec::new();
    let mut answer = 0;
    while let Some(current) = queue.dequeue() {
        if visited.contains(&current.id) {
            continue;
        }
        visited.insert(current.id);

        if test_win(&current) {
            answer = current.cost;
            break;
        }

        current.improve(&mut next_outfits);
        for next in next_outfits.drain(..) {
            let cost = next.cost;
            queue.enqueue(next, cost);
        }
    }

    assert_eq!(answer, 91);
}

#[test]
fn part2() {
    let mut queue: PriorityQueue<Outfit, Reverse<i32>> = PriorityQueue::new();
    let initial = Outfit::new(WEAPONS.len() - 1, Some(ARMORS.len() - 1), Some(RINGS.len() - 1), Some(RINGS.len() - 2));
    let initial_cost = initial.cost;
    queue.enqueue(initial, Reverse(initial_cost));
    let mut visited: HashSet<usize> = HashSet::new();

    let mut next_outfits: Vec<Outfit> = Vec::new();
    let mut answer = 0;
    while let Some(current) = queue.dequeue() {
        if visited.contains(&current.id) {
            continue;
        }
        visited.insert(current.id);

        if !test_win(&current) {
            answer = current.cost;
            break;
        }

        current.diminish(&mut next_outfits);
        for next in next_outfits.drain(..) {
            let cost = next.cost;
            queue.enqueue(next, Reverse(cost));
        }
    }

    assert_eq!(answer, 158);
}