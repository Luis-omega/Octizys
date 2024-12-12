use std::fmt::Display;

use crate::{
    equivalence::Equivalence,
    identifier::{Identifier, IdentifierError},
};
use octizys_text_store::store::Store;

pub const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogicPathError {
    NotIdentifier,
    //To be used by the TryFrom, not by the make smart constructor
    EmptyString,
}

impl Equivalence for LogicPathError {
    fn equivalent(self, other: Self) -> bool {
        self == other
    }
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
        let mut splited: Vec<&str> =
            s.split(MODULE_LOGIC_PATH_SEPARATOR).collect();
        splited.pop();
        let v: Vec<Identifier> = splited
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
    fn equivalent(self, other: Self) -> bool {
        self.0.equivalent(other.0)
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
