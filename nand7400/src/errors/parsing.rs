use miette::{Diagnostic, SourceSpan};
use snafu::Snafu;

use crate::parsing::Span;

/// The public error type used to report errors.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub enum ParsingError {
    /// An unexpected character was found.
    #[snafu(display("Unexpected character '{}'.", character))]
    #[diagnostic(
        code(nand7400::errors::parsing::unexpected_character),
        help("Try removing this character.")
    )]
    UnexpectedCharacter {
        /// The unexpected character.
        character: char,

        /// The span of the unexpected character in the source code.
        #[label("here")]
        span: SourceSpan,

        /// The source code that was being assembled.
        #[source_code]
        source_code: String,
    },

    /// An opcode does not exist for an instruction.
    #[snafu(display("Opcode '{}' does not exist.", opcode))]
    #[diagnostic(
        code(nand7400::errors::opcode_does_not_exist),
        help("Try using a different opcode.")
    )]
    OpcodeDNE {
        /// The opcode that does not exist.
        opcode: String,

        /// The span of the opcode in the source code.
        #[label("here")]
        span: SourceSpan,

        /// The source code that was being assembled.
        #[source_code]
        source_code: String,
    },

    /// There are too many arguments for an instruction.
    #[snafu(display("Too many arguments for instruction '{}'.", instruction))]
    #[diagnostic(
        code(nand7400::errors::parsing::too_many_args),
        help("Try removing some arguments.")
    )]
    TooManyArgs {
        /// The instruction that has too many arguments.
        instruction: String,

        /// The span of the extra arguments in the source code.
        #[label("here")]
        span: SourceSpan,

        /// The source code that was being assembled.
        #[source_code]
        source_code: String,
    },
}
