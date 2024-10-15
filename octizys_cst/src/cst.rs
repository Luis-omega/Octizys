use std::{collections::HashSet, ops::Add, rc::Rc};

use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_pretty::{
    combinators::text,
    types::{Document, NoLineBreaksString},
};

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

impl Add for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let start =
            std::cmp::max(self.start.source_index, rhs.start.source_index);
        let end = std::cmp::max(self.end.source_index, rhs.end.source_index);
        (start, end).into()
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

    pub fn to_document(&self, config: u8) -> Document {
        return text(self.content.clone());
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

impl From<CommentBraceKind> for (&str, &str) {
    fn from(value: CommentBraceKind) -> Self {
        match value {
            CommentBraceKind::Brace0 => ("{-", "-}"),
            CommentBraceKind::Brace1 => ("{--", "--}"),
            CommentBraceKind::Brace2 => ("{---", "---}"),
            CommentBraceKind::Brace3 => ("{----", "----}"),
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
    pub kind: CommentKind,
    pub brace: CommentBraceKind,
    pub content: Vec<CommentLineContent>,
    pub span: Span,
}
impl CommentBlock {
    pub fn make(
        kind: CommentKind,
        brace: CommentBraceKind,
        full_text: &str,
        start_pos: Position,
        end_pos: Position,
    ) -> Self {
        let content = CommentLineContent::decompose(full_text);
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

    pub fn to_document(&self, configuration: u8) -> Document {
        let line_start = match self.brace {
            CommentBraceKind::Brace0=> match self.kind {
               CommentKind::Documentation=>"{- |" 
               CommentKind::Documentation=>"{-  ",
            } 
        }; 
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

impl Comment {
    pub fn get_span(self) -> Span {
        match self {
            Self::Line(CommentLine { span, .. }) => span,
            Self::Block(CommentBlock { span, .. }) => span,
        }
    }
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

    pub fn extend<T>(&mut self, remmain: T) -> ()
    where
        T: Iterator<Item = Comment>,
    {
        self.before.extend(remmain)
    }

    pub fn push(&mut self, new: Comment) -> () {
        self.before.push(new)
    }

    pub fn move_after_to_before(self) -> Self {
        let CommentsInfo { mut before, after } = self;
        match after {
            Some(c) => {
                before.push(c);
                CommentsInfo {
                    before,
                    after: None,
                }
            }
            None => CommentsInfo { before, after },
        }
    }
    //TODO: this is wrong, check the docummentation on grammar about
    //the expected behaviour
    pub fn absorb_info(&mut self, other: CommentsInfo) -> () {
        let CommentsInfo { before, after } = other;
        self.extend(before.into_iter());
        match after {
            Some(c) => self.push(c),
            None => (),
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

    pub fn consume_info(&mut self, other: Self) -> () {
        todo!()
    }
}

#[derive(Debug)]
pub struct Token<T> {
    pub value: T,
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

type LocalVariable = Token<Identifier>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedVariable {
    prefix: ModuleLogicPath,
    name: Identifier,
}

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
        operator: OperatorName,
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
pub enum TypeBase {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct TypeRecordItem {
    pub variable: Token<Identifier>,
    pub separator: TokenInfo,
    // This is needed as TrailingList stores a T
    // otherwise we can drop the Box, maybe put
    // the box in the TrailingList?
    pub expression: Box<Type>,
}

#[derive(Debug)]
pub enum Type {
    Base(Token<TypeBase>),
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    Tuple(Between<TrailingList<Box<Type>>>),
    Record(Between<TrailingList<TypeRecordItem>>),
    Parens(Between<Box<Type>>),
    Application {
        start: Box<Type>,
        second: Box<Type>,
        remain: Vec<Type>,
    },
    Arrow {
        first: Box<Type>,
        remain: Vec<(TokenInfo, Type)>,
    },
    Scheme {
        forall: TokenInfo,
        first_variable: Token<Identifier>,
        remain_variables: Vec<Token<Identifier>>,
        dot: TokenInfo,
        expression: Box<Type>,
    },
}

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

#[derive(Debug)]
pub struct PatternMatchBind {
    pub variable: Token<Identifier>,
    pub at: TokenInfo,
    pub pattern: Box<PatternMatch>,
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

#[derive(Debug)]
pub struct LetBinding {
    pub pattern: PatternMatch,
    pub equal: TokenInfo,
    pub value: Expression,
    pub semicolon: TokenInfo,
}

#[derive(Debug)]
pub struct Let {
    pub let_: TokenInfo,
    pub bindings: Vec<LetBinding>,
    pub in_: TokenInfo,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct CaseItem {
    pub pattern: PatternMatch,
    pub arrow: TokenInfo,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct Case {
    pub case: TokenInfo,
    pub expression: Box<Expression>,
    pub of: TokenInfo,
    pub cases: Between<TrailingList<CaseItem>>,
}

#[derive(Debug)]
pub struct BinaryOperator {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub name: Token<OperatorName>,
}

#[derive(Debug)]
pub struct LambdaExpression {
    pub variable: Token<Identifier>,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct ApplicationExpression {
    pub start: Box<Expression>,
    pub remain: Vec<Expression>,
}

#[derive(Debug)]
pub enum ExpressionRecordItem {
    SingleVariable {
        variable: Token<Identifier>,
    },
    Assignation {
        variable: Token<Identifier>,
        equal: TokenInfo,
        expression: Box<Expression>,
    },
}

#[derive(Debug)]
pub struct ExpressionSelector {
    pub expression: Box<Expression>,
    pub accessor: Token<Identifier>,
}

#[derive(Debug)]
pub enum Expression {
    String(Token<String>),
    Character(Token<char>),
    //TODO: make the lexer return the right type instead of string?
    //The main problem is with floats and uints, they must be in
    //the range or we should issue a warning or error about
    //maximum literal
    Uint(Token<String>),
    UFloat(Token<String>),
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    NamedHole(Token<u64>),
    Tuple(Between<TrailingList<Box<Expression>>>),
    Record(Between<TrailingList<ExpressionRecordItem>>),
    Case(Case),
    Parens(Between<Box<Expression>>),
    Selector(ExpressionSelector),
    Interrogation {
        expression: Box<Expression>,
        symbol: TokenInfo,
    },
    TypeArgument {
        at: TokenInfo,
        _type: Type,
    },

    Let(Let),
    BinaryOperator(BinaryOperator),
    Lambda(LambdaExpression),
    Application(ApplicationExpression),
}

impl Expression {
    pub fn selector_from_args(
        e: Box<Self>,
        s: Token<Identifier>,
        symbol: Option<TokenInfo>,
    ) -> Self {
        let selector = Expression::Selector(ExpressionSelector {
            expression: e,
            accessor: s,
        });
        match symbol {
            Some(info) => Expression::Interrogation {
                expression: Box::new(selector),
                symbol: info,
            },
            None => selector,
        }
    }
}
