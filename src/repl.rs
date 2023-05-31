use std::io::{self, Write};
use crate::lexer::{Lexer};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            break;
        }

        let lexer = Lexer::new(input.trim().into());

        for token in lexer {
            print!("{:?} ", token)
        }
        println!();

        // let result = eval(trimmed_input);
        println!("{}", trimmed_input);
    }
}
