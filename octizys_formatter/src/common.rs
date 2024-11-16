use octizys_common::identifier::{Identifier, IdentifierError};
use octizys_common::module_logic_path::{
    ModuleLogicPath, ModuleLogicPathError, MODULE_LOGIC_PATH_SEPARATOR,
};
use octizys_common::span::Span;
use octizys_pretty::combinators::{self, external_text};
use octizys_pretty::document::Document;
use octizys_pretty::store::NonLineBreakStr;

use crate::to_document::ToDocument;

impl<T> ToDocument<T> for Identifier {
    fn to_document(&self, _configuration: &T) -> Document {
        let (symbol, len) = self.as_tuple();
        Document::from_symbol_and_len(symbol, len)
    }
}

impl<T> ToDocument<T> for IdentifierError {
    fn to_document(&self, _configuration: &T) -> Document {
        match self {
            IdentifierError::ContainsInvalidCodePoint(s)=> {
                    Document::static_str(NonLineBreakStr::new("The passed string is not a valid identifier, it contains invalid characters: "))
                    + external_text(&s.replace("\n","\\n"))
            }
            IdentifierError::EmptyIdentifier => {
                Document::static_str(NonLineBreakStr::new("The passed string is not a valid identifier, it is seen as a empty string"))
            }
        }
    }
}

impl<T> ToDocument<T> for ModuleLogicPathError {
    fn to_document(&self, _configuration: &T) -> Document {
        match self {
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

impl<T> ToDocument<T> for ModuleLogicPath {
    fn to_document(&self, _configuration: &T) -> Document {
        combinators::intersperse(
            (<&Vec<Identifier>>::from(self))
                .iter()
                .map(|x| x.to_document(_configuration)),
            MODULE_LOGIC_PATH_SEPARATOR,
        )
    }
}

impl<T> ToDocument<T> for Span {
    fn to_document(&self, _configuration: &T) -> Document {
        combinators::external_text(&format!(
            "start: {}, end: {}",
            self.start.source_index, self.end.source_index
        ))
    }
}
