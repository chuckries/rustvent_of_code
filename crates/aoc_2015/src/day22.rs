use aoc_common::PriorityQueue;

type Spell = fn(&Game) -> Option<Game>;

const SPELLS: [Spell; 5] = [
    Game::magic_missile,
    Game::drain,
    Game::shield,
    Game::poison,
    Game::recharge,
];

#[derive(Clone)]
struct Game {
    player_hp: i32,
    player_manna: i32,
    boss_hp: i32,
    boss_damage: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
    player_turn: bool,
    mana_spent: i32,
    turn_damage: i32,
}

impl Game {
    fn new(player_hp: i32, player_manna: i32, boss_hp: i32, boss_damage: i32, turn_damage: i32) -> Self {
        Game {
            player_hp,
            player_manna,
            boss_hp,
            boss_damage,
            shield: 0,
            poison: 0,
            recharge: 0,
            player_turn: true,
            mana_spent: 0,
            turn_damage
        }
    }

    fn next_states(mut self, games: &mut Vec<Game>) {
        if self.player_turn {
            self.player_hp -= self.turn_damage;
            if self.player_hp <= 0 {
                return;
            }
        }

        let armor = if self.shield > 0 {
            self.shield -= 1;
            7
        } else {
            0
        };

        if self.poison > 0 {
            self.poison -= 1;
            self.boss_hp -= 3;

            if self.boss_hp <= 0 {
                games.push(self);
                return;
            }
        }

        if self.recharge > 0 {
            self.recharge -= 1;
            self.player_manna += 101;
        }

        let player_turn = self.player_turn;
        self.player_turn = !self.player_turn;

        if player_turn {
            for spell in SPELLS {
                if let Some(next) = spell(&self) {
                    games.push(next);
                }
            }
        } else {
            let attack = (self.boss_damage - armor).max(1);
            self.player_hp -= attack;
            if self.player_hp > 0 {
                games.push(self);
            }
        }
    }

    fn magic_missile(&self) -> Option<Self> {
        const COST: i32 = 53;
        if self.player_manna >= COST {
            let mut next = self.clone();
            next.player_manna -= COST;
            next.mana_spent += COST;
            next.boss_hp -= 4;
            Some(next)
        } else {
            None
        }
    }

    fn drain(&self) -> Option<Self> {
        const COST: i32 = 73;
        if self.player_manna >= COST {
            let mut next = self.clone();
            next.player_manna -= COST;
            next.mana_spent += COST;
            next.boss_hp -= 2;
            next.player_hp += 2;
            Some(next)
        } else {
            None
        }
    }

    fn shield(&self) -> Option<Self> {
        const COST: i32 = 113;
        if self.player_manna >= COST && self.shield == 0 {
            let mut next = self.clone();
            next.player_manna -= COST;
            next.mana_spent += COST;
            next.shield = 6;
            Some(next)
        } else {
            None
        }
    }

    fn poison(&self) -> Option<Self> {
        const COST: i32 = 173;
        if self.player_manna >= COST && self.poison == 0 {
            let mut next = self.clone();
            next.player_manna -= COST;
            next.mana_spent += COST;
            next.poison = 6;
            Some(next)
        } else {
            None
        }
    }

    fn recharge(&self) -> Option<Self> {
        const COST: i32 = 229;
        if self.player_manna >= COST && self.recharge == 0 {
            let mut next = self.clone();
            next.player_manna -= COST;
            next.mana_spent += COST;
            next.recharge = 5;
            Some(next)
        } else {
            None
        }
    }
}

const BOSS_HP: i32 = 51;
const BOSS_DAMAGE: i32 = 9;

fn run(turn_damage: i32) -> i32 {
    let initial = Game::new(50, 500, BOSS_HP, BOSS_DAMAGE, turn_damage);
    let mut queue: PriorityQueue<Game, i32> = PriorityQueue::new();
    queue.enqueue(initial, 0);

    let mut next_games: Vec<Game> = Vec::new();
    while let Some(current) = queue.dequeue() {
        if current.boss_hp <= 0 {
            return current.mana_spent;
        }

        current.next_states(&mut next_games);
        for game in next_games.drain(..) {
            let spent = game.mana_spent;
            queue.enqueue(game, spent);
        }
    }

    panic!();
}

#[test]
fn part1() {
    let answer = run(0);
    assert_eq!(answer, 900);
}

#[test]
fn part2() {
    let answer = run(1);
    assert_eq!(answer, 1216);
}