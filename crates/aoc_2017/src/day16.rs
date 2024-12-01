use aoc_common::{file_string, IteratorExt};

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn input() -> Vec<Move> {
    file_string("inputs/day16.txt").split(',').map(|s| {
        let bytes = s.as_bytes();
        match bytes[0] {
            b's' => Move::Spin(s[1..].parse().unwrap()),
            b'x' => {
                let idx = s.find('/').unwrap();
                Move::Exchange(s[1..idx].parse().unwrap(), s[idx + 1..].parse().unwrap())
            }
            b'p' => {
                Move::Partner(bytes[1], bytes[3])
            }
            _ => panic!()
        }
    }).collect()
}

struct Dancer {
    moves: Vec<Move>,
    programs: Vec<u8>,
}

impl Dancer {
    fn new(moves: Vec<Move>) -> Self {
        let programs = (0..16).map(|i| b'a' + i).to_vec();
        Self {
            moves,
            programs,
        }
    }

    fn dance(&mut self) {
        for m in self.moves.iter() {
            match m {
                Move::Spin(i) => self.programs.rotate_right(*i),
                Move::Exchange(a, b ) => {
                    self.programs.swap(*a, *b);
                },
                Move::Partner(a, b) => {
                    let mut found_idx = 0;
                    let mut found = [0, 0];
                    for (idx, p) in self.programs.iter().enumerate() {
                        if p == a || p == b {
                            found[found_idx] = idx;
                            found_idx += 1;
                            if found_idx == 2 {
                                break;
                            }
                        }
                    }
                    self.programs.swap(found[0], found[1]);
                }
            }
        }
    }
}

#[test]
fn part1() {
    let mut dancer = Dancer::new(input());
    dancer.dance();
    let s = String::from_utf8(dancer.programs.to_vec()).unwrap();
    assert_eq!(s, "padheomkgjfnblic")
}

#[test]
fn part2() {
    let mut dancer = Dancer::new(input());
    let mut positions = vec![dancer.programs.clone()];
    let target = dancer.programs.clone();

    loop {
        dancer.dance();

        if dancer.programs == target {
            break;
        }

        positions.push(dancer.programs.clone());
    }

    let program = positions[1000000000 % positions.len()].clone();

    let s = String::from_utf8(program).unwrap();
    assert_eq!(s, "bfcdeakhijmlgopn")
}