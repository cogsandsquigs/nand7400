use crate::errors::parsing::ParsingError;

use super::Span;
use miette::SourceSpan;
use nom::{
    character::complete::{anychar, newline, space0},
    error::ParseError,
    multi::many_till,
    sequence::delimited,
    IResult, Parser,
};

/// A function to convert a `Span` into a `SourceSpan`.
pub fn into_source_span(span: &Span) -> SourceSpan {
    SourceSpan::new(span.location_offset().into(), span.len().into())
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace (excluding carriage returns and line feeds), returning the output of `inner`.
pub fn ws<'a, F, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Parser<Span<'a>, O, E>,
{
    delimited(space0, inner, space0)
}

/// Evaluate `parser` and wrap the result in a `Some(_)`. Otherwise, emit the  provided `error_msg` and return
/// a `None` while allowing parsing to continue.
fn expect<'a, F, E, T>(parser: F, error_msg: E) -> impl Fn(Span<'a>) -> IResult<Span, Option<T>>
where
    F: Fn(Span<'a>) -> IResult<Span, T>,
    E: Fn() -> ParsingError,
{
    move |input| match parser(input.clone()) {
        // No errors: keep parsing as usual.
        Ok((remaining, out)) => Ok((remaining, Some(out))),

        // Parsing failed, but keep going.
        Err(nom::Err::Error(err)) | Err(nom::Err::Failure(err)) => {
            // Push error onto stack.
            err.input.extra.report_error(error_msg());

            // Parsing failed, but keep going.
            Ok((input, None))
        }

        // Parsing failed, and stop. This is on `incomplete` because we don't want to consume
        // input if we're going to stop.
        Err(err) => Err(err),
    }
}
/// Consume arbitrary data up to and including the next newline. This is a "reset" combinator that
/// allows us to recover from errors in the middle of a line.
pub fn consume_line<'a>(input: Span<'a>) -> IResult<Span<'a>, ()> {
    let (input, _) = delimited(space0, many_till(anychar, newline), space0)(input)?;
    Ok((input, ()))
}
