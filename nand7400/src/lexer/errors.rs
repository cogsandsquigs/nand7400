use crate::position::Position;
use miette::Diagnostic;

/// The error type for lexing errors.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Diagnostic)]
pub enum LexingError {
    /// Unknown character in source code.
    #[error("Unknown character '{}'", character)]
    #[diagnostic(code(nand7400::errors::lexing::unknown_character))]
    UnknownCharacter {
        /// The character that was unknown.
        character: char,

        /// The span of the character in the source code.
        #[label("Here")]
        span: Position,
    },

    /// Unknown keyword in source code.
    #[error("Unknown keyword '{}'", keyword)]
    #[diagnostic(code(nand7400::errors::lexing::unknown_keyword))]
    UnknownKeyword {
        /// The keyword that was unknown.
        keyword: String,

        /// The span of the keyword in the source code.
        #[label("Here")]
        span: Position,
    },
}
