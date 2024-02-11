use std::process::exit;
use lox::Lox;

mod lox;
mod scanner;
mod token_type;
mod token;
mod parser;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Hello, world!");

    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        lox.run_file(args[1].clone());
    } else {
        lox.run_prompt();
    }
}
