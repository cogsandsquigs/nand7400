use miette::SourceSpan;

/// A single binary instruction. This can either be a literal binary instruction or a label.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// A literal binary instruction.
    Literal(u8),

    /// A label that will be resolved later.
    Label(Label),
}

/// A label that can be used to refer to a specific instruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    /// The name of the label.
    pub name: String,

    /// The index of the label in the binary.
    pub memory_index: usize,

    /// The span of the label in the source code.
    pub span: SourceSpan,
}
