use std::{fmt::Display, ops::Add};

/// Represents a Position on a file.
/// The Eq and Ord instances are based only on the index, is
/// user responsibility to be congruent with this.
#[derive(Debug, Copy, Clone, Hash)]
pub struct Position {
    pub source_index: usize,
    /// This imposes a maximum amount of lines above 400,000,000
    /// Current lsp (3.17) uses 2**31 -1  as maximum value
    pub line: u32,
    /// As in line  this limits to 400,000,000 of bytes per column
    pub column: u32,
}

impl PartialEq for Position {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.source_index == other.source_index
    }
}

impl Eq for Position {}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source_index.cmp(&other.source_index)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.source_index.partial_cmp(&other.source_index)
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            source_index: 0,
            line: 0,
            column: 0,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.line, self.column)
    }
}

impl From<(usize, u32, u32)> for Position {
    fn from(input: (usize, u32, u32)) -> Self {
        let (source_index, line, column) = input;
        Position {
            source_index,
            line,
            column,
        }
    }
}

/// A region of text between two source positions
/// The add implementation returns the minimal region
/// that contains both regions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl From<((usize, u32, u32), (usize, u32, u32))> for Span {
    fn from(value: ((usize, u32, u32), (usize, u32, u32))) -> Self {
        Span {
            start: value.0.into(),
            end: value.1.into(),
        }
    }
}

impl From<(Position, Position)> for Span {
    fn from(value: (Position, Position)) -> Self {
        Span {
            start: value.0,
            end: value.1,
        }
    }
}

impl Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let start = std::cmp::min(self.start, rhs.start);
        let end = std::cmp::max(self.end, rhs.end);
        (start, end).into()
    }
}
