use std::{collections::HashSet, rc::Rc};

use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_pretty::types::NoLineBreaksString;

#[derive(Debug)]
struct StringArena {
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

#[derive(Debug)]
pub struct Position {
    source_index: u64,
}

#[derive(Debug)]
pub struct Span {
    start: Position,
    end: Position,
}

#[derive(Debug)]
pub struct CommentLineContent {
    content: NoLineBreaksString,
}

impl CommentLineContent {
    pub fn make(value: Rc<str>) -> Result<Self, String> {
        let content = NoLineBreaksString::make(&value)?;
        Ok(CommentLineContent { content })
    }
}

#[derive(Debug)]
pub enum CommentKind {
    Documentation,
    NonDocumentation,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum LineCommentStart {
    // --
    DoubleHypen,
    // //
    DoubleSlash,
}

#[derive(Debug)]
pub struct CommentBlock {
    kind: CommentKind,
    brace: CommentBraceKind,
    content: Vec<CommentLineContent>,
    span: Span,
}

#[derive(Debug)]
pub struct CommentLine {
    kind: CommentKind,
    start: LineCommentStart,
    content: CommentLineContent,
    span: Span,
}
impl CommentLine {
    fn trim(s: &str) -> &str {
        let start = 2;
        let end = s.len() - 1;
        &s[start..end]
    }

    pub fn make(
        arena: &mut StringArena,
        kind: CommentKind,
        start: LineCommentStart,
        full_text: &str,
        start_pos: Position,
        end_pos: Position,
    ) -> Result<Self, String> {
        let cut_text: &str = CommentLine::trim(full_text);
        let text_rc: Rc<str> = arena.insert(cut_text);
        let content = CommentLineContent::make(text_rc)?;
        Ok(CommentLine {
            kind,
            start,
            content,
            span: Span {
                start: start_pos,
                end: end_pos,
            },
        })
    }
}

#[derive(Debug)]
pub enum Comment {
    Line(CommentLine),
    Block(CommentBlock),
}

#[derive(Debug)]
pub struct CommentsInfo {
    before: Vec<Comment>,
    after: Option<Comment>,
}

#[derive(Debug)]
pub struct TokenInfo {
    pub comments: CommentsInfo,
    pub span: Span,
}

impl TokenInfo {
    pub fn make(
        start: u64,
        end: u64,
        comments_info: CommentsInfo,
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

#[derive(Debug)]
//TODO: Smart constructor, it only allows combinations
//of +-*?<>=/@~#"
pub struct OperatorName(String);

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
pub struct NamedItem<T> {
    pub name: T,
    pub comments: CommentsInfo,
    pub Span: Span,
}

#[derive(Debug)]
pub struct Between<T> {
    pub left: TokenInfo,
    pub right: TokenInfo,
    pub value: T,
}

pub struct SepByItem<T> {
    pub item: T,
    pub separator: TokenInfo,
}

pub struct SepBy<T> {
    pub items: Vec<SepByItem<T>>,
    pub final_item: T,
    pub final_comma: Option<TokenInfo>,
}

#[derive(Debug)]
pub enum ImportItem {
    Variable(NamedItem<Identifier>),
    Operator(NamedItem<OperatorName>),
    TypeOperator(NamedItem<OperatorName>),
}

#[derive(Debug)]
pub struct Import {
    module_path: NamedItem<ModuleLogicPath>,
    unqualified_allowed: Option<TokenInfo>,
    import_list: Option<Between<Vec<ImportItem>>>,
    qualified_name: Option<Identifier>,
}

#[derive(Debug)]
pub enum PatternMatch {
    Variable(NamedItem<Identifier>),
    AnonHole(NamedItem<()>),
    NamedHole(NamedItem<Identifier>),
    Application(NamedItem<Identifier>, Vec<PatternMatch>),
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
    name: NamedItem<OperatorName>,
}

#[derive(Debug)]
struct LambdaExpression {
    variable: NamedItem<Identifier>,
    expression: Box<Expression>,
}

#[derive(Debug)]
struct ApplicationExpression {
    start: Box<Expression>,
    Remain: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Variable(NamedItem<NamedVariable>),
    Let(Let),
    Case(Case),
    BinaryOperator(BinaryOperator),
    Lambda(LambdaExpression),
    Application(ApplicationExpression),
}
