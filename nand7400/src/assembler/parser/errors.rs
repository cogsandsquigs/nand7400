use super::lexer::token::TokenKind;
use crate::assembler::position::Position;
use miette::Diagnostic;

/// The error type for parsing errors.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Diagnostic)]
pub enum ParsingError {
    /// Unknown character in source code.
    #[error("Unknown character '{}'", character)]
    #[diagnostic(code(nand7400::errors::lexing::unknown_character))]
    UnknownCharacter {
        /// The character that was unknown. Currently a string due to FFI limitations.
        character: String,

        /// The span of the character in the source code.
        #[label("Here")]
        span: Position,
    },

    /// There was an unexpected token in the source code.
    #[error(
        "Expected {}, but found {} instead.",
        join_expects_together(expected),
        found
    )]
    #[diagnostic(code(nand7400::errors::unexpected))]
    Unexpected {
        /// The things that weren't expected.
        expected: Vec<TokenKind>,

        /// The things that should've been there instead.
        found: TokenKind,

        /// The span of the token in the source code.
        #[label("Here")]
        span: Position,
    },

    /// There is an overflow parsing a literal.
    #[error("Literal value '{}' is too large.", literal)]
    #[diagnostic(code(nand7400::errors::overflow))]
    Overflow {
        /// The literal value that overflowed.
        literal: String,

        /// The span of the literal in the source code.
        #[label("This number")]
        span: Position,
    },

    /// There is an underflow parsing a literal.
    #[error("Literal value '{}' is too small.", literal)]
    #[diagnostic(code(nand7400::errors::underflow))]
    Underflow {
        /// The literal value that overflowed.
        literal: String,

        /// The span of the literal in the source code.
        #[label("This number")]
        span: Position,
    },

    /// The number is empty.
    #[error("Literal value is empty.")]
    #[diagnostic(
        code(nand7400::errors::empty_literal),
        help("Numbers should have a value after their base signifier (i.e. '0x', '0o', '0b')")
    )]
    EmptyLiteral {
        /// The span of the literal in the source code.
        #[label("This number")]
        span: Position,
    },

    /// A keyword does not exist.
    #[error("Keyword '{}' does not exist.", mnemonic)]
    #[diagnostic(
        code(nand7400::errors::keyword_dne),
        help("The only defined keywords are '.byte' and '.org'. All others are invalid.")
    )]
    KeywordDNE {
        /// The keyword that does not exist.
        mnemonic: String,

        /// The span of the keyword in the source code.
        #[label("This keyword")]
        span: Position,
    },
}

// Helper function to join a list of strings with commas, replace the last comma with "or", and return the result.
// If nothing is in the list, return "nothing". This is used in the Unexpected error so that the printed-out error
// gives human-readable sentances instead of weird garbage.
fn join_expects_together(list: &[TokenKind]) -> String {
    if list.is_empty() {
        "nothing".to_string()
    } else if list.len() == 1 {
        list[0].to_string()
    } else {
        let mut result = String::new();

        for (i, item) in list.iter().enumerate() {
            if i == list.len() - 1 {
                result += "or ";
            }

            result += &item.to_string();

            if i != list.len() - 1 {
                result += ", ";
            }
        }

        result
    }
}
