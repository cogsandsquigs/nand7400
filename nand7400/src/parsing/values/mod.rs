mod tests;

use std::num::{IntErrorKind, ParseIntError};

use super::Span;
use crate::{errors::parsing::ParsingError, parsing::utils::into_source_span};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    character::complete::{char, one_of},
    combinator::{opt, recognize},
    multi::{many0_count, many1},
    sequence::{pair, preceded},
    IResult,
};

/// Parse a single decimal value.
/// TODO: allow for negatives.
pub fn decimal(input: Span) -> IResult<Span, i8> {
    let (input, (neg_sign, value_str)) =
        pair(opt(char('-')), recognize(many1(one_of("0123456789"))))(input)?;

    let value = match convert_to_i8(&value_str, neg_sign.is_some(), 10) {
        Ok(v) => v,
        Err(e) => {
            input.extra.report_error(e);
            0
        }
    };

    Ok((input, value))
}

/// Parse a single hexidecimal value.
/// TODO: allow for negatives.
pub fn hexadecimal(input: Span) -> IResult<Span, i8> {
    let (input, (neg_sign, value_str)) = pair(
        opt(char('-')),
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(one_of("0123456789abcdefABCDEF"))),
        ),
    )(input)?;

    let value = match convert_to_i8(&value_str, neg_sign.is_some(), 16) {
        Ok(v) => v,
        Err(e) => {
            input.extra.report_error(e);
            0
        }
    };

    Ok((input, value))
}

/// Parse a single octal value.
/// TODO: allow for negatives.
pub fn octal(input: Span) -> IResult<Span, i8> {
    let (input, (neg_sign, value_str)) = pair(
        opt(char('-')),
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(one_of("01234567"))),
        ),
    )(input)?;

    let value = match convert_to_i8(&value_str, neg_sign.is_some(), 8) {
        Ok(v) => v,
        Err(e) => {
            input.extra.report_error(e);
            0
        }
    };

    Ok((input, value))
}

/// Parse a single binary value.
/// TODO: allow for negatives.
pub fn binary(input: Span) -> IResult<Span, i8> {
    let (input, (neg_sign, value_str)) = pair(
        opt(char('-')),
        preceded(alt((tag("0b"), tag("0B"))), recognize(many1(one_of("01")))),
    )(input)?;

    let value = match convert_to_i8(&value_str, neg_sign.is_some(), 2) {
        Ok(v) => v,
        Err(e) => {
            input.extra.report_error(e);
            0
        }
    };

    Ok((input, value))
}

/// Parse a single identifier.
pub fn identifier(input: Span) -> IResult<Span, String> {
    let (input, ident) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    Ok((input, ident.to_string()))
}

/// Converts a parsed string into an i8.
fn convert_to_i8(parsed: &Span, neg: bool, radix: u32) -> Result<i8, ParsingError> {
    i8::from_str_radix(&parsed.replace('_', ""), radix)
        // Convert the value to a signed value.
        .map(|v| v * if neg { -1 } else { 1 })
        // If there's an error, report it.
        .map_err(|e| match e.kind() {
            IntErrorKind::PosOverflow => ParsingError::Overflow {
                value: parsed.to_string(),
                span: into_source_span(parsed),
            },

            _ => todo!(),
        })
}
