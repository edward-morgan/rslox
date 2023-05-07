use core::any::Any;
use crate::scanner::Token;

pub enum Expr {
    // Literal(Box<dyn Any>),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    Unary(Box<Expr>, Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

pub fn visit(e: Expr) -> String {
    match e {
        // Expr::Literal(t) => { 
        //     if let Some(v) = (*t).downcast_ref::<String>() {
        //         format!("{}", v)
        //     } 
        //     else {
        //         if let Some(f) = (*t).downcast_ref::<f64>() {
        //             format!("{}", f)
        //         }
        //         else {
        //             if let Some(i) = (*t).downcast_ref::<i64>() {
        //                 format!("{}", i)
        //             }
        //             else {
        //                 format!("UNKNOWN TYPE")
        //             }
        //         }
        //     }
        // },
        Expr::StringLiteral(v) => format!("{}", v),
        Expr::IntLiteral(v) => format!("{}", v),
        Expr::FloatLiteral(v) => format!("{}", v),
        Expr::BoolLiteral(v) => format!("{}", v),
        Expr::Unary(e, t) => format!("( {} {} )", t.lexeme, visit(*e)),
        Expr::Binary(e1, t, e2) => format!("( {} {} {} )", t.lexeme, visit(*e1), visit(*e2)),
        Expr::Grouping(e) => format!("( {} )", visit(*e)),
    }
}
