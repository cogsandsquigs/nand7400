use crate::position::Position;

/// The size of label memory addresses, in bytes.
pub const LABEL_SIZE: u16 = 2;

/// A collection of binary instructions that form a binary. This is the output of the assembler, and has a maximum length
/// of `u16::MAX` bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast {
    /// The current length of the binary, in bytes.
    length: u16,

    /// The statements that make up the binary.
    statements: Vec<Statement>,
}

impl Ast {
    /// Create a new binary  with no instructions.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            length: 0,
        }
    }

    /// Gets the length of the binary, in bytes.
    pub fn len(&self) -> u16 {
        self.length
    }

    /// Check if the binary is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns an array of all the binary instructions in the binary.
    pub fn statements(&self) -> &[Statement] {
        &self.statements
    }

    /// Push a general binary instruction to the binary.
    pub(crate) fn push(&mut self, binary: Statement) {
        match binary {
            Statement::Literal { .. } => self.length += 1,
            Statement::Label { .. } => self.length += LABEL_SIZE,
        }

        self.statements.push(binary);
    }
}

/// A type of binary instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    /// A literal binary instruction.
    Literal {
        /// The value of the literal.
        value: u8,

        /// The span of the literal in the source code.
        span: Position,
    },

    /// A label that can be used to refer to a specific instruction.
    Label {
        /// The name of the label.
        name: String,

        /// The span of the label in the source code.
        span: Position,
    },
}

impl Statement {
    /// Gets the span of the binary instruction.
    pub fn span(&self) -> &Position {
        match self {
            Statement::Literal { span, .. } => span,
            Statement::Label { span, .. } => span,
        }
    }
}
