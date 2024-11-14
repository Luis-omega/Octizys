use crate::{
    base::{Between, ImportedVariable, Token, TokenInfo, TrailingList},
    pretty::{Braces, Colon, Comma, Parens, PrettyCST, PrettyCSTContext},
};
use octizys_common::identifier::Identifier;
use octizys_pretty::{
    combinators::{concat, concat_iter, soft_break},
    document::Document,
};

#[derive(Debug)]
pub enum PatternMatchRecordItem {
    OnlyVariable {
        variable: Token<Identifier>,
    },
    WithPattern {
        variable: Token<Identifier>,
        separator: TokenInfo,
        pattern: Box<PatternMatch>,
    },
}

impl PrettyCST for PatternMatchRecordItem {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match self {
            PatternMatchRecordItem::OnlyVariable { variable } => {
                variable.to_document(context)
            }
            PatternMatchRecordItem::WithPattern {
                variable,
                separator,
                pattern,
            } => concat(vec![
                variable.to_document(context),
                separator.to_document(context, context.cache.colon),
                pattern.to_document(context),
            ]),
        }
    }
}

#[derive(Debug)]
pub struct PatternMatchBind {
    pub variable: Token<Identifier>,
    pub at: TokenInfo,
    pub pattern: Box<PatternMatch>,
}

impl PrettyCST for PatternMatchBind {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        concat(vec![
            self.variable.to_document(context),
            self.at.to_document(context, context.cache.at),
            self.pattern.to_document(context),
        ])
    }
}

#[derive(Debug)]
pub enum PatternMatch {
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    String(Token<String>),
    Char(Token<char>),
    AnonHole(TokenInfo),
    Tuple(Between<TrailingList<Box<PatternMatch>, Comma>, Parens>),
    Record(Between<TrailingList<PatternMatchRecordItem, Comma>, Braces>),
    Bind(PatternMatchBind),
    Application {
        start: Box<PatternMatch>,
        second: Box<PatternMatch>,
        remain: Vec<PatternMatch>,
    },
    Parens(Between<Box<PatternMatch>, Parens>),
}

impl PatternMatch {
    fn to_document_application_argument(
        &self,
        context: &PrettyCSTContext,
    ) -> Document {
        match &self {
            PatternMatch::Bind(b) => match *b.pattern {
                PatternMatch::Application { .. } => concat(vec![
                    context.cache.lparen.into(),
                    soft_break(),
                    self.to_document(context),
                    soft_break(),
                    context.cache.rparen.into(),
                ]),
                _ => self.to_document(context),
            },
            PatternMatch::Application { .. } => concat(vec![
                context.cache.lparen.into(),
                soft_break(),
                self.to_document(context),
                soft_break(),
                context.cache.rparen.into(),
            ]),
            _ => self.to_document(context),
        }
    }
}

impl PrettyCST for PatternMatch {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match self {
            PatternMatch::LocalVariable(tok) => tok.to_document(context),
            PatternMatch::ImportedVariable(tok) => tok.to_document(context),
            PatternMatch::String(tok) => tok.to_document(context),
            PatternMatch::Char(tok) => tok.to_document(context),
            PatternMatch::AnonHole(info) => {
                info.to_document(context, context.cache.underscore)
            }
            PatternMatch::Tuple(t) => t.to_document(context),
            PatternMatch::Record(r) => r.to_document(context),
            PatternMatch::Bind(b) => b.to_document(context),
            PatternMatch::Application {
                start,
                second,
                remain,
            } => concat(vec![
                start.to_document_application_argument(context),
                second.to_document_application_argument(context),
                concat_iter(
                    remain
                        .into_iter()
                        .map(|x| x.to_document_application_argument(context)),
                ),
            ]),
            PatternMatch::Parens(x) => x.to_document(context),
        }
    }
}
