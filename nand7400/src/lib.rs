pub mod ast;
pub mod config;
pub mod errors;
pub mod position;

mod lexer;
mod parser;
mod tests;
mod token;

use ast::{Ast, Statement};
use config::AssemblerConfig;
use errors::AssemblerError;
use std::collections::HashMap;

/// The main assember structure to be used.
pub struct Assembler {
    /// The configuration for the assembler.
    config: AssemblerConfig,

    /// The symbol table for the assembler. It maps a label name to its location in memory.
    symbols: HashMap<String, u16>,
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
        // // First, we should parse the source code with Pest.
        // let parsed_file = self
        //     .parse(source)
        //     .map_err(|err| vec![err])?
        //     .next()
        //     .expect("This should always parse a file if the parsing didn't fail!");

        // // Convert into an "AST", basically a list of instructions or labels.
        // let ast = self.parse_file(parsed_file)?;

        // // Then, we should turn the AST into a binary.
        // let binary = self.to_binary(&ast)?;

        // // Finally, we can call `reset` to reset the internal state of the assembler.
        // self.reset();

        // Ok((binary, ast))

        todo!()
    }
}

/// Private API for the assembler.
impl Assembler {
    /// Resets the internal state of the assembler, WITHOUT resetting the configuration.
    fn reset(&mut self) {
        self.symbols.clear();
    }

    /// Turn a `Ast` into a `Vec<u8>` using the symbol table.
    fn to_binary(&self, ast: &Ast) -> Result<Vec<u8>, Vec<AssemblerError>> {
        // All the collected errors from the first pass. We can use this to report multiple errors at once, and
        // it's safe to do so because 1) we already know the structure of the file, and 2) we won't output this
        // binary if there are any errors.
        let mut errors = vec![];
        let mut binary = vec![];

        for instruction in ast.statements() {
            match instruction {
                Statement::Literal { value, .. } => {
                    binary.push(*value);
                }

                Statement::Label { name, span } => {
                    // Get the location of the label.
                    let location = self
                        .symbols
                        .get(name)
                        .copied()
                        // If the label doesn't exist, then we should report an error.
                        .unwrap_or_else(|| {
                            errors.push(AssemblerError::LabelDNE {
                                mnemonic: name.clone(),
                                span: *span,
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
}
