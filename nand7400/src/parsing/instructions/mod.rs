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
use miette::SourceSpan;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{consumed, map},
    multi::many0,
    sequence::pair,
    IResult,
};
use std::cmp::{max, min};

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
        match args.len().cmp(&(opcode.num_args as usize)) {
            // If the number of arguments we found is greater than the number of args the opcode expects, then we
            // have too many arguments.
            std::cmp::Ordering::Greater => {
                let first_arg = &args[0];

                let last_arg = &args[args.len() - 1];

                let extra_args_span = (
                    // The first argument is always the first (AST in-place), so we can just get the offset
                    // of the first argument.
                    first_arg.span.offset(),
                    // The distance between the start of the first argument and the end of the last argument
                    // is the difference of the offsets plus the length of the last argument.
                    (last_arg.span.offset() - first_arg.span.offset()) + last_arg.span.len(),
                );

                let err = ParsingError::TooManyArgs {
                    opcode: opcode.name.clone(),
                    expected: opcode.num_args as usize,
                    got: args.len(),
                    span: extra_args_span.into(),
                };

                input.extra.report_error(err);
            }

            // If the number of arguments we found is less than the number of args the opcode expects, then we have too
            // few arguments.
            std::cmp::Ordering::Less => {
                // Get the end of the last argument -- this is where the next argument should start.
                let last_arg_end = args
                    .last()
                    // Get the end of the last argument.
                    .map(|arg| arg.span.offset() + arg.span.len())
                    // Do the end of the mnemonic if there are no arguments.
                    .unwrap_or_else(|| consumed_opcode.location_offset() + consumed_opcode.len());

                // Create the error span.
                let err = ParsingError::TooFewArgs {
                    opcode: opcode.name.clone(),
                    expected: opcode.num_args as usize,
                    got: args.len(),
                    span: (last_arg_end, last_arg_end).into(),
                };

                input.extra.report_error(err);
            }

            // No error to report, all is good!
            std::cmp::Ordering::Equal => {}
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
