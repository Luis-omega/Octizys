use crate::base::{
    Between, Braces, Comma, ImportedVariable, Parens, RightArrow, Token,
    TokenInfo, TrailingList, TrailingListItem,
};
use derivative::Derivative;
use octizys_common::identifier::Identifier;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeBase {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    String,
}

#[derive(Debug, Derivative)]
pub struct TypeRecordItem {
    pub variable: Token<Identifier>,
    pub separator: TokenInfo,
    // This is needed as TrailingList stores a T
    // otherwise we can drop the Box, maybe put
    // the box in the TrailingList?
    pub expression: Box<Type>,
}

#[derive(Debug)]
pub enum Type {
    Base(Token<TypeBase>),
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    Tuple(Between<TrailingList<Box<Type>, Comma>, Parens>),
    Record(Between<TrailingList<TypeRecordItem, Comma>, Braces>),
    Parens(Between<Box<Type>, Parens>),
    Application {
        start: Box<Type>,
        second: Box<Type>,
        remain: Vec<Type>,
    },
    Arrow {
        first: Box<Type>,
        remain: Vec<TrailingListItem<Type, RightArrow>>,
    },
    Scheme {
        forall: TokenInfo,
        first_variable: Token<Identifier>,
        remain_variables: Vec<Token<Identifier>>,
        dot: TokenInfo,
        expression: Box<Type>,
    },
}

impl Type {
    ///This function tell the pretty printer if the type needs to be
    ///surrounded by parentheses if the type is a argument in a
    ///application.
    pub fn need_parens_application(&self) -> bool {
        match self {
            Type::Base(_) => false,
            Type::LocalVariable(_) => false,
            Type::ImportedVariable(_) => false,
            Type::Tuple(_) => false,
            Type::Record(_) => false,
            Type::Parens(_) => false,
            Type::Application { .. } => true,
            Type::Arrow { .. } => true,
            Type::Scheme { .. } => true,
        }
    }

    ///This function tell the pretty printer if the type needs to be
    ///surrounded by parentheses if the type is a argument in a
    ///arrow.
    pub fn need_parens_arrow(&self) -> bool {
        match self {
            Type::Base(_) => false,
            Type::LocalVariable(_) => false,
            Type::ImportedVariable(_) => false,
            Type::Tuple(_) => false,
            Type::Record(_) => false,
            Type::Parens(_) => false,
            Type::Application { .. } => false,
            Type::Arrow { .. } => true,
            Type::Scheme { .. } => true,
        }
    }
}
