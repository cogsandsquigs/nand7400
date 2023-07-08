use crate::errors::AssemblerError;
use miette::Diagnostic;
use snafu::Snafu;

/// The assembler error type.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub enum AssemblerErrorFfi {
    /// Just a wrapper around an error.
    Error {
        #[diagnostic_source]
        source: AssemblerError,
    },
}
