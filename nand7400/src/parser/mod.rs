mod tests;

use super::Assembler;
use crate::{
    ast::{Ast, Statement, LABEL_SIZE},
    config::Opcode,
    errors::{position::Position, AssemblerError},
};

use itertools::Itertools;
use num_traits::Num;
use pest::{
    error::InputLocation,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::{
    fmt,
    num::{IntErrorKind, ParseIntError},
};

/// The main parsing structure to be used.
#[derive(Parser)]
#[grammar = "parser/assembly.pest"]
pub struct AssemblyParser;

impl fmt::Display for Rule {
    /// Display the rule as a string in a human-readable format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::EOI => write!(f, "the end of the file"),
            Rule::EOL => write!(f, "the end of the line"),
            Rule::COMMENT => write!(f, "a comment"),
            Rule::WHITESPACE => write!(f, "whitespace"),
            Rule::File => write!(f, "an assembly file"),
            Rule::Label => write!(f, "a label"),
            Rule::Instruction => write!(f, "an instruction"),
            Rule::Identifier => write!(f, "an argument"),
            Rule::Colon => write!(f, "a colon"),
            Rule::Literal
            | Rule::HexLiteral
            | Rule::BinaryLiteral
            | Rule::OctalLiteral
            | Rule::DecimalLiteral => {
                write!(f, "a literal")
            }
        }
    }
}

/// Crate-public parsing API for the assembler.
impl Assembler {
    /// Parses the given source code into instructions.
    pub(crate) fn parse<'a>(&mut self, source: &'a str) -> Result<Pairs<'a, Rule>, AssemblerError> {
        match AssemblyParser::parse(Rule::File, source) {
            // Just return the source code if there are no errors.
            Ok(source) => Ok(source),

            // If there's a parsing error, then we should return an error.
            Err(pest::error::Error {
                variant:
                    pest::error::ErrorVariant::ParsingError {
                        positives,
                        negatives,
                    },
                location,
                ..
            }) => {
                // Convert the location into a span so that it can be used with miette.
                let span = input_location_to_position(location);

                // Return the error.
                Err(AssemblerError::Unexpected {
                    span,
                    positives: positives.iter().map(|r| r.to_string()).unique().collect(),
                    negatives: negatives.iter().map(|r| r.to_string()).unique().collect(),
                })
            }

            // TODO: Handle other errors (these are custom messages that should never occur, but still).
            Err(_) => todo!(),
        }
    }

    /// Does the first-pass assembly of the given source code.
    pub(crate) fn parse_file(
        &mut self,
        parsed_file: Pair<'_, Rule>,
    ) -> Result<Ast, Vec<AssemblerError>> {
        // All the collected errors from the first pass. We can use this to report multiple errors at once, and
        // it's safe to do so because 1) we already know the structure of the file, and 2) we won't output this
        // binary if there are any errors.
        let mut errors = vec![];

        // The binary that we will be assembling.
        let mut ast = Ast::new();

        // For every pair, we either turn it into binary or hook it into the symbol table for later.
        // Every pair should be a top-level instruction or label. No other rules should be present
        // at the top level, except EOI/SOI, which we can ignore.
        for pair in parsed_file.into_inner() {
            match pair.as_rule() {
                // Skip over EOI. Apparently, SOI is not a `Rule`, so we don't need to worry about it.
                Rule::EOI => (),

                // If we reach a lable, we should add it to the symbol table and keep track of its location in memory.
                Rule::Label => self.parse_label(&mut ast, pair),

                // If we reach an instruction, we should add it to the binary.
                Rule::Instruction => {
                    self.parse_instruction(&mut ast, &mut errors, pair);
                }

                // The only top-level rules are Literals and Identifiers
                _ => unreachable!(),
            }
        }

        if errors.is_empty() {
            Ok(ast)
        } else {
            Err(errors)
        }
    }
}

/// Private parsing API for the assembler.
impl Assembler {
    /// Parses a single label line and puts it in the symbol table.
    fn parse_label(&mut self, ast: &mut Ast, pair: Pair<'_, Rule>) {
        // Get the name of the label.
        let name = pair
            .as_str()
            // Get rid of whitespace around the label, as parsing carries with it some whitespace.
            .trim()
            // Get only everything before the colon, as the colon is not part of the label.
            .trim_end_matches(':');

        // Add the label to the symbol table.
        self.symbols.insert(name.to_string(), ast.len()); // No +1 because `len` is 0-indexed.

        // We don't insert it into the binary because it doesn't actually take up any space.
    }

