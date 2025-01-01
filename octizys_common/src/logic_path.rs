use std::fmt::Display;

use crate::{
    equivalence::{make_report, Equivalence},
    identifier::{Identifier, IdentifierError},
};
use octizys_pretty::{
    combinators::{external_text, static_str},
    store::NonLineBreakStr,
};
use octizys_text_store::store::Store;

pub const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogicPathError {
    NotIdentifier,
    //To be used by the TryFrom, not by the make smart constructor
    EmptyString,
}

impl Display for LogicPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicPathError::NotIdentifier => {
                write!(f, "invalid char in Identifier while building path")
            }
            LogicPathError::EmptyString => {
                write!(f, "attempt to build empty identifier")
            }
        }
    }
}

impl From<Identifier> for LogicPath {
    fn from(value: Identifier) -> Self {
        LogicPath(vec![value])
    }
}

/// The abstract representation of a logic path inside the language.
///
/// Example:
///
/// ```txt
/// a::b::cder::
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicPath(Vec<Identifier>);

impl LogicPath {
    /// Expect a string of the following characteristics:
    /// `Identifier::Identifier::...`
    /// It must end in :: or we are going to forget the last one
    pub fn make(
        s: &str,
        store: &mut Store,
    ) -> Result<LogicPath, LogicPathError> {
        let mut split_vector: Vec<&str> =
            s.split(MODULE_LOGIC_PATH_SEPARATOR).collect();
        split_vector.pop();
        let v: Vec<Identifier> = split_vector
            .into_iter()
            .map(|x| Identifier::make(x, store))
            .collect::<Result<Vec<Identifier>, IdentifierError>>()
            .map_err(|_x| LogicPathError::NotIdentifier)?;
        Ok(LogicPath(v))
    }

    pub fn push(&mut self, i: Identifier) {
        self.0.push(i)
    }
}

/// Equivalence use is to compare trees without positional information.
/// As such Identifiers are opaque!
impl Equivalence for LogicPath {
    fn equivalent(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }

    fn equivalence_or_diff(
        &self,
        other: &Self,
    ) -> Result<(), octizys_pretty::document::Document> {
        if self.0.len() == other.0.len() {
            Ok(())
        } else {
            Err(make_report(self, other))
        }
    }

    fn represent(&self) -> octizys_pretty::document::Document {
        const PATH: NonLineBreakStr = NonLineBreakStr::new("LogicPath::");
        static_str(PATH) + external_text(&(self.0.len().to_string()))
    }
}

impl TryFrom<Vec<Identifier>> for LogicPath {
    type Error = LogicPathError;
    fn try_from(value: Vec<Identifier>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            Err(LogicPathError::EmptyString)
        } else {
            Ok(LogicPath(value))
        }
    }
}

impl From<LogicPath> for Vec<Identifier> {
    fn from(value: LogicPath) -> Self {
        value.0
    }
}

impl<'a> From<&'a LogicPath> for &'a Vec<Identifier> {
    fn from(value: &'a LogicPath) -> Self {
        &value.0
    }
}

impl Display for LogicPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LogicPath::")
    }
}
