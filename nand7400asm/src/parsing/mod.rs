mod instructions;
mod tests;
mod values;

use crate::{errors::AssemblerError, Assembler};
use miette::SourceSpan;
use nom_locate::LocatedSpan;

/// The span type used for parsing.
pub type Span<'a> = LocatedSpan<&'a str>;

/// A function to convert a `Span` into a `SourceSpan`.
pub fn into_source_span(span: Span) -> SourceSpan {
    SourceSpan::new(span.location_offset().into(), span.len().into())
}

/// Private parsing API for `Assembler`
impl Assembler {
    fn parse(&mut self, _source: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();
    }
}
