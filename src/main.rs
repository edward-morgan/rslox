use std::{
    env,
    fs::File,
    path::Path,
    io::{prelude::*, stdin, stdout}
};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 { // First arg is the executable name
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        run(&line);
    }
}

fn run_file(file_path: &String) -> () {
    let path = Path::new(file_path);
    let mut file = File::open(path).unwrap();
    let mut program = String::new();
    file.read_to_string(&mut program).unwrap();

    run(&program);
}

fn run(program: &String) {
    ()
}