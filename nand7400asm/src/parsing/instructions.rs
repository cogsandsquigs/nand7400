use super::{
    into_source_span,
    values::{binary, decimal, hexadecimal, identifier, octal},
    Span,
};
use crate::ast::{Argument, ArgumentKind, Instruction};
use nom::{
    branch::alt,
    character::{
        complete::{newline, space0},
        is_newline, is_space,
    },
    combinator::{consumed, map},
    error::ParseError,
    multi::{many_till, separated_list0},
    sequence::delimited,
    IResult, Parser,
};

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
impl super::Assembler {
    /// Parse a single instruction mnemonic. This parses until the end of the line.
    fn instruction<'a>(&self, input: Span<'a>) -> IResult<Span<'a>, Instruction> {
        let (input, mnemonic) = identifier(input)?;

        let (input, (args, _)) = many_till(ws(argument), newline)(input)?;

        let opcode = self
            .config
            .opcodes
            .iter()
            .find(|opcode| opcode.name == mnemonic)
            .ok_or_else(|| todo!())?;

        Ok((
            input,
            Instruction::new(opcode.clone(), args, into_source_span(input)),
        ))
    }
}

/// Parse a single instruction argument.
pub fn argument(input: Span) -> IResult<Span, Argument> {
    map(
        consumed(alt((
            map(decimal, |v| ArgumentKind::Literal(v as u8)),
            map(hexadecimal, |v| ArgumentKind::Literal(v as u8)),
            map(octal, |v| ArgumentKind::Literal(v as u8)),
            map(binary, |v| ArgumentKind::Literal(v as u8)),
            map(identifier, |v| ArgumentKind::Label(v.to_string())),
        ))),
        |(span, kind)| Argument::new(kind, into_source_span(span)),
    )(input)
}
