use octizys_common::identifier::Identifier;
use octizys_macros::Equivalence;

use crate::{
    base::{
        AliasKeyword, DataKeyword, NewTypeKeyword, Pipe, PublicKeyword,
        SemiColon, ShowableToken, Token, TokenInfo, TokenInfoWithPhantom,
        TrailingList,
    },
    comments::Comment,
    imports::Import,
    types::Type,
};

#[derive(Debug, Equivalence)]
pub struct Constructor {
    pub name: Token<Identifier>,
    pub type_: Option<Type>,
}

#[derive(Debug, Equivalence)]
pub struct DataConstructors {
    #[equivalence(ignore)]
    pub eq: TokenInfo,
    pub constructors: TrailingList<Constructor, Pipe>,
}

mod private {
    pub trait Sealed {}
}

pub trait TopTypeName: private::Sealed + ShowableToken {}

macro_rules! impl_data_keyword_type {
    ($name:ident) => {
        impl private::Sealed for $name {}
        impl TopTypeName for $name {}
    };
}

impl_data_keyword_type!(DataKeyword);
impl_data_keyword_type!(AliasKeyword);
impl_data_keyword_type!(NewTypeKeyword);

#[derive(Debug, Equivalence)]
#[equivalence(ignore=Keyword)]
pub struct TopTypeDefinitionLeft<Keyword>
where
    Keyword: TopTypeName,
{
    pub public: Option<TokenInfoWithPhantom<PublicKeyword>>,
    pub statement_keyword: TokenInfoWithPhantom<Keyword>,
    pub name: Token<Identifier>,
    pub variables: Vec<Token<Identifier>>,
}

#[derive(Debug, Equivalence)]
pub struct Data {
    pub left_part: TopTypeDefinitionLeft<DataKeyword>,
    pub constructors: Option<DataConstructors>,
}

#[derive(Debug, Equivalence)]
pub struct Alias {
    pub left_part: TopTypeDefinitionLeft<AliasKeyword>,
    #[equivalence(ignore)]
    pub eq: TokenInfo,
    pub type_: Type,
}

#[derive(Debug, Equivalence)]
pub struct NewType {
    pub left_part: TopTypeDefinitionLeft<NewTypeKeyword>,
    #[equivalence(ignore)]
    pub eq: TokenInfo,
    pub constructor: Constructor,
}

#[derive(Debug, Equivalence)]
pub enum TopItem {
    Data(Data),
    Alias(Alias),
    NewType(NewType),
}

#[derive(Debug, Equivalence)]
pub struct Top {
    pub imports: Option<TrailingList<Import, SemiColon>>,
    pub items: Option<TrailingList<TopItem, SemiColon>>,
    pub last_comment: Option<Comment>,
}
