use crate::assembler::{parser::errors::ParsingError, position::Position};
use miette::Diagnostic;

/// The public error type used to report errors.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Diagnostic)]
pub enum AssemblerError {
    /// A parsing error occurred.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parsing {
        /// The source parsing error.
        #[from]
        source: ParsingError,
    },

    /// There are a wrong number of arguments for an opcode.
    #[error(
        "'{}' expects {} arguments, but {} were given.",
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
        #[label("This mnemonic")]
        mnemonic_span: Position,

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
