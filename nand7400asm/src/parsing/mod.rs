mod tests;
mod values;

use crate::{errors::AssemblerError, Assembler};
use nom_locate::LocatedSpan;

/// The span type used for parsing.
pub type Span<'a> = LocatedSpan<&'a str>;

/// Private parsing API for `Assembler`
impl Assembler {
    fn parse(&mut self, _source: String) -> Result<Vec<u8>, AssemblerError> {
        unimplemented!();
    }
}
