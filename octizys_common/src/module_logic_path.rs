use std::fmt::Display;

use crate::identifier::{Identifier, IdentifierError};
use octizys_text_store::store::Store;

pub const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModuleLogicPathError {
    NotIdentifier,
    //To be used by the TryFrom, not by the make smart constructor
    EmptyString,
}

impl Display for ModuleLogicPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleLogicPathError::NotIdentifier => {
                write!(f, "invalid char in Identifier while building path")
            }
            ModuleLogicPathError::EmptyString => {
                write!(f, "attempt to build empty identifier")
            }
        }
    }
}

impl From<Identifier> for ModuleLogicPath {
    fn from(value: Identifier) -> Self {
        ModuleLogicPath(vec![value])
    }
}

/// The abstract representation of a importation path inside the language,
/// usually called `logic path`.
///
/// Example:
///
/// ```txt
/// a::b::cder::
/// ```
///
// TODO: maybe abbreviate  the name to just `LogicPath` ?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleLogicPath(Vec<Identifier>);

impl ModuleLogicPath {
    // TODO: Change the definition of this to allow spaces between items
    // The problem with that is that the grammar has ambiguities if we do that
    // as we may need to allow comments between items to be consistent
    // with allowing comments everywhere.
    pub fn make(
        s: &str,
        store: &mut Store,
    ) -> Result<ModuleLogicPath, ModuleLogicPathError> {
        let mut splited: Vec<&str> =
            s.split(MODULE_LOGIC_PATH_SEPARATOR).collect();
        splited.pop();
        let v: Vec<Identifier> = splited
            .into_iter()
            .map(|x| Identifier::make(x, store))
            .collect::<Result<Vec<Identifier>, IdentifierError>>()
            .map_err(|_x| ModuleLogicPathError::NotIdentifier)?;
        Ok(ModuleLogicPath(v))
    }
}

impl TryFrom<Vec<Identifier>> for ModuleLogicPath {
    type Error = ModuleLogicPathError;
    fn try_from(value: Vec<Identifier>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            Err(ModuleLogicPathError::EmptyString)
        } else {
            Ok(ModuleLogicPath(value))
        }
    }
}

impl From<ModuleLogicPath> for Vec<Identifier> {
    fn from(value: ModuleLogicPath) -> Self {
        value.0
    }
}

impl<'a> From<&'a ModuleLogicPath> for &'a Vec<Identifier> {
    fn from(value: &'a ModuleLogicPath) -> Self {
        &value.0
    }
}

impl Display for ModuleLogicPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logic::Path")
    }
}
