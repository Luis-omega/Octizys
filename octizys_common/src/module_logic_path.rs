use std::rc::Rc;

use crate::error::{error_from_pretty, Error};
use crate::identifier::{Identifier, IdentifierError};
use crate::newtype::Newtype;
use octizys_pretty::combinators::{concat_sep_by, from_str};
use octizys_pretty::types::{Document, Pretty};

const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug)]
pub enum ModuleLogicPathError {
    NotIdentifier,
    //To be used by the TryFrom, not by the make smart constructor
    EmptyString,
}

impl Pretty for ModuleLogicPathError {
    fn to_document(&self) -> Document {
        match self {
            Self::NotIdentifier => {
                "The passed string contains a non valid Identifier component"
                    .into()
            }
            Self::EmptyString => {
                "Attempt to build from a empty vector or string".into()
            }
        }
    }
}

impl Into<Error> for ModuleLogicPathError {
    fn into(self) -> Error {
        error_from_pretty(&self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleLogicPath(Vec<Identifier>);

impl ModuleLogicPath {
    pub fn make(s: &str) -> Result<ModuleLogicPath, ModuleLogicPathError> {
        let v: Vec<Identifier> = s
            .split(MODULE_LOGIC_PATH_SEPARATOR)
            .map(|x| Identifier::make(x))
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

impl Pretty for ModuleLogicPath {
    fn to_document(&self) -> Document {
        concat_sep_by(
            self.0.iter().map(|x| x.to_document()).collect(),
            from_str("::"),
        )
    }
}

impl Newtype<ModuleLogicPath, Vec<Identifier>> for ModuleLogicPath {
    fn extract(self) -> Vec<Identifier> {
        self.0
    }
}

impl Into<String> for ModuleLogicPath {
    fn into(self) -> String {
        self.extract()
            .into_iter()
            .map(|x| x.extract().extract())
            .collect::<Vec<Rc<str>>>()
            .join(MODULE_LOGIC_PATH_SEPARATOR)
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
