use crate::error::{error_from_pretty, Error};
use crate::identifier::{Identifier, IdentifierError};
use crate::newtype::Newtype;
use octizys_pretty::types::{Document, NoLineBreaksString, Pretty};

const MODULE_LOGIC_PATH_SEPARATOR: &str = "::";

#[derive(Debug)]
pub enum ModuleLogicPathError {
    NotIdentifier,
}

impl<'a, 'b> Pretty<'a, 'b> for ModuleLogicPathError {
    fn to_document(&'a self) -> Document<'b> {
        match self {
            Self::NotIdentifier => {
                "The passed string contains a non valid Identifier component"
                    .into()
            }
        }
    }
}

impl<'e> Into<Error<'e>> for ModuleLogicPathError {
    fn into(self) -> Error<'e> {
        error_from_pretty(&self)
    }
}

#[derive(Debug)]
pub struct ModuleLogicPath<'a>(Vec<Identifier<'a>>);

impl<'a> ModuleLogicPath<'a> {
    pub fn make(
        s: &'a str,
    ) -> Result<ModuleLogicPath<'a>, ModuleLogicPathError> {
        let v: Vec<Identifier> = s
            .split(MODULE_LOGIC_PATH_SEPARATOR)
            .map(|x| Identifier::make(x))
            .collect::<Result<Vec<Identifier>, IdentifierError>>()
            .map_err(|_x| ModuleLogicPathError::NotIdentifier)?;
        Ok(ModuleLogicPath(v))
    }
}

impl<'a> Newtype<ModuleLogicPath<'a>, Vec<Identifier<'a>>>
    for ModuleLogicPath<'a>
{
    fn extract(self) -> Vec<Identifier<'a>> {
        self.0
    }
}

impl<'a> Into<String> for ModuleLogicPath<'a> {
    fn into(self) -> String {
        self.extract()
            .into_iter()
            .map(|x| x.extract().extract())
            .collect::<Vec<&str>>()
            .join(MODULE_LOGIC_PATH_SEPARATOR)
    }
}

impl<'a> Into<ModuleLogicPath<'a>> for Vec<Identifier<'a>> {
    fn into(self) -> ModuleLogicPath<'a> {
        ModuleLogicPath(self)
    }
}

impl<'a> Into<Vec<Identifier<'a>>> for ModuleLogicPath<'a> {
    fn into(self) -> Vec<Identifier<'a>> {
        self.0
    }
}
