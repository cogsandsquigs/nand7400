pub mod config;
pub mod error;

use config::Config;
use error::AssemblerError;
use std::collections::HashMap;

// If we are using uniffi, then include the scaffolding.
#[cfg(feature = "uniffi")]
uniffi::include_scaffolding!("lib");

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn flip(a: bool) -> bool {
    !a
}

pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

/// The main assember structure to be used.
pub struct Assembler {
    /// The configuration for the assembler.
    config: Config,

    /// The symbol table for the assembler.
    symbols: HashMap<String, u8>,
}

/// Public API for the assembler.
impl Assembler {
    /// Create a new assembler with the given configuration.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            symbols: HashMap::new(),
        }
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, code: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();

        // Reset the symbol table and stuff, as we don't need it anymore.
        self.reset();

        Ok(todo!())
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&mut self, config: Config) {
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
