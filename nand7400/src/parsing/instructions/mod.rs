mod tests;

use super::{
    utils::{consume_line, into_source_span, ws},
    values::{binary, decimal, hexadecimal, identifier, octal},
    Span,
};
use crate::{
    ast::{Argument, ArgumentKind, Instruction, Label},
    config::Opcode,
    errors::parsing::ParsingError,
    Assembler,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{consumed, map},
    multi::many0,
    sequence::pair,
    IResult,
};

/// Private parsing API for `Assembler`
impl Assembler {
    /// Parse a single line of assembly code. If it returns `None`, then the line was a label
    /// definition. If it returns `Some(_)`, then the line was an instruction.
    fn line<'a>(&'a mut self, input: Span<'a>) -> IResult<Span<'a>, Option<Instruction>> {
        let (input, instruction) = alt((
            // Parse a label. We don't update the current byte index here, as we don't want to count
            // labels as instructions.
            map(label(self.current_byte_index), |label| {
                // We don't update the current byte index here, as we don't want to count labels as
                // instructions.

                // Insert the label into the symbol table.
                self.symbols.insert(label.name.clone(), label);

                None
            }),
            // Parse an instruction. We need to update the current byte index here, as we want to
            // count instructions as bytes.
            map(instruction(&self.config.opcodes), |instruction| {
                match instruction {
                    Some(instruction) => {
                        // Update the current byte index.
                        self.current_byte_index += instruction.opcode.num_args as usize;

                        Some(instruction)
                    }
                    None => None,
                }
            }),
        ))(input)?;

        // Consume the rest of the line.
        let (input, _) = consume_line(input)?;

        Ok((input, instruction))
    }
}

/// Parse a single label definition, which is the current location in code when translated into
/// machine code. This parses until the end of the line. It requires the current byte index to be
/// passed in, as it needs to know the current location in code.
fn label<'a>(current_byte_index: usize) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Label> {
    move |input: Span| {
        let (input, ((label_span, label_name), _)) =
            pair(ws(consumed(map(identifier, String::from))), ws(tag(":")))(input)?;

        let label = Label {
            name: label_name,
            value: current_byte_index + 1, // +1 because the current byte index is the next byte to be written.
            span: into_source_span(&label_span),
        };

        Ok((input, label))
    }
}

/// Parse a single instruction mnemonic. This parses until the end of the line.
fn instruction<'a>(
    opcodes: &'a [Opcode],
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, Option<Instruction>> {
    |input| {
        // Get the opcode mnemonic.
        let (input, (consumed_opcode, mnemonic)): (Span, (Span, String)) =
            consumed(ws(identifier))(input)?;

        // Get all the arguments for the instruction.
        let (input, args) = many0(ws(argument))(input)?;

        // Consume the rest of the whitespace on the line.
        let (input, _) = multispace0(input)?;

        // Get the opcode from the config.
        let opcode = opcodes
            .iter()
            .find(|opcode| opcode.name == mnemonic)
            // TODO: Error if the opcode doesn't exist.
            .or_else(|| {
                let err = ParsingError::OpcodeDNE {
                    opcode: consumed_opcode.to_string(),
                    span: into_source_span(&consumed_opcode),
                };

                input.extra.report_error(err);

                None
            });

        let Some(opcode) = opcode else {
            return Ok((input, None));
        };

        // TODO: Error if the number of args don't match the number of args the opcode expects.
        if args.len() != opcode.num_args as usize {
            todo!();
        }

        Ok((
            input,
            Some(Instruction::new(
                opcode.clone(),
                args,
                into_source_span(&consumed_opcode),
            )),
        ))
    }
}

/// Parse a single instruction argument.
pub fn argument(input: Span) -> IResult<Span, Argument> {
    map(
        consumed(alt((
            map(hexadecimal, ArgumentKind::Literal),
            map(octal, ArgumentKind::Literal),
            map(binary, ArgumentKind::Literal),
            map(decimal, ArgumentKind::Literal),
            map(identifier, ArgumentKind::Label),
        ))),
        |(span, kind)| Argument::new(kind, into_source_span(&span)),
    )(input)
}
