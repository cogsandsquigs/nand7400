pub mod config;
pub mod errors;
pub mod ffi;

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
}

/// Public API for the assembler.
impl Assembler {
    /// Create a new assembler with the given configuration.
    pub fn new(config: AssemblerConfig) -> Self {
        Self {
            config,
            symbols: HashMap::new(),
        }
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, source: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();

        // Reset the symbol table and stuff, as we don't need it anymore. This also allows
        // for multiple calls to assemble() without having to create a new assembler.
        self.reset();

        Ok(todo!())
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
    }
}
