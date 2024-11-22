use aoc_common::file_string;
use json::*;


fn input() -> JsonDocument {
    JsonDocument::parse(&file_string("inputs/day12.txt"))
}

fn run(object_test: fn(&JsonValue) -> bool) -> i32 {
    fn count(value: &JsonValue, object_test: fn(&JsonValue) -> bool) -> i32 {
        match value {
            JsonValue::Number(n) => *n,
            JsonValue::Array(a) => a.iter().map(|v| count(v, object_test)).sum(),
            JsonValue::Object(o) => {
                if object_test(value) {
                    o.values().map(|v| count(v, object_test)).sum()
                } else {
                    0
                }
            },
            _ => 0,
        }
    }

    count(input().value(), object_test)
}

#[test]
fn part1() {
    fn object_test(_: &JsonValue) -> bool { true }

    let answer = run(object_test);
    assert_eq!(answer, 111754);
}

#[test]
fn part2() {
    fn object_test(o: &JsonValue) -> bool { 
        match o {
            JsonValue::Object(o) => {
                !o.values().any(|v| matches!(v, JsonValue::String(ref s) if s == "red"))
            },
            _ => panic!()
        }
     }

    let answer = run(object_test);
    assert_eq!(answer, 65402);
}

mod json {
    use std::collections::HashMap;

    pub enum JsonValue {
        Object(Box<HashMap<String, JsonValue>>),
        Array(Box<Vec<JsonValue>>),
        String(String),
        Number(i32),
        True,
        False,
        Null,
    }

    pub struct JsonDocument(JsonValue);

    impl JsonDocument {
        pub fn parse(s: &str) -> Self {
            JsonDocument(Parser::parse(s))
        }

        pub fn value(&self) -> &JsonValue {
            &self.0
        }
    }

    struct Parser {
        tokens: Vec<Token>,
        idx: usize,
    }

    impl Parser {
        fn parse(s: &str) -> JsonValue {
            let mut parser = Self {
                tokens: scan(s),
                idx: 0
            };

            parser.parse_value()
        }

        fn parse_value(&mut self) -> JsonValue {
            match self.tokens[self.idx] {
                Token::OpenBracket => self.parse_array(),
                Token::OpenBrace => self.parse_object(),
                Token::DoubleQuote => self.parse_string(),
                Token::Number(n) => {
                    self.idx += 1;
                    JsonValue::Number(n)
                } 
                Token::Identifier(ref s) => {
                    self.idx += 1;
                    match s.as_str() {
                        "true" => JsonValue::True,
                        "false" => JsonValue::False,
                        "null" => JsonValue::Null,
                        _ => panic!()
                    }
                },
                _ => panic!()
            }
        }

        fn parse_array(&mut self) -> JsonValue {
            if !matches!(self.tokens[self.idx], Token::OpenBracket) {
                panic!();
            }
            self.idx += 1;

            let mut array: Vec<JsonValue> = Vec::new();

            if !matches!(self.tokens[self.idx], Token::CloseBracket) {
                loop {
                    array.push(self.parse_value());
    
                    match self.tokens[self.idx] {
                        Token::CloseBracket => break,
                        Token:: Comma => (),
                        _ => panic!()
                    }
    
                    self.idx += 1;
                }
            }

            self.idx += 1;

            JsonValue::Array(Box::new(array))
        }

        fn parse_object(&mut self) -> JsonValue {
            if !matches!(self.tokens[self.idx], Token::OpenBrace) {
                panic!();
            }
            self.idx += 1;

            let mut members: HashMap<String, JsonValue> = HashMap::new();

            if !matches!(self.tokens[self.idx], Token::CloseBrace) {
                loop {
                    if !matches!(self.tokens[self.idx], Token::DoubleQuote) { panic!(); }
                    self.idx += 1;
    
                    let s = match self.tokens[self.idx] {
                        Token::Identifier(ref s) => s.to_string(),
                        _ => panic!()
                    };
                    self.idx += 1;
    
                    if !matches!(self.tokens[self.idx], Token::DoubleQuote) { panic!(); }
                    self.idx += 1;
    
                    if !matches!(self.tokens[self.idx], Token::Colon) { panic!(); }
                    self.idx += 1;

                    members.insert(s, self.parse_value());

                    match self.tokens[self.idx] {
                        Token::CloseBrace => break,
                        Token::Comma => (),
                        _ => panic!()
                    }

                    self.idx += 1;
                }
            }

            self.idx += 1;

            JsonValue::Object(Box::new(members))
        }

        fn parse_string(&mut self) -> JsonValue {
            if !matches!(self.tokens[self.idx], Token::DoubleQuote) {
                panic!();
            }

            self.idx += 1;
            let s = match self.tokens[self.idx] {
                Token::Identifier(ref s) => s.to_string(),
                _ => panic!(),
            };

            self.idx += 1;
            if !matches!(self.tokens[self.idx], Token::DoubleQuote) {
                panic!();
            }

            self.idx += 1;
            JsonValue::String(s)
        }
    }

    enum Token {
        OpenBracket,
        CloseBracket,
        OpenBrace,
        CloseBrace,
        DoubleQuote,
        Colon,
        Comma,
        Number(i32),
        Identifier(String),
    }

    fn scan(s: &str) -> Vec<Token> {
        let bytes = s.as_bytes();
        let mut idx = 0;
        let mut tokens = Vec::new();

        while idx < bytes.len() {
            let start = idx;

            match bytes[idx] {
                b'[' => tokens.push(Token::OpenBracket),
                b']' => tokens.push(Token::CloseBracket),
                b'{' => tokens.push(Token::OpenBrace),
                b'}' => tokens.push(Token::CloseBrace),
                b'"' => tokens.push(Token::DoubleQuote),
                b':' => tokens.push(Token::Colon),
                b',' => tokens.push(Token::Comma),
                b'-' | b'0'..=b'9' => {
                    idx += 1;
                    while idx < bytes.len() && matches!(bytes[idx], b'0'..=b'9') {
                        idx += 1;
                    }
                    tokens.push(Token::Number(String::from_utf8(bytes[start..idx].to_vec()).unwrap().parse().unwrap()));
                    idx -= 1;
                },
                b'a'..=b'z' => {
                    idx += 1;
                    while idx < bytes.len() && matches!(bytes[idx], b'a'..=b'z') {
                        idx += 1;
                    }
                    tokens.push(Token::Identifier(String::from_utf8(bytes[start..idx].to_vec()).unwrap()));
                    idx -= 1;
                },
                b if b.is_ascii_whitespace() => {
                    idx += 1;
                    while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                        idx += 1;
                    }
                    idx -= 1;
                },
                _ => panic!() 
            }

            idx += 1;
        }

        tokens
    }
}