mod instructions;
mod tests;
mod utils;
mod values;

use self::{
    instructions::{instruction, label},
    utils::consume_line,
};
use crate::{
    ast::Instruction,
    errors::{parsing::ParsingError, AssemblerError},
    Assembler,
};
use nom::{
    branch::alt, character::complete::multispace0, combinator::map, multi::many0,
    sequence::delimited,
};
use nom_locate::LocatedSpan;
use std::{cell::RefCell, rc::Rc};

/// The state of the parser's errors that we use to report errors. You can clone this to create a
/// new parser state with the same reference to the errors stack -- It uses an `Rc` and `RefCell`
/// internally.
///
/// We use this to track errors instead of using `nom`'s default error handling, because it's too
/// limited for our use case. We want to be able to report errors from within a parser combinator
/// without stopping parsing, and `nom`'s default error handling doesn't allow for that.
#[derive(Debug, Clone)]
pub struct State {
    /// A list of errors that have occurred during parsing.
    /// TODO: is this really the best way to do this?
    pub errors: Rc<RefCell<Vec<ParsingError>>>,
}

impl State {
    /// Creates a new `State` with no errors.
    pub fn new() -> Self {
        Self {
            errors: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Pushes an error onto the errors stack from within a `nom`
    /// parser combinator while still allowing parsing to continue.
    pub fn report_error(&self, error: ParsingError) {
        self.errors.borrow_mut().push(error);
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// The span type used for parsing.
pub type Span<'a> = LocatedSpan<&'a str, State>;

/// Private parsing API for `Assembler`
impl Assembler {
    /// Parse a single line of assembly code. If it returns `None`, then the line was a label
    /// definition. If it returns `Some(_)`, then the line was an instruction.
    pub fn parse_file(&mut self, input: String) -> Result<Vec<Instruction>, AssemblerError> {
        let (output, instructions) = many0(
            // This is the parser for a single line of assembly code.
            delimited(
                multispace0,
                alt((
                    // Parse a label. We don't update the current byte index here, as we don't want to count
                    // labels as instructions.
                    map(label(self.current_byte_index), |label| {
                        // We don't update the current byte index here, as we don't want to count labels as
                        // instructions.

                        // Insert the label into the symbol table.
                        self.symbols.insert(label.name.clone(), label);

                        None
                    }),
                    // Parse an instruction. We need to update the current byte index here, as we want to
                    // count instructions as bytes.
                    map(instruction(&self.config.opcodes), |instruction| {
                        match instruction {
                            Some(instruction) => {
                                // Update the current byte index.
                                self.current_byte_index += 1 + instruction.opcode.num_args as usize;

                                dbg!(&instruction.opcode.name, self.current_byte_index);

                                Some(instruction)
                            }
                            None => None,
                        }
                    }),
                )),
                multispace0,
            ),
        )(Span::new_extra(&input, State::new()))
        .expect("TODO: Implement nom errors");

        // If there are any errors, return them.
        if !output.extra.errors.borrow().is_empty() {
            return Err(output.extra.errors.borrow().clone().into());
        }

        let instructions = instructions
            .into_iter()
            // Filter out the labels.
            .flatten()
            // Collect the instructions into a vector.
            .collect::<Vec<_>>();

        Ok(instructions)
    }
}
