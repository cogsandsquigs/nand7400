use crate::assembler::position::Position;
use std::collections::HashMap;

/// The size of labels, in bytes.
pub const LABEL_SIZE: u16 = 2;

/// The entire AST. This includes the set of instructions, as well as the symbol table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast {
    /// The instructions in the AST.
    pub instructions: Vec<Instruction>,

    /// The symbol table in the AST. This translates from label names to the memory address they correspond to.
    pub symbols: HashMap<Label, u16>,
}

/// An actual instruction, which contains the position and instruction kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// The kind of instruction.
    pub kind: InstructionKind,

    /// The span of the instruction in the source code.
    pub instruction_span: Position,

    /// The span of the 1st token of the instruction in the source code. Essentially, this is either the
    /// span of the opcode, keyword, or label without any arguments.
    pub token_span: Position,
}

impl Instruction {
    /// Create a new instruction.
    pub fn new(kind: InstructionKind, instruction_span: Position, token_span: Position) -> Self {
        Self {
            kind,
            instruction_span,
            token_span,
        }
    }

    /// Gets the binary length of the instruction. This is used for calculating the memory address of the next instruction.
    pub fn binary_len(&self) -> u16 {
        match &self.kind {
            InstructionKind::Label(_) | InstructionKind::Keyword { .. } => 0, // 0 because labels and keywords don't take up any space
            InstructionKind::Opcode { arguments, .. } => {
                arguments // Account for the space that labels take up (more than a single byte)
                    .iter()
                    .map(|arg| match &arg.kind {
                        ArgumentKind::ImmediateNumber(_) | ArgumentKind::IndirectNumber(_) => 1, // 1 because arguments for opcodes are always 1 byte
                        ArgumentKind::Label(_) => LABEL_SIZE, // LABEL_SIZE because labels can be more than 1 byte
                    })
                    .sum::<u16>()
                    + 1 // +1 for the opcode itself
            }
        }
    }
}

/// The type of instruction in the assembly code. Each instruction is one line of assembly code. These can be
/// opcodes, labels, or keyword instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionKind {
    /// A label, which is a name followed by a colon.
    Label(Label),

    /// An opcode, which is a mnemonic followed by a list of arguments.
    Opcode {
        /// The mnemonic of the opcode.
        mnemonic: String,

        /// The arguments of the opcode.
        arguments: Vec<Argument<u8>>,
    },

    /// A keyword instruction, which is a keyword followed by a list of arguments.
    Keyword {
        /// The keyword in question.
        keyword: Keyword,

        /// The arguments of the instruction.
        arguments: Vec<Argument<u16>>,
    },
}

/// An argument to a keyword instruction or opcode. `T` is the integer type of the argument, because arguments are generally the
/// same, but opcodes are 8-bit and keyword instructions are 16-bit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument<T> {
    /// The kind of argument.
    pub kind: ArgumentKind<T>,

    /// The instruction_span of the argument in the source code.
    pub span: Position,
}

/// The kind of argument that an opcode takes. This is used for type safety.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgumentKind<T> {
    /// A raw or immediate numeric value.
    ImmediateNumber(T),

    /// An indirect numeric value, referencing a point in memory.
    IndirectNumber(T),

    /// A label, which is a name.
    Label(Label),
}

/// The type of keyword instruction in the assembly code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    /// The `.org` keyword, which sets the origin of the program at that point.
    Org,

    /// The `.byte` keyword, which defines the byte at the current location.
    Byte,
}

/// A label type, which is a wrapper around a string. This is mainly used for enforcing type safety.
pub type Label = String;

/*** IMPLICATIONS ***/

impl Ast {
    /// Create a new, empty AST.
    pub fn empty() -> Self {
        Self {
            instructions: Vec::new(),
            symbols: HashMap::new(),
        }
    }
}

impl<T> Argument<T> {
    /// Create a new argument.
    pub fn new(kind: ArgumentKind<T>, span: Position) -> Self {
        Self { kind, span }
    }
}
