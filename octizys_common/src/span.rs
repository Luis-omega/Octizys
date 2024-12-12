use std::{fmt::Display, ops::Add};

/// Represents a Position on a file.
/// The Eq and Ord instances are based only on the index, is
/// user responsibility to be congruent with this.
#[derive(Debug, Copy, Clone, Hash)]
pub struct Position {
    pub source_index: usize,
    pub line: usize,
    pub column: usize,
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

/// Takes a index and try to find the boundary for a char before the index.
fn split_string_before(src: &str, i: usize) -> (&str, usize) {
    let mut index = i;
    for _ in [0..4] {
        match src.split_at_checked(index) {
            Some((_, x)) => {
                return (x, index);
            }
            None => (),
        };
        index = index.saturating_sub(1);
    }
    // given the 4 bytes boundary we may succed in 4 steps always.
    return (src, 0);
}

/// Takes a index and try to find the boundary for a char after the index.
fn split_string_after(src: &str, i: usize) -> (&str, usize) {
    let mut index = i;
    for _ in [0..4] {
        match src.split_at_checked(index) {
            Some((_, x)) => {
                return (x, index);
            }
            None => (),
        };
        index = index.saturating_add(1);
    }
    // given the 4 bytes boundary we may succed in 4 steps always.
    return (src, src.len());
}

impl Position {
    /// Lookups the [`Position::source_index`] in the text and returns a
    /// little before and after as much as it can without line breaks.
    /// The first element is before the text (exclusive).
    /// The last element is after the text (inclusive).
    pub fn get_text_at<'source>(
        &self,
        src: &'source str,
        max_len: Option<usize>,
    ) -> (&'source str, &'source str) {
        let size_range: usize =
            max_len.map(|x| if x < 2 { 2 } else { x }).unwrap_or(20) / 2;
        let (_, start_index) = split_string_before(
            src,
            self.source_index.saturating_sub(size_range),
        );
        let (_, end_index) = split_string_after(
            src,
            self.source_index.saturating_add(size_range),
        );
        let before_pre = &src[start_index..self.source_index];
        let after_pre = &src[self.source_index..end_index];
        let before =
            before_pre.split("\n").collect::<Vec<&str>>().pop().unwrap();
        let after = after_pre.split("\n").collect::<Vec<&str>>()[0];
        (before, after)
    }
}

impl From<(usize, usize, usize)> for Position {
    fn from(input: (usize, usize, usize)) -> Self {
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

impl From<((usize, usize, usize), (usize, usize, usize))> for Span {
    fn from(value: ((usize, usize, usize), (usize, usize, usize))) -> Self {
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

impl Span {
    /// Returns the span content including line breaks plus some surrounding text. Always return
    /// all the span content.
    /// The max_len is the maximal length of the surrounding text.
    /// The surrounding text is cut to avoid containing line breaks.
    /// This doesn't take in account graphemes, only char boundaries!.
    pub fn get_text_at<'source>(
        &self,
        src: &'source str,
        max_len: Option<usize>,
    ) -> (&'source str, &'source str, &'source str) {
        match max_len {
            Some(max_len) => {
                let size_range = max_len / 2;
                if size_range == 0 {
                    (
                        "",
                        &src[self.start.source_index..self.end.source_index],
                        "",
                    )
                } else {
                    let (_, start_index) = split_string_before(
                        src,
                        self.start.source_index.saturating_sub(size_range),
                    );
                    let (_, end_index) = split_string_after(
                        src,
                        self.end.source_index.saturating_add(size_range),
                    );
                    let before_pre = &src[start_index..self.start.source_index];
                    let after_pre = &src[self.end.source_index..end_index];
                    let before = before_pre
                        .split("\n")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap();
                    let after = after_pre.split("\n").collect::<Vec<&str>>()[0];
                    (
                        before,
                        &src[self.start.source_index..self.end.source_index],
                        after,
                    )
                }
            }
            None => {
                ("", &src[self.start.source_index..self.end.source_index], "")
            }
        }
    }
}
