use crate::{
    base::{
        Between, Enclosures, ImportedVariable, ItemSeparator, Token, TokenInfo,
        TrailingList,
    },
    pretty::{PrettyCST, PrettyCSTConfig},
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
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        match self {
            PatternMatchRecordItem::OnlyVariable { variable } => {
                variable.to_document(configuration)
            }
            PatternMatchRecordItem::WithPattern {
                variable,
                separator,
                pattern,
            } => concat(vec![
                variable.to_document(configuration),
                separator.to_document(configuration, ":".into()),
                pattern.to_document(configuration),
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
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.variable.to_document(configuration),
            self.at.to_document(configuration, "@".into()),
            self.pattern.to_document(configuration),
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
    Tuple(Between<TrailingList<Box<PatternMatch>>>),
    Record(Between<TrailingList<PatternMatchRecordItem>>),
    Bind(PatternMatchBind),
    Application {
        start: Box<PatternMatch>,
        second: Box<PatternMatch>,
        remain: Vec<PatternMatch>,
    },
    Parens(Between<Box<PatternMatch>>),
}

impl PatternMatch {
    fn to_document_application_argument(
        &self,
        configuration: PrettyCSTConfig,
    ) -> Document {
        match &self {
            PatternMatch::Bind(b) => match *b.pattern {
                PatternMatch::Application { .. } => concat(vec![
                    "(".into(),
                    soft_break(),
                    self.to_document(configuration),
                    soft_break(),
                    ")".into(),
                ]),
                _ => self.to_document(configuration),
            },
            PatternMatch::Application { .. } => concat(vec![
                "(".into(),
                soft_break(),
                self.to_document(configuration),
                soft_break(),
                ")".into(),
            ]),
            _ => self.to_document(configuration),
        }
    }
}

impl PrettyCST for PatternMatch {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        match self {
            PatternMatch::LocalVariable(tok) => tok.to_document(configuration),
            PatternMatch::ImportedVariable(tok) => {
                tok.to_document(configuration)
            }
            PatternMatch::String(tok) => tok.to_document(configuration),
            PatternMatch::Char(tok) => tok.to_document(configuration),
            PatternMatch::AnonHole(info) => {
                info.to_document(configuration, "_".into())
            }
            PatternMatch::Tuple(t) => {
                t.to_document(configuration, Enclosures::Parens, |l, c| {
                    l.to_document(c, ItemSeparator::Comma)
                })
            }
            PatternMatch::Record(r) => {
                r.to_document(configuration, Enclosures::Braces, |l, c| {
                    l.to_document(c, ItemSeparator::Comma)
                })
            }
            PatternMatch::Bind(b) => b.to_document(configuration),
            PatternMatch::Application {
                start,
                second,
                remain,
            } => concat(vec![
                start.to_document_application_argument(configuration),
                second.to_document_application_argument(configuration),
                concat_iter(remain.into_iter().map(|x| {
                    x.to_document_application_argument(configuration)
                })),
            ]),
            PatternMatch::Parens(x) => {
                x.to_document(configuration, Enclosures::Parens, |l, c| {
                    l.to_document(c)
                })
            }
        }
    }
}
