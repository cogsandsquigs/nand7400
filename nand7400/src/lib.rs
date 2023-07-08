mod ast;
pub mod config;
pub mod errors;
mod parser;
mod tests;

use crate::{
    ast::{Binary, BinaryKind, LABEL_SIZE},
    config::Opcode,
    parser::{AssemblyParser, Rule},
};
use config::AssemblerConfig;
use errors::{AssemblerError, AssemblerErrorKind};
use itertools::Itertools;
use miette::SourceSpan;
use pest::{
    error::InputLocation,
    iterators::{Pair, Pairs},
    Parser, Span,
};
use std::{
    collections::HashMap,
    num::{IntErrorKind, ParseIntError},
};

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
    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        // First, we should parse the source code with Pest.
        let parsed_file = self
            .parse(source)
            .map_err(|err| vec![err.with_source_code(source.to_string())])?
            .next()
            .expect("This should always parse a file if the parsing didn't fail!");

        // Convert into an "AST", basically a list of instructions or labels.
        let ast = self.get_instructions(parsed_file).map_err(|errs| {
            errs.into_iter()
                .map(|e| AssemblerError::new(e).with_source_code(source.to_string()))
                .collect_vec()
        })?;

        // Then, we should turn the AST into a binary.
        let binary = self.to_binary(ast).map_err(|errs| {
            errs.into_iter()
                .map(|e| AssemblerError::new(e).with_source_code(source.to_string()))
                .collect_vec()
        })?;

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
    fn to_binary(&self, ast: Binary) -> Result<Vec<u8>, Vec<AssemblerErrorKind>> {
        // All the collected errors from the first pass. We can use this to report multiple errors at once, and
        // it's safe to do so because 1) we already know the structure of the file, and 2) we won't output this
        // binary if there are any errors.
        let mut errors = vec![];
        let mut binary = vec![];

        for instruction in ast.binary {
            match instruction {
                BinaryKind::Literal(value) => {
                    binary.push(value);
                }

                BinaryKind::Label { name, span } => {
                    // Get the location of the label.
                    let location = self
                        .symbols
                        .get(&name)
                        .copied()
                        // If the label doesn't exist, then we should report an error.
                        .unwrap_or_else(|| {
                            errors.push(AssemblerErrorKind::LabelDNE {
                                mnemonic: name.clone(),
                                span,
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

        if errors.is_empty() {
            Ok(binary)
        } else {
            Err(errors)
        }
    }

    /// Does the first-pass assembly of the given source code.
    fn get_instructions(
        &mut self,
        parsed_file: Pair<'_, Rule>,
    ) -> Result<Binary, Vec<AssemblerErrorKind>> {
        // All the collected errors from the first pass. We can use this to report multiple errors at once, and
        // it's safe to do so because 1) we already know the structure of the file, and 2) we won't output this
        // binary if there are any errors.
        let mut errors = vec![];

        // The binary that we will be assembling.
        let mut ast = Binary::new();

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
                        // -1 because the length accounts for the first byte of the label
                        .insert(name.to_string(), ast.len() + LABEL_SIZE - 1);
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
                        .map(|arg| (get_argument(&arg), arg.as_span()))
                        .map(|(arg, span)| match arg {
                            Ok(arg) => (arg, span),
                            Err(err) => {
                                errors.push(err);

                                (BinaryKind::Literal(0xFF), span)
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

                            errors.push(AssemblerErrorKind::OpcodeDNE {
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
                    // we should report an error. We use a custom counting function because we need to count
                    // the number of bytes total, not the number of arguments (which can be less than the bytes).
                    if opcode.num_args
                        != arguments.iter().fold(0, |acc, (arg, _)| match arg {
                            BinaryKind::Literal { .. } => acc + 1,
                            BinaryKind::Label { .. } => acc + LABEL_SIZE as u32,
                        })
                    {
                        let mnemonic_span = mnemonic.as_span();

                        let arg_span = if arguments.is_empty() {
                            (mnemonic_span.end() + 1).into()
                        } else {
                            combine_spans(arguments.iter().map(|(_, span)| *span).collect_vec())
                        };

                        // Get the total span of the arguments.
                        errors.push(AssemblerErrorKind::WrongNumArgs {
                            mnemonic: mnemonic_span.as_str().to_string(),
                            expected: opcode.num_args as usize,
                            given: arguments.len(),
                            opcode: span_to_sourcespan(mnemonic_span),
                            wrong_args: arg_span,
                        });
                    }

                    // Add the opcode to the binary.
                    ast.push_literal(opcode.binary);

                    // Add the arguments to the binary.
                    for (arg, _) in arguments {
                        ast.push(arg);
                    }
                }

                //The only top-level rules are Literals and Identifiers
                x => {
                    dbg!(x);
                    unreachable!()
                }
            }
        }

        if errors.is_empty() {
            Ok(ast)
        } else {
            Err(errors)
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

/// Gets the span of multiple `Span`s. This will panic if the vector is empty.
fn combine_spans(spans: Vec<Span<'_>>) -> SourceSpan {
    let first_span = &spans[0];

    let last_span = &spans[spans.len() - 1];

    let combined_spans = (
        // The first argument is always the first (AST in-place), so we can just get the offset
        // of the first argument.
        first_span.start(),
        // The distance between the start of the first argument and the end of the last argument
        // is the difference of the offsets plus the length of the last argument.
        last_span.end(),
    );

    combined_spans.into()
}

/// Parses an argument into either a literal or a label.
fn get_argument(parsed_arg: &Pair<'_, Rule>) -> Result<BinaryKind, AssemblerErrorKind> {
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

            Ok(BinaryKind::Label {
                name: identifier.to_string(),
                span: span_to_sourcespan(parsed_arg.as_span()),
            })
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
    // If the literal is at least two characters long, then we should check if it's a hexadecimal, binary, or octal
    // literal. If it is, then we should parse it as such. Otherwise, we should parse it as a decimal literal.
    if literal.len() >= 2 {
        match &literal[0..2] {
            "0x" | "0X" => u8::from_str_radix(&literal[2..], 16),
            "0b" | "0B" => u8::from_str_radix(&literal[2..], 2),
            "0o" | "0O" => u8::from_str_radix(&literal[2..], 8),
            _ => literal.parse(),
        }
    }
    // If the literal is only one character, then we should parse it as a decimal literal. This is bvecause all
    // literals are numeric values, and only a decimal literal can be one character long.
    else {
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
