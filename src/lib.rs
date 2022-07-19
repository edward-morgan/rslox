use std::process::exit;
use std::{
    any,
    fs::File,
    io::{prelude::*, stdin, stdout},
    path::Path,
};

mod scanner;

pub enum Either<L, R> {
    Left(L),
    Right(R),
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
        for token in scanner.tokens {
            println!("{:?}", token);
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