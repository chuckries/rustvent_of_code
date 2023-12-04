use aoc_common::{file_lines, Vec2us};

fn input() -> Map {
    let map = file_lines("inputs/day03.txt").map(|l| l.bytes().collect()).collect();
    Map::new(map)
}

struct Span {
    _begin: usize,
    end: usize,
}

struct Number {
    num: i32,
    span: Span,
}

struct Map {
    map: Vec<Vec<u8>>
}

impl Map {
    fn new(map: Vec<Vec<u8>>) -> Self {
        Self { map }
    }

    fn nums_adjacent_to_symbols(&self) -> Vec<i32> {
        let bounds = Vec2us::new(self.map[0].len(), self.map.len());

        let mut nums = Vec::new();
        let mut j = 0;
        while j < bounds.y {
            let mut i = 0;
            while i < bounds.x {
                if let Some(num) = self.try_parse_number((i, j).into()) {

                    let begin = if i == 0 { 0 } else { i - 1};
                    let end = if num.span.end == bounds.x - 1 { num.span.end } else { num.span.end + 1 };

                    if (j > 0 && self.map[j - 1][begin..=end].iter().copied().any(Self::is_symbol)) ||
                        Self::is_symbol(self.map[j][begin]) ||
                        Self::is_symbol(self.map[j][end]) ||
                        (j < bounds.y - 1 && self.map[j + 1][begin..=end].iter().copied().any(Self::is_symbol)) {
                            nums.push(num.num);
                        }

                    i = num.span.end + 1;
                } else {
                    i += 1;
                }
            }
            j += 1;
        }

        nums
    }

    fn gear_ratios(&self) -> Vec<i32> {
        let mut ratios: Vec<i32> = Vec::new();
        for j in 0..self.map.len() {
            for i in 0..self.map[0].len() {
                if self.map[j][i] == b'*' {
                    let mut adjacents: Vec<i32> = Vec::new();

                    if j > 0 {
                        let v = j - 1;
                        let mut u = if i > 0 { i - 1 } else { i };

                        while u < self.map[0].len() && u <= i + 1 {
                            if let Some(num) = self.try_parse_number((u, v).into()) {
                                adjacents.push(num.num);
                                u = num.span.end + 2;
                            } else {
                                u += 1;
                            }
                        }
                    }

                    if i > 0 {
                        if let Some(num) = self.try_parse_number((i - 1, j).into()) {
                            adjacents.push(num.num);
                        }
                    }

                    if i < self.map[0].len() - 1 { 
                        if let Some(num) = self.try_parse_number((i + 1, j).into()) {
                            adjacents.push(num.num);
                        }
                    }

                    if j < self.map.len() - 1 {
                        let v = j + 1;
                        let mut u = if i > 0 { i - 1 } else { i };

                        while u < self.map[0].len() && u <= i + 1 {
                            if let Some(num) = self.try_parse_number((u, v).into()) {
                                adjacents.push(num.num);
                                u = num.span.end + 2;
                            } else {
                                u += 1;
                            }
                        }
                    }

                    if adjacents.len() == 2 {
                        ratios.push(adjacents[0] * adjacents[1]);
                    }
                }
            }
        }

        ratios
    }
    
    fn try_parse_number(&self, p: Vec2us) -> Option<Number> {
        let line = &self.map[p.y];
        if !line[p.x].is_ascii_digit() {
            return None
        }

        let mut begin = p.x;
        while begin > 0 && line[begin - 1].is_ascii_digit() {
            begin -= 1;
        }

        let mut end = p.x;
        while end < line.len() - 1 && line[end + 1].is_ascii_digit() {
            end += 1;
        }

        Some(Number {
            num: String::from_utf8(line[begin..=end].iter().copied().collect()).unwrap().parse().unwrap(),
            span: Span { _begin: begin, end }
        })
    }

    fn is_symbol(c: u8) -> bool {
        c != b'.' && !c.is_ascii_digit()
    }
}

#[test]
fn part1() {
    let map = input();
    let answer = map.nums_adjacent_to_symbols().iter().sum();

    assert_eq!(527144, answer);
}

#[test]
fn part2() {
    let map = input();
    let answer: i32 = map.gear_ratios().iter().sum();

    assert_eq!(81463996, answer);
}