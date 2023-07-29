pub mod config;
pub mod errors;
pub mod parser;
pub mod position;

mod lexer;
mod tests;

use config::AssemblerConfig;
use errors::AssemblerError;
use parser::ast::Ast;

/// The main assember structure to be used.
pub struct Assembler {
    /// The configuration for the assembler.
    config: AssemblerConfig,
}

/// Public API for the assembler.
impl Assembler {
    /// Create a new assembler with the given configuration.
    pub fn new(config: AssemblerConfig) -> Self {
        Self { config }
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&mut self, config: AssemblerConfig) {
        self.config = config;
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        Ok(self.assemble_with_ast(source)?.0)
    }

    /// Assembles the given assembly code into binary and associated AST.
    pub fn assemble_with_ast(
        &mut self,
        source: &str,
    ) -> Result<(Vec<u8>, Ast), Vec<AssemblerError>> {
        todo!()
    }
}
