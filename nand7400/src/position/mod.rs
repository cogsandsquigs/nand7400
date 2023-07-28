mod tests;

use miette::SourceSpan;

/// A position or span of text in the source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The starting index of the position, byte-wise. This is inclusive.
    pub start: u32,

    /// The ending index of the position, byte-wise. This is exclusive.
    pub end: u32,
}

impl Position {
    /// Creates a new position from a start and end index.
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start as u32,
            end: end as u32,
        }
    }

    /// Returns the starting index of the position.
    pub fn starting_char(&self) -> usize {
        self.start as usize
    }

    /// Returns the ending index of the position. Note that this is exclusive, so
    /// `self.end - 1` is the last index of the position.
    pub fn ending_char(&self) -> usize {
        (self.end - 1) as usize // - 1 because the end is exclusive
    }

    /// Joins two positions together.
    pub fn join(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Returns the length of the position.
    pub fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
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
