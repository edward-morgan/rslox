use std::any::Any;
use crate::err;

pub struct Scanner {
    source: Vec<u8>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub cur: usize,
    pub line: u32,
}
impl Scanner {
    /**
     * Initialize a Scanner from a source of program instructions.
     */
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.bytes().collect(),
            tokens: vec![],
            start: 0,
            cur: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        while self.cur < self.source.len() {
            self.start = self.cur;
            let res: Result<(), String> = self.scan_token();
            if !res.is_ok() {
                return res;
            }
        }
        Ok(self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Box::new(0),
            self.line,
        )))
    }

    fn scan_token(&mut self) -> Result<(), String> {
        //TODO: This only holds if we use ASCII strings ONLY
        let c = self.advance() as char;
        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            ';' => Ok(self.add_token(TokenType::Semicolon)),
            '*' => Ok(self.add_token(TokenType::Star)),
            '!' => {
                let matched_char = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Ok(self.add_token(matched_char))
            }
            '=' => {
                let matched_char = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Ok(self.add_token(matched_char))
            }
            '<' => {
                let matched_char = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                Ok(self.add_token(matched_char))
            }
            '>' => {
                let matched_char = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                Ok(self.add_token(matched_char))
            }
            '/' => {
                if self.match_char('/') {
                    // If a comment, advance to the end of the line
                    while self.peek() != '\n' && self.cur < self.source.len() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenType::Slash))
                }
            }
            // Ignore whitespace
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => Ok(self.line += 1),
            '"' => self.string(),
            x => err(self.line, format!("Unexpected token `{}` ({}).", x, x as usize).as_str()),
        }
    }

    /**
     * Conditionally advance `cur` based on whether the current character matches `expected`. `cur` is advanced _after_
     * the comparison.
     */
    fn match_char(&mut self, expected: char) -> bool {
        if self.cur == self.source.len() || self.source[self.cur] as char != expected {
            false
        } else {
            self.cur += 1;
            true
        }
    }

    /**
     * Return the next character without advancing `cur`.
     */
    fn peek(&self) -> char {
        if self.cur == self.source.len() {
            '\0'
        } else {
            self.source[self.cur] as char
        }
    }

    /**
     * Returns the u8 at the current position in the scanner and increments the current position _afterwards_.
     */
    fn advance(&mut self) -> u8 {
        let res = self.source[self.cur];
        self.cur += 1;
        return res;
    }

    fn add_token(&mut self, token_t: TokenType) {
        self._add_token(token_t, Box::new(0));
    }

    fn _add_token(&mut self, token_t: TokenType, literal: Box<dyn Any>) {
        let text: &[u8] = &self.source[self.start..self.cur];
        self.tokens.push(Token::new(
            token_t,
            String::from_utf8(text.to_vec()).ok().unwrap(),
            literal,
            self.line,
        ));
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.cur < self.source.len() {
            self.advance();
        }

        if self.cur == self.source.len() - 1 {
            return err(self.line, "unterminated string")
        }

        // Grab the closing '"'
        self.advance();

        match std::str::from_utf8(&self.source[(self.start - 1)..(self.cur - 1)]) {
            Ok(v) => {
                let s = String::from(v);
                self._add_token(TokenType::Str, Box::new(s));
                Ok(())
            }
            Err(e) => {
                println!("{}", e);
                err(self.line, "could not parse source")
            }
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Box<dyn Any>,
    pub line: u32,
}
impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Box<dyn Any>, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// Debug is good enough here, it prints the enum name
#[derive(Debug)]
pub enum TokenType {
    // One-character tokens
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

    // // One- or two-character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    Str,
    Number,

    //  Keywords
    And,
    Class,
    Else,
    False,
    Func,
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

    Eof,
}
