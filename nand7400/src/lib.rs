mod ast;
pub mod config;
pub mod errors;
pub mod ffi;
mod parser;
mod tests;

use crate::{
    ast::BinaryKind,
    config::Opcode,
    parser::{AssemblyParser, Rule},
};
use ast::{Binary, Label, LABEL_SIZE};
use config::AssemblerConfig;
use errors::{AssemblerError, AssemblerErrorKind};
use itertools::Itertools;
use miette::SourceSpan;
use pest::{
    error::InputLocation,
    iterators::{Pair, Pairs},
    Parser,
};
use std::{
    collections::HashMap,
    num::{IntErrorKind, ParseIntError},
};

#[cfg(feature = "uniffi")]
use crate::ffi::{errors::*, *};
#[cfg(feature = "uniffi")]
use config::Opcode;

// If we are using uniffi, then include the scaffolding.
#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("lib");

/// The main assember structure to be used.
pub struct Assembler {
    /// The configuration for the assembler.
    config: AssemblerConfig,

    /// The symbol table for the assembler. It maps a label name to its location in memory.
    symbols: HashMap<String, u16>,

    /// The source code that was assembled. This is mostly used for
    /// error reporting.
    source_code: Option<String>,
}

/// Public API for the assembler.
impl Assembler {
    /// Create a new assembler with the given configuration.
    pub fn new(config: AssemblerConfig) -> Self {
        Self {
            config,
            symbols: HashMap::new(),
            source_code: None,
        }
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&mut self, config: AssemblerConfig) {
        self.config = config;
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, AssemblerError> {
        // First, we should parse the source code with Pest.
        let parsed_file = self
            .parse(source)?
            .next()
            .expect("This should always parse a file if the parsing didn't fail!");

        // Convert into an "AST", basically a list of instructions or labels.
        let ast = self.get_instructions(parsed_file)?;

        // Then, we should turn the AST into a binary.
        let binary = self.to_binary(ast)?;

        // Finally, we can call `reset` to reset the internal state of the assembler.
        self.reset();

        Ok(binary)
    }
}

/// Private API for the assembler.
impl Assembler {
    /// Resets the internal state of the assembler, WITHOUT resetting the configuration.
    fn reset(&mut self) {
        self.symbols.clear();
        self.source_code = None;
    }

    /// Turn a `Binary` into a `Vec<u8>` using the symbol table.
    fn to_binary(&self, ast: Binary) -> Result<Vec<u8>, AssemblerError> {
        let mut errors = AssemblerError::empty();
        let mut binary = vec![];

        for instruction in ast.binary {
            match instruction {
                BinaryKind::Literal(literal) => binary.push(literal),

                BinaryKind::Label(label) => {
                    // Get the location of the label.
                    let location = self
                        .symbols
                        .get(&label.name)
                        .copied()
                        // If the label doesn't exist, then we should report an error.
                        .unwrap_or_else(|| {
                            errors.report(AssemblerErrorKind::LabelDNE {
                                mnemonic: label.name.clone(),
                                span: label.span,
                            });

                            // Return a placeholder value.
                            u16::MAX
                        });

                    // Add the location to the binary.
                    binary.push((location >> 8) as u8);
                    binary.push((location & 0xFF) as u8);
                }
            }
        }

        Ok(binary)
    }

    /// Does the first-pass assembly of the given source code.
    fn get_instructions(&mut self, parsed_file: Pair<'_, Rule>) -> Result<Binary, AssemblerError> {
        // All the collected errors from the first pass. We can use this to report multiple errors at once, and
        // it's safe to do so because 1) we already know the structure of the file, and 2) we won't output this
        // binary if there are any errors.
        let mut errors = AssemblerError::empty();

        // The binary that we will be assembling.
        let mut binary = Binary::new();

        // For every pair, we either turn it into binary or hook it into the symbol table for later.
        // Every pair should be a top-level instruction or label. No other rules should be present
        // at the top level, except EOI/SOI, which we can ignore.
        for pair in parsed_file.into_inner() {
            match pair.as_rule() {
                // Skip over EOI. Apparently, SOI is not a `Rule`, so we don't need to worry about it.
                Rule::EOI => (),

                // If we reach a lable, we should add it to the symbol table and keep track of its location in memory.
                Rule::Label => {
                    // Get the name of the label.
                    let name = pair.as_str().trim_end_matches(':');

                    // Add the label to the symbol table.
                    self.symbols
                        .insert(name.to_string(), binary.len() + LABEL_SIZE);

                    // We don't insert it into the binary because it doesn't actually take up any space.
                }

                // If we reach an instruction, we should add it to the binary.
                Rule::Instruction => {
                    // The raw parsed instruction.
                    let mut raw_instruction = pair.into_inner();

                    // Every instruction should have at least a mnemonic, so this is safe.
                    let mnemonic = raw_instruction
                        .next()
                        .expect("Every instruction should have a mnemonic!");

                    // Collect all the arguments into a vector.
                    let arguments = raw_instruction
                        .map(get_argument)
                        .map(|arg| match arg {
                            Ok(arg) => arg,
                            Err(err) => {
                                errors.report(err);
                                BinaryKind::Literal(0xFF)
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

                            errors.report(AssemblerErrorKind::OpcodeDNE {
                                mnemonic: span.as_str().to_string(),
                                span: span_to_sourcespan(span),
                            });

                            Opcode {
                                mnemonic: mnemonic.as_str().to_string(),
                                binary: 0xFF,
                                num_args: arguments.len() as u32,
                            }
                        });

                    // If the number of arguments doesn't match the number of arguments the opcode takes, then
                    // we should report an error.
                    // TODO: Do this!
                    if opcode.num_args != arguments.len() as u32 {
                        let span = mnemonic.as_span();

                        todo!()
                    }

                    // Add the opcode to the binary.
                    binary.push_literal(opcode.binary);
                }

                //The only top-level rules are Literals and Identifiers
                x => {
                    dbg!(x);
                    unreachable!()
                }
            }
        }

        if errors.is_empty() {
            Ok(binary)
        } else {
            Err(errors.with_source_code(self.source_code.clone().unwrap_or_default()))
        }
    }

    /// Parses the given source code into instructions.
    fn parse<'a>(&mut self, source: &'a str) -> Result<Pairs<'a, Rule>, AssemblerError> {
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
                let span = input_location_to_sourcespan(location);

