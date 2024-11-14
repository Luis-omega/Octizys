use std::marker::PhantomData;

use crate::pretty::{Delimiters, PrettyCST, Separator};
use derivative::Derivative;
use octizys_common::identifier::Identifier;
use octizys_common::module_logic_path::ModuleLogicPath;
use octizys_common::span::{Position, Span};
use octizys_pretty::combinators::{
    concat, empty, empty_break, group, nest, soft_break,
};
use octizys_pretty::document::Document;

use crate::{comments::CommentsInfo, pretty::PrettyCSTContext};

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
    pub fn to_document<ToDoc: Into<Document>>(
        &self,
        context: &PrettyCSTContext,
        value: ToDoc,
    ) -> Document {
        self.comments.to_document(context, value.into())
    }
}

pub struct TokenInfoWithPhantom<P> {
    info: TokenInfo,
    _phantom: PhantomData<P>,
}

impl<P> From<TokenInfoWithPhantom<P>> for TokenInfo {
    fn from(value: TokenInfoWithPhantom<P>) -> Self {
        value.info
    }
}

impl<P> From<TokenInfo> for TokenInfoWithPhantom<P> {
    fn from(value: TokenInfo) -> Self {
        TokenInfoWithPhantom {
            info: value,
            _phantom: Default::default(),
        }
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
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        self.info
            .to_document(context, self.value.to_document(context))
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
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let x = match self {
            OperatorName::Interrogation => context.cache.interrogation,
            OperatorName::Exclamation => context.cache.exclamation,
            OperatorName::Hash => context.cache.hash,
            OperatorName::Comma => context.cache.comma,
            OperatorName::Colon => context.cache.colon,
            OperatorName::StatementEnd => context.cache.semicolon,
            OperatorName::Dot => context.cache.dot,
            OperatorName::ModuleSeparator => context.cache.module_separator,
            OperatorName::Minus => context.cache.hypen,
            OperatorName::CompositionLeft => context.cache.composition_left,
            OperatorName::CompositionRight => context.cache.composition_right,
            OperatorName::Plus => context.cache.plus,
            OperatorName::Power => context.cache.power,
            OperatorName::Star => context.cache.star,
            OperatorName::Div => context.cache.div,
            OperatorName::Module => context.cache.percentage,
            OperatorName::ShiftLeft => context.cache.shift_left,
            OperatorName::ShiftRigth => context.cache.shift_rigth,
            OperatorName::Map => context.cache.map,
            OperatorName::MapConstRigth => context.cache.map_const_rigth,
            OperatorName::MapConstLeft => context.cache.map_const_left,
            OperatorName::Appliative => context.cache.appliative,
            OperatorName::ApplicativeRight => context.cache.applicative_right,
            OperatorName::ApplicativeLeft => context.cache.applicative_left,
            OperatorName::Equality => context.cache.asignation,
            OperatorName::NotEqual => context.cache.not_equal,
            OperatorName::LessOrEqual => context.cache.less_or_equal,
            OperatorName::MoreOrEqual => context.cache.more_or_equal,
            OperatorName::LessThan => context.cache.less_than,
            OperatorName::MoreThan => context.cache.more_than,
            OperatorName::And => context.cache.and,
            OperatorName::Or => context.cache.or,
            OperatorName::ReverseAppliation => context.cache.reverse_appliation,
            OperatorName::DollarApplication => context.cache.dollar_application,
            OperatorName::Asignation => context.cache.asignation,
            OperatorName::At => context.cache.at,
            OperatorName::Pipe => context.cache.pipe,
            OperatorName::RightArrow => context.cache.right_arrow,
            OperatorName::LeftArrow => context.cache.left_arrow,
            OperatorName::LambdaStart => context.cache.lambda_start,
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
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        concat(vec![
            self.path.to_document(),
            context.cache.module_separator.into(),
            (&self.name).into(),
        ])
    }
}

#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Between<T, Enclosure>
where
    Enclosure: Delimiters,
{
    #[derivative(PartialEq = "ignore")]
    pub left: TokenInfo,
    #[derivative(PartialEq = "ignore")]
    pub right: TokenInfo,
    pub value: T,
    pub _enclosure_phantom: PhantomData<Enclosure>,
}

impl<T, Enclosure> PrettyCST for Between<T, Enclosure>
where
    Enclosure: Delimiters,
    T: PrettyCST,
{
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let (start, end) = Enclosure::to_documents(context);
        //TODO: after the refactor, lookup if there are comments
        //inside the span of this between, if they are,
        //enforce the ")" to end in a separate line always
        //otherwise keep this.
        self.left.to_document(context, start)
            + group(concat(vec![
                nest(
                    context.configuration.indentation_deep,
                    empty_break() + self.value.to_document(context),
                ),
                empty_break(),
                self.right.to_document(context, end),
            ]))
    }
}

/// The separator came before the item in the stream
#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TrailingListItem<T, SeparatorPhantom>
where
    SeparatorPhantom: Separator,
{
    #[derivative(PartialEq = "ignore")]
    pub separator: TokenInfo,
    pub item: T,
    pub _phantom_separator: PhantomData<SeparatorPhantom>,
}

impl<T, SeparatorPhantom> PrettyCST for TrailingListItem<T, SeparatorPhantom>
where
    T: PrettyCST,
    SeparatorPhantom: Separator,
{
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let separator_doc = if context.configuration.leading_commas {
            soft_break() + SeparatorPhantom::to_document(context)
        } else {
            SeparatorPhantom::to_document(context) + soft_break()
        };
        concat(vec![
            self.separator.to_document(context, separator_doc),
            self.item.to_document(context),
        ])
    }
}

#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq, Eq)]
pub struct TrailingList<T, SeparatorPhantom>
where
    SeparatorPhantom: Separator,
{
    pub first: T,
    pub items: Vec<TrailingListItem<T, SeparatorPhantom>>,
    #[derivative(PartialEq = "ignore")]
    pub trailing_sep: Option<TokenInfo>,
}

impl<T, SeparatorPhantom> PrettyCST for TrailingList<T, SeparatorPhantom>
where
    T: PrettyCST,
    SeparatorPhantom: Separator,
{
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let separator_doc = if context.configuration.leading_commas {
            soft_break() + SeparatorPhantom::to_document(context)
        } else {
            SeparatorPhantom::to_document(context)
        };
        let trailing = match &self.trailing_sep {
            Some(separator_info) => {
                separator_info.to_document(context, separator_doc)
            }
            None => {
                if context.configuration.add_trailing_separator {
                    separator_doc
                } else {
                    empty()
                }
            }
        };
        concat(vec![
            self.first.to_document(context),
            concat(self.items.iter().map(|x| x.to_document(context)).collect()),
            trailing,
        ])
    }
}

impl<T, ToInfo, SeparatorPhantom> From<(T, Vec<(ToInfo, T)>, Option<ToInfo>)>
    for TrailingList<T, SeparatorPhantom>
where
    ToInfo: Into<TokenInfo>,
    SeparatorPhantom: Separator,
{
    fn from(
        value: (T, Vec<(ToInfo, T)>, Option<ToInfo>),
    ) -> TrailingList<T, SeparatorPhantom> {
        let items = value
            .1
            .into_iter()
            .map(|(separator, item)| TrailingListItem {
                separator: separator.into(),
                item,
                _phantom_separator: Default::default(),
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
