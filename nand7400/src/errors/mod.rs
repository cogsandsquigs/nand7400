pub mod position;

use self::position::Position;
use miette::Diagnostic;

/// The public error type used to report errors.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Diagnostic)]
pub enum AssemblerError {
    /// There was an unexpected token in the source code.
    /// TODO: Make this more idiomatic in the english language.
    #[error("Expected {}, but found {} instead.", positives.join(", "), negatives.join(", "))]
    #[diagnostic(code(nand7400::errors::unexpected))]
    Unexpected {
        /// The things that weren't expected.
        negatives: Vec<String>,

        /// The things that should've been there instead.
        positives: Vec<String>,

        /// The span of the token in the source code.
        #[label("Here")]
        span: Position,
    },

    /// There is an overflow parsing a literal.
    #[error("Literal value '{}' is too large.", literal)]
    #[diagnostic(
        code(nand7400::errors::overflow),
        help("The maximum possible value is 255, so try using a smaller value.")
    )]
    Overflow {
        /// The literal value that overflowed.
        literal: String,

        /// The span of the literal in the source code.
        #[label("Here")]
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
        help("Try using a different opcode or changing the arguments.")
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
        help("Try using a different opcode.")
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
        help("Try defining this label somewhere else in your code.")
    )]
    LabelDNE {
        /// The label that does not exist.
        mnemonic: String,

        /// The span of the label in the source code.
        #[label("Here")]
        span: Position,
    },
}

/// Public API for AssemblerError.
impl AssemblerError {
    /// Converts this into a miette report (so you can add source code).
    pub fn into_report(self) -> miette::Report {
        self.into()
    }

    /// Directly adds source code to this error. Note that this converts the error into a `miette::Report`,
    /// so you can't use it with `?` in library code. This should be done in application code anyways.
    pub fn with_source_code(self, source: String) -> miette::Report {
        self.into_report().with_source_code(source)
    }
}
