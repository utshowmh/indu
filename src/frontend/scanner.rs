use std::collections::HashMap;

use crate::common::{
    error::{Error, ErrorKind},
    object::Object,
    position::Position,
    token::{Token, TokenKind},
};

pub(crate) struct Scanner {
    source: Vec<char>,

    start_index: usize,
    current_index: usize,
    current_position: Position,

    keywords: HashMap<String, TokenKind>,
}

impl Scanner {
    pub(crate) fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),

            start_index: 0,
            current_index: 0,
            current_position: Position::new(1, 1),

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
            None,
            self.current_position.clone(),
        ));

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let current_char = self.next_charecter();
        match current_char {
            ' ' | '\t' | '\r' => Ok(None),
            '\n' => {
                self.current_position.column = 1;
                self.current_position.line += 1;
                Ok(None)
            }

            '+' => Ok(Some(Token::new(
                TokenKind::Plus,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            '-' => Ok(Some(Token::new(
                TokenKind::Minus,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            '*' => Ok(Some(Token::new(
                TokenKind::Star,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            '/' => {
                if self.current_charecter() == '/' {
                    while self.current_charecter() != '\n' && self.index_in_bound() {
                        self.advance_current_index();
                    }
                    Ok(None)
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Slash,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                }
            }

            '(' => Ok(Some(Token::new(
                TokenKind::OpenParen,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            ')' => Ok(Some(Token::new(
                TokenKind::CloseParen,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            '{' => Ok(Some(Token::new(
                TokenKind::OpenBrace,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            '}' => Ok(Some(Token::new(
                TokenKind::CloseBrace,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            ',' => Ok(Some(Token::new(
                TokenKind::Comma,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            '.' => Ok(Some(Token::new(
                TokenKind::Dot,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),
            ';' => Ok(Some(Token::new(
                TokenKind::Semicolon,
                self.generate_lexeme(),
                None,
                self.current_position.clone(),
            ))),

            '=' => {
                if self.current_charecter() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::Equal,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Assign,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                }
            }

            '!' => {
                if self.current_charecter() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::NotEqual,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Not,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                }
            }

            '>' => {
                if self.current_charecter() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::GreaterEqual,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Greater,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                }
            }

            '<' => {
                if self.current_charecter() == '=' {
                    self.advance_current_index();
                    Ok(Some(Token::new(
                        TokenKind::LesserEqual,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                } else {
                    Ok(Some(Token::new(
                        TokenKind::Lesser,
                        self.generate_lexeme(),
                        None,
                        self.current_position.clone(),
                    )))
                }
            }

            _ => {
                if current_char.is_alphabetic() || current_char == '_' {
                    while self.current_charecter().is_ascii_alphanumeric()
                        || self.current_charecter() == '_'
                    {
                        self.advance_current_index();
                    }
                    let lexeme = self.generate_lexeme();
                    if let Some(token_kind) = self.keywords.get(&lexeme) {
                        match token_kind {
                            TokenKind::True => Ok(Some(Token::new(
                                token_kind.clone(),
                                lexeme,
                                Some(Object::Boolean(true)),
                                self.current_position.clone(),
                            ))),
                            TokenKind::False => Ok(Some(Token::new(
                                token_kind.clone(),
                                lexeme,
                                Some(Object::Boolean(false)),
                                self.current_position.clone(),
                            ))),
                            TokenKind::Nil => Ok(Some(Token::new(
                                token_kind.clone(),
                                lexeme,
                                Some(Object::Nil),
                                self.current_position.clone(),
                            ))),
                            _ => Ok(Some(Token::new(
                                token_kind.clone(),
                                lexeme,
                                None,
                                self.current_position.clone(),
                            ))),
                        }
                    } else {
                        Ok(Some(Token::new(
                            TokenKind::Identifier,
                            lexeme,
                            None,
                            self.current_position.clone(),
                        )))
                    }
                } else if current_char.is_ascii_digit() {
                    while self.current_charecter().is_ascii_digit() {
                        self.advance_current_index();
                    }
                    if self.current_charecter() == '.' {
                        self.advance_current_index();
                        while self.current_charecter().is_ascii_digit() {
                            self.advance_current_index();
                        }
                    }
                    let lexeme = self.generate_lexeme();
                    if let Ok(num) = lexeme.parse() {
                        Ok(Some(Token::new(
                            TokenKind::Number,
                            lexeme,
                            Some(Object::Number(num)),
                            self.current_position.clone(),
                        )))
                    } else {
                        Err(self
                            .generate_error(format!("Could not convert `{}` to `Number`", lexeme)))
                    }
                } else if current_char == '"' {
                    while self.current_charecter() != '"' && self.index_in_bound() {
                        self.advance_current_index();
                    }
                    if self.current_charecter() == '"' {
                        self.advance_current_index();
                        let lexeme: String = self.source
                            [self.start_index + 1..self.current_index - 1]
                            .iter()
                            .collect();
                        Ok(Some(Token::new(
                            TokenKind::String,
                            lexeme.clone(),
                            Some(Object::String(lexeme)),
                            self.current_position.clone(),
                        )))
                    } else {
                        Err(self.generate_error(format!(
                            "Unterminated string, expected `\"` after `{}`",
                            self.current_charecter()
                        )))
                    }
                } else {
                    Err(self.generate_error(format!("Unrecognized charecter `{}`", current_char)))
                }
            }
        }
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("and".to_string(), TokenKind::And);
        self.keywords.insert("class".to_string(), TokenKind::Class);
        self.keywords.insert("else".to_string(), TokenKind::Else);
        self.keywords.insert("false".to_string(), TokenKind::False);
        self.keywords.insert("fun".to_string(), TokenKind::Fun);
        self.keywords.insert("for".to_string(), TokenKind::For);
        self.keywords.insert("if".to_string(), TokenKind::If);
        self.keywords.insert("nil".to_string(), TokenKind::Nil);
        self.keywords.insert("or".to_string(), TokenKind::Or);
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

    fn current_charecter(&self) -> char {
        if self.index_in_bound() {
            self.source[self.current_index]
        } else {
            '\0'
        }
    }

    fn advance_current_index(&mut self) {
        self.current_index += 1;
        self.current_position.column += 1;
    }

    fn next_charecter(&mut self) -> char {
        let char = self.current_charecter();
        self.advance_current_index();
        char
    }

    fn generate_lexeme(&self) -> String {
        self.source[self.start_index..self.current_index]
            .iter()
            .collect()
    }

    fn generate_error(&self, message: String) -> Error {
        Error::new(
            ErrorKind::LexerError,
            message,
            self.current_position.clone(),
        )
    }
}
