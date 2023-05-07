use crate::scanner::*;
use crate::syntax_tree::*;

struct Parser {
  pub tokens: Vec<Token>,
  cur: i32,
}
// impl Parser {

//   fn expression() -> Expr {
//     equality()
//   }

//   fn equality() -> Expr {
//     let expr = comparison();

//     loop {
//       matchToken(&[TokenType::BangEqual, TokenType::EqualEqual]);
      
//     }
//   }

//   fn comparison() -> Expr {
    
//   }

//   fn matchToken(types: &[TokenType]) -> bool {

//   }

// }

pub fn parse_tokens(tokens: &Vec<Token>, cur: i32, ast: &mut Expr) {
  if tokens.len() > 0 {
    let t =  &tokens[cur as usize];
    match t {
      Token { token_type: TokenType::LeftParen, lexeme: lex, .. }=> println!("{}", lex),
      _ => println!("Not found."),
    }
  } 
}