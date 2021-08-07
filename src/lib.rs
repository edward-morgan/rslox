use std::{
    fs::File,
    path::Path,
    io::{prelude::*, stdin, stdout}
};

/**
 * Runs a REPL
 */
pub fn run_prompt() {
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            println!("");
            break;
        }
        run(line);
    }
}

/**
 * Reads from a file with rslox statements in it
 */
pub fn run_file(file_path: &String) -> () {
    let path = Path::new(file_path);
    let mut file = File::open(path).unwrap();
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();

    run(program);
}

fn run(program: String) {
    let scanner = Scanner::new(program);
    let tokens: Vec<Token> = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}

struct Scanner {
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

#[derive(Debug)]
struct Token {
}