mod tests;

use super::Span;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    character::complete::{char, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many0_count, many1},
    sequence::pair,
    sequence::{preceded, terminated},
    IResult,
};

/// Parse a single decimal value.
/// TODO: allow for negatives.
pub fn decimal(input: Span) -> IResult<Span, u8> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: Span| out.replace('_', "").parse::<u8>(),
    )(input)
}

/// Parse a single hexidecimal value.
/// TODO: allow for negatives.
pub fn hexadecimal(input: Span) -> IResult<Span, u8> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: Span| u8::from_str_radix(&out.replace('_', ""), 16),
    )(input)
}

/// Parse a single octal value.
/// TODO: allow for negatives.
pub fn octal(input: Span) -> IResult<Span, u8> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        |out: Span| u8::from_str_radix(&out.replace('_', ""), 8),
    )(input)
}

/// Parse a single binary value.
/// TODO: allow for negatives.
pub fn binary(input: Span) -> IResult<Span, u8> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        |out: Span| u8::from_str_radix(&out.replace('_', ""), 2),
    )(input)
}

/// Parse a single identifier.
pub fn identifier(input: Span) -> IResult<Span, String> {
    let (input, ident) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    Ok((input, ident.to_string()))
}
