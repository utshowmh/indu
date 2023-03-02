use super::{object::Object, position::Position};

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier,
    Number,
    String,

    Plus,
    Minus,
    Star,
    Slash,

    Assign,
    Equal,
    Not,
    NotEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Semicolon,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub position: Position,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        lexeme: String,
        literal: Option<Object>,
        position: Position,
    ) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            position,
        }
    }
}
