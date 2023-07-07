use miette::{Diagnostic, SourceSpan};
use snafu::Snafu;

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
        #[label("Here")]
        span: SourceSpan,
    },

    /// There is an overflow when parsing a value.
    #[snafu(display("Overflow when parsing value '{}'.", value))]
    #[diagnostic(
        code(nand7400::errors::parsing::overflow),
        help("Try using a smaller value, under 128.")
    )]
    Overflow {
        /// The value that overflowed.
        value: String,

        /// The span of the value in the source code.
        #[label("Here")]
        span: SourceSpan,
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
        #[label("Here")]
        span: SourceSpan,
    },

    /// There are too many arguments for an instruction.
    #[snafu(display(
        "Too many arguments for instruction '{}': expected {} arguments, but found {}",
        opcode,
        expected,
        got
    ))]
    #[diagnostic(
        code(nand7400::errors::parsing::too_many_args),
        help("Try removing some arguments.")
    )]
    TooManyArgs {
        /// The instruction that has too many arguments.
        opcode: String,

        /// Expected number of arguments.
        expected: usize,

        /// Actual number of arguments.
        got: usize,

        /// The span of the extra arguments in the source code.
        #[label("These arguments")]
        span: SourceSpan,
    },

    /// There are too few arguments for an instruction.
    #[snafu(display(
        "Too few arguments for instruction '{}': expected {} arguments, but found {}",
        opcode,
        expected,
        got
    ))]
    #[diagnostic(
        code(nand7400::errors::parsing::too_few_args),
        help("Try adding some arguments.")
    )]
    TooFewArgs {
        /// The instruction that has too few arguments.
        opcode: String,

        /// Expected number of arguments.
        expected: usize,

        /// Actual number of arguments.
        got: usize,

        /// The span of the extra arguments in the source code.
        #[label("These arguments")]
        span: SourceSpan,
    },
}