                // Return the error.
                Err(AssemblerErrorKind::Unexpected {
                    span,
                    positives: positives.iter().map(|r| r.to_string()).unique().collect(),
                    negatives: negatives.iter().map(|r| r.to_string()).unique().collect(),
                }
                .into_err()
                .with_source_code(self.source_code.clone().unwrap_or_default()))
            }

            // TODO: Handle other errors (these are custom messages that should never occur, but still).
            Err(_) => todo!(),
        }
    }
}

/// Parses an argument into either a literal or a label.
fn get_argument(parsed_arg: Pair<'_, Rule>) -> Result<BinaryKind, AssemblerErrorKind> {
    match parsed_arg.as_rule() {
        // If the argument is a literal, then we should parse it into a `u8`.
        Rule::Literal => {
            // Get the literal as a string.
            let literal = parsed_arg.as_str();

            // Parse the literal into a `u8`.
            let literal = parse_literal(literal).map_err(|err| match err.kind() {
                // If the literal is too large, then we should report an error.
                IntErrorKind::PosOverflow => AssemblerErrorKind::Overflow {
                    literal: literal.to_string(),
                    span: span_to_sourcespan(parsed_arg.as_span()),
                },

                // TODO: Handle other errors.
                _ => todo!(),
            })?;

            Ok(BinaryKind::Literal(literal))
        }

        // If the argument is an identifier, then we should parse it into a label.
        Rule::Identifier => {
            // Get the identifier as a string.
            let identifier = parsed_arg.as_str();

            Ok(BinaryKind::Label(Label {
                name: identifier.to_string(),
                span: span_to_sourcespan(parsed_arg.as_span()),
            }))
        }

        //The only top-level rules are Literals and Identifiers
        x => {
            dbg!(x);
            unreachable!()
        }
    }
}

/// Parses a generic string literal into a `u8`.
fn parse_literal(literal: &str) -> Result<u8, ParseIntError> {
    if literal.starts_with("0x") {
        u8::from_str_radix(
            literal
                .strip_prefix("0x")
                .expect("We've already confirmed that the string contains this prefix!"),
            16,
        )
    } else if literal.starts_with("0X") {
        u8::from_str_radix(
            literal
                .strip_prefix("0X")
                .expect("We've already confirmed that the string contains this prefix!"),
            16,
        )
    } else if literal.starts_with("0b") {
        u8::from_str_radix(
            literal
                .strip_prefix("0b")
                .expect("We've already confirmed that the string contains this prefix!"),
            2,
        )
    } else if literal.starts_with("0B") {
        u8::from_str_radix(
            literal
                .strip_prefix("0B")
                .expect("We've already confirmed that the string contains this prefix!"),
            2,
        )
    } else if literal.starts_with("0o") {
        u8::from_str_radix(
            literal
                .strip_prefix("0o")
                .expect("We've already confirmed that the string contains this prefix!"),
            8,
        )
    } else if literal.starts_with("0O") {
        u8::from_str_radix(
            literal
                .strip_prefix("0O")
                .expect("We've already confirmed that the string contains this prefix!"),
            8,
        )
    } else {
        literal.parse()
    }
}

/// Turns a `Span` into a `SourceSpan`.
fn span_to_sourcespan(span: pest::Span<'_>) -> SourceSpan {
    (span.start()..span.end()).into()
}

/// Turn a pest `InputLocation` into a miette `SourceSpan`.
fn input_location_to_sourcespan(location: InputLocation) -> SourceSpan {
    match location {
        InputLocation::Pos(pos) => pos.into(),
        InputLocation::Span((start, end)) => (start..end).into(),
    }
}
