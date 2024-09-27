use std::rc::Rc;

use crate::error::{error_from_pretty, Error};
use crate::identifier::{Identifier, IdentifierError};
use crate::newtype::Newtype;
use octizys_pretty::types::{Document, Pretty};

const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug)]
pub enum ModuleLogicPathError {
    NotIdentifier,
}

impl Pretty for ModuleLogicPathError {
    fn to_document(&self) -> Document {
        match self {
            Self::NotIdentifier => {
                "The passed string contains a non valid Identifier component"
                    .into()
            }
        }
    }
}

impl Into<Error> for ModuleLogicPathError {
    fn into(self) -> Error {
        error_from_pretty(&self)
    }
}

#[derive(Debug)]
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

impl Into<ModuleLogicPath> for Vec<Identifier> {
    fn into(self) -> ModuleLogicPath {
        ModuleLogicPath(self)
    }
}

impl Into<Vec<Identifier>> for ModuleLogicPath {
    fn into(self) -> Vec<Identifier> {
        self.0
    }
}
