pub mod parsing;

use miette::{Diagnostic, SourceSpan};
use snafu::Snafu;

/// The public error type used to report errors.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub enum AssemblerError {
    /// A label does not exist for an argument.
    #[snafu(display("Label '{}' does not exist.", label))]
    #[diagnostic(
        code(nand7400::errors::label_does_not_exist),
        help("Try defining this label somewhere else in your code.")
    )]
    LabelDNE {
        /// The label that does not exist.
        label: String,

        /// The span of the label in the source code.
        #[label("here")]
        span: SourceSpan,

        /// The source code that was being assembled.
        #[source_code]
        source_code: String,
    },
}
