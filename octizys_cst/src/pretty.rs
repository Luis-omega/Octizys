use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_pretty::{
    combinators::{concat_iter, group, nest},
    document::Document,
};

#[derive(Debug, Copy, Clone)]
pub struct PrettyCSTConfig {
    //TODO: add option to sort imports alphabetically  (Ord<String>)
    // An even better options may be to sort first by package and then sort
    // alphabetically
    pub line_width: u16,
    pub indentation_deep: u16,
    pub leading_commas: bool,
    pub add_trailing_separator: bool,
    pub move_documentantion_before_object: bool,
    pub indent_comment_blocks: bool,
    pub separe_comments_by: u8,
    pub compact_comments: bool,
}

impl PrettyCSTConfig {
    pub fn new() -> PrettyCSTConfig {
        PrettyCSTConfig {
            line_width: 80,
            indentation_deep: 2,
            leading_commas: true,
            add_trailing_separator: true,
            move_documentantion_before_object: true,
            indent_comment_blocks: true,
            separe_comments_by: 1,
            compact_comments: true,
        }
    }
}

pub fn indent(configuration: PrettyCSTConfig, doc: Document) -> Document {
    group(nest(configuration.indentation_deep, doc))
}

pub trait PrettyCST {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document;
}

impl PrettyCST for Identifier {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        self.into()
    }
}

impl PrettyCST for ModuleLogicPath {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        self.into()
    }
}

impl<T> PrettyCST for Box<T>
where
    T: PrettyCST,
{
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        (**self).to_document(configuration)
    }
}

impl<T> PrettyCST for Vec<T>
where
    T: PrettyCST,
{
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        concat_iter(self.into_iter().map(|x| x.to_document(configuration)))
    }
}

impl PrettyCST for &str {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        //TODO:Correctly print strings escaping things
        todo!()
    }
}

impl PrettyCST for String {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        //TODO:Correctly print strings escaping things
        todo!()
    }
}

impl PrettyCST for char {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        //TODO:Correctly print strings escaping things
        todo!()
    }
}
