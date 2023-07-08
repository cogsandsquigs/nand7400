mod tests;

use std::fmt;

// use pest::Parser;
use pest_derive::Parser;

/// The main parsing structure to be used.
#[derive(Parser)]
#[grammar = "parser/assembly.pest"]
pub struct AssemblyParser;

impl fmt::Display for Rule {
    /// Display the rule as a string in a human-readable format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::EOI => write!(f, "end of file"),
            Rule::COMMENT => write!(f, "comment"),
            Rule::WHITESPACE => write!(f, "whitespace"),
            Rule::File => write!(f, "register"),
            Rule::Label => write!(f, "label"),
            Rule::Instruction => write!(f, "instruction"),
            Rule::Identifier => write!(f, "argument"),
            Rule::Literal
            | Rule::HexLiteral
            | Rule::BinaryLiteral
            | Rule::OctalLiteral
            | Rule::DecimalLiteral => {
                write!(f, "literal")
            }
        }
    }
}