    /// Parses a single instruction line and puts it in `ast`.
    fn parse_instruction(
        &mut self,
        ast: &mut Ast,
        errors: &mut Vec<AssemblerError>,
        pair: Pair<'_, Rule>,
    ) {
        // The raw parsed instruction.
        let mut raw_instruction = pair.into_inner();

        // Every instruction should have at least a mnemonic, so this is safe.
        let mnemonic = raw_instruction
            .next()
            .expect("Every instruction should have a mnemonic!");

        // Collect all the arguments into a vector.
        let arguments = raw_instruction
            // // Get rid of EOLs because they are not real arguments.
            // .filter(|arg| arg.as_rule() != Rule::EOL)
            // Get the argument and its span.
            .map(|arg| get_argument(&arg))
            // Make sure that errors are handled properly.
            .map(|arg| match arg {
                Ok(arg) => arg,
                Err(err) => {
                    errors.push(err);

                    Statement::Literal {
                        value: 0xFF,
                        span: (0, 0).into(), // HACK: This is a placeholder span.
                    }
                }
            })
            .collect_vec();

        // Get the actual opcode and use that to get it's binary representation. If the opcode
        // doesn't exist, then we add it to the errors and use `0xFF` as a placeholder.
        let opcode = self
            .config
            .get_opcode(mnemonic.as_str())
            .cloned()
            // If the opcode doesn't exist, create a "fake" one with the mnemonic and the number of
            // arguments and report an error.
            .unwrap_or_else(|| {
                let span = mnemonic.as_span();

                errors.push(AssemblerError::OpcodeDNE {
                    mnemonic: span.as_str().to_string(),
                    span: span_to_position(span),
                });

                Opcode {
                    mnemonic: mnemonic.as_str().to_string(),
                    binary: 0xFF,
                    num_args: arguments.len() as u32,
                }
            });

        let mnemonic_span = mnemonic.as_span();

        // If the number of arguments doesn't match the number of arguments the opcode takes, then
        // we should report an error. We use a custom counting function because we need to count
        // the number of bytes total, not the number of arguments (which can be less than the bytes).
        if opcode.num_args
            != arguments.iter().fold(0, |acc, arg| match arg {
                Statement::Literal { .. } => acc + 1,
                Statement::Label { .. } => acc + LABEL_SIZE as u32,
            })
        {
            let args_span = if arguments.is_empty() {
                (mnemonic_span.end() + 1).into()
            } else {
                let first_arg = &arguments[0];
                let last_arg = &arguments[arguments.len() - 1];

                first_arg.span().join(last_arg.span())
            };

            // Get the total span of the arguments.
            errors.push(AssemblerError::WrongNumArgs {
                mnemonic: mnemonic_span.as_str().to_string(),
                expected: opcode.num_args as u16,
                given: arguments.len() as u16,
                opcode_span: span_to_position(mnemonic_span),
                args_span,
            });
        }

        // Add the opcode to the binary.
        ast.push(Statement::Literal {
            value: opcode.binary,
            span: span_to_position(mnemonic_span),
        });

        // Add the arguments to the binary.
        for arg in arguments {
            ast.push(arg);
        }
    }
}

/// Parses an argument into either a literal or a label.
fn get_argument(parsed_arg: &Pair<'_, Rule>) -> Result<Statement, AssemblerError> {
    match parsed_arg.as_rule() {
        // If the argument is a literal, then we should parse it into a `u8`.
        Rule::Literal => {
            // Get the literal as a string.
            let literal = parsed_arg.as_str();

            // Parse the literal into a `u8` depending of if it's signed.
            let literal = parse_signed_literal(literal).map_err(|err| match err.kind() {
                // If the literal is too large, then we should report an error.
                IntErrorKind::PosOverflow => AssemblerError::Overflow {
                    literal: literal.to_string(),
                    span: span_to_position(parsed_arg.as_span()),
                },

                // Ditto if a literal is too small
                IntErrorKind::NegOverflow => AssemblerError::Underflow {
                    literal: literal.to_string(),
                    span: span_to_position(parsed_arg.as_span()),
                },

                // Check if the digits are invalid.
                // TODO: Parse a general number w/o respect for digits and then check if the digits are invalid, instead
                // of filtering out digits in parse-time.
                IntErrorKind::InvalidDigit => AssemblerError::InvalidDigit {
                    literal: literal.to_string(),
                    span: span_to_position(parsed_arg.as_span()),
                },

                // Unreachable because our parser will never parse an empty string as a number.
                IntErrorKind::Empty => unreachable!("Parser should never parse \"\" as a number!"),

                // Unreachable because i8s and u8s allow for 0 as a valid value.
                IntErrorKind::Zero => unreachable!("i8s and u8s should allow 0 as a value!"),

                // Unreachable because there should be no more errors to consider.
                _ => unreachable!("There should be no more errors to consider!"),
            })?;

            Ok(Statement::Literal {
                value: literal,
                span: span_to_position(parsed_arg.as_span()),
            })
        }

        // If the argument is an identifier, then we should parse it into a label.
        Rule::Identifier => {
            // Get the identifier as a string.
            let identifier = parsed_arg.as_str();

            Ok(Statement::Label {
                name: identifier.to_string(),
                span: span_to_position(parsed_arg.as_span()),
            })
        }

        // The only top-level rules are Literals and Identifiers
        _ => unreachable!(),
    }
}

/// Parses either an unsigned or signed string literal into a `u8`.
fn parse_signed_literal(literal: &str) -> Result<u8, ParseIntError> {
    // If the literal starts with '+' or '-', then we should remove it and parse it as a signed
    // literal.
    if literal.starts_with('+') || literal.starts_with('-') {
        // Strip the sign from the literal so we can check it later.
        let (sign, literal) = literal.split_at(1);

        let value = parse_literal::<i8>(literal)?;

        match sign {
            "+" => Ok(value as u8),
            "-" => Ok((-value) as u8),
            _ => todo!("Report this as having an incorrect prefix!"),
        }
    }
    // Otherwise, we should parse it as an unsigned literal.
    else {
        parse_literal::<u8>(literal)
    }
}

/// Parses a generic string literal into a `T` where T is a number.
fn parse_literal<T>(literal: &str) -> Result<T, T::FromStrRadixErr>
where
    T: Num,
{
    match &literal[0..2] {
        "0x" | "0X" => T::from_str_radix(&literal[2..], 16),
        "0b" | "0B" => T::from_str_radix(&literal[2..], 2),
        "0o" | "0O" => T::from_str_radix(&literal[2..], 8),
        _ => T::from_str_radix(literal, 10),
    }
}

/// Turns a `Span` into a `Position`.
fn span_to_position(span: pest::Span<'_>) -> Position {
    (span.start(), span.end()).into()
}

/// Turn a pest `InputLocation` into a `Position`.
fn input_location_to_position(location: InputLocation) -> Position {
    match location {
        InputLocation::Pos(pos) => pos.into(),
        InputLocation::Span((start, end)) => (start, end).into(),
    }
}
