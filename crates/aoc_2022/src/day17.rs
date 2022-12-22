use std::{collections::HashMap};

use aoc_common::{file_string, IteratorExt, Vec2us};

struct Piece {
    width: usize,
    height: usize,
    pattern: [[char; 4]; 4]
}

const PIECES: [Piece; 5] = [
    Piece {
        width: 4,
        height: 1,
        pattern: [['#', '#', '#', '#'],
                  ['.', '.', '.', '.'],
                  ['.', '.', '.', '.'],
                  ['.', '.', '.', '.']],
    },

    Piece {
        width: 3,
        height: 3,
        pattern: [['.', '#', '.', '.'],
                  ['#', '#', '#', '.'],
                  ['.', '#', '.', '.'],
                  ['.', '.', '.', '.']],
    },

    Piece {
        width: 3,
        height: 3,
        pattern : [['#', '#', '#', '.'],
                   ['.', '.', '#', '.'],
                   ['.', '.', '#', '.'],
                   ['.', '.', '.', '.']],
    },

    Piece {
        width: 1,
        height: 4,
        pattern: [['#', '.', '.', '.'],
                  ['#', '.', '.', '.'],
                  ['#', '.', '.', '.'],
                  ['#', '.', '.', '.']],
    },

    Piece {
        width: 2,
        height: 2,
        pattern: [['#', '#', '.', '.'],
                  ['#', '#', '.', '.'],
                  ['.', '.', '.', '.'],
                  ['.', '.', '.', '.']],
    },
];

struct PieceIter(usize);

impl PieceIter {
    fn new() -> Self {
        Self(0)
    }
}

impl Iterator for PieceIter {
    type Item = &'static Piece;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= PIECES.len() {
            self. 0 = 0;
        }

        self.0 += 1;
        Some(&PIECES[self.0 - 1])
    }
}

struct MoveIter(usize, Vec<char>);

impl Iterator for MoveIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= self.1.len() {
            self.0 = 0;
        }

        self.0 += 1;
        Some(self.1[self.0 - 1])
    }
}

struct Board<'a, T: Iterator<Item = &'a Piece>, const N: usize> {
    pieces: T,
    board: Vec<[char; N]>,
    max_height: usize,
}

impl<'a, T: Iterator<Item = &'a Piece>, const N: usize> Board<'a, T, N> {
    fn new(pieces: T) -> Self {
        Self {
            pieces,
            board: Vec::new(),
            max_height: 0,
        }
    }

    fn run(&mut self, pieces: usize, mut moves: MoveIter) {
        for _ in 0..pieces {
            self.do_piece(&mut moves);
        }
    }

    fn do_piece(&mut self, moves: &mut MoveIter) {
        let piece = self.pieces.next().unwrap();
        let mut loc = Vec2us::new(2, self.max_height + 3);
        self.ensure_height(loc.y + piece.height);

        loop {
            match moves.next().unwrap() {
                '<' if self.can_move_left(loc, piece) => loc.x -= 1,
                '>' if self.can_move_right(loc, piece) => loc.x += 1,
                _ => (),
            }

            if self.can_move_down(loc, piece) {
                loc.y -= 1;
            } else {
                self.place(loc, piece);
                self.max_height = self.max_height.max(loc.y + piece.height);
                break;
            }
        }
    }

    fn ensure_height(&mut self, height: usize) {
        while self.board.len() <= height {
            self.board.push(['.'; N]);
        }
    }

    fn place(&mut self, loc: Vec2us, piece: &Piece) {
        for j in 0..piece.height {
            for i in 0..piece.width{
                if piece.pattern[j][i] == '#' {
                    self.board[loc.y + j][loc.x + i] = '#';
                }
            }
        }
    }

    fn can_move_left(&self, loc: Vec2us, piece: &Piece) -> bool {
        if loc.x == 0 {
            return false;
        }

        self.can_place(loc - Vec2us::unit_x(), piece)
    }

    fn can_move_right(&self, loc: Vec2us, piece: &Piece) -> bool {
        if loc.x + piece.width >= N {
            return false;
        }

        self.can_place(loc + Vec2us::unit_x(), piece)
    }

    fn can_move_down(&self, loc: Vec2us, piece: &Piece) -> bool {
        if loc.y == 0 {
            return false;
        }

        self.can_place(loc - Vec2us::unit_y(), piece)
    }

    fn can_place(&self, loc: Vec2us, piece: &Piece) -> bool {
        for j in 0..piece.height {
            for i in 0..piece.width {
                if piece.pattern[j][i] == '#' {
                    let board_x = loc.x + i;
                    let board_y = loc.y + j;
                    if self.board[board_y][board_x] == '#' {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn input() -> Vec<char> {
    file_string("inputs/day17.txt").chars().to_vec()
}

#[test]
fn part1() {
    let  input = input();
    let mut board: Board<_, 7> = Board::new(PieceIter::new());
    let moves = MoveIter(0, input);
    board.run(2022, moves);

    let answer = board.max_height;
    assert_eq!(answer, 3219);
}

#[derive(Clone, Copy, Default)]
struct State {
    piece_delta: usize,
    total_pieces: usize,
    height_delta: usize,
    total_height: usize,
}

#[test]
fn part2() {
    let  input = input();
    let mut board: Board<_, 7> = Board::new(PieceIter::new());
    let mut moves_iter = MoveIter(0, input);

    let mut states: HashMap<(usize, usize), State> = HashMap::new();

    let mut pieces = 0;

    let cycle_state;
    loop {
        board.do_piece(&mut moves_iter);
        pieces += 1;

        let previous = states.entry((board.pieces.0, moves_iter.0)).or_insert(State::default());

        if pieces - previous.total_pieces == previous.piece_delta &&
           board.max_height - previous.total_height == previous.height_delta
        {
            cycle_state = *previous;
            break;
        } else {
            previous.piece_delta = pieces - previous.total_pieces;
            previous.total_pieces = pieces;
            previous.height_delta = board.max_height - previous.total_height;
            previous.total_height = board.max_height;
        }
    }

    let pieces_remaining: usize = 1000000000000 - pieces;
    let cycles = pieces_remaining / cycle_state.piece_delta;
    let pieces_after_cycles = pieces_remaining % cycle_state.piece_delta;

    for _ in 0..pieces_after_cycles {
        board.do_piece(&mut moves_iter);
    }

    let answer = board.max_height + cycles * cycle_state.height_delta;

    assert_eq!(answer, 1582758620701);
}