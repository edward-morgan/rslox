use crate::scanner::*;
use crate::syntax_tree::*;
use crate::RsLox;

/*
Grammar:
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
*/
pub struct Parser {
    tokens: Vec<Token>,
    cur: i32,
}
impl Parser {
  pub fn new(tk: Vec<Token>) -> Self {
    Parser { tokens: tk, cur: 0 }
  }
}

pub fn expression(p: &mut Parser) -> Result<Expr, String> {
    println!("Parsing an expression...");
    equality(p)
}

fn binary_expr(
    p: &mut Parser,
    sub_expr: fn(&mut Parser) -> Result<Expr, String>,
    token_match: &[TokenType],
) -> Result<Expr, String> {
    match sub_expr(p) {
        Ok(mut left) => {
            loop {
                if p.tokens.len() == 0 {
                    return Ok(left);
                } else if token_matches(&p.tokens[p.cur as usize], token_match) {
                    let op: Token = p.tokens.pop().unwrap();
                    match sub_expr(p) {
                        Ok(right) => left = Expr::Binary(Box::new(left), op, Box::new(right)),
                        e2 @ Err(_) => return e2,
                    }
                } else {
                    break;
                }
            }
            Ok(left)
        }
        e @ Err(_) => e,
    }
}

fn equality(p: &mut Parser) -> Result<Expr, String> {
    binary_expr(
        p,
        comparison,
        &[TokenType::BangEqual, TokenType::EqualEqual],
    )
}
fn comparison(p: &mut Parser) -> Result<Expr, String> {
    binary_expr(
        p,
        term,
        &[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ],
    )
}
fn term(p: &mut Parser) -> Result<Expr, String> {
    binary_expr(p, factor, &[TokenType::Plus, TokenType::Minus])
}
fn factor(p: &mut Parser) -> Result<Expr, String> {
    binary_expr(p, unary, &[TokenType::Slash, TokenType::Star])
}

fn unary(p: &mut Parser) -> Result<Expr, String> {
    if p.tokens.len() == 0 {
      parse_error(&Token::new(TokenType::Eof, String::from(""), Box::new(-1), 0), String::from("reached EOF"))
    } else {
        let t = p.tokens.pop().unwrap();
        if token_matches(&t, &[TokenType::Bang, TokenType::Minus]) {
            // Recursive unary
            // let right = unary(p);
            // Expr::Unary(Box::new(right), t)
            match unary(p) {
                Ok(right) => Ok(Expr::Unary(Box::new(right), t)),
                e @ Err(_) => e,
            }
        } else {
            // Primary
            primary(p)
        }
    }
}

fn primary(p: &mut Parser) -> Result<Expr, String> {
    if p.tokens.len() == 0 {
      // TODO: How to grab the line?
      parse_error(&Token::new(TokenType::Eof, String::from(""), Box::new(-1), 0), String::from("reached EOF"))
    } else {
        let t = p.tokens.pop().unwrap();
        if token_matches(&t, &[TokenType::False]) {
            Ok(Expr::BoolLiteral(false))
        } else if token_matches(&t, &[TokenType::True]) {
            Ok(Expr::BoolLiteral(true))
        } else if token_matches(&t, &[TokenType::Nil]) {
            Ok(Expr::NilLiteral())
        } else if token_matches(&t, &[TokenType::Number]) {
            match t.lexeme.parse::<i64>() {
                Err(_) => {
                    // Handle errors here
                    let res = t.lexeme.parse::<f64>().unwrap();
                    Ok(Expr::FloatLiteral(res))
                }
                Ok(v) => Ok(Expr::IntLiteral(v)),
            }
        } else if token_matches(&t, &[TokenType::Str]) {
            Ok(Expr::StringLiteral(t.lexeme))
        } else if token_matches(&t, &[TokenType::LeftParen]) {
            match expression(p) {
                Ok(sub) => {
                    if p.tokens.len() == 0 {
                      parse_error(&t, String::from("reached EOF"))
                    } else {
                        let end = p.tokens.pop().unwrap();
                        if token_matches(&end, &[TokenType::RightParen]) {
                            Ok(Expr::Grouping(Box::new(sub)))
                        } else {
                          parse_error(&end, String::from("could not find matching right paren"))
                        }
                    }
                }
                e @ Err(_) => e,
            }
        } else {
            parse_error(&t, String::from("unknown token"))
        }
    }
}

fn parse_error(t: &Token, msg: String) -> Result<Expr, String> {
  if t.token_type == TokenType::Eof {
    Err(format!("[line {}] Error at end: {}", t.line, msg))
  } else {
    Err(format!("[line {}] Error at '{}': {}", t.line, t.lexeme, msg))
  }
}

// Figure out if <t> matches any type in <types>
fn token_matches(t: &Token, types: &[TokenType]) -> bool {
    for ty in types {
        if t.token_type == *ty {
            return true;
        }
    }
    return false;
}
