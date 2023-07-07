mod instructions;
mod tests;
mod values;

use crate::{errors::AssemblerError, Assembler};
use miette::SourceSpan;
use nom::{character::complete::space0, error::ParseError, sequence::delimited, IResult, Parser};
use nom_locate::LocatedSpan;

/// The span type used for parsing.
pub type Span<'a> = LocatedSpan<&'a str>;

/// A function to convert a `Span` into a `SourceSpan`.
pub fn into_source_span(span: Span) -> SourceSpan {
    SourceSpan::new(span.location_offset().into(), span.len().into())
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace (excluding carriage returns and line feeds), returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Parser<Span<'a>, O, E>,
{
    delimited(space0, inner, space0)
}

/// Private parsing API for `Assembler`
impl Assembler {
    fn parse(&mut self, _source: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();
    }
}
