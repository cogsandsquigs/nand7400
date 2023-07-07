mod tests;

use super::{
    into_source_span,
    values::{binary, decimal, hexadecimal, identifier, octal},
    ws, Span,
};
use crate::ast::{Argument, ArgumentKind, Instruction};
use nom::{
    branch::alt,
    character::complete::newline,
    combinator::{consumed, map},
    multi::many_till,
    IResult,
};

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
            map(hexadecimal, |v| ArgumentKind::Literal(v)),
            map(octal, |v| ArgumentKind::Literal(v)),
            map(binary, |v| ArgumentKind::Literal(v)),
            map(decimal, |v| ArgumentKind::Literal(v)),
            map(identifier, ArgumentKind::Label),
        ))),
        |(span, kind)| Argument::new(kind, into_source_span(span)),
    )(input)
}
