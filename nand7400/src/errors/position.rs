use miette::SourceSpan;

/// A position or span of text in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The starting index of the position, byte-wise. This is inclusive.
    pub start: u32,

    /// The ending index of the position, byte-wise. This is exclusive.
    pub end: u32,
}

/// Creates a position from a start index.
impl From<usize> for Position {
    fn from(start: usize) -> Self {
        Self {
            start: start as u32,
            end: start as u32,
        }
    }
}

/// Creates a position from a start and end index.
impl From<(usize, usize)> for Position {
    fn from((start, end): (usize, usize)) -> Self {
        Self {
            start: start as u32,
            end: end as u32,
        }
    }
}

/// Creates a `miette::SourceSpan` from a `Position`.
impl From<Position> for SourceSpan {
    fn from(val: Position) -> Self {
        SourceSpan::new(
            (val.start as usize).into(),
            ((val.end - val.start) as usize).into(),
        )
    }
}
