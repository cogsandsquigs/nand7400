use serde::{Deserialize, Serialize};

/// The main configuration type for the assembler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssemblerConfig {
    /// The opcodes to be used by the assembler.
    pub opcodes: Vec<Opcode>,
}

/// An opcode to be parsed by the assembler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opcode {
    /// The name of the opcode.
    pub name: String,

    /// The binary representation of the opcode, as a byte.
    pub binary: u8,

    /// The number of arguments the opcode takes.
    #[serde(alias = "numArgs")]
    pub num_args: u32,
}
