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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleLogicPath(Vec<Identifier>);

impl ModuleLogicPath {
    pub fn make(
        s: &str,
        store: &mut Store,
    ) -> Result<ModuleLogicPath, ModuleLogicPathError> {
        let v: Vec<Identifier> = s
            .split(MODULE_LOGIC_PATH_SEPARATOR)
            .map(|x| Identifier::make(x, store))
            .collect::<Result<Vec<Identifier>, IdentifierError>>()
            .map_err(|_x| ModuleLogicPathError::NotIdentifier)?;
        Ok(ModuleLogicPath(v))
    }

    pub fn split_head(self) -> (Option<ModuleLogicPath>, Identifier) {
        //the split used in the make constructor guaranties that we
        //always have a element in the internal vector
        let mut v = self.0;
        let last = v.pop().unwrap();
        if v.len() == 0 {
            (None, last)
        } else {
            (Some(ModuleLogicPath(v)), last)
        }
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
