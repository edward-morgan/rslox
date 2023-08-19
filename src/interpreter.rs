use crate::{
    scanner::{Token, TokenType},
    syntax_tree::Expr,
};
use core::any::Any;
use std::any::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct Nil {}

pub fn interpret(e: Expr) -> Result<Box<dyn Any>, String> {
    match e {
        Expr::StringLiteral(v) => Ok(Box::new(v)),
        Expr::IntLiteral(v) => Ok(Box::new(v as f64)),
        Expr::FloatLiteral(v) => Ok(Box::new(v)),
        Expr::BoolLiteral(v) => Ok(Box::new(v)),
        Expr::NilLiteral() => Ok(Box::new(Nil {})),
        Expr::Grouping(e) => interpret(*e),
        Expr::Unary(ue, t) => match interpret(*ue) {
            Ok(value) => match t.token_type {
                TokenType::Minus => match (*value).downcast_ref::<f64>() {
                    Some(f) => Ok(Box::new(*f)),
                    None => Err(format!("failed to convert value to f64")),
                },
                TokenType::Bang => Ok(Box::new(!is_truthful(value))),
                _ => Err(format!(
                    "could not evaluate unary expression {:?} {:?}.",
                    t, value
                )),
            },
            x @ Err(_) => x,
        },
        Expr::Binary(left, op, right) => match interpret(*left) {
            // First interpet the left expression
            Ok(left_value) => match interpret(*right) {
                // If that succeeds, interpret the right expression
                Ok(right_value) => match op.token_type {
                    TokenType::Minus => {
                        eval_binary_numeric_op(&left_value, &right_value, |x, y| x - y)
                    }
                    TokenType::Star => {
                        eval_binary_numeric_op(&left_value, &right_value, |x, y| x * y)
                    }
                    TokenType::Slash => {
                        eval_binary_numeric_op(&left_value, &right_value, |x, y| x / y)
                    }
                    TokenType::Plus =>
                    // Try to add strings; else, default to numerical otherwise
                    {
                        if (*left_value).is::<String>() && (*right_value).is::<String>() {
                            Ok(Box::new(
                                (*left_value).downcast_ref::<String>().unwrap().to_owned()
                                    + (*right_value).downcast_ref::<String>().unwrap(),
                            ))
                        } else {
                            eval_binary_numeric_op(&left_value, &right_value, |x, y| x + y)
                        }
                    }
                    TokenType::Greater => {
                        eval_binary_boolean_op(&left_value, &right_value, |x, y| x > y)
                    }
                    TokenType::GreaterEqual => {
                        eval_binary_boolean_op(&left_value, &right_value, |x, y| x >= y)
                    }
                    TokenType::Less => {
                        eval_binary_boolean_op(&left_value, &right_value, |x, y| x < y)
                    }
                    TokenType::LessEqual => {
                        eval_binary_boolean_op(&left_value, &right_value, |x, y| x <= y)
                    }
                    TokenType::EqualEqual => Ok(Box::new(are_equal(&left_value, &right_value))),
                    TokenType::BangEqual => Ok(Box::new(!are_equal(&left_value, &right_value))),
                    _ => todo!(),
                },
                Err(right_reason) => Err(format!(
                    "Could not evaluate right operand of binary expression. Reason = {}",
                    right_reason
                )),
            },
            Err(left_reason) => Err(format!(
                "Could not evaluate left operand of binary expression. Reason = {}",
                left_reason
            )),
        },
        _ => todo!(),
    }
}

pub fn eval_binary_numeric_op(
    left_value: &Box<dyn Any>,
    right_value: &Box<dyn Any>,
    op: fn(f64, f64) -> f64,
) -> Result<Box<dyn Any>, String> {
    if (*left_value).is::<i64>() {
        match (*right_value).downcast_ref::<i64>() {
            Some(right_i64) => Ok(Box::new(op((*(*left_value).downcast_ref::<i64>().unwrap()) as f64, (*right_i64) as f64))),
            None => Err(format!(
                "failed to convert right value to i64: {:?}",
                *right_value
            )),
        }
    } else if (*left_value).is::<f64>() {
        match (*right_value).downcast_ref::<f64>() {
            Some(right_f64) => Ok(Box::new(op(*(*left_value).downcast_ref::<f64>().unwrap(), *right_f64))),
            None => Err(format!(
                "failed to convert right value to f64: {:?}",
                *right_value
            )),
        }
    } else {
        Err(format!("Left value is not a numeric type."))
    }
    // match (*left_value).downcast_ref::<f64>() {
    //     Some(left_f64) => match (*right_value).downcast_ref::<f64>() {
    //         Some(right_f64) => Ok(Box::new(op(*left_f64, *right_f64))),
    //         None => Err(format!(
    //             "failed to convert right value to f64: {:?}",
    //             *right_value
    //         )),
    //     },
    //     None => Err(format!(
    //         "failed to convert left value to f64: {:?} (type: {left_value:?})",
    //         *left_value
    //     )),
    // }
}

pub fn eval_binary_boolean_op(
    left_value: &Box<dyn Any>,
    right_value: &Box<dyn Any>,
    op: fn(f64, f64) -> bool,
) -> Result<Box<dyn Any>, String> {
    match (*left_value).downcast_ref::<f64>() {
        Some(left_f64) => match (*right_value).downcast_ref::<f64>() {
            Some(right_f64) => Ok(Box::new(op(*left_f64, *right_f64))),
            None => Err(format!(
                "failed to convert left value to f64: {:?}",
                right_value
            )),
        },
        None => Err(format!(
            "failed to convert left value to f64: {:?}",
            left_value
        )),
    }
}

/**
 * Determine a boolean value from an expression. Currently:
 * - Boolean values are evaluated as-is.
 * - Nil evaluates as false.
 * - Everything else evaluates to true.
 */
pub fn is_truthful(v: Box<dyn Any>) -> bool {
    if (*v).is::<Nil>() {
        false
    } else if (*v).is::<bool>() {
        *(*v).downcast_ref::<bool>().unwrap()
    } else {
        true
    }
}

// TODO: This should be tested
pub fn are_equal(left_value: &Box<dyn Any>, right_value: &Box<dyn Any>) -> bool {
    if (*left_value).type_id() == (*right_value).type_id() {
        // Without restricting all values to dyn Any + Eq, you have to explicitly compare on each supported type.
        match (*left_value).downcast_ref::<f64>() {
            Some(l) => return *l == *(right_value).downcast_ref::<f64>().unwrap(),
            None => (),
        };
        match (*left_value).downcast_ref::<bool>() {
            Some(l) => return *l == *(right_value).downcast_ref::<bool>().unwrap(),
            None => (),
        };
        match (*left_value).downcast_ref::<String>() {
            Some(l) => return *l == *(right_value).downcast_ref::<String>().unwrap(),
            None => (),
        };
        match (*left_value).downcast_ref::<Nil>() {
            Some(l) => true,
            None => false,
        }
    } else {
        false
    }
}
