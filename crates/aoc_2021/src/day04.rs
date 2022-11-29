use aoc_common::file_lines;

struct Board {
    numbers: Vec<Vec<i32>>,
    marked: Vec<Vec<bool>>
}

impl Board {
    fn new(lines: &mut impl Iterator<Item = String>) -> Board {
        let mut rows: Vec<Vec<i32>> = Vec::new();
        for _ in 0..5 {
            let row: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            rows.push(row);
        }

        let marked = vec![vec![false; 5]; 5];

        Board { numbers: rows, marked: marked }
    }

    fn call(&mut self, called: i32) {
        for (j, row) in self.numbers.iter().enumerate() {
            for (i , num) in row.iter().enumerate() {
                if *num == called {
                    self.marked[j][i] = true;
                    return;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        for i in 0..5 {
            let mut win = true;
            for j in 0..5 {
                win = win && self.marked[j][i];
            }
            if win { return true; }
        }

        for j in 0..5 {
            let mut win = true;
            for i in 0..5 {
                win = win && self.marked[j][i];
            }
            if win { return true; }
        }

        false
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for (row_marks, row) in self.numbers.iter().zip(&self.marked) {
            sum += row.iter().zip(row_marks).filter_map(|(mark, num)| if *mark { None } else { Some (*num) }).sum::<i32>();
        }

        sum
    }
}

fn input() -> (Vec<i32>, Vec<Board>) {
    let mut lines = file_lines("inputs/day04.txt");

    let numbers: Vec<i32> = lines.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();
    while let Some(_) = lines.next() {
        boards.push(Board::new(&mut lines));
    }

    (numbers, boards)
}

#[test]
fn part1() {
    let (numbers, mut boards) = input();

    'outer: for num in numbers {
        for board in boards.iter_mut() {
            board.call(num);

            if board.is_win() {
                let answer = num * board.sum_unmarked();
                assert_eq!(answer, 54275);
                break 'outer;
            }
        }
    }
}

#[test]
fn part2() {
    let (numbers, mut boards) = input();

    let mut last_win: Option<Board> = None;
    let mut last_num: i32 = 0;

    for num in numbers {
        for b in boards.iter_mut() {
            b.call(num);
        }

        boards = boards.into_iter().filter_map(|b| {
            if b.is_win() {
                last_num = num;
                last_win = Some(b);
                return None
            }
            Some(b)
        }).collect();
    }

    let answer = last_num * last_win.unwrap().sum_unmarked();
    assert_eq!(answer, 13158);
}