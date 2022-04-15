use std::any::Any;

pub struct Scanner {
    source: Vec<u8>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub cur: usize,
    pub line: u32
}
impl Scanner {
    /**
     * Initialize a Scanner from a source of program instructions.
     */
    pub fn new(source: String) -> Scanner {
        Scanner { source: source.bytes().collect(), tokens: vec![], start: 0, cur: 0, line: 1}
    }

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        while self.cur < self.source.len() {
            self.start = self.cur;
            let res: Result<(), String> = self.scan_token();
            if !res.is_ok() {
                return res;
            }
        }
        Ok(self.tokens.push(Token::new(TokenType::Eof, "".to_string(), Box::new(0), self.line)))
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
            '/' => Ok(self.add_token(TokenType::Slash)),
            '*' => Ok(self.add_token(TokenType::Star)),
            x => Err(format!("Unexpected token `{}` ({}).", x, x as usize))
        }    
    }

    fn advance(&mut self) -> u8 {
        self.cur += 1;
        return self.source[self.cur]
    }

    fn add_token(&mut self, token_t: TokenType) {
        self._add_token(token_t, Box::new(0));
    }

    fn _add_token(&mut self, token_t: TokenType, literal: Box<dyn Any>) {
        let text: &[u8] = &self.source[self.start..self.cur];
        self.tokens.push(Token::new(token_t, String::from_utf8(text.to_vec()).ok().unwrap(), literal, self.line));
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
            line
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