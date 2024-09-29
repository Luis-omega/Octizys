use std::{collections::HashSet, rc::Rc};

use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_pretty::types::NoLineBreaksString;

#[derive(Debug)]
pub struct StringArena {
    arena: HashSet<Rc<str>>,
}

impl StringArena {
    pub fn new() -> Self {
        StringArena {
            arena: HashSet::new(),
        }
    }

    pub fn insert(self: &mut Self, value: &str) -> Rc<str> {
        let rc: Rc<str> = Rc::from(value);
        //TODO: use get_or_insert when it became stable
        match self.arena.get(&rc) {
            Some(old_rc) => old_rc.clone(),
            None => {
                let new_rc = rc.clone();
                self.arena.insert(rc);
                new_rc
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub source_index: usize,
}

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        Position {
            source_index: value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Span {
            start: value.0.into(),
            end: value.1.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLineContent {
    pub content: NoLineBreaksString,
}

impl CommentLineContent {
    pub fn make(value: Rc<str>) -> Result<Self, String> {
        let content = NoLineBreaksString::make(&value)?;
        Ok(CommentLineContent { content })
    }

    pub fn decompose(value: &str) -> Vec<Self> {
        NoLineBreaksString::decompose(value)
            .into_iter()
            .map(|x| CommentLineContent { content: x })
            .collect()
    }
}

impl From<CommentLineContent> for Rc<str> {
    fn from(value: CommentLineContent) -> Self {
        value.content.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Documentation,
    NonDocumentation,
}

impl From<CommentKind> for &'static str {
    fn from(value: CommentKind) -> Self {
        match value {
            CommentKind::Documentation => " |",
            _ => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentBraceKind {
    // "{- asdf -}"
    Brace0,
    // "{-- asdf --}"
    Brace1,
    // "{--- asdf ---}"
    Brace2,
    // "{---- asdf ----}"
    Brace3,
}
impl CommentBraceKind {
    pub fn len(self) -> usize {
        match self {
            Self::Brace0 => 2,
            Self::Brace1 => 3,
            Self::Brace2 => 4,
            Self::Brace3 => 5,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineCommentStart {
    // --
    DoubleHypen,
    // //
    DoubleSlash,
}

impl From<LineCommentStart> for char {
    fn from(value: LineCommentStart) -> Self {
        match value {
            LineCommentStart::DoubleHypen => '-',
            LineCommentStart::DoubleSlash => '/',
        }
    }
}

impl From<LineCommentStart> for &'static str {
    fn from(value: LineCommentStart) -> Self {
        match value {
            LineCommentStart::DoubleHypen => "--",
            LineCommentStart::DoubleSlash => "//",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentBlock {
    kind: CommentKind,
    brace: CommentBraceKind,
    content: Vec<CommentLineContent>,
    span: Span,
}
impl CommentBlock {
    fn trim(s: &str, kind: CommentKind, brace: CommentBraceKind) -> &str {
        let comment_bracket_len = brace.len();
        let start = comment_bracket_len
            + match kind {
                CommentKind::Documentation => 2,
                CommentKind::NonDocumentation => 0,
            };
        let end = s.len() - comment_bracket_len;
        &s[start..end]
    }

    pub fn make(
        kind: CommentKind,
        brace: CommentBraceKind,
        full_text: &str,
        start_pos: Position,
        end_pos: Position,
    ) -> Self {
        let cut_text: &str = CommentBlock::trim(full_text, kind, brace);
        let content = CommentLineContent::decompose(cut_text);
        CommentBlock {
            kind,
            brace,
            content,
            span: Span {
                start: start_pos,
                end: end_pos,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLine {
    pub kind: CommentKind,
    pub start: LineCommentStart,
    pub content: CommentLineContent,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
    Line(CommentLine),
    Block(CommentBlock),
}

impl From<CommentLine> for Comment {
    fn from(value: CommentLine) -> Self {
        Comment::Line(value)
    }
}

impl From<CommentBlock> for Comment {
    fn from(value: CommentBlock) -> Self {
        Comment::Block(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentsInfo {
    pub before: Vec<Comment>,
    pub after: Option<Comment>,
}

impl CommentsInfo {
    pub fn empty() -> Self {
        CommentsInfo {
            before: vec![],
            after: None,
        }
    }
}

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
}

#[derive(Debug)]
pub struct Token<T> {
    pub value: T,
    pub info: TokenInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
//TODO: Smart constructor, it only allows combinations
//of +-*?<>=/@~#"
pub struct OperatorName(pub String);

#[derive(Debug)]
pub enum NamedVariable {
    SingleVariable(Identifier),
    SingleOperator(OperatorName),
    PrefixedVariable {
        prefix: ModuleLogicPath,
        name: Identifier,
    },
    PrefixedOperator {
        prefix: ModuleLogicPath,
        name: Identifier,
    },
}

#[derive(Debug)]
pub struct Between<T> {
    pub left: TokenInfo,
    pub right: TokenInfo,
    pub value: T,
}

#[derive(Debug)]
pub struct TrailingListItem<T> {
    pub separator: TokenInfo,
    pub item: T,
}

#[derive(Debug)]
pub struct TrailingList<T> {
    pub first: T,
    pub items: Vec<TrailingListItem<T>>,
    pub trailing_sep: Option<TokenInfo>,
}

impl<T, ToInfo> Into<TrailingList<T>> for (T, Vec<(ToInfo, T)>, Option<ToInfo>)
where
    ToInfo: Into<TokenInfo>,
{
    fn into(self) -> TrailingList<T> {
        let items = self
            .1
            .into_iter()
            .map(|(separator, item)| TrailingListItem {
                separator: separator.into(),
                item,
            })
            .collect();
        let first = self.0;
        let trailing_sep = self.2;
        TrailingList {
            first,
            items,
            trailing_sep: trailing_sep.map(|x| x.into()),
        }
    }
}

#[derive(Debug)]
pub enum ImportItem {
    Variable(Token<Identifier>),
    Operator(Token<OperatorName>),
    TypeOperator(TokenInfo, Token<OperatorName>),
}

#[derive(Debug)]
pub struct Import {
    // import unqualified S.O.M.E.Path(a,b,c) as N.A.Me
    pub import: TokenInfo,
    pub unqualified: Option<TokenInfo>,
    pub module_path: Token<ModuleLogicPath>,
    pub import_list: Option<Between<TrailingList<ImportItem>>>,
    // "as name"
    pub qualified_path: Option<(TokenInfo, Token<ModuleLogicPath>)>,
}

#[derive(Debug)]
pub enum PatternMatch {
    Variable(Token<Identifier>),
    AnonHole(Token<()>),
    NamedHole(Token<Identifier>),
    Application(Token<Identifier>, Vec<PatternMatch>),
    Parens(Between<Box<PatternMatch>>),
}

#[derive(Debug)]
pub struct LetBinding {
    pattern: PatternMatch,
    equal: TokenInfo,
    value: Expression,
    semicolon: TokenInfo,
}

#[derive(Debug)]
pub struct Let {
    let_: TokenInfo,
    bindings: Vec<LetBinding>,
    in_: TokenInfo,
    expression: Box<Expression>,
}

//TODO: Whats a good closer for a case expression?
//Current options are :
//Use brackets
//Use `with` and `end`
//Allow both previous
//Use a punctuation symbol
//(but then we are forced to do :  let x = match _  |  => _ MatchEnd LetEnd )
//In the other side a nested match can mix the `|` (making the grammar ambiguous)
#[derive(Debug)]
pub struct OneCase {
    pattern: PatternMatch,
    arrow: TokenInfo,
    expression: Expression,
    end: TokenInfo,
}

#[derive(Debug)]
pub struct Case {
    case: TokenInfo,
    expression: Box<Expression>,
    of: TokenInfo,
    cases: Between<Vec<OneCase>>,
}

#[derive(Debug)]
pub struct BinaryOperator {
    left: Box<Expression>,
    right: Box<Expression>,
    name: Token<OperatorName>,
}

#[derive(Debug)]
struct LambdaExpression {
    variable: Token<Identifier>,
    expression: Box<Expression>,
}

#[derive(Debug)]
struct ApplicationExpression {
    start: Box<Expression>,
    remain: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Variable(Token<NamedVariable>),
    Let(Let),
    Case(Case),
    BinaryOperator(BinaryOperator),
    Lambda(LambdaExpression),
    Application(ApplicationExpression),
}
