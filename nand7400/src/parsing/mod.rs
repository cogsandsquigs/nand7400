mod tests;

// use pest::Parser;
use pest_derive::Parser;

/// The main parsing structure to be used.
#[derive(Parser)]
#[grammar = "parsing/assembly.pest"]
pub struct AssemblyParser {}
