mod ast;
pub mod config;
pub mod errors;
pub mod ffi;
mod parsing;
mod tests;

use ast::{ArgumentKind, Instruction, Label};
use config::AssemblerConfig;
use errors::AssemblerError;
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

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, source: String) -> Result<Vec<u8>, AssemblerError> {
        // First, we need to set the source code, so that we can use it for error reporting.
        self.source_code = Some(source.clone());

        // Parse the source code into instructions.
        // let instructions = self.parse_file(source)?;
        todo!();

        let binary = self.instructions_to_binary(todo!())?;

        // Reset the symbol table and stuff, as we don't need it anymore. This also allows
        // for multiple calls to assemble() without having to create a new assembler.
        self.reset();

        Ok(binary)
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&mut self, config: AssemblerConfig) {
        self.config = config;
    }
}

/// Private API for the assembler.
impl Assembler {
    /// Resets the internal state of the assembler, WITHOUT resetting the configuration.
    fn reset(&mut self) {
        self.symbols.clear();
        self.source_code = None;
    }

    /// Compiles the instructions into binary.
    fn instructions_to_binary(
        &self,
        instructions: Vec<Instruction>,
    ) -> Result<Vec<u8>, AssemblerError> {
        let mut binary = Vec::new();

        for instruction in &instructions {
            binary.push(instruction.opcode.binary);

            for argument in &instruction.arguments {
                match &argument.kind {
                    // If the argument is a literal, we just push the literal to the binary.
                    ArgumentKind::Literal(literal) => binary.push(*literal as u8),

                    // If it's a label, we need to look it up in the symbol table, and then
                    // push that value to the binary.
                    ArgumentKind::Label(label) => {
                        // Get the label from the symbol table.
                        let label = self
                            .symbols
                            .get(label)
                            // If the label doesn't exist, return an error.
                            .ok_or_else(|| AssemblerError::LabelDNE {
                                label: label.clone(),
                                span: argument.span,
                                source_code: self.source_code.clone().unwrap_or_default(),
                            })?;

                        binary.push(label.value as u8); // TODO: What if the code is longer than 255 bytes?
                    }
                }
            }
        }

        Ok(binary)
    }
}
