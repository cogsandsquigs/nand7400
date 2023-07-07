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
pub fn decimal(input: Span) -> IResult<Span, i64> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |out: Span| out.replace('_', "").parse::<i64>(),
    )(input)
}

/// Parse a single hexidecimal value.
pub fn hexadecimal(input: Span) -> IResult<Span, i64> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: Span| i64::from_str_radix(&out.replace('_', ""), 16),
    )(input)
}

/// Parse a single octal value.
pub fn octal(input: Span) -> IResult<Span, i64> {
    map_res(
        preceded(
            alt((tag("0o"), tag("0O"))),
            recognize(many1(terminated(one_of("01234567"), many0(char('_'))))),
        ),
        |out: Span| i64::from_str_radix(&out.replace('_', ""), 8),
    )(input)
}

/// Parse a single binary value.
pub fn binary(input: Span) -> IResult<Span, i64> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(terminated(one_of("01"), many0(char('_'))))),
        ),
        |out: Span| i64::from_str_radix(&out.replace('_', ""), 2),
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

/// Tests for parsing values.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test parsing a decimal value, seeing if we stop at the right place (before the extra characters).
    #[test]
    fn test_decimal() {
        assert_eq!(
            decimal(Span::new("123_456_789_0_abcdefg")).unwrap().1,
            1234567890
        );
    }

    /// Test parsing a hexadecimal value, seeing if we stop at the right place (before the extra characters).
    #[test]
    fn test_hexadecimal() {
        assert_eq!(
            hexadecimal(Span::new("0xF00D_BABE_ghijk")).unwrap().1,
            4027431614
        );
    }

    /// Test parsing an octal value, seeing if we stop at the right place (before the extra characters).
    #[test]
    fn test_octal() {
        assert_eq!(
            octal(Span::new("0o012_345_67_890abcdefg")).unwrap().1,
            342391
        );
    }

    /// Test parsing a binary value, seeing if we stop at the right place (before the extra characters).
    #[test]
    fn test_binary() {
        assert_eq!(
            binary(Span::new("0b1010_1010_234567890abcdefg")).unwrap().1,
            170
        );
    }

    /// Test parsing an identifier, seeing if we stop at the right place (before the extra characters).
    #[test]
    fn test_identifier() {
        assert_eq!(
            identifier(Span::new("foo_bar_123 asdfawefi3")).unwrap().1,
            "foo_bar_123"
        );
    }
}
