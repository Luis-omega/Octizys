use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub source_index: usize,
}

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        Position {
            source_index: value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Span {
            start: value.0.into(),
            end: value.1.into(),
        }
    }
}

impl Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let start =
            std::cmp::max(self.start.source_index, rhs.start.source_index);
        let end = std::cmp::max(self.end.source_index, rhs.end.source_index);
        (start, end).into()
    }
}