use crate::base::{Between, Token, TokenInfo, TrailingList};
use crate::pretty::{indent, Comma, Parens, PrettyCST, PrettyCSTContext};
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
    pub import_list:
        Option<Between<TrailingList<Token<Identifier>, Comma>, Parens>>,
    // "as name"
    pub qualified_path: Option<(TokenInfo, Token<ModuleLogicPath>)>,
}

impl PrettyCST for Import {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let import = self.import.to_document(context, context.cache.import);
        let unqualified: Document = match &self.unqualified {
            Some(x) => {
                soft_break() + x.to_document(context, context.cache.unqualified)
            }
            None => empty(),
        };

        let path = soft_break() + self.module_path.to_document(context);
        let imports = match &self.import_list {
            Some(x) => x.to_document(context),
            None => empty(),
        };
        let _as = match &self.qualified_path {
            Some((ti, tm)) => {
                soft_break()
                    + concat(vec![
                        ti.to_document(context, context.cache._as),
                        indent(context, soft_break() + tm.to_document(context)),
                    ])
            }
            None => empty(),
        };
        import + indent(context, concat(vec![unqualified, path, imports, _as]))
    }
}
