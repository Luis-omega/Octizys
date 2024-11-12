use octizys_common::{
    identifier::Identifier,
    module_logic_path::ModuleLogicPath,
    span::{Position, Span},
};
use octizys_pretty::{
    combinators::{
        self, concat, empty, group, hard_break, intersperse, nest, soft_break,
        text,
    },
    document::Document,
};

#[derive(Debug, Copy, Clone)]
pub struct PrettyCSTConfig {
    //TODO: add option to sort imports alphabetically  (Ord<String>)
    // An even better options may be to sort first by package and then sort
    // alphabetically
    pub line_width: u16,
    pub indentation_deep: u16,
    pub leading_commas: bool,
    pub add_trailing_separator: bool,
    pub move_documentantion_before_object: bool,
    pub indent_comment_blocks: bool,
    pub separe_comments_by: u8,
    pub compact_comments: bool,
}

impl PrettyCSTConfig {
    pub fn new() -> PrettyCSTConfig {
        PrettyCSTConfig {
            line_width: 80,
            indentation_deep: 2,
            leading_commas: true,
            add_trailing_separator: true,
            move_documentantion_before_object: true,
            indent_comment_blocks: true,
            separe_comments_by: 1,
            compact_comments: true,
        }
    }
}

pub trait PrettyCST {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document;
}

impl PrettyCST for ModuleLogicPath {
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        self.into()
    }
}

impl<T> PrettyCST for Box<T>
where
    T: PrettyCST,
{
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        (*self).to_document(configuration)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLineContent {
    pub content: String,
}

impl CommentLineContent {
    pub fn make(value: &str) -> Result<Self, String> {
        if value.find("\n").is_some() {
            return Err(String::from(
                "CommentLines souldn't containt line breaks!",
            ));
        }
        let content = String::from(value);
        Ok(CommentLineContent { content })
    }

    pub fn decompose(value: &str) -> Vec<Self> {
        value
            .split("\n")
            .map(|x| CommentLineContent {
                content: x.to_string(),
            })
            .collect()
    }
}

impl PrettyCST for CommentLineContent {
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        text(&self.content)
    }
}

impl Into<Document> for CommentLineContent {
    fn into(self) -> Document {
        text(&self.content)
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
}

impl PrettyCST for CommentBlock {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        let (block_start, block_end) = match self.brace {
            CommentBraceKind::Brace0 => match self.kind {
                CommentKind::Documentation => ("{- |", "-}"),
                CommentKind::NonDocumentation => ("{-", "-}"),
            },
            CommentBraceKind::Brace1 => match self.kind {
                CommentKind::Documentation => ("{-- |", "--}"),
                CommentKind::NonDocumentation => ("{--", "--}"),
            },
            CommentBraceKind::Brace2 => match self.kind {
                CommentKind::Documentation => ("{--- |", "---}"),
                CommentKind::NonDocumentation => ("{---", "---}"),
            },
            CommentBraceKind::Brace3 => match self.kind {
                CommentKind::Documentation => ("{---- |", "----}"),
                CommentKind::NonDocumentation => ("{----", "----}"),
            },
        };
        let content_raw = intersperse(self.content, hard_break());

        let content = if configuration.indent_comment_blocks {
            group(nest(
                configuration.indentation_deep,
                concat(vec![hard_break(), content_raw]),
            )) + hard_break()
        } else {
            content_raw
        };

