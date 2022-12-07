use aoc_common::{IteratorExt, file_lines};

#[derive(Copy, Clone)]
enum Token {
    Plus,
    Star,
    LeftParen,
    RightParen,
    Digit(u64),
}

fn lex(s: &str) -> Vec<Token> {
    s.chars().filter_map(|c| {
        match c {
            '+' => Some(Token::Plus),
            '*' => Some(Token::Star),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            c if c.is_digit(10) => Some(Token::Digit(c.to_digit(10).unwrap() as u64)),
            ' ' => None,
            _ => panic!(),
        }
    }).to_vec()
}

enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(u64),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(s: &str) -> Parser {
        Parser { 
            tokens: lex(s),
            current: 0
        }
    }

    fn parse_no_prec(&mut self) -> Expr {
        let mut expr = self.primary(Self::parse_no_prec);

        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Star)) {
            let tok = self.next_token().unwrap();
            let right = self.primary(Self::parse_no_prec);
            expr = Expr::Binary(Box::new(expr), tok, Box::new(right));
        }

        expr
    }

    fn parse_prec(&mut self) -> Expr {
        let mut expr = self.term();

        while matches!(self.peek(), Some(Token::Star)) {
            self.next_token().unwrap();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), Token::Star, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.primary(Self::parse_prec);

        while matches!(self.peek(), Some(Token::Plus)) {
            self.next_token().unwrap();
            let right = self.primary(Self::parse_prec);
            expr = Expr::Binary(Box::new(expr), Token::Plus, Box::new(right));
        }

        expr
    }

    fn primary(&mut self, recurse: fn(&mut Self) -> Expr) -> Expr {
        match self.next_token().unwrap() {
            Token::Digit(i) => Expr::Literal(i),
            Token::LeftParen => {
                let expr = recurse(self);
                self.next_token().unwrap();
                Expr::Grouping(Box::new(expr))
            }
            _ => panic!()
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.current < self.tokens.len() {
            Some(self.tokens[self.current])
        } else {
            None
        }
    }

    fn next_token(&mut self) -> Option<Token> { 
        if self.current < self.tokens.len() {
            self.current += 1;
            Some(self.tokens[self.current - 1])
        } else {
            None
        }
    }
}

fn evaluate_expression(expr: &Expr) -> u64 {
    match expr {
        Expr::Literal(i) => *i,
        Expr::Grouping(expr) => evaluate_expression(expr),
        Expr::Binary(left, op, right) => {
            let left = evaluate_expression(left);
            let right = evaluate_expression(right);

            match op {
                Token::Plus => left + right,
                Token::Star => left * right,
                _ => panic!()
            }
        }
    }
}
fn run(parse: fn(&mut Parser) -> Expr) -> u64 {
    file_lines("inputs/day18.txt").map(|s| {
        let mut parser = Parser::new(&s);
        evaluate_expression(&parse(&mut parser))
    }).sum::<u64>()
}

#[test]
fn part1() {
    let answer = run(Parser::parse_no_prec);
    assert_eq!(answer, 701339185745);
}

#[test]
fn part2() {
    let answer = run(Parser::parse_prec);
    assert_eq!(answer, 4208490449905);
}