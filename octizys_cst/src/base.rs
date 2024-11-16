use std::marker::PhantomData;

use crate::comments::CommentsInfo;
use derivative::Derivative;
use octizys_common::identifier::Identifier;
use octizys_common::module_logic_path::ModuleLogicPath;
use octizys_common::span::{Position, Span};
use octizys_text_store::store::NonLineBreakStr;

mod private {
    pub trait Sealed {}
}

pub trait Separator: private::Sealed {
    fn to_str() -> NonLineBreakStr;
}

macro_rules! make_separator_type {
    ($name:ident, $value:expr ) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $name {}
        impl private::Sealed for $name {}
        impl Separator for $name {
            fn to_str() -> NonLineBreakStr {
                NonLineBreakStr::new($value)
            }
        }
    };
}

make_separator_type!(Pipe, "|");
make_separator_type!(Comma, ",");
make_separator_type!(RightArrow, "->");
make_separator_type!(Colon, ":");

pub trait Delimiters: private::Sealed {
    fn to_strs() -> (NonLineBreakStr, NonLineBreakStr);
}

macro_rules! make_delimiter_type {
    ($name:ident, $left:expr,$right:expr ) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $name {}
        impl private::Sealed for $name {}
        impl Delimiters for $name {
            fn to_strs() -> (NonLineBreakStr, NonLineBreakStr) {
                (NonLineBreakStr::new($left), NonLineBreakStr::new($right))
            }
        }
    };
}

make_delimiter_type!(Parens, "(", ")");
make_delimiter_type!(Brackets, "[", "]");
make_delimiter_type!(Braces, "{", "}");

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedVariable {
    pub path: ModuleLogicPath,
    pub name: Identifier,
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
