mod lexer;

use lexer::Lexer;

fn main() {
    let source = String::from("");
    let mut lexer = Lexer::new(source);
    println!("Hello, world!");
}
