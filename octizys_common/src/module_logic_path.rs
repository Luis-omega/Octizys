use crate::error::{error_from_document, Error};
use crate::identifier::{Identifier, IdentifierError};
use octizys_pretty::combinators;
use octizys_pretty::document::{Document, Interner};

const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug)]
pub enum ModuleLogicPathError {
    NotIdentifier,
    //To be used by the TryFrom, not by the make smart constructor
    EmptyString,
}

impl From<&ModuleLogicPathError> for Document {
    fn from(value: &ModuleLogicPathError) -> Document {
        match value {
            &ModuleLogicPathError::NotIdentifier => {
                "The passed string contains a non valid Identifier component"
                    .into()
            }
            &ModuleLogicPathError::EmptyString => {
                "Attempt to build from a empty vector or string".into()
            }
        }
    }
}

impl From<Identifier> for ModuleLogicPath {
    fn from(value: Identifier) -> Self {
        ModuleLogicPath(vec![value])
    }
}

impl From<&ModuleLogicPathError> for Error {
    fn from(vaue: &ModuleLogicPathError) -> Error {
        error_from_document(vaue)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleLogicPath(Vec<Identifier>);

impl ModuleLogicPath {
    pub fn make(
        s: &str,
        interner: &mut Interner,
    ) -> Result<ModuleLogicPath, ModuleLogicPathError> {
        let v: Vec<Identifier> = s
            .split(MODULE_LOGIC_PATH_SEPARATOR)
            .map(|x| Identifier::make(x, interner))
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

impl From<&ModuleLogicPath> for Document {
    fn from(value: &ModuleLogicPath) -> Document {
        value.to_document()
    }
}

impl ModuleLogicPath {
    pub fn to_document(&self) -> Document {
        combinators::intersperse(self.0.iter(), MODULE_LOGIC_PATH_SEPARATOR)
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
