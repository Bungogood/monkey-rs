#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifiers + literals
    IDENT(String),
    INT(i32),

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    GT,
    LT,
    BANG,
    ASTERISK,
    SLASH,
    EQ,
    NEQ,
    GEQ,
    LEQ,

    // delimiters
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    SEMICOLON,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
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
            '=' => self.double_token('=', Token::ASSIGN, Token::EQ),
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '>' => self.double_token('=', Token::GT, Token::GEQ),
            '<' => self.double_token('=', Token::LT, Token::LEQ),
            '!' => self.double_token('=', Token::BANG, Token::NEQ),
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            ',' => Token::COMMA,
            ';' => Token::SEMICOLON,
            '0'..='9' => Token::INT(self.read_int()),
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "fn" => Token::FUNCTION,
                    "let" => Token::LET,
                    "true" => Token::TRUE,
                    "false" => Token::FALSE,
                    "if" => Token::IF,
                    "else" => Token::ELSE,
                    "return" => Token::RETURN,
                    _ => Token::IDENT(ident),
                }
            }
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
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
            Token::EOF => None,
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
            Token::ASSIGN,
            Token::PLUS,
            Token::MINUS,
            Token::GT,
            Token::LT,
            Token::BANG,
            Token::ASTERISK,
            Token::SLASH,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
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
            Token::EQ,
            Token::NEQ,
            Token::GEQ,
            Token::LEQ,
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
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("max")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IF,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::GT,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::IDENT(String::from("x")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];

        for (expected, actual) in zip(tokens, lexer) {
            println!("expected: {:?} recieved: {:?}", expected, actual);
            assert_eq!(expected, actual);
        }
    }
}
