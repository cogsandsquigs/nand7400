pub mod config;
pub mod errors;
pub mod parser;
pub mod position;

mod tests;

use std::collections::HashMap;

use self::parser::{
    ast::{Argument, ArgumentKind, Ast, InstructionKind, Keyword, Label},
    Parser,
};
use config::AssemblerConfig;
use errors::AssemblerError;
use num_traits::{FromPrimitive, Num, ToBytes, ToPrimitive};

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
    pub fn assemble(&mut self, source: &str) -> Result<Vec<u8>, AssemblerError> {
        Ok(self.assemble_with_ast(source)?.0)
    }

    /// Assembles the given assembly code into binary and associated AST.
    pub fn assemble_with_ast(&mut self, source: &str) -> Result<(Vec<u8>, Ast), AssemblerError> {
        let ast = Parser::new(source)?.parse()?; // Parse the source into an AST.
        let mut next_mem_location = 0; // The next memory location to write to.
        let mut binary = vec![]; // The binary to write to.

        for instruction in &ast.instructions {
            dbg!(instruction);
            dbg!(&binary);
            match &instruction.kind {
                // Skip labels, as they;ve already been loaded into the symbol table.
                InstructionKind::Label(_) => continue,

                // Execute the keywords as they come in.
                InstructionKind::Keyword { keyword, arguments } => match keyword {
                    // Set the memory location to the 1st argument.
                    Keyword::Org => {
                        // Make sure there's only 1 argument.
                        if arguments.len() != 1 {
                            return Err(AssemblerError::WrongNumArgs {
                                mnemonic: keyword.to_string(),
                                expected: 1,
                                given: arguments.len() as u16,
                                mnemonic_span: instruction.token_span,
                                args_span: arguments
                                    .iter()
                                    .map(|arg| arg.span)
                                    .fold(instruction.token_span, |acc, span| acc.join(&span)),
                            });
                        }

                        let arg = &arguments[0]; // This is safe because we already checked the length.

                        // Now adjust the memory location based on the argument.
                        next_mem_location = decode_arg_u16(&ast.symbols, arg)? as usize;

                        // Adjust the binary buffer if the next location is out-of-range.
                        if next_mem_location >= binary.len() {
                            binary.resize(next_mem_location, 0);
                        }
                    }

                    // Set the next byte(s) to the arguments.
                    Keyword::Byte => {
                        let mut bytes = vec![];

                        for arg in arguments {
                            bytes.extend(decode_arg_bytes(&ast.symbols, arg)?);
                        }

                        for (i, byte) in bytes.iter().enumerate() {
                            binary.insert(next_mem_location + i, *byte);
                        }

                        next_mem_location += bytes.len();
                    }
                },

                InstructionKind::Opcode {
                    mnemonic,
                    arguments,
                } => todo!(),
            }
        }

        Ok((binary, ast))
    }
}

/// Decodes an argument into a series of bytes. Numbers and labels are converted to little endian. However, if the trailing
/// byte(s) is/are 0, then they are cut. This does not happen to labels, and only happens to numbers.
pub fn decode_arg_bytes<T>(
    symbol_table: &HashMap<Label, u16>,
    arg: &Argument<T>,
) -> Result<Vec<u8>, AssemblerError>
where
    T: Num + ToBytes + ToPrimitive + Clone,
{
    match &arg.kind {
        ArgumentKind::ImmediateNumber(num) | ArgumentKind::IndirectNumber(num) => {
            let mut bytes = num.to_le_bytes().as_ref().to_vec();

            // Remove trailing 0s, except for the last one.
            while bytes.last() == Some(&0) && bytes.len() > 1 {
                bytes.pop();
            }

            Ok(bytes)
        }

        ArgumentKind::Label(label) => Ok(symbol_table
            .get(label)
            .ok_or_else(|| AssemblerError::LabelDNE {
                mnemonic: label.clone(),
                span: arg.span,
            })?
            .to_le_bytes()
            .to_vec()),
    }
}

/// Decodes an argument into a `u16` type.
pub fn decode_arg_u16<T>(
    symbol_table: &HashMap<Label, u16>,
    arg: &Argument<T>,
) -> Result<u16, AssemblerError>
where
    T: Num + ToBytes + ToPrimitive + Clone + FromPrimitive,
{
    match &arg.kind {
        ArgumentKind::ImmediateNumber(num) | ArgumentKind::IndirectNumber(num) => {
            Ok(num.to_u16().expect("Need to catch this error!"))
        }

        ArgumentKind::Label(label) => {
            Ok(*symbol_table
                .get(label)
                .ok_or_else(|| AssemblerError::LabelDNE {
                    mnemonic: label.clone(),
                    span: arg.span,
                })?)
        }
    }
}
