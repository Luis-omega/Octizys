use crate::base::{
    Between, Comma, Parens, Token, TokenInfo, TokenInfoWithPhantom,
    TrailingList, UnqualifiedKeyword,
};
use octizys_common::{identifier::Identifier, logic_path::LogicPath};
use octizys_macros::Equivalence;

#[derive(Debug, PartialEq, Eq, Clone, Equivalence)]
pub struct AsPath {
    #[equivalence(ignore)]
    pub _as: TokenInfo,
    pub path: Token<LogicPath>,
}

#[derive(Debug, PartialEq, Eq, Clone, Equivalence)]
pub struct Import {
    // import unqualified S.O.M.E.Path(a,b,c) as N.A.Me
    #[equivalence(ignore)]
    pub import: TokenInfo,
    pub unqualified: Option<TokenInfoWithPhantom<UnqualifiedKeyword>>,
    pub logic_path: Token<LogicPath>,
    pub import_list:
        Option<Between<TrailingList<Token<Identifier>, Comma>, Parens>>,
    // "as name"
    pub qualified_path: Option<AsPath>,
}
