use crate::{scanner::{Token, TokenType}, interpreter::Nil};
use core::any::Any;

#[derive(Debug)]
pub enum Expr {
    // Literal(Box<dyn Any>),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    NilLiteral(),
    Unary(Box<Expr>, Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
}

pub fn visit(e: Expr) -> String {
    match e {
        Expr::StringLiteral(v) => format!("{}", v),
        Expr::IntLiteral(v) => format!("{}", v),
        Expr::FloatLiteral(v) => format!("{}", v),
        Expr::BoolLiteral(v) => format!("{}", v),
        Expr::NilLiteral() => format!("nil"),
        Expr::Unary(e, t) => format!("( {} {} )", t.lexeme, visit(*e)),
        Expr::Binary(e1, t, e2) => format!("( {} {} {} )", t.lexeme, visit(*e1), visit(*e2)),
        Expr::Ternary(e1, e2, e3) => {
            format!("( {} ? {} : {} )", visit(*e1), visit(*e2), visit(*e3))
        }
        Expr::Grouping(e) => format!("( {} )", visit(*e)),
    }
}

pub fn visit_any(b: Box<dyn Any>) -> String {
    if (*b).is::<String>() {
       String::from((*b).downcast_ref::<String>().unwrap().as_str())
    }
    else if (*b).is::<i64>() {
        format!("{}", *(*b).downcast_ref::<i64>().unwrap())
    }
    else if (*b).is::<f64>() {
        format!("{}", *(*b).downcast_ref::<f64>().unwrap())
    }
    else if (*b).is::<bool>() {
        format!("{}", *(*b).downcast_ref::<bool>().unwrap())
    }
    else if (*b).is::<Nil>() {
        String::from("nil")
    } else {
        String::from("")
    }
}


