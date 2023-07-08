use miette::SourceSpan;

/// A collection of binary instructions that form a binary. This is the output of the assembler, and has a maximum length
/// of `u16::MAX` bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Binary {
    /// The current length of the binary, in bytes.
    length: u16,

    /// The binary itself. This is a vector of bytes.
    pub(crate) binary: Vec<BinaryKind>,
}

impl Binary {
    /// Create a new binary  with no instructions.
    pub fn new() -> Self {
        Self {
            binary: Vec::new(),
            length: 0,
        }
    }

    /// Gets the length of the binary, in bytes.
    pub fn len(&self) -> u16 {
        self.length
    }

    /// Push a literal binary instruction to the binary.
    pub fn push_literal(&mut self, literal: u8) {
        self.binary.push(BinaryKind::Literal(literal));
        self.length += 1;
    }
}

/// A type of binary instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryKind {
    /// A literal binary instruction.
    Literal(u8),

    /// A label that can be used to refer to a specific instruction.
    Label(Label),
}

/// A label that can be used to refer to a specific instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    /// The name of the label.
    pub name: String,

    /// The span of the label in the source code.
    pub span: SourceSpan,
}

impl Label {
    /// Create a new label with the given name and span.
    pub fn new(name: String, span: SourceSpan) -> Label {
        Label { name, span }
    }
}
