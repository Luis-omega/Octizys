use std::marker::PhantomData;

use crate::comments::CommentsInfo;
use derivative::Derivative;
use octizys_common::identifier::Identifier;
use octizys_common::logic_path::LogicPath;
use octizys_common::span::{Position, Span};
use octizys_text_store::store::NonLineBreakStr;

mod private {
    /// We want it to create phantom types internally
    pub trait Sealed {}
}

/// Used to statically determine the kind of separator to use
/// and tell rust how to represent it as string.
/// Outside of this crate you can't create  new Separators
/// We have defined :
/// - [`Pipe`]
/// - [`Comma`]
/// - [`RightArrow`]
/// - [`Colon`]
/// - [`LogicPathSeparator`]
pub trait Separator: private::Sealed {
    fn to_str() -> NonLineBreakStr;
}

/// Macro used to generate the diverse separators,
/// all of them must implement the same things and the
/// only difference is the string they correspond to.
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
make_separator_type!(LogicPathSeparator, "::");

/// Used to statically determine the kind of delimiters to use
/// and tell rust how to represent it as string.
/// Outside of this crate you can't create  new Separators
/// We have defined:
/// - [`Parens`]
/// - [`Brackets`]
/// - [`Braces`]
pub trait Delimiters: private::Sealed {
    fn to_strs() -> (NonLineBreakStr, NonLineBreakStr);
}

/// Macro used to generate the diverse delimiters,
/// all of them must implement the same things and the
/// only difference is the string they correspond to.
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

/// Used to store the commentaries that belong to a token
/// and the region of the token in the source file.
/// See [`CommentsInfo`] for details about how the comments information
/// is stored.
/// See [`Span`] for details on how it is calculated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInfo {
    pub comments: CommentsInfo,
    pub span: Span,
}

impl TokenInfo {
    pub fn make(
        comments_info: CommentsInfo,
        start: Position,
        end: Position,
    ) -> TokenInfo {
        TokenInfo {
            comments: comments_info,
            span: Span { start, end },
        }
    }

    /// Merges the information of other comment into the
    /// information of this comment.
    /// For details look at [`CommentsInfo::consume_info`]
    /// and [`Span::add`].
    pub fn consume_info(&mut self, other: Self) -> () {
        self.comments.consume_info(other.comments);
        self.span = self.span + other.span;
    }
}

/// A new type around a token Info.
/// The principal use of this is for the grammar to type check as
/// separators and brackets.
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

/// A Token has two pieces, a value (the content) and information
/// like the comments around it and the source position.
/// We never build a [`Token`] for punctuation elements or keywords,
/// instead we build a [`TokenInfoWithPhantom`] with the appropriate
/// phantom type.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct Token<T> {
    pub value: T,
    #[derivative(PartialEq = "ignore")]
    pub info: TokenInfo,
}

impl<T> Token<T> {
    /// Creates a new token with the same info as the one provided
    /// and transform the content using the given function.
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

/// Any set of symbols that aren't identifiers, keywords or brackets, allowed
/// inside a expression (and maybe in the future to types).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Representation of a variable qualified by some path.
///
/// # Example
///
/// ```txt
/// core::main::path
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedVariable {
    pub path: LogicPath,
    pub name: Identifier,
}

/// Some structure surrounded by delimiters like `()` and `{}`
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

/// A item on a list of items separated by some separator like `,` or `|`.
/// This item contains the separation comma between itself and the
/// previous item.
///
/// Example:
///
/// ```txt
/// ,b
/// ```
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

/// A list of items separated by some separator like `,` and `|`
/// that allow optional final separator without item.
///
/// Example
///
/// ```txt
/// a , b, c, d,
/// ```
///
/// The last `,` is optional.
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

impl From<TrailingList<Token<Identifier>, LogicPathSeparator>>
    for Token<LogicPath>
{
    fn from(
        value: TrailingList<Token<Identifier>, LogicPathSeparator>,
    ) -> Self {
        let mut info = value.first.info;
        let mut acc = vec![value.first.value];
        for i in value.items {
            acc.push(i.item.value);
            info.comments.consume_info(i.item.info.comments)
        }
        // Safe to unwrap since acc has at least one element
        Token {
            value: acc.try_into().unwrap(),
            info,
        }
    }
}
