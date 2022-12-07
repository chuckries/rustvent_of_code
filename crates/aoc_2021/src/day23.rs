use aoc_common::IteratorExt;

use Cell::*;

#[test]
fn part1() {
    let board = Board::new(&[
        &[Pod::D, Pod::C],
        &[Pod::C, Pod::D],
        &[Pod::A, Pod::A],
        &[Pod::B, Pod::B],
    ]);

    let answer = board.solve();
    assert_eq!(answer, 17400);
}

#[test]
fn part2() {
    let board = Board::new(&[
        &[Pod::D, Pod::D, Pod::D, Pod::C],
        &[Pod::C, Pod::C, Pod::B, Pod::D],
        &[Pod::A, Pod::B, Pod::A, Pod::A],
        &[Pod::B, Pod::A, Pod::C, Pod::B],
    ]);

    let answer = board.solve();
    assert_eq!(answer, 46120);
}

#[derive(Copy, Clone)]
enum Cell {
    Hall(Option<Pod>),
    Room(usize),
}

struct Move {
    from: usize,
    to: usize,
    steps: usize,
    occupant: Pod,
}

struct Board {
    board: [Cell; 11],
    rooms: [Room; 4],
    cost: usize,
    min: usize,
    sum_unsolved: usize,
}

impl Board {
    fn new(start: &[&[Pod]]) -> Board {
        let rooms = [
            Room::new(0, start[0]),
            Room::new(1, start[1]),
            Room::new(2, start[2]),
            Room::new(3, start[3]),
        ];

        let board: [Cell; 11] = [
            Hall(None), 
            Hall(None), 
            Room(0),
            Hall(None),
            Room(1),
            Hall(None),
            Room(2),
            Hall(None), 
            Room(3),
            Hall(None),
            Hall(None)
        ];
        
        let sum_unsolved = ((0..rooms[0].capacity).map(|i| i + 2).sum::<usize>() * 1111) +
            rooms.iter().map(|r| r.iter().enumerate().map(|(i, p)| (r.capacity - i + 1) * p.weight()).sum::<usize>()).sum::<usize>();

        Board {
            board,
            rooms,
            cost: 0,
            min: usize::MAX,
            sum_unsolved,
        }
    }

    fn solve(mut self) -> usize {
        self.solve_recurse();
        self.min
    }

    fn solve_recurse(&mut self) {
        if self.cost + self.sum_unsolved >= self.min {
            return;
        }

        if self.sum_unsolved == 0 {
            self.min = self.min.min(self.cost);
            return;
        }

        self.try_next_moves();
    }

    fn try_next_moves(&mut self) {
        for idx in 0..self.board.len() {
            let cell = self.board[idx];
            match cell {
                Cell::Hall(Some(occupant)) => {
                    if let Some(next_move) = self.try_get_move_to_room(idx, occupant) {
                        self.try_move(next_move);
                    }
                },
                Cell::Room(room) => {
                    let room = &mut self.rooms[room];
                    if !room.can_move_out() {
                        continue;
                    }

                    let occupant = room.peek();
                    if let Some(next_move) = self.try_get_move_to_room(idx, occupant) {
                        self.try_move(next_move);
                    } else {
                        for adj_left in (0..idx).rev() {
                            match &self.board[adj_left] {
                                Hall(Some(_)) => break,
                                Hall(None) => {
                                    let steps = idx - adj_left;
                                    self.try_move(Move { from: idx, to: adj_left, steps, occupant });
                                }
                                _ => (),
                            }
                        }
                        
                        for adj_right in idx + 1..self.board.len() {
                            match &self.board[adj_right] {
                                Hall(Some(_)) => break,
                                Hall(None) => {
                                    let steps = adj_right - idx;
                                    self.try_move(Move { from: idx, to: adj_right, steps, occupant });
                                }
                                _ => ()
                            }
                        }
                    }
                }
                _ => ()
            }
        }
    }

    fn try_move(&mut self, next_move: Move) {
        let mut steps = next_move.steps;
        let pod = next_move.occupant;
        let weight = pod.weight();
        let mut sum_unsolved_delta = 0;

        match &mut self.board[next_move.from] {
            Hall(hall) => *hall = None,
            Room(room) => {
                let room = &mut self.rooms[*room];
                let move_out_cost = room.move_out_cost();
                steps += move_out_cost;
                sum_unsolved_delta += move_out_cost + 1;
                room.move_out();
            }
        };

        match &mut self.board[next_move.to] {
            Hall(hall) => *hall = Some(pod),
            Room(room) => {
                let room = &mut self.rooms[*room];
                let move_in_cost = room.move_in_cost();
                steps += move_in_cost;
                sum_unsolved_delta += move_in_cost + 1;
                room.move_in(pod);
            }
        }

        sum_unsolved_delta *= weight;
        let cost = steps * weight;
        self.cost += cost;
        self.sum_unsolved -= sum_unsolved_delta;

        self.solve_recurse();

        self.cost -= cost;
        self.sum_unsolved += sum_unsolved_delta;

        match &mut self.board[next_move.to] {
            Hall(hall) => *hall = None,
            Room(room) => _ = self.rooms[*room].move_out(),
        }

        match &mut self.board[next_move.from] {
            Hall(hall) => *hall = Some(pod),
            Room(room) => self.rooms[*room].move_in(pod),
        }
    }

    fn try_get_move_to_room(&mut self, idx: usize, occupant: Pod) -> Option<Move> {
        let target_idx = occupant.home_board_idx();
        let target_room = &mut self.rooms[occupant.home_room_idx()];
        if target_room.can_move_in() {
            let (lo, hi) = if idx < target_idx {
                (idx, target_idx)
            } else {
                (target_idx, idx)
            };

            if self.board[lo + 1..hi].iter().all(|c| matches!(c, Hall(None) | Room(_))) {
                let steps = hi - lo;
                return Some(Move { from: idx, to: target_idx, steps, occupant });
            }
        }

        None
    }
}

struct Room {
    id: usize,
    occupants: Vec<Pod>,
    capacity: usize,
}

impl Room {
    fn new(id: usize, occupants: &[Pod]) -> Room {
        let capacity = occupants.len();
        let occupants = occupants.iter().rev().copied().to_vec();

        Room {
            id,
            occupants,
            capacity,
        }
    }

    fn move_in(&mut self, pod: Pod) {
        self.occupants.push(pod);
    }

    fn move_out(&mut self) -> Pod {
        self.occupants.pop().unwrap()
    }

    fn peek(&self) -> Pod {
        *self.occupants.last().unwrap()
    }

    fn can_move_in(&mut self) -> bool {
        self.occupants.is_empty() || self.occupants.iter().all(|p| p.home_room_idx() == self.id)
    }

    fn can_move_out(&mut self) -> bool {
        !self.can_move_in()
    }

    fn move_out_cost(&self) -> usize {
        self.move_in_cost() + 1
    }

    fn move_in_cost(&self) -> usize {
        self.capacity - self.occupants.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Pod> {
        self.occupants.iter()
    }
}

#[derive(Copy, Clone)]
enum Pod {
    A,
    B,
    C,
    D,
}

impl Pod {
    fn home_room_idx(&self) -> usize {
        match self {
            Pod::A => 0,
            Pod::B => 1,
            Pod::C => 2,
            Pod::D => 3,
        }
    }

    fn home_board_idx(&self) -> usize {
        match self {
            Pod::A => 2,
            Pod::B => 4,
            Pod::C => 6,
            Pod::D => 8,
        }
    }

    fn weight(&self) -> usize {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
        }
    }
}