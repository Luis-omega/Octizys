use std::ops::Add;

use octizys_pretty::{combinators, document::Document};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Position {
    ///The index position based on rust chars
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

impl From<&Span> for Document {
    fn from(value: &Span) -> Document {
        combinators::text(&format!(
            "start: {}, end: {}",
            value.start.source_index, value.end.source_index
        ))
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
