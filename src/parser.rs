use crate::lexer::{Lexer, Token};

pub enum ASTNode {
    Print(String),
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        loop {
            match self.current_token {
                Token::Eof => break,
                _ => {
                    if let Some(node) = self.parse_statement() {
                        nodes.push(node);
                    }
                },
            }
            self.next_token();
        }
        nodes
    }

    fn parse_statement(&mut self) -> Option<ASTNode> {
        match &self.current_token {
            Token::Print => {
                self.next_token();
                self.expect_token(&Token::Whitespace);
                self.next_token();
                if let Token::StringLiteral(string) = &self.current_token {
                    let value = string.clone();
                    self.next_token();
                    self.go_to_next_token(Token::ExclamationMark);
                    self.go_to_next_different_token(Token::ExclamationMark);
                    Some(ASTNode::Print(value))
                } else {
                    panic!("Expected string literal, found {:?}", self.current_token);
                }
            },
            Token::Whitespace => {
                None
            },
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn expect_token(&mut self, expected: &Token) {
        if &self.current_token != expected {
            panic!("Expected {:?}, found {:?}", expected, self.current_token);
        }
    }

    fn expect_any_token(&mut self, expected: &[Token]) {
        if !expected.contains(&self.current_token) {
            panic!("Expected any of {:?}, found {:?}", expected, self.current_token);
        }
    }

    fn go_to_next_token(&mut self, expected: Token) {
        loop {
            if self.current_token == expected {
                return;
            }

            if self.current_token == Token::Eof {
                panic!("Expected {:?}, found EOF", expected);
            }

            self.next_token();
        }
    }

    fn go_to_next_different_token(&mut self, expected: Token) {
        loop {
            if self.current_token != expected {
                return;
            }

            if self.current_token == Token::Eof {
                panic!("Expected a token different from {:?}, found EOF", expected);
            }

            self.next_token();
        }
    }
}