use std::any::Any;
use std::collections::HashMap;
use crate::err;

pub struct Scanner {
    source: Vec<u8>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub cur: usize,
    pub line: u32,
    pub reserved_identifiers: HashMap<String, TokenType>,
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
            reserved_identifiers: reserved_identifiers(),
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
            '?' => Ok(self.add_token(TokenType::QuestionMark)),
            ':' => Ok(self.add_token(TokenType::Colon)),
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
                } else if self.match_char('*') {
                    // If a multiline comment, advance until you see the closing */
                    let mut star_found = false;
                    while self.cur < self.source.len() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        } else if self.peek() == '/' && star_found {
                            self.advance();
                            break;
                        } else if self.peek() == '*' {
                            star_found = true;
                        } else {
                            star_found = false;
                        }
                        // Don't advance until the end of the loop iteration. This is because match_char also advances, so we don't want to skip
                        // over the character immediately following the '/*'
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
            x if is_digit(x) => self.number(x),
            x if is_alpha(x) => self.identifier(),
            x => {
                println!("Current tokens: {:?}", self.tokens);
                err(self.line, format!("Unexpected token `{}` ({}).", x, x as usize).as_str())
            },
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
     * Return the current character without advancing `cur`.
     */
    fn peek(&self) -> char {
        if self.cur == self.source.len() {
            '\0'
        } else {
            self.source[self.cur] as char
        }
    }

    /**
     * Return the next character without advancing `cur`. Use for single-character lookahead.
     */
    fn peek_next(&self) -> char {
        if self.cur + 1 > self.source.len() { 
            '\0'
        } else {
            self.source[self.cur + 1] as char
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

    fn number(&mut self, start: char) -> Result<(), String> {
        let mut num: Vec<u8> = vec![start as u8];
        // Grab all digits
        while is_digit(self.peek()) {
            num.push(self.advance());
        }

        // Look for a decimal and consume it
        if self.peek() == '.' && is_digit(self.peek_next()) {
            num.push(self.advance());
        }

        // Consume the fractional part
        while is_digit(self.peek()) {
            num.push(self.advance());
        }

        println!("{:?}", num);
        let num_str: String = String::from_utf8(num).unwrap();
        match num_str.parse::<f64>() {
            Ok(n) => {
                self._add_token(TokenType::Number, Box::new(n));
                Ok(())
            }
            Err(exc) => {
                err(self.line, format!("could not parse number `{}`: {}", num_str, exc).as_str())
            }
        }
    }

    /**
     * Parse a string out-may be multiple characters. Returns an error if the string is unterminated. Multi-line strings are allowed.
     */
    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && self.cur < self.source.len() {
            if self.peek() == '\n' {
                self.line += 1;
            }
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

    /**
     * Parse an identifier, which may be multiple characters. After identifier scanning has completed, the resulting token will be checked
     * to see if it matches any reserved words; this can only be done after scanning because of the requirement for maximal munch.
     */
    fn identifier(&mut self) -> Result<(), String> {
        while is_alphanumeric(self.peek()) && self.cur < self.source.len() {
            self.advance();
        }
        
        match std::str::from_utf8(&self.source[self.start..(self.cur - 1)]) {
            Ok(v) => {
                let s = String::from(v);
                match self.reserved_identifiers.get(&s) {
                    Some(reserved) => self._add_token(*reserved, Box::new(s)),
                    None => self._add_token(TokenType::Str, Box::new(s)),
                }
                Ok(())
            }
            Err(e) => {
                println!("{}", e);
                err(self.line, "could not parse source")
            }
        }
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

/**
 * Set the list of reserved words to be used.
 */
fn reserved_identifiers() -> HashMap<String, TokenType> {
    // Populate reserved identifiers
    let mut reserved_identifiers = HashMap::<String, TokenType>::new();
    reserved_identifiers.insert(String::from("and"), TokenType::And);
    reserved_identifiers.insert(String::from("class"), TokenType::Class);
    reserved_identifiers.insert(String::from("else"), TokenType::Else);
    reserved_identifiers.insert(String::from("false"), TokenType::False);
    reserved_identifiers.insert(String::from("for"), TokenType::For);
    reserved_identifiers.insert(String::from("func"), TokenType::Func);
    reserved_identifiers.insert(String::from("if"), TokenType::If);
    reserved_identifiers.insert(String::from("nil"), TokenType::Nil);
    reserved_identifiers.insert(String::from("or"), TokenType::Or);
    reserved_identifiers.insert(String::from("print"), TokenType::Print);
    reserved_identifiers.insert(String::from("return"), TokenType::Return);
    reserved_identifiers.insert(String::from("super"), TokenType::Super);
    reserved_identifiers.insert(String::from("this"), TokenType::This);
    reserved_identifiers.insert(String::from("true"), TokenType::True);
    reserved_identifiers.insert(String::from("var"), TokenType::Var);
    reserved_identifiers.insert(String::from("while"), TokenType::While);
    return reserved_identifiers;
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Box<dyn Any>,
    pub line: u32,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Box<dyn Any>, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// Debug is good enough here, it prints the enum name
#[derive(Debug, Clone, Copy, PartialEq)]
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
    QuestionMark,
    Colon,

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
