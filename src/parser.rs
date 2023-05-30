use crate::scanner::*;
use crate::syntax_tree::*;

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
struct Parser {
  pub tokens: Vec<Token>,
  cur: i32,
}

  fn expression(p: &mut Parser) -> Expr {
    equality(p)
  }

  fn binary_expr(p: &mut Parser, sub_expr: fn(&mut Parser) -> Expr, token_match: &[TokenType]) -> Expr {
    let mut expr: Expr = sub_expr(p);
    loop {
      if p.tokens.len() == 0 {
        return expr
      } else if token_matches(&p.tokens[p.cur as usize], token_match) {
        let op: Token = p.tokens.pop().unwrap();
        let right: Expr = sub_expr(p);
        expr = Expr::Binary(Box::new(expr), op, Box::new(right));
      } else {
        break
      }
    }
    expr
  }

  fn equality(p: &mut Parser) -> Expr {
    binary_expr(p, comparison, &[TokenType::BangEqual, TokenType::EqualEqual])
  }
  fn comparison(p: &mut Parser) -> Expr {
    binary_expr(p, term, &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual])
  }
  fn term(p: &mut Parser) -> Expr {
    binary_expr(p, factor, &[TokenType::Plus, TokenType::Minus])
  }
  fn factor(p: &mut Parser) -> Expr {
    binary_expr(p, unary, &[TokenType::Slash, TokenType::Star])
  }

  fn unary(p: &mut Parser) -> Expr {
    if p.tokens.len() == 0 {
      panic!("unary() Empty tokens.")
    } else {
      let t = p.tokens.pop().unwrap();
      if token_matches(&t, &[TokenType::Bang, TokenType::Minus]) { // Recursive unary
        let right = unary(p);
        Expr::Unary(Box::new(right), t)
      } else { // Primary
        primary(p)
      }
    }
  }

  fn primary(p: &mut Parser) -> Expr {
    if p.tokens.len() == 0 {
      panic!("unary() Empty tokens.")
    } else {
      let t = p.tokens.pop().unwrap();
      if token_matches(&t, &[TokenType::False]) {
        Expr::BoolLiteral(false)
      } else if token_matches(&t, &[TokenType::True]) {
        Expr::BoolLiteral(true)
      } else if token_matches(&t, &[TokenType::Nil]) {
        Expr::NilLiteral()
      } else if token_matches(&t, &[TokenType::Number]) {
        match t.lexeme.parse::<i64>() {
          Err(_) => {
            let res = t.lexeme.parse::<f64>().unwrap();
            Expr::FloatLiteral(res)
          },
          Ok(v) => Expr::IntLiteral(v),
        }
      } else if token_matches(&t, &[TokenType::Str]) {
        Expr::StringLiteral(t.lexeme)
      } else if token_matches(&t, &[TokenType::LeftParen]) {
        let sub = expression(p);
        if p.tokens.len() == 0 {
          panic!("");
        } else {
          let end = p.tokens.pop().unwrap();
          if token_matches(&end, &[TokenType::RightParen]) {
            Expr::Grouping(Box::new(sub))
          } else {
            panic!("")
          }
        }
      } else {
        panic!("Unknown token {:?}", t);
      }
    }
  }

  // fn equality(p: &mut Parser) -> Expr {
  //   let mut expr: Expr = comparison(p);
  //   loop {
  //     if p.tokens.len() == 0 {
  //       return expr
  //     } else if token_matches(&p.tokens[p.cur as usize], &[TokenType::BangEqual, TokenType::EqualEqual]) {
  //       let op: Token = p.tokens.pop().unwrap();
  //       let right: Expr = comparison(p);
  //       expr = Expr::Binary(Box::new(expr), op, Box::new(right));
  //     } else {
  //       break
  //     }
  //   }
  //   expr
  // }

  // fn comparison(p: &mut Parser) -> Expr {
  //   let mut expr: Expr = term(p);
  //   loop {
  //     if p.tokens.len() == 0 {
  //       return expr
  //     } else if token_matches(&p.tokens[p.cur as usize], &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
  //       let op: Token = p.tokens.pop().unwrap();
  //       let right: Expr = term(p);
  //       expr = Expr::Binary(Box::new(expr), op, Box::new(right));
  //     } else {
  //       break
  //     }
  //   }
  //   expr
  // }

  // fn term(p: &mut Parser) -> Expr {
  //   let mut expr: Expr = factor(p);
  //   loop {
  //     if p.tokens.len() == 0 {
  //       return expr
  //     } else if token_matches(&p.tokens[p.cur as usize], &[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
  //       let op: Token = p.tokens.pop().unwrap();
  //       let right: Expr = factor(p);
  //       expr = Expr::Binary(Box::new(expr), op, Box::new(right));
  //     } else {
  //       break
  //     }
  //   }
  //   expr
  // }

  fn token_matches(t: &Token, types: &[TokenType]) -> bool {
    for ty in types {
      if t.token_type == *ty {
        return true
      } 
    }
    return false
  }
