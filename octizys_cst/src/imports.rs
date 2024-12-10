use crate::base::{Between, Comma, Parens, Token, TokenInfo, TrailingList};
use octizys_common::{identifier::Identifier, logic_path::LogicPath};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Import {
    // import unqualified S.O.M.E.Path(a,b,c) as N.A.Me
    pub import: TokenInfo,
    pub unqualified: Option<TokenInfo>,
    pub logic_path: Token<LogicPath>,
    pub import_list:
        Option<Between<TrailingList<Token<Identifier>, Comma>, Parens>>,
    // "as name"
    pub qualified_path: Option<(TokenInfo, Token<LogicPath>)>,
}
