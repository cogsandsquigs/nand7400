use serde::{Deserialize, Serialize};

/// The main configuration type for the assembler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssemblerConfig {
    /// The opcodes to be used by the assembler.
    pub opcodes: Vec<Opcode>,
}

/// Public API for the assembler configuration.
impl AssemblerConfig {
    /// Gets an opcode by its name.
    pub fn get_opcode(&self, mnemonic: &str) -> Option<&Opcode> {
        self.opcodes
            .iter()
            .find(|opcode| opcode.mnemonic == mnemonic)
    }
}

/// An opcode to be parsed by the assembler.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opcode {
    /// The name of the opcode.
    pub mnemonic: String,

    /// The binary representation of the opcode, as a byte.
    pub binary: u8,

    /// The number of arguments the opcode takes.
    #[serde(alias = "numArgs")]
    pub num_args: u32,
}
