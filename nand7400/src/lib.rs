mod ast;
pub mod config;
pub mod errors;
pub mod ffi;
mod parser;
mod tests;

use crate::parser::{AssemblyParser, Rule};
use ast::Label;
use config::AssemblerConfig;
use errors::{AssemblerError, AssemblerErrorKind};
use itertools::Itertools;
use miette::SourceSpan;
use pest::{
    error::InputLocation,
    iterators::{Pair, Pairs},
    Parser,
};
use std::collections::HashMap;

#[cfg(feature = "uniffi")]
use crate::ffi::*;
#[cfg(feature = "uniffi")]
use config::Opcode;

// If we are using uniffi, then include the scaffolding.
#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("lib");

/// The main assember structure to be used.
pub struct Assembler {
    /// The configuration for the assembler.
    config: AssemblerConfig,

    /// The symbol table for the assembler.
    symbols: HashMap<String, Label>,

    /// The current byte index we are at in the produced binary.
    current_byte_index: usize,

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
            current_byte_index: 0,
            source_code: None,
        }
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&mut self, config: AssemblerConfig) {
        self.config = config;
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, source: String) -> Result<Vec<u8>, AssemblerError> {
        // First, we should parse the source code with Pest.
        let source = self.parse(&source)?;

        // The binary code eventually produced by the assembler.
        let mut binary = Vec::new();

        // For every pair, we either turn it into binary or hook it into the symbol table for later.
        // Every pair should be a top-level instruction or label. No other rules should be present
        // at the top level.
        for pair in source {
            let tokens = pair.tokens();

            todo!()
        }

        todo!();

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
                let span = input_location_to_span(location);

                // C
                Err(AssemblerErrorKind::Unexpected {
                    span,
                    positives: positives.iter().map(|r| r.to_string()).unique().collect(),
                    negatives: negatives.iter().map(|r| r.to_string()).unique().collect(),
                }
                .into_err())
            }

            // TODO: Handle other errors (these are custom messages that should never occur, but still).
            Err(_) => todo!(),
        }
    }
}

/// Turn a pest `InputLocation` into a miette `SourceSpan`.
fn input_location_to_span(location: InputLocation) -> SourceSpan {
    match location {
        InputLocation::Pos(pos) => pos.into(),
        InputLocation::Span((start, end)) => (start..end).into(),
    }
}
