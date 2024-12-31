use octizys_common::identifier::Identifier;
use octizys_macros::Equivalence;

use crate::{
    base::{Pipe, SemiColon, Token, TokenInfo, TrailingList},
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

#[derive(Debug, Equivalence)]
pub struct Data {
    #[equivalence(ignore)]
    //TODO: add Phantom for public
    pub public: Option<TokenInfo>,
    #[equivalence(ignore)]
    pub data: TokenInfo,
    pub name: Token<Identifier>,
    pub variables: Vec<Token<Identifier>>,
    pub constructors: Option<DataConstructors>,
}

#[derive(Debug, Equivalence)]
pub enum TopItem {
    Data(Data),
}

#[derive(Debug, Equivalence)]
pub struct Top {
    pub imports: Option<TrailingList<Import, SemiColon>>,
    pub items: Option<TrailingList<TopItem, SemiColon>>,
    pub last_comment: Option<Comment>,
}
