use super::errors::LexingError;
use crate::position::Position;
use std::fmt::Display;

/// Represents a token of source code. Tokens are produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    // The type of token that this token is.
    pub kind: TokenKind,

    // The position of the token in the input.
    pub position: Position,

    // The literal value of the token.
    pub literal: String,
}

impl Token {
    /// Creates a new token.
    pub fn new<S>(kind: TokenKind, position: Position, literal: S) -> Self
    where
        S: ToString,
    {
        Self {
            literal: literal.to_string(),
            position,
            kind,
        }
    }

    /// Creates a new token from an identifier.
    pub fn from_ident(ident: String, start_index: usize) -> Self {
        Self {
            literal: ident.to_string(),
            position: Position::new(start_index, start_index + ident.len()),
            kind: TokenKind::Ident,
        }
    }

    /// Creates a new token from a keyword (e.g. `.byte`, `.org`, etc.).
    pub fn from_keyword(keyword: String, start_index: usize) -> Result<Self, LexingError> {
        Ok(Self {
            literal: keyword.to_string(),
            position: Position::new(start_index, start_index + keyword.len()),
            kind: TokenKind::Keyword,
        })
    }
}

/// Represents the kind of a token.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Misc. tokens
    /// The end of the input.
    Eof,

    /// A newline.
    Newline,

    /// An identifier.
    Ident,

    /// A colon.
    Colon,

    /// A plus sign.
    Plus,

    /// A minus sign.
    Minus,

    /// A pound sign.
    Hash,

    // Values
    /// A decimal number.
    DecNum,

    /// A hexadecimal number.
    HexNum,

    /// An octal number.
    OctNum,

    /// A binary number.
    BinNum,

    /// A keyword (e.g. `.byte`, `.org`, etc.).
    Keyword,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenKind::Eof => "the end of the file",
                TokenKind::Newline => "a newline",
                TokenKind::Colon => "a ':'",
                TokenKind::Hash => "a '#'",
                TokenKind::Plus => "a '+'",
                TokenKind::Minus => "a '-'",
                TokenKind::Ident => "an identifier",
                TokenKind::DecNum => "a decimal number",
                TokenKind::HexNum => "a hex number",
                TokenKind::OctNum => "an octal number",
                TokenKind::BinNum => "a binary number",
                TokenKind::Keyword => "a keyword",
            }
        )
    }
}
