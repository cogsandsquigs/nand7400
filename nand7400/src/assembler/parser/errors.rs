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
        /// The character that was unknown.
        character: char,

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

    /// There are a wrong number of arguments for an opcode.
    #[error(
        "Opcode '{}' expects {} arguments, but {} were given.",
        mnemonic,
        expected,
        given
    )]
    #[diagnostic(
        code(nand7400::errors::wrong_num_args),
        help("Check the number of arguments the opcode expects.")
    )]
    WrongNumArgs {
        /// The opcode that was given the wrong number of arguments.
        mnemonic: String,

        /// The number of arguments that the opcode expects.
        expected: u16,

        /// The number of arguments that were given.
        given: u16,

        /// The span of the opcode in the source code.
        #[label("This opcode")]
        opcode_span: Position,

        /// The span of the arguments in the source code.
        #[label("These arguments")]
        args_span: Position,
    },

    /// An opcode does not exist.
    #[error("Opcode '{}' does not exist.", mnemonic)]
    #[diagnostic(
        code(nand7400::errors::opcode_dne),
        help("Opcodes (besides HLT and RST) need to be pre-defined.")
    )]
    OpcodeDNE {
        /// The opcode that does not exist.
        mnemonic: String,

        /// The span of the opcode in the source code.
        #[label("This opcode")]
        span: Position,
    },

    /// A label does not exist for an argument.
    #[error("Label '{}' does not exist.", mnemonic)]
    #[diagnostic(
        code(nand7400::errors::label_dne),
        help("Labels need to be defined to be used.")
    )]
    LabelDNE {
        /// The label that does not exist.
        mnemonic: String,

        /// The span of the label in the source code.
        #[label("This label")]
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
