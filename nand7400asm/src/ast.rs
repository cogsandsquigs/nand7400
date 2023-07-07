use crate::config::Opcode;

/// An instruction in the assembler.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    /// The opcode of the instruction.
    pub opcode: Opcode,

    /// The memory address of the instruction.
    pub memory_address: usize,

    /// The arguments of the instruction.
    pub arguments: Vec<Argument>,
}

/// An argument to an instruction.
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    /// The kind of argument that is expected.
    pub kind: ArgumentKind,
}

/// The type of argument that is expected.
#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentKind {
    /// A literal argument. This is simply a number.
    Literal(u8),

    /// A label argument. This is a label that will be resolved to a memory address.
    Label(String),
}
