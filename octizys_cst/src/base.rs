use crate::pretty::{indent, PrettyCST};
use derivative::Derivative;
use octizys_common::identifier::Identifier;
use octizys_common::module_logic_path::ModuleLogicPath;
use octizys_common::span::{Position, Span};
use octizys_pretty::combinators::{
    concat, empty, group, intersperse, nest, soft_break,
};
use octizys_pretty::document::Document;

use crate::{comments::CommentsInfo, pretty::PrettyCSTConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInfo {
    pub comments: CommentsInfo,
    pub span: Span,
}

impl TokenInfo {
    pub fn make(
        comments_info: CommentsInfo,
        start: usize,
        end: usize,
    ) -> TokenInfo {
        TokenInfo {
            comments: comments_info,
            span: Span {
                start: Position {
                    source_index: start,
                },
                end: Position { source_index: end },
            },
        }
    }

    pub fn consume_info(&mut self, _other: Self) -> () {
        todo!()
    }
    pub fn to_document(
        &self,
        configuration: PrettyCSTConfig,
        value: Document,
    ) -> Document {
        self.comments.to_document(configuration, value)
    }
}

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct Token<T> {
    pub value: T,
    #[derivative(PartialEq = "ignore")]
    pub info: TokenInfo,
}

impl<T> Token<T> {
    pub fn map<Out>(self, f: fn(T) -> Out) -> Token<Out> {
        Token {
            value: f(self.value),
            info: self.info,
        }
    }

    pub fn consume_token<U>(&mut self, other: Token<U>) -> U {
        self.info.consume_info(other.info);
        other.value
    }
}

impl<T> PrettyCST for Token<T>
where
    T: PrettyCST,
{
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        self.info
            .to_document(configuration, self.value.to_document(configuration))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorName {
    Interrogation,
    Exclamation,
    Hash,
    Comma,
    Colon,
    StatementEnd,
    Dot,
    ModuleSeparator,
    Minus,
    CompositionLeft,
    CompositionRight,
    Plus,
    Power,
    Star,
    Div,
    Module,
    ShiftLeft,
    ShiftRigth, //TODO: Add "<&>" = \ x y -> y $ x
    Map,
    MapConstRigth,
    MapConstLeft, //TODO: add <|> and <?>
    Appliative,
    ApplicativeRight,
    ApplicativeLeft,
    Equality,
    NotEqual,
    LessOrEqual,
    MoreOrEqual,
    LessThan,
    MoreThan,
    And,
    Or,
    ReverseAppliation,
    DollarApplication,
    Asignation,
    At,
    Pipe,
    RightArrow,
    LeftArrow,
    LambdaStart,
}

impl PrettyCST for OperatorName {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        let x = match self {
            OperatorName::Interrogation => "?",
            OperatorName::Exclamation => "!",
            OperatorName::Hash => "#",
            OperatorName::Comma => ",",
            OperatorName::Colon => ":",
            OperatorName::StatementEnd => ";",
            OperatorName::Dot => ".",
            OperatorName::ModuleSeparator => "::",
            OperatorName::Minus => "-",
            OperatorName::CompositionLeft => "<|",
            OperatorName::CompositionRight => "|>",
            OperatorName::Plus => "+",
            OperatorName::Power => "^",
            OperatorName::Star => "*",
            OperatorName::Div => "/",
            OperatorName::Module => "%",
            OperatorName::ShiftLeft => "<<",
            OperatorName::ShiftRigth => ">>", //TODO: Add "<&>" = \ x y -> y $ x
            OperatorName::Map => "<$>",
            OperatorName::MapConstRigth => "$>",
            OperatorName::MapConstLeft => "<$", //TODO: add <|> and <?>
            OperatorName::Appliative => "<*>",
            OperatorName::ApplicativeRight => "*>",
            OperatorName::ApplicativeLeft => "<*",
            OperatorName::Equality => "==",
            OperatorName::NotEqual => "!=",
            OperatorName::LessOrEqual => "<=",
            OperatorName::MoreOrEqual => "=>",
            OperatorName::LessThan => "<",
            OperatorName::MoreThan => ">",
            OperatorName::And => "&&",
            OperatorName::Or => "||",
            OperatorName::ReverseAppliation => "&",
            OperatorName::DollarApplication => "$",
            OperatorName::Asignation => "=",
            OperatorName::At => "&",
            OperatorName::Pipe => "|",
            OperatorName::RightArrow => "<-",
            OperatorName::LeftArrow => "->",
            OperatorName::LambdaStart => "\\",
        };
        x.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedVariable {
    path: ModuleLogicPath,
    name: Identifier,
}

impl PrettyCST for ImportedVariable {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.path.to_document(),
            "::".into(),
            (&self.name).into(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedOperator {
    path: ModuleLogicPath,
    name: Identifier,
}

impl PrettyCST for ImportedOperator {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.path.to_document(),
            "::".into(),
            (&self.name).into(),
        ])
    }
}
#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Between<T> {
    #[derivative(PartialEq = "ignore")]
    pub left: TokenInfo,
    #[derivative(PartialEq = "ignore")]
    pub right: TokenInfo,
    pub value: T,
}

pub enum Enclosures {
    Parens,
    Brackets,
    Braces,
}

impl<T> Between<T> {
    pub fn to_document(
        &self,
        configuration: PrettyCSTConfig,
        enclosure: Enclosures,
        to_document: impl FnOnce(&T, PrettyCSTConfig) -> Document,
    ) -> Document {
        let (start, end): (Document, Document) = match enclosure {
            Enclosures::Parens => ("(".into(), ")".into()),
            Enclosures::Braces => ("{".into(), "}".into()),
            Enclosures::Brackets => ("[".into(), "]".into()),
        };

        concat(vec![
            self.left.to_document(configuration, start),
            indent(configuration, to_document(&self.value, configuration)),
            self.right.to_document(configuration, end),
        ])
    }
}

/// The separator came before the item in the stream
#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TrailingListItem<T> {
    #[derivative(PartialEq = "ignore")]
    pub separator: TokenInfo,
    pub item: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemSeparator {
    Comma,
    Pipe,
    Arrow,
}
impl PrettyCST for ItemSeparator {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        match self {
            ItemSeparator::Comma => ",".into(),
            ItemSeparator::Pipe => "|".into(),
            ItemSeparator::Arrow => "->".into(),
        }
    }
}

