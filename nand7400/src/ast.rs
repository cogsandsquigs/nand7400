use crate::config::Opcode;
use miette::SourceSpan;

/// A label in the assembler.
#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    /// The name of the label.
    pub name: String,

    /// The value that the label holds.
    pub value: usize,

    /// The span of the label in the source code.
    pub span: SourceSpan,
}

/// An instruction in the assembler.
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    /// The opcode of the instruction.
    pub opcode: Opcode,

    /// The arguments of the instruction.
    pub arguments: Vec<Argument>,

    /// The span of the opcode in the source code.
    pub opcode_span: SourceSpan,
}

/// Public API for `Instruction`.
impl Instruction {
    /// Creates a new instruction.
    pub fn new(opcode: Opcode, arguments: Vec<Argument>, opcode_span: SourceSpan) -> Self {
        Self {
            opcode,
            arguments,
            opcode_span,
        }
    }
}

/// An argument to an instruction.
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    /// The kind of argument that is expected.
    pub kind: ArgumentKind,

    /// The span of the argument in the source code.
    pub span: SourceSpan,
}

/// Public API for `Argument`.
impl Argument {
    /// Creates a new argument.
    pub fn new(kind: ArgumentKind, span: SourceSpan) -> Self {
        Self { kind, span }
    }
}

/// The type of argument that is expected.
#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentKind {
    /// A literal argument. This is simply a number.
    Literal(i8),

    /// A label argument. This is a label that will be resolved to a memory address.
    Label(String),
}