        concat(vec![
            block_start.into(),
            content,
            block_end.into(),
            hard_break(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLine {
    pub kind: CommentKind,
    pub start: LineCommentStart,
    pub content: CommentLineContent,
    pub span: Span,
}

impl PrettyCST for CommentLine {
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        let line_start = match self.start {
            LineCommentStart::DoubleHypen => match self.kind {
                CommentKind::Documentation => "-- |",
                CommentKind::NonDocumentation => "--",
            },
            LineCommentStart::DoubleSlash => match self.kind {
                CommentKind::Documentation => "// |",
                CommentKind::NonDocumentation => "//",
            },
        };
        concat(vec![line_start.into(), self.content.into(), hard_break()])
    }
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

impl PrettyCST for Comment {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        match self {
            Comment::Line(l) => l.to_document(configuration),
            Comment::Block(l) => l.to_document(configuration),
        }
    }
}

impl Into<Comment> for CommentLine {
    fn into(self) -> Comment {
        Comment::Line(self)
    }
}

impl Into<Comment> for CommentBlock {
    fn into(self) -> Comment {
        Comment::Block(self)
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

    pub fn move_after_to_before(mut self) -> Self {
        match self.after {
            Some(c) => {
                self.before.push(c);
                self.after = None;
                return self;
            }
            None => return self,
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

    /// Take a CommentsInfo and transform all the contiguous occurrences
    /// of comments of the same type in a single block
    /// Example: Multiple documentation lines became a  documentation block
    /// Example: Multiple NonDocumentation lines became a block
    pub fn compact_comments(mut self) -> Self {
        todo!()
    }

    pub fn to_document(
        mut self,
        configuration: PrettyCSTConfig,
        doc: Document,
    ) -> Document {
        if configuration.move_documentantion_before_object {
            self = self.move_after_to_before();
        }
        if configuration.compact_comments {
            self = self.compact_comments();
        }
        let separe_by = usize::from(configuration.separe_comments_by);
        concat(vec![
            intersperse(
                self.before
                    .into_iter()
                    .map(|x| x.to_document(configuration)),
                combinators::repeat(hard_break(), separe_by),
            ),
            doc,
            self.after.map_or(empty(), |x| x.to_document(configuration)),
        ])
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
    pub fn to_document(
        self,
        configuration: PrettyCSTConfig,
        value: Document,
    ) -> Document {
        self.comments.to_document(configuration, value)
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

impl<T> PrettyCST for Token<T>
where
    T: PrettyCST,
{
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
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
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
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

type LocalVariable = Token<Identifier>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportedVariable {
    path: ModuleLogicPath,
    name: Identifier,
}

impl PrettyCST for ImportedVariable {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.path.to_document(configuration),
            "::".into(),
            self.name.into(),
        ])
    }
}

#[derive(Debug)]
pub struct Between<T> {
    pub left: TokenInfo,
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
        self,
        configuration: PrettyCSTConfig,
        enclosure: Enclosures,
        to_document: impl FnOnce(T, PrettyCSTConfig) -> Document,
    ) -> Document {
        let (start, end): (Document, Document) = match enclosure {
            Enclosures::Parens => ("(".into(), ")".into()),
            Enclosures::Braces => ("{".into(), "}".into()),
            Enclosures::Brackets => ("[".into(), "]".into()),
        };

        concat(vec![
            self.left.to_document(configuration, start),
            group(nest(
                configuration.indentation_deep,
                to_document(self.value, configuration),
            )),
            self.right.to_document(configuration, end),
        ])
    }
}

/// The separator came before the item in the stream
#[derive(Debug)]
pub struct TrailingListItem<T> {
    pub separator: TokenInfo,
    pub item: T,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemSeparator {
    Comma,
    Pipe,
}
impl PrettyCST for ItemSeparator {
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        match self {
            ItemSeparator::Comma => ",".into(),
            ItemSeparator::Pipe => "|".into(),
        }
    }
}

impl<T> TrailingListItem<T>
where
    T: PrettyCST,
{
    pub fn to_document(
        self,
        configuration: PrettyCSTConfig,
        separator: ItemSeparator,
    ) -> Document {
        concat(vec![
            self.separator.to_document(
                configuration,
                separator.to_document(configuration),
            ),
            self.item.to_document(configuration),
        ])
    }
}

#[derive(Debug)]
pub struct TrailingList<T> {
    pub first: T,
    pub items: Vec<TrailingListItem<T>>,
    pub trailing_sep: Option<TokenInfo>,
}

impl<T> TrailingList<T>
where
    T: PrettyCST,
{
    pub fn to_document(
        self,
        configuration: PrettyCSTConfig,
        separator: ItemSeparator,
    ) -> Document {
        let trailing = match self.trailing_sep {
            Some(separator_info) => separator_info.to_document(
                configuration,
                separator.to_document(configuration),
            ),
            None => {
                if configuration.add_trailing_separator {
                    separator.to_document(configuration)
                } else {
                    combinators::empty()
                }
            }
        };
        concat(vec![
            self.first.to_document(configuration),
            concat(
                self.items
                    .into_iter()
                    .map(|x| x.to_document(configuration, separator))
                    .collect(),
            ),
            trailing,
        ])
    }
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

impl PrettyCST for ImportItem {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        match self {
            Self::Variable(t) => {
                t.info.to_document(configuration, t.value.into())
            }
            Self::Operator(t) => t.to_document(configuration),
            Self::TypeOperator(ti, t) => concat(vec![
                ti.to_document(configuration, "type ".into()),
                t.to_document(configuration),
            ]),
        }
    }
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

impl PrettyCST for Import {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        let import = self.import.to_document(configuration, "import ".into());
        let unqualified: Document = self.unqualified.map_or(empty(), |x| {
            x.to_document(configuration, "unqualified ".into())
        });
        let path = self.module_path.to_document(configuration);
        let imports = match self.import_list {
            Some(x) => {
                x.to_document(configuration, Enclosures::Parens, |l, c| {
                    l.to_document(c, ItemSeparator::Comma)
                })
            }
            None => "()".into(),
        };
        let _as = match self.qualified_path {
            Some((ti, tm)) => concat(vec![
                ti.to_document(configuration, "as ".into()),
                tm.to_document(configuration),
            ]),
            None => empty(),
        };
        // TODO: add soft_break between items (but be careful that two optional items don't overlap
        // their soft_breaks if they don't exists!
        concat(vec![import, unqualified, path, imports, _as])
    }
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
    Char,
    String,
}

impl PrettyCST for TypeBase {
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        match self {
            TypeBase::U8 => "U8".into(),
            TypeBase::U16 => "U16".into(),
            TypeBase::U32 => "U32".into(),
            TypeBase::U64 => "U64".into(),
            TypeBase::I8 => "I8".into(),
            TypeBase::I16 => "I16".into(),
            TypeBase::I32 => "I32".into(),
            TypeBase::I64 => "I64".into(),
            TypeBase::F32 => "F32".into(),
            TypeBase::F64 => "F64".into(),
            TypeBase::Char => "Char".into(),
            TypeBase::String => "String".into(),
        }
    }
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

impl PrettyCST for TypeRecordItem {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.variable
                .info
                .to_document(configuration, self.variable.value.into()),
            self.separator.to_document(configuration, ":".into()),
            self.expression.to_document(configuration),
        ])
    }
}

fn pretty_betwee_trailing<T: PrettyCST>(
    between: Between<TrailingList<T>>,
    configuration: PrettyCSTConfig,
    sep: ItemSeparator,
    enclosure: Enclosures,
) -> Document {
    between.to_document(configuration, enclosure, |l, c| l.to_document(c, sep))
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

impl PrettyCST for Type {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        match self {
            Type::Base(token) => token.to_document(configuration),
            Type::LocalVariable(token) => {
                token.info.to_document(configuration, token.value.into())
            }
            Type::ImportedVariable(token) => token.to_document(configuration),
            Type::Tuple(between) => pretty_betwee_trailing(
                between,
                configuration,
                ItemSeparator::Comma,
                Enclosures::Parens,
            ),
            Type::Record(between) => pretty_betwee_trailing(
                between,
                configuration,
                ItemSeparator::Comma,
                Enclosures::Braces,
            ),
            Type::Parens(between) => between.to_document(
                configuration,
                Enclosures::Parens,
                |t, c| t.to_document(c),
            ),
            Type::Application {
                start,
                second,
                remain,
            } =>
            //TODO: Add parens here as needed
            {
                group(nest(
                    configuration.indentation_deep,
                    concat(vec![
                        start.to_document(configuration),
                        soft_break(),
                        second.to_document(configuration),
                        soft_break(),
                        intersperse(
                            remain
                                .into_iter()
                                .map(|x| x.to_document(configuration)),
                            soft_break(),
                        ),
                    ]),
                ))
            }
            Type::Arrow { first, remain } => empty(),
            _ => todo!(),
        }
    }
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
