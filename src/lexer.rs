#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // identifiers + literals
    Ident(String),
    Int(i32),

    // operators
    Assign,
    Plus,
    Minus,
    Gt,
    Lt,
    Bang,
    Asterisk,
    Slash,
    Eq,
    Neq,
    Geq,
    Leq,

    // delimiters
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Semicolon,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => self.double_token('=', Token::Assign, Token::Eq),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '>' => self.double_token('=', Token::Gt, Token::Geq),
            '<' => self.double_token('=', Token::Lt, Token::Leq),
            '!' => self.double_token('=', Token::Bang, Token::Neq),
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '0'..='9' => Token::Int(self.read_int()),
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                }
            }
            '\0' => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();
        tok
    }

    fn double_token(&mut self, second: char, single: Token, double: Token) -> Token {
        if second == self.peak_char() {
            self.read_char();
            double
        } else {
            single
        }
    }

    fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.ch {
            self.read_char();
        }
    }

    fn peak_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_ident(&mut self) -> String {
        let start = self.position;
        while let 'a'..='z' | 'A'..='Z' | '_' = self.peak_char() {
            self.read_char();
        }
        String::from(&self.input[start..=self.position])
    }

    fn read_int(&mut self) -> i32 {
        let start = self.position;
        while let '0'..='9' = self.peak_char() {
            self.read_char();
        }
        String::from(&self.input[start..=self.position])
            .parse()
            .unwrap()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::Eof => None,
            token => Some(token),
        }
    }
}

#[cfg(test)]
mod test {
    use std::iter::zip;

    use super::{Lexer, Token};

    #[test]
    fn single_next_token() {
        let input = String::from("=+-><!*/(){},;");
        let lexer = Lexer::new(input);

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Minus,
            Token::Gt,
            Token::Lt,
            Token::Bang,
            Token::Asterisk,
            Token::Slash,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for (expected, actual) in zip(tokens, lexer) {
            println!("expected: {:?} recieved: {:?}", expected, actual);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn double_next_token() {
        let input = String::from("== != >= <=");
        let lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Eq,
            Token::Neq,
            Token::Geq,
            Token::Leq,
        ];

        for (expected, actual) in zip(tokens, lexer) {
            println!("expected: {:?} recieved: {:?}", expected, actual);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn simple_next_token() {
        let input = String::from("
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let max = fn(x, y) {
                if (x > y) {
                    return x;
                } else {
                    return y;
                }
            };
            let result = add(five, ten);
        ");
        let lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("max")),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::If,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Gt,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::Ident(String::from("x")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::Rbrace,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            Token::Eof,
        ];

        for (expected, actual) in zip(tokens, lexer) {
            println!("expected: {:?} recieved: {:?}", expected, actual);
            assert_eq!(expected, actual);
        }
    }
}
