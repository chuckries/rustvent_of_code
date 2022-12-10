const A_START: usize = 8;
const B_START: usize = 5;

struct Dice {
    current: usize
}

impl Dice {
    fn new() ->Dice {
        Dice {
            current: 1
        }
    }
}

impl Iterator for Dice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current += 1;
        if self.current > 100 {
            self.current = 1;
        }

        Some(result)
    }
}

#[test] 
fn part1() {
    let mut dice = Dice::new();
    let mut state = [(0usize, 0usize); 2];
    state[0] = (A_START - 1, 0);
    state[1] = (B_START - 1, 0);

    let mut rolls = 0;
    let answer;
    'outer: loop {
        for turn in 0..2 {
            let roll: usize = dice.by_ref().take(3).sum();
            rolls += 3;

            let (pos, score) = &mut state[turn];
            *pos += roll;
            *pos %= 10;
            *score += *pos + 1;

            if *score >= 1000 {
                answer = state[turn ^ 1].1 * rolls;
                break 'outer;
            }
        }
    }

    assert_eq!(answer, 597600);
}

#[test]
fn part2() {
    let mut wins = [0usize; 2];
    let mut positions = [[[[[0usize; 2]; 10] ; 10]; 21]; 21];
    positions[0][0][A_START - 1][B_START - 1][0] = 1;

    for a_score in 0..21 {
        for b_score in 0..21 {
            for a_pos in 0..10 {
                for b_pos in 0..10 {
                    for turn in 0..2 {
                        for i in 1..=3 {
                            for j in 1..=3 {
                                for k in 1..=3 {
                                    let count = positions[a_score][b_score][a_pos][b_pos][turn];
                                    if count == 0 {
                                        continue;
                                    }

                                    let (mut a_pos, mut a_score, mut b_pos, mut b_score) = (a_pos, a_score, b_pos, b_score);
                                    let (pos, score) = if turn == 0 {
                                        (&mut a_pos, &mut a_score)
                                    } else {
                                        (&mut b_pos, &mut b_score)
                                    };

                                    *pos += i + j + k;
                                    *pos %= 10;

                                    *score += *pos + 1;
                                    if *score >= 21 {
                                        wins[turn] += count;
                                    } else {
                                        positions[a_score][b_score][a_pos][b_pos][turn ^ 1] += count;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let answer = wins[0].max(wins[1]);
    assert_eq!(answer, 634769613696613);
}