mod ast;
pub mod config;
pub mod errors;
pub mod ffi;
mod parsing;

use ast::{ArgumentKind, Instruction};
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
    symbols: HashMap<String, u8>,

    /// The list of instructions.
    pub instructions: Vec<Instruction>,
}

/// Public API for the assembler.
impl Assembler {
    /// Create a new assembler with the given configuration.
    pub fn new(config: AssemblerConfig) -> Self {
        Self {
            config,
            symbols: HashMap::new(),
            instructions: Vec::new(),
        }
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, _source: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();

        let binary = self.instructions_to_binary()?;

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
        self.instructions.clear();
    }

    /// Compiles the instructions into binary.
    fn instructions_to_binary(&self) -> Result<Vec<u8>, AssemblerError> {
        let mut binary = Vec::new();

        for instruction in &self.instructions {
            binary.push(instruction.opcode.binary);

            for argument in &instruction.arguments {
                match &argument.kind {
                    // If the argument is a literal, we just push the literal to the binary.
                    ArgumentKind::Literal(literal) => binary.push(*literal),

                    // If it's a label, we need to look it up in the symbol table, and then
                    // push that value to the binary.
                    ArgumentKind::Label(label) => {
                        let address = self.symbols.get(label).unwrap();
                        binary.push(*address);
                    }
                }
            }
        }

        Ok(binary)
    }
}
