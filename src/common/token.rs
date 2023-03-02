use super::{object::Object, position::Position};

#[derive(Debug, Clone)]
pub(crate) enum TokenKind {
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
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Object>,
    pub(crate) position: Position,
}

impl Token {
    pub(crate) fn new(
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
