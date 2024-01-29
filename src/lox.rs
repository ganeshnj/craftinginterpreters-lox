use std::process::exit;
use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub(crate) fn new() -> Lox {
        Lox { had_error: false }
    }

    pub(crate) fn error(line: usize, message: String) {
        Lox::report(line, "".to_string(), message);
    }

    fn report(line: usize, where_: String, message: String) {
        println!("[line {}] Error {}: {}", line, where_, message);
    }

    pub(crate) fn run_file(&self, path: String) {
        let contents = std::fs::read_to_string(path)
            .expect("Something went wrong reading the file");
        self.run(contents);

        if self.had_error {
            exit(65);
        }
    }

    pub(crate) fn run_prompt(&mut self) {
        loop {
            println!("> ");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)
                .expect("Failed to read line");
            if line == "" {
                break;
            }
            self.run(line);
            self.had_error = false;
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
