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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => Token::ASSIGN,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '>' => Token::GT,
            '<' => Token::LT,
            '!' => Token::BANG,
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

    fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.ch {
            self.read_char();
        }
    }

    fn peak_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
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
        return String::from(&self.input[start..=self.position]);
    }

    fn read_int(&mut self) -> i32 {
        let start = self.position;
        while let '0'..='9' = self.peak_char() {
            self.read_char();
        }
        return String::from(&self.input[start..=self.position]).parse().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn char_next_token() {
        let input = "=+-><!*/(){},;";
        let mut lexer = Lexer::new(input.into());

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

        for expected in tokens {
            let token = lexer.next_token();
            println!("expected: {:?} recieved: {:?}", expected, token);
            assert_eq!(expected, token);
        }
    }

    #[test]
    fn simple_next_token() {
        let input = "
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
        ";
        let mut lexer = Lexer::new(input.into());

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

        for expected in tokens {
            let token = lexer.next_token();
            println!("expected: {:?} recieved: {:?}", expected, token);
            assert_eq!(expected, token);
        }
    }
}
