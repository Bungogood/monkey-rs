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

    // delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '=' => Token::ASSIGN,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod test {
    use super::{ Lexer, Token };

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];

        for expected in tokens {
            let token = lexer.next_token();
            assert_eq!(expected, token);
        }
    }
}
