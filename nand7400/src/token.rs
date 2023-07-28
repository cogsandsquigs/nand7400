use super::position::Position;
use std::fmt::Display;

/// Represents a token of source code. Tokens are produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    // The type of token that this token is.
    pub kind: TokenType,

    // The position of the token in the input.
    pub position: Position,

    // The literal value of the token.
    pub literal: String,
}

impl Token {
    /// Creates a new token.
    pub fn new<S>(kind: TokenType, position: Position, literal: S) -> Self
    where
        S: ToString,
    {
        Self {
            literal: literal.to_string(),
            position,
            kind,
        }
    }

    // /// Creates a new token from an identifier or keyword (e.g. `.byte`, `.org`, etc.).
    // pub fn from_ident_or_keyword(ident: String, start_index: usize) -> Self {
    //     Self {
    //         literal: ident.to_string(),
    //         position: Position::new(start_index, start_index + ident.len()),
    //         kind: match ident.as_str() {
    //             ".byte" => TokenType::Byte,
    //             ".org" => TokenType::Org,
    //             _ => TokenType::Ident,
    //         },
    //     }
    // }

    /// Creates a new token from an identifier.
    pub fn from_ident(ident: String, start_index: usize) -> Self {
        Self {
            literal: ident.to_string(),
            position: Position::new(start_index, start_index + ident.len()),
            kind: TokenType::Ident,
        }
    }

    /// Creates a new token from a keyword (e.g. `.byte`, `.org`, etc.).
    pub fn from_keyword(keyword: String, start_index: usize) -> Self {
        Self {
            literal: keyword.to_string(),
            position: Position::new(start_index, start_index + keyword.len()),
            kind: match keyword.as_str() {
                ".byte" => TokenType::Byte,
                ".org" => TokenType::Org,
                _ => TokenType::Illegal,
            },
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenType::Illegal,
            position: Position::new(0, 0),
            literal: String::new(),
        }
    }
}

/// Represents the kind of a token.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Misc. tokens
    /// The end of the input.
    Eof,

    /// Illegal/invalid token.
    Illegal,

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

    // Keywords
    /// The `.byte` keyword.
    Byte,

    /// The `.org` keyword.
    Org,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Illegal => "ILLEGAL",
                TokenType::Eof => "the end of the file",
                TokenType::Newline => "a newline",
                TokenType::Colon => "a ':'",
                TokenType::Hash => "a '#'",
                TokenType::Plus => "a '+'",
                TokenType::Minus => "a '-'",
                TokenType::Ident => "an identifier",
                TokenType::DecNum => "a decimal number",
                TokenType::HexNum => "a hexadecimal number",
                TokenType::OctNum => "an octal number",
                TokenType::BinNum => "a binary number",
                TokenType::Byte => "'.byte'",
                TokenType::Org => "'.org'",
            }
        )
    }
}