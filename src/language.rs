//use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[allow(dead_code)]
struct Token {
    tok_type: TokenType,
    start: i32,
    length: i32,
    line: i32,
}

struct Scanner {
    start: i32,
    current: i32,
    line: i32,
    buffer: String,
}

struct Compiler {
    scanner: Scanner,
}

impl Scanner {
    fn advance(&mut self) -> char
    {
        self.current += 1;
        self.buffer.as_bytes()[(self.current - 1) as usize] as char
    }
    fn is_at_end(&self) -> bool
    {
        let len = self.buffer.len();
        if (len != 0) & ((self.current as usize) < len)
        { return false; }
        true
    }
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        let curr_char = self.buffer.as_bytes()[self.current as usize] as char;
        if curr_char != expected { return false; }
        self.current += 1;
        true
    }
    fn peek(&self) -> char
    {
        self.buffer.as_bytes()[self.current as usize] as char
    }
    fn peek_next(&self) -> char
    {
        if self.is_at_end() { return '\0'; }
        self.buffer.as_bytes()[(self.current + 1) as usize] as char
    }
    fn skip_whitespaces(&mut self)
    {
        loop {
            let c = self.peek();
            match c {
                ' ' => self.advance(),
                '\r' => self.advance(),
                '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                    '\n'
                }
                '/' => {
                    let p = self.peek_next();
                    match p {
                        '/' => {
                            //swallow the comment until end of the line
                            while !self.is_at_end() && self.peek() != '\n' {
                                self.advance();
                            };
                            if self.is_at_end() { return; }
                            '\0'
                        }
                        _ => return,
                    }
                }
                _ => return,
            };
        }
    }
}

impl Compiler {
    fn from_source(source: &String) -> Self {
        Self {
            scanner: Scanner {
                start: 0,
                current: 0,
                line: 1,
                buffer: source.to_string(),
            },
        }
    }
    fn make_token(&self, tok_type: TokenType) -> Token {
        Token {
            tok_type,
            start: self.scanner.start,
            length: self.scanner.current - self.scanner.start,
            line: self.scanner.line,
        }
    }
    fn scan_token(&mut self) -> Token {
        //if we are at the end of the file we return
        if self.scanner.is_at_end() { return self.make_token(TokenType::EOF); }

        self.scanner.skip_whitespaces();

        //we skipped the whitespaces, that might have left us at the end of file, so we do an
        //extra check
        if self.scanner.is_at_end() { return self.make_token(TokenType::EOF); }

        let c = self.scanner.advance();

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => return match self.scanner.match_next('=') {
                true => self.make_token(TokenType::BangEqual),
                false => self.make_token(TokenType::Bang),
            },
            '=' => return match self.scanner.match_next('=') {
                true => self.make_token(TokenType::EqualEqual),
                false => self.make_token(TokenType::Equal),
            },
            '<' => return match self.scanner.match_next('=') {
                true => self.make_token(TokenType::LessEqual),
                false => self.make_token(TokenType::Less),
            },
            '>' => return match self.scanner.match_next('=') {
                true => self.make_token(TokenType::GreaterEqual),
                false => self.make_token(TokenType::Greater),
            },
            //'"' => return self.scanner.string(),
            _ => (),
        }


        Token {
            tok_type: TokenType::EOF,
            start: 0,
            length: 0,
            line: 1,
        }
    }
    fn parse(&mut self) -> bool {
        let mut line = -1;
        loop {
            let token = self.scan_token();
            if token.line != line {
                print!("{} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            print!("{:?} {} '{}'\n", token.tok_type, token.length, token.start);

            if token.tok_type == TokenType::EOF {
                break;
            }
        }
        return true;
    }
}

/*
#[wasm_bindgen]
pub fn test_me(a: i32, b: i32) -> i32 {
    println!("hello world from rust");
    a + b
}
*/

#[test]
fn compiler_start_up() {
    let source = "".to_string();
    let compiler = Compiler::from_source(&source);
    assert_eq!(compiler.scanner.current, 0);
}

#[test]
fn compiler_parse_empty() {
    let source = "".to_string();
    let mut compiler = Compiler::from_source(&source);
    let result = compiler.parse();
    assert_eq!(result, true);
}

#[test]
fn scanner_parse_unary_values() {
    let source = "(){};,.-+/*!<!===<=<>=>".to_string();
    let mut compiler = Compiler::from_source(&source);
    let mut tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LeftParen, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightParen, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LeftBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Semicolon, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Comma, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Dot, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Minus, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Plus, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Slash, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Star, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Bang, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Less, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::BangEqual, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EqualEqual, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LessEqual, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Less, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::GreaterEqual, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Greater, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EOF, true);
}

#[test]
fn scanner_parse_unary_values_white_spaces() {
    let source = " (      \t  )\r\r{".to_string();
    let mut compiler = Compiler::from_source(&source);
    let mut tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LeftParen, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightParen, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LeftBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EOF, true);
}

#[test]
fn scanner_parse_unary_values_new_line() {
    let source = "}}}\n{{{".to_string();
    let mut compiler = Compiler::from_source(&source);
    let mut tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(compiler.scanner.line == 2, true);
    assert_eq!(tok.tok_type == TokenType::LeftBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LeftBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::LeftBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EOF, true);
}

#[test]
fn scanner_parse_unary_values_comment_1() {
    let source = "}}}//{{{".to_string();
    let mut compiler = Compiler::from_source(&source);
    let mut tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EOF, true);
}

#[test]
fn scanner_parse_unary_values_comment_2() {
    let source = "}}}\n//{{{".to_string();
    let mut compiler = Compiler::from_source(&source);
    let mut tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EOF, true);
}

#[test]
fn scanner_parse_unary_values_comment_3() {
    let source = "}}}\n//{{{\n//fksldjfsl\n}!".to_string();
    let mut compiler = Compiler::from_source(&source);
    let mut tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::RightBrace, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::Bang, true);
    tok = compiler.scan_token();
    assert_eq!(tok.tok_type == TokenType::EOF, true);
}
