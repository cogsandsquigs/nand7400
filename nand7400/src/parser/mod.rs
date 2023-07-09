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
            Rule::EOI => write!(f, "the end of the file"),
            Rule::EOL => write!(f, "the end of the line"),
            Rule::COMMENT => write!(f, "a comment"),
            Rule::WHITESPACE => write!(f, "whitespace"),
            Rule::File => write!(f, "an assembly file"),
            Rule::Label => write!(f, "a label"),
            Rule::Instruction => write!(f, "an instruction"),
            Rule::Identifier => write!(f, "an argument"),
            Rule::Colon => write!(f, "a colon"),
            Rule::Literal
            | Rule::HexLiteral
            | Rule::BinaryLiteral
            | Rule::OctalLiteral
            | Rule::DecimalLiteral => {
                write!(f, "a literal")
            }
        }
    }
}
