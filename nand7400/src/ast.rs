use miette::SourceSpan;

/// The size of label memory addresses, in bytes.
pub const LABEL_SIZE: u16 = 2;

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

    /// Push a general binary instruction to the binary.
    pub fn push(&mut self, binary: BinaryKind) {
        match binary {
            BinaryKind::Literal { .. } => self.length += 1,
            BinaryKind::Label { .. } => self.length += LABEL_SIZE,
        }

        self.binary.push(binary);
    }

    /// Push a literal value to the binary.
    pub fn push_literal(&mut self, value: u8) {
        self.push(BinaryKind::Literal(value));
    }
}

/// A type of binary instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryKind {
    /// A literal binary instruction.
    Literal(u8),

    /// A label that can be used to refer to a specific instruction.
    Label {
        /// The name of the label.
        name: String,

        /// The span of the label in the source code.
        span: SourceSpan,
    },
}
