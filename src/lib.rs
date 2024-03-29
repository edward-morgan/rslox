use std::process::exit;
use std::{
    any,
    fs::File,
    io::{prelude::*, stdin, stdout},
    path::Path,
};
use scanner::*;
use syntax_tree::*;
use parser::*;

use crate::interpreter::interpret;

mod scanner;
mod syntax_tree;
mod parser;
mod interpreter;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub fn print_ast() {
    let expr = Expr::Binary(
        Box::new(Expr::Unary(
            Box::new(Expr::IntLiteral(123 as i64)),
            Token::new(TokenType::Minus, "-".to_string(), Box::new(""), 1),
        )),
        Token::new(TokenType::Star, "*".to_string(), Box::new(""), 1),
        Box::new(Expr::Grouping(
            Box::new(Expr::FloatLiteral(45.67)),
        ))
    );
    println!("{}", visit(expr));
}

/**
 * Runs a REPL
 */
pub fn run_prompt() {
    let mut rslox = RsLox::new();
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            println!("");
            break;
        }
        rslox.run(line);
        rslox.had_error = false;
    }
}

/**
 * Reads from a file with rslox statements in it
 */
pub fn run_file(file_path: &String) -> () {
    let mut rslox = RsLox::new();
    let path = Path::new(file_path);
    let mut file = File::open(path).unwrap();
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();

    rslox.run(program);
    if rslox.had_error {
        exit(65);
    }
}

pub fn err(line: u32, msg: &str) -> Result<(), String> {
    Err(format!("{}: `{}`", line, msg))
}

/**
 * Core program runner
 */
pub struct RsLox {
    had_error: bool,
}
impl RsLox {
    pub fn new() -> Self {
        RsLox { had_error: false }
    }

    /**
     * Evaluate a string of tokens and execute them.
     */
    fn run(&mut self, program: String) {
        let mut scanner = scanner::Scanner::new(program);
        let res: Result<(), String> = scanner.scan_tokens();
        if res.is_err() {
            self.error(scanner.line, res.err().unwrap());
            exit(1);
        }
        // Print each token
        // for token in &scanner.tokens {
        //     println!("{:?}", token);
        // }
        let mut p: Parser = Parser::new(scanner.tokens);
        match expression(&mut p) {
            Ok(expr) => {
                match interpret(expr) {
                    // TODO: why are result values string?
                    Ok(v) => println!("{}", visit_any(v)), //println!(": {:?}", visit_any(v)),
                    Err(msg) => println!("{}", msg)
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    /*
     * Error handling
     */

    pub fn error(&mut self, line: u32, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: u32, where_at: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, where_at, message);
        self.had_error = true;
    }
}