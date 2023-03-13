use std::fmt::{Display, Formatter, Result};

use super::{object::Object, position::Position};

#[derive(Debug, Clone, PartialEq)]
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
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Identifier => write!(f, "identifier"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),

            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Assign => write!(f, "="),
            Self::Equal => write!(f, "=="),
            Self::Not => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Lesser => write!(f, "<"),
            Self::LesserEqual => write!(f, "<="),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Semicolon => write!(f, ";"),

            Self::And => write!(f, "and"),
            Self::Class => write!(f, "class"),
            Self::Else => write!(f, "else"),
            Self::False => write!(f, "false"),
            Self::Fun => write!(f, "fun"),
            Self::For => write!(f, "for"),
            Self::If => write!(f, "if"),
            Self::Nil => write!(f, "nil"),
            Self::Or => write!(f, "or"),
            Self::Return => write!(f, "return"),
            Self::Super => write!(f, "super"),
            Self::This => write!(f, "this"),
            Self::True => write!(f, "true"),
            Self::Var => write!(f, "var"),
            Self::While => write!(f, "while"),
            Self::EOF => write!(f, "\0"),
        }
    }
}

#[derive(Debug, Clone)]
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
