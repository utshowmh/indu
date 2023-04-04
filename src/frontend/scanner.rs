use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorKind},
    position::Position,
    token::{Token, TokenKind},
};

pub(crate) struct Scanner {
    source: Vec<char>,

    start_index: usize,
    current_index: usize,
    current_line: usize,

    keywords: HashMap<String, TokenKind>,
}

impl Scanner {
    pub(crate) fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),

            start_index: 0,
            current_index: 0,
            current_line: 1,

            keywords: HashMap::new(),
        }
    }

    pub(crate) fn scan(&mut self) -> Result<Vec<Token>, Error> {
        self.init_keywords();

        let mut tokens = Vec::new();

        while self.index_in_bound() {
            self.start_index = self.current_index;
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(
            TokenKind::EOF,
            String::from("\0"),
            self.generate_position(),
        ));

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let current_char = self.next_character();
        match current_char {
            ' ' | '\t' | '\r' => Ok(None),

            '\n' => {
                self.current_line += 1;
                Ok(None)
            }

            '+' => Ok(Some(Token::new(
                TokenKind::Plus,
                self.generate_lexeme(),
                self.generate_position(),
            ))),
            '-' => Ok(Some(Token::new(
                TokenKind::Minus,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            '*' => Ok(Some(Token::new(
                TokenKind::Star,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            '/' => {
                if self.current_character() == '/' {
                    while self.current_character() != '\n' && self.index_in_bound() {
                        self.advance_current_index();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Slash,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                }
            }

            '(' => Ok(Some(Token::new(
                TokenKind::OpenParen,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            ')' => Ok(Some(Token::new(
                TokenKind::CloseParen,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            '{' => Ok(Some(Token::new(
                TokenKind::OpenBrace,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            '}' => Ok(Some(Token::new(
                TokenKind::CloseBrace,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            ',' => Ok(Some(Token::new(
                TokenKind::Comma,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            '.' => Ok(Some(Token::new(
                TokenKind::Dot,
                self.generate_lexeme(),
                self.generate_position(),
            ))),

            '=' => {
                if self.current_character() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::Equal,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Assign,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                }
            }

            '!' => {
                if self.current_character() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::NotEqual,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Not,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                }
            }

            '>' => {
                if self.current_character() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::GreaterEqual,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Greater,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                }
            }

            '<' => {
                if self.current_character() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::LesserEqual,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Lesser,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                }
            }

            '&' => {
                if self.current_character() == '&' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::And,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                } else {
                    Ok(None)
                }
            }

            '|' => {
                if self.current_character() == '|' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::Or,
                        self.generate_lexeme(),
                        self.generate_position(),
                    )))
                } else {
                    Ok(None)
                }
            }

            _ => {
                if current_char.is_alphabetic() || current_char == '_' {
                    self.make_identifier()
                } else if current_char.is_ascii_digit() {
                    self.make_number()
                } else if current_char == '"' {
                    self.make_string()
                } else {
                    Err(self.generate_error(format!(
                        "Unrecognized character. Character '{current_char}' is not valid."
                    )))
                }
            }
        }
    }

    fn make_identifier(&mut self) -> Result<Option<Token>, Error> {
        while self.current_character().is_ascii_alphanumeric() || self.current_character() == '_' {
            self.advance_current_index();
        }
        let lexeme = self.generate_lexeme();
        if let Some(token_kind) = self.keywords.get(&lexeme) {
            match token_kind {
                TokenKind::True => Ok(Some(Token::new(
                    token_kind.clone(),
                    lexeme,
                    self.generate_position(),
                ))),
                TokenKind::False => Ok(Some(Token::new(
                    token_kind.clone(),
                    lexeme,
                    self.generate_position(),
                ))),
                TokenKind::Nil => Ok(Some(Token::new(
                    token_kind.clone(),
                    lexeme,
                    self.generate_position(),
                ))),
                _ => Ok(Some(Token::new(
                    token_kind.clone(),
                    lexeme,
                    self.generate_position(),
                ))),
            }
        } else {
            Ok(Some(Token::new(
                TokenKind::Identifier,
                lexeme,
                self.generate_position(),
            )))
        }
    }

    fn make_number(&mut self) -> Result<Option<Token>, Error> {
        while self.current_character().is_ascii_digit() {
            self.advance_current_index();
        }
        if self.current_character() == '.' {
            self.advance_current_index();
            while self.current_character().is_ascii_digit() {
                self.advance_current_index();
            }
        }
        let lexeme = self.generate_lexeme();
        if lexeme.parse::<f64>().is_ok() {
            Ok(Some(Token::new(
                TokenKind::Number,
                lexeme,
                self.generate_position(),
            )))
        } else {
            Err(self.generate_error(format!(
                "Invalid number. Could not convert '{lexeme}' to 64 bit float."
            )))
        }
    }

    fn make_string(&mut self) -> Result<Option<Token>, Error> {
        while self.current_character() != '"' && self.index_in_bound() {
            self.advance_current_index();
        }
        if self.current_character() == '"' {
            self.advance_current_index();
            let lexeme: String = self.source[self.start_index + 1..self.current_index - 1]
                .iter()
                .collect();
            Ok(Some(Token::new(
                TokenKind::String,
                lexeme,
                self.generate_position(),
            )))
        } else {
            Err(self.generate_error(format!(
                "Unterminated string. Expected '\"' after '{}'.",
                self.current_character()
            )))
        }
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("class".to_string(), TokenKind::Class);
        self.keywords.insert("else".to_string(), TokenKind::Else);
        self.keywords.insert("false".to_string(), TokenKind::False);
        self.keywords.insert("fun".to_string(), TokenKind::Fun);
        self.keywords.insert("for".to_string(), TokenKind::For);
        self.keywords.insert("if".to_string(), TokenKind::If);
        self.keywords.insert("nil".to_string(), TokenKind::Nil);
        self.keywords.insert("print".to_string(), TokenKind::Print);
        self.keywords
            .insert("return".to_string(), TokenKind::Return);
        self.keywords.insert("super".to_string(), TokenKind::Super);
        self.keywords.insert("this".to_string(), TokenKind::This);
        self.keywords.insert("true".to_string(), TokenKind::True);
        self.keywords.insert("var".to_string(), TokenKind::Var);
        self.keywords.insert("while".to_string(), TokenKind::While);
    }

    fn index_in_bound(&self) -> bool {
        self.current_index < self.source.len()
    }

    fn current_character(&self) -> char {
        if self.index_in_bound() {
            self.source[self.current_index]
        } else {
            '\0'
        }
    }

    fn advance_current_index(&mut self) {
        self.current_index += 1;
    }

    fn next_character(&mut self) -> char {
        let char = self.current_character();
        self.advance_current_index();
        char
    }

    fn generate_lexeme(&self) -> String {
        self.source[self.start_index..self.current_index]
            .iter()
            .collect()
    }

    fn generate_position(&self) -> Position {
        Position::new(self.start_index, self.current_index, self.current_line)
    }

    fn generate_error(&self, message: String) -> Error {
        Error::new(ErrorKind::Lexer, message, Some(self.generate_position()))
    }
}
