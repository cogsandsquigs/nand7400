use miette::{Diagnostic, SourceSpan};
use snafu::Snafu;

/// The public error type used to report errors.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub struct AssemblerError {
    /// The root error(s) that occurred.
    #[related]
    kind: Vec<AssemblerErrorKind>,

    /// The source code that was assembled. This is mostly used for nice error reporting.
    #[source_code]
    source_code: String,
}

impl AssemblerError {
    /// Create an empty assembler error.
    pub fn empty() -> Self {
        Self {
            kind: Vec::new(),
            source_code: "".to_string(),
        }
    }

    /// Create a new assembler error with the given source code.
    pub fn new(kind: Vec<AssemblerErrorKind>) -> Self {
        Self {
            kind,
            source_code: "".to_string(),
        }
    }

    /// Report an error with the given kind.
    pub fn report(&mut self, kind: AssemblerErrorKind) {
        self.kind.push(kind);
    }

    /// Check if there are any errors accumulated.
    pub fn is_empty(&self) -> bool {
        self.kind.is_empty()
    }

    /// Add source code to the error.
    pub fn with_source_code(mut self, source_code: String) -> Self {
        self.source_code = source_code;
        self
    }
}

/// The type of error that can occur when assembling.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub enum AssemblerErrorKind {
    /// There was an unexpected token in the source code.
    /// TODO: Make this more idiomatic in the english language.
    #[snafu(display("Expected {}, but found {} instead.", positives.join(", "), negatives.join(", ")))]
    #[diagnostic(code(nand7400::errors::unexpected))]
    Unexpected {
        /// The things that weren't expected.
        negatives: Vec<String>,

        /// The things that should've been there instead.
        positives: Vec<String>,

        /// The span of the token in the source code.
        #[label("here")]
        span: SourceSpan,
    },

    /// An opcode does not exist.
    #[snafu(display("Opcode '{}' does not exist.", mnemonic))]
    #[diagnostic(
        code(nand7400::errors::opcode_dne),
        help("Try using a different opcode.")
    )]
    OpcodeDNE {
        /// The opcode that does not exist.
        mnemonic: String,

        /// The span of the opcode in the source code.
        #[label("here")]
        span: SourceSpan,
    },

    /// A label does not exist for an argument.
    #[snafu(display("Label '{}' does not exist.", mnemonic))]
    #[diagnostic(
        code(nand7400::errors::label_dne),
        help("Try defining this label somewhere else in your code.")
    )]
    LabelDNE {
        /// The label that does not exist.
        mnemonic: String,

        /// The span of the label in the source code.
        #[label("here")]
        span: SourceSpan,
    },
}

impl AssemblerErrorKind {
    /// Convert this into an `AssemblerError` without any source code.
    pub fn into_err(self) -> AssemblerError {
        AssemblerError::new(vec![self])
    }
}
