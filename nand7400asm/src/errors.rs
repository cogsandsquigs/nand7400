use miette::Diagnostic;
use snafu::Snafu;

/// The public error type used to report errors.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub enum AssemblerError {
    /// A testing error.
    #[snafu(display("Test error: {}", message))]
    #[diagnostic(code(nand7400asm::errors::test_error), help("This is a test error."))]
    Test {
        /// The error message.
        message: String,
    },

    /// A label does not exist for an argument.
    #[snafu(display("Label '{}' does not exist.", label))]
    #[diagnostic(
        code(nand7400asm::errors::label_does_not_exist),
        help("Try defining this label somewhere else in your code.")
    )]
    LabelDoesNotExist {
        /// The label that does not exist.
        label: String,
    },
}
