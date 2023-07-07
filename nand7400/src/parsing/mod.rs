mod instructions;
mod tests;
mod utils;
mod values;

use crate::{
    errors::{parsing::ParsingError, AssemblerError},
    Assembler,
};
use nom_locate::LocatedSpan;
use std::{cell::RefCell, rc::Rc};

/// The state of the parser's errors that we use to report errors. You can clone this to create a
/// new parser state with the same reference to the errors stack -- It uses an `Rc` and `RefCell`
/// internally.
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
    fn parse(&mut self, _source: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();
    }
}
