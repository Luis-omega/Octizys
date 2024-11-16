use crate::base::{Between, Comma, Parens, Token, TokenInfo, TrailingList};
use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Import {
    // import unqualified S.O.M.E.Path(a,b,c) as N.A.Me
    pub import: TokenInfo,
    pub unqualified: Option<TokenInfo>,
    pub module_path: Token<ModuleLogicPath>,
    pub import_list:
        Option<Between<TrailingList<Token<Identifier>, Comma>, Parens>>,
    // "as name"
    pub qualified_path: Option<(TokenInfo, Token<ModuleLogicPath>)>,
}
