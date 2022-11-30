use std::{str::CharIndices, iter::Peekable, collections::{HashMap, HashSet, VecDeque}};

use aoc_common::{file_string, Vec2i32};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    Carrot,
    Dollar,
    Pipe,
    LeftParen,
    RightParen,
    Str(&'a str)
}

struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn is_dir(c: char) -> bool {
        matches!(c, 'N' | 'S' | 'E' | 'W')
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, c)) = self.chars.next() {
            let token = match c {
                '^' => Token::Carrot,
                '$' => Token::Dollar,
                '|' => Token::Pipe,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                c if Self::is_dir(c) => {
                    loop {
                        let (end, c) = *self.chars.peek().unwrap();
                        if !Self::is_dir(c) {
                            return Some(Token::Str(&self.input[start..end]));
                        }
                        self.chars.next();
                    }
                }
                _ => panic!()
            };

            Some(token)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Stmt<'a> {
    Literal(&'a str),
    Group(Vec<Vec<Stmt<'a>>>)
}

struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser {
        Parser { 
            lexer: Lexer::new(input).peekable()
        }
    }

    fn parse(&mut self) -> Vec<Stmt<'a>> {
        self.consume(&Token::Carrot);
        let stmts = self.parse_straight_line();
        self.consume(&Token::Dollar);
        stmts
    }

    fn parse_straight_line(&mut self) -> Vec<Stmt<'a>> {
        let mut stmts: Vec<Stmt> = Vec::new();

        loop {
            let tok = self.lexer.peek().unwrap();
            match tok {
                Token::Str(s) => {
                    stmts.push(Stmt::Literal(*s));
                    self.lexer.next();
                },
                Token::LeftParen => {
                    self.lexer.next();
                    let stmt = self.parse_group();
                    stmts.push(stmt);
                    self.consume(&Token::RightParen);
                }
                _ => break,
            }
        }

        stmts
    }

    fn parse_group(&mut self) -> Stmt<'a> {
        let mut group: Vec<Vec<Stmt>> = Vec::new();

        loop {
            group.push(self.parse_straight_line());

            let tok = self.lexer.peek().unwrap();
            match tok {
                Token::RightParen => break,
                Token::Pipe => {
                    self.lexer.next();
                    if self.lexer.peek().unwrap() == &Token::RightParen {
                        group.push(Vec::new());
                        break;
                    }
                }
                _ => panic!()
            }
        }

        Stmt::Group(group)
    }

    fn consume(&mut self, token: &Token) {
        if self.lexer.peek().unwrap() == token {
            self.lexer.next();
        } else {
            panic!();
        }
    }
}

type Map = HashMap<Vec2i32, HashSet<Vec2i32>>;

struct MapBuilder
{
    map: Map,
}

impl MapBuilder
{
    fn build(stmts: Vec<Stmt>) -> Map {
        let mut builder = MapBuilder {
            map: Map::new()
        };

        builder.explore_stmts(&stmts, (0, 0).into());

        builder.map
    }

    fn explore_stmts(&mut self, stmts: &[Stmt], pos: Vec2i32) -> HashSet<Vec2i32> {
        let mut next_positions: HashSet<Vec2i32> = HashSet::new();
        next_positions.insert(pos);

        for stmt in stmts {
            for p in next_positions.drain().collect::<Vec<_>>() {
                match stmt {
                    Stmt::Literal(s) => {
                        next_positions.insert(self.explore_literal(*s, p));
                    }
                    Stmt::Group(g) => {
                        self.explore_group(g, p, &mut next_positions);
                    }
                }
            }
        }

        next_positions
    }

    fn explore_literal(&mut self, literal: &str, mut pos: Vec2i32) -> Vec2i32 {
        for c in literal.chars() {
            let dir = match c {
                'N' => -Vec2i32::unit_y(),
                'S' => Vec2i32::unit_y(),
                'E' => Vec2i32::unit_x(),
                'W' => -Vec2i32::unit_x(),
                _ => panic!()
            };

            self.map.entry(pos).or_default().insert(pos + dir);
            pos += dir;
            self.map.entry(pos).or_default().insert(pos - dir);
        }

        pos
    }

    fn explore_group(&mut self, group: &[Vec<Stmt>], pos: Vec2i32, next_positions: &mut HashSet<Vec2i32>) {
        for stmts in group {
            next_positions.extend(self.explore_stmts(stmts, pos));
        }
    }
}

fn input() -> Map {
    let input = file_string("inputs/day20.txt");
    let mut parser = Parser::new(&input);
    let stmts = parser.parse();
    MapBuilder::build(stmts)
}

fn do_problem() -> (usize, usize) {
    let map = input();

    let mut visited: HashSet<Vec2i32> = HashSet::new();
    let mut to_visit: VecDeque<(Vec2i32, usize)> = VecDeque::new();
    to_visit.push_back(((0, 0).into(), 0));

    let mut max = 0;
    let mut count_1000 = 0;
    while let Some((current, dist)) = to_visit.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        max = dist;
        if dist >= 1000 {
            count_1000 += 1;
        }

        for adj in map.get(&current).unwrap() {
            to_visit.push_back((*adj, dist + 1));
        }
    }

    (max, count_1000)
}

#[test]
fn part1() {
    let (answer, _) = do_problem();
    assert_eq!(answer, 3739);
}

#[test]
fn part2() {
    let (_, answer) = do_problem();
    assert_eq!(answer, 8409);
}