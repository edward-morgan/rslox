

pub struct Scanner {
    source: String,
}
impl Scanner {
    /**
     * Initialize a Scanner from a source of program instructions.
     */
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        println!("unimplemented");
        Vec::new()
    }
}

//TODO: make pub?
#[derive(Debug)]
pub struct Token {}

enum TokenType {
    // One-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // // One- or two-character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, Str, Number,

    //  Keywords
    And, Class, Else, False, Func, For, If, Nil, Or, 
    Print, Return, Super, This, True, Var, While,

    Eof
}