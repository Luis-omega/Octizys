use crate::base::{
    Between, Enclosures, ItemSeparator, Token, TokenInfo, TrailingList,
};
use crate::pretty::{indent, PrettyCST, PrettyCSTConfig};
use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_pretty::combinators::*;
use octizys_pretty::document::Document;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Import {
    // import unqualified S.O.M.E.Path(a,b,c) as N.A.Me
    pub import: TokenInfo,
    pub unqualified: Option<TokenInfo>,
    pub module_path: Token<ModuleLogicPath>,
    pub import_list: Option<Between<TrailingList<Token<Identifier>>>>,
    // "as name"
    pub qualified_path: Option<(TokenInfo, Token<ModuleLogicPath>)>,
}

impl PrettyCST for Import {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        let import = self.import.to_document(configuration, "import".into());
        let unqualified: Document = match &self.unqualified {
            Some(x) => {
                soft_break()
                    + x.to_document(configuration, "unqualified".into())
            }
            None => empty(),
        };

        let path = soft_break() + self.module_path.to_document(configuration);
        let imports = match &self.import_list {
            Some(x) => {
                x.to_document(configuration, Enclosures::Parens, |l, c| {
                    l.to_document(c, ItemSeparator::Comma)
                })
            }
            None => empty(),
        };
        let _as = match &self.qualified_path {
            Some((ti, tm)) => {
                soft_break()
                    + concat(vec![
                        ti.to_document(configuration, "as".into()),
                        soft_break(),
                        tm.to_document(configuration),
                    ])
            }
            None => empty(),
        };
        import
            + indent(
                configuration,
                concat(vec![unqualified, path, imports, _as]),
            )
    }
}
