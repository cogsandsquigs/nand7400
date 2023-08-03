use super::parser::ast::{Argument, ArgumentKind};
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

    /// The list of arguments for the opcode. If this list is empty, then the opcode has no arguments.
    /// Note that this does not map to the literal count of arguments (i.e. `len(args)`), but rather the
    /// length of the arguments in bytes. For example, labels are 1 argument but map to 2 bytes.
    pub args: Vec<OpcodeArg>,
}

/// The argument kind for an opcode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpcodeArg {
    /// A indirect number.
    Indirect,

    /// A direct/immediate number.
    Immediate,
}

impl<T> From<&Argument<T>> for OpcodeArg {
    fn from(arg: &Argument<T>) -> Self {
        match arg.kind {
            ArgumentKind::IndirectNumber(_) => Self::Indirect,
            ArgumentKind::ImmediateNumber(_) | ArgumentKind::Label(_) => Self::Immediate,
        }
    }
}
