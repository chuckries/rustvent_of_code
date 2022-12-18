use aoc_common::{file_string, IteratorExt, Vec2us};

type Piece = &'static [[char; 4]; 4];

const PIECES: [[[char; 4]; 4]; 5] = [
    [['#', '#', '#', '#'],
     ['.', '.', '.', '.'],
     ['.', '.', '.', '.'],
     ['.', '.', '.', '.']],
    
    [['.', '#', '.', '.'],
     ['#', '#', '#', '.'],
     ['.', '#', '.', '.'],
     ['.', '.', '.', '.'],],

    [['#', '#', '#', '.'],
     ['.', '.', '#', '.'],
     ['.', '.', '#', '.'],
     ['.', '.', '.', '.']],

    [['#', '.', '.', '.'],
     ['#', '.', '.', '.'],
     ['#', '.', '.', '.'],
     ['#', '.', '.', '.']],

    [['#', '#', '.', '.'],
     ['#', '#', '.', '.'],
     ['.', '.', '.', '.'],
     ['.', '.', '.', '.']],
];

struct PieceIter(usize);

impl PieceIter {
    fn new() -> Self {
        Self(0)
    }
}

impl Iterator for PieceIter {
    type Item = Piece;

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

struct Board<T: Iterator<Item = Piece>, const N: usize> {
    pieces: T,
    board: Vec<[char; N]>,
    max_height: usize,
}

impl<T: Iterator<Item = Piece>, const N: usize> Board<T, N> {
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

    fn do_piece(&mut self, moves: &mut MoveIter) -> i64 {
        let piece = self.pieces.next().unwrap();
        let mut loc = Vec2us::new(2, self.max_height + 3);
        self.ensure_height(loc.y + 4);

        let mut count = 0;
        while let Some(dir) = moves.next() {
            count += 1;
            match dir {
                '<' if self.can_move_left(loc, piece) => loc.x -= 1,
                '>' if self.can_move_right(loc, piece) => loc.x += 1,
                _ => (),
            }

            if self.can_move_down(loc, piece) {
                loc.y -= 1;
            } else {
                self.place(loc, piece);

                let mut height = 0;
                for row in piece.iter() {
                    if row.iter().any(|c| *c == '#') {
                        height += 1;
                    } else {
                        break;
                    }
                }

                if loc.y + height > self.max_height {
                    self.max_height = loc.y + height;
                }

                break;
            }
        }

        count
    }

    fn ensure_height(&mut self, height: usize) {
        while self.board.len() <= height {
            self.board.push(['.'; N]);
        }
    }

    fn place(&mut self, loc: Vec2us, piece: Piece) {
        for j in 0..piece.len() {
            for i in 0..piece[j].len() {
                if piece[j][i] == '#' {
                    self.board[loc.y + j][loc.x + i] = '#';
                }
            }
        }
    }

    fn can_move_left(&self, loc: Vec2us, piece: Piece) -> bool {
        if loc.x == 0 {
            return false;
        }

        self.can_place(loc - Vec2us::unit_x(), piece)
    }

    fn can_move_right(&self, loc: Vec2us, piece: Piece) -> bool {
        self.can_place(loc + Vec2us::unit_x(), piece)
    }

    fn can_move_down(&self, loc: Vec2us, piece: Piece) -> bool {
        if loc.y == 0 {
            return false;
        }

        self.can_place(loc - Vec2us::unit_y(), piece)
    }

    fn can_place(&self, loc: Vec2us, piece: Piece) -> bool {
        for j in 0..piece.len() {
            for i in 0..piece[j].len() {
                if piece[j][i] == '#' {
                    let board_x = loc.x + i;
                    if board_x >= N {
                        return false;
                    }
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

    // for row in board.board.iter().rev() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    let answer = board.max_height;
    assert_eq!(answer, 3219);
}

#[test]
fn part2() {
    let  input = input();
    let mut board: Board<_, 7> = Board::new(PieceIter::new());
    let mut moves_iter = MoveIter(0, input);

    // loop {
    //     board.do_piece(&mut moves_iter);
    //     if moves_iter.0 >= moves_iter.1.len() {
    //         break;
    //     }
    // }

    // let mut total_moves: i64 = 1000000000000;

    // let mut height_delta = 0;
    // let mut move_delta = 0;

    // 'outer: loop {
    //     let mut moves = 0;
    //     let mut height = 0;

    //     loop {
    //         moves += board.do_piece(&mut moves_iter);
    //         total_moves -= moves;
    //         if moves_iter.0 >= moves_iter.1.len() {
    //             if board.max_height - height == height_delta && moves == move_delta {
    //                 break 'outer;
    //             } else {
    //                 height_delta = board.max_height - height;
    //                 height = board.max_height;
    //                 move_delta = moves;
    //                 moves = 0;
    //             }
    //         }
    //     }
    // }
}