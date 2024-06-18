#[derive(Debug, PartialEq)]
pub enum Token {
    Print,
    Whitespace,
    StringLiteral(String),
    ExclamationMark,
    Eof,
}

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            let ch = self.input.chars().nth(self.position);
            self.position += 1;
            ch
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.position)
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.next_char() {
            Some('"') => {
                let mut string_lit = String::new();
                while let Some(ch) = self.next_char() {
                    if ch == '"' {
                        break;
                    }
                    string_lit.push(ch);
                }
                Token::StringLiteral(string_lit)
            },
            Some(' ') => Token::Whitespace,
            Some('!') => Token::ExclamationMark,
            Some('\n') => Token::Whitespace,
            Some(ch) if ch.is_alphabetic() => {
                let mut ident = String::new();
                ident.push(ch);
                while let Some(next_ch) = self.peek_char() {
                    if next_ch.is_alphabetic() {
                        ident.push(next_ch);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "print" => Token::Print,
                    _ => panic!("Unexpected identifier: {}", ident),
                }
            },
            None => Token::Eof,
            Some(ch) => panic!("Unexpected character: {}", ch),
        }
    }
}