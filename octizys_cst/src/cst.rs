use octizys_pretty::types::NoLineBreaksString;

#[derive(Debug)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

#[derive(Debug)]
// TODO: use a owned Cow instead of String
pub struct CommentLineContent {
    content: NoLineBreaksString,
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
pub struct CommentBlock<'a> {
    kind: CommentKind,
    brace: CommentBraceKind,
    content: Vec<CommentLineContent<'a>>,
    position: Position,
}

#[derive(Debug)]
pub struct CommentLine<'a> {
    kind: CommentKind,
    start: LineCommentStart,
    content: CommentLineContent<'a>,
    position: Position,
}

#[derive(Debug)]
pub enum Comment<'comment> {
    Line(CommentLine<'comment>),
    Block(CommentBlock<'comment>),
}

#[derive(Debug)]
pub struct CommentsInfo<'comments> {
    before: Vec<Comment<'comments>>,
    after: Option<Comment<'comments>>,
}

#[derive(Debug)]
pub enum TokenKind {
    Let,
    In,
    Equal,
    Colon,
    Semicolon,
    Dot,
    Identifier(Identifier),
    Case,
    Pipe,
}

#[derive(Debug)]
pub struct Token<'comments> {
    pub kind: TokenKind,
    pub comments: CommentsInfo<'comments>,
    pub position: Position,
}

#[derive(Debug)]
//TODO: Smart constructor, in should
//disallow numbers/spaces/line_breaks
//at the begining of the string
pub struct Identifier(String);

#[derive(Debug)]
//TODO: Smart constructor, it only allows combinations
//of +-*?<>=/@~#"
pub struct OperatorName(String);

pub struct ModulePrefix {
    prefix: Vec<Identifier>,
}

pub enum NamedVariable {
    SingleVariable(Identifier),
    SingleOperator(OperatorName),
    PrefixedVariable {
        prefix: ModulePrefix,
        name: Identifier,
    },
    PrefixedOperator {
        prefix: ModulePrefix,
        name: Identifier,
    },
}

#[derive(Debug)]
pub struct NamedItem<'comments, T> {
    pub name: T,
    pub comments: CommentsInfo<'comments>,
    pub position: Position,
}

#[derive(Debug)]
pub struct Between<'comments, T> {
    pub left: Token<'comments>,
    pub right: Token<'comments>,
    pub value: T,
}

pub struct SepBy<T> {
    pub item: T,
    pub separator: Token,
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
    equal: Token,
    value: Expression,
    semicolon: Token,
}

#[derive(Debug)]
pub struct Let {
    let_: Token,
    bindings: Vec<LetBinding>,
    in_: Token,
    expression: Expression,
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
    arrow: Token,
    expression: Expression,
    end: Token,
}

#[derive(Debug)]
pub struct Case {
    case: Token,
    expression: Expression,
    cases: Between<Vec<OneCase>>,
}

#[derive(Debug)]
pub struct BinaryOperator {
    left: Expression,
    right: Expression,
    name: NamedItem<OperatorName>,
}

#[derive(Debug)]
pub enum Expression {}