impl<T> TrailingListItem<T>
where
    T: PrettyCST,
{
    pub fn to_document(
        &self,
        configuration: PrettyCSTConfig,
        separator: ItemSeparator,
    ) -> Document {
        let separator_doc = if configuration.leading_commas {
            soft_break() + separator.to_document(configuration)
        } else {
            separator.to_document(configuration) + soft_break()
        };
        concat(vec![
            self.separator.to_document(configuration, separator_doc),
            self.item.to_document(configuration),
        ])
    }
}

#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TrailingList<T> {
    pub first: T,
    pub items: Vec<TrailingListItem<T>>,
    #[derivative(PartialEq = "ignore")]
    pub trailing_sep: Option<TokenInfo>,
}

impl<T> TrailingList<T>
where
    T: PrettyCST,
{
    pub fn to_document(
        &self,
        configuration: PrettyCSTConfig,
        separator: ItemSeparator,
    ) -> Document {
        let separator_doc = if configuration.leading_commas {
            soft_break() + separator.to_document(configuration)
        } else {
            separator.to_document(configuration) + soft_break()
        };
        let trailing = match &self.trailing_sep {
            Some(separator_info) => {
                separator_info.to_document(configuration, separator_doc)
            }
            None => {
                if configuration.add_trailing_separator {
                    separator_doc
                } else {
                    empty()
                }
            }
        };
        concat(vec![
            self.first.to_document(configuration),
            concat(
                self.items
                    .iter()
                    .map(|x| x.to_document(configuration, separator))
                    .collect(),
            ),
            trailing,
        ])
    }
}

impl<T, ToInfo> From<(T, Vec<(ToInfo, T)>, Option<ToInfo>)> for TrailingList<T>
where
    ToInfo: Into<TokenInfo>,
{
    fn from(value: (T, Vec<(ToInfo, T)>, Option<ToInfo>)) -> TrailingList<T> {
        let items = value
            .1
            .into_iter()
            .map(|(separator, item)| TrailingListItem {
                separator: separator.into(),
                item,
            })
            .collect();
        let first = value.0;
        let trailing_sep = value.2;
        TrailingList {
            first,
            items,
            trailing_sep: trailing_sep.map(|x| x.into()),
        }
    }
}