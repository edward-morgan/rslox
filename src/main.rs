use std::env;
use std::process::exit;
use rslox::*;

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

