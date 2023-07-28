use crate::position::Position;
use miette::Diagnostic;

/// The error type for parsing errors.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Diagnostic)]
pub enum ParsingError {}
