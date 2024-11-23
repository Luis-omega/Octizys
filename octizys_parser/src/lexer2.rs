use lalrpop_util::ParseError;
use logos::{Lexer, Logos, SpannedIter};
use octizys_common::span::{Position, Span};
use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_cst::base;
use octizys_cst::comments::{
    CommentBlock, CommentBraceKind, CommentKind, CommentLine,
    CommentLineContent, CommentsInfo, LineCommentStart,
};
use octizys_cst::{base::TokenInfo, comments::Comment};
use octizys_text_store::store::Store;

#[derive(Debug)]
pub struct LogosLexerContext<'src> {
    store: &'src mut Store,
}

use paste::paste;

impl<'input> LogosLexerContext<'input> {
    pub fn new(store: &'input mut Store) -> Self {
        LogosLexerContext { store }
    }
}
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\r]+")]
#[logos(extras = &'s mut Store)]
pub enum LogosToken {
    #[token("?")]
    Interrogation,
    #[token("!")]
    Exclamation,
    #[token("#")]
    Hash,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    StatementEnd,
    #[token(".")]
    Dot,
    #[token("::")]
    ModuleSeparator,
    #[token("-")]
    Minus,
    #[token("|>")]
    CompositionRight,
    #[token("<|")]
    CompositionLeft,
    #[token("+")]
    Plus,
    #[token("^")]
    Power,
    #[token("*")]
    Star,
    #[token("/")]
    Div,
    #[token("%")]
    Module,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRigth,
    #[token("<$>")]
    Map,
    #[token("$>")]
    MapConstRigth,
    #[token("<$")]
    MapConstLeft,
    #[token("<*>")]
    Appliative,
    #[token("*>")]
    ApplicativeRight,
    #[token("<*")]
    ApplicativeLeft,
    #[token("==")]
    Equality,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessOrEqual,
    #[token(">=")]
    MoreOrEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    MoreThan,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("&")]
    ReverseAppliation,
    #[token("$")]
    DollarApplication,
    #[token("=")]
    Asignation,
    #[token("@")]
    At,
    #[token("|")]
    Pipe,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("->")]
    RightArrow,
    #[token("<-")]
    LeftArrow,
    #[token("\\")]
    LambdaStart,
    #[token("let")]
    Let,
    #[token("in")]
    In,
    #[token("case")]
    Case,
    #[token("of")]
    Of,
    #[token("import")]
    Import,
    #[token("data")]
    Data,
    #[token("newtype")]
    Newtype,
    #[token("class")]
    Class,
    #[token("instance")]
    Instance,
    #[token("public")]
    Public,
    #[token("alias")]
    Alias,
    #[token("as")]
    As,
    #[token("unqualified")]
    Unqualified,
    #[token("forall")]
    Forall,
    #[token("type")]
    Type,
    #[token("U8")]
    U8,
    #[token("U16")]
    U16,
    #[token("U32")]
    U32,
    #[token("U64")]
    U64,
    #[token("I8")]
    I8,
    #[token("I16")]
    I16,
    #[token("I32")]
    I32,
    #[token("I64")]
    I64,
    #[token("F32")]
    F32,
    #[token("F64")]
    F64,
    #[regex("Char")]
    CharType,
    #[regex("String")]
    StringType,
    #[regex("--[^\n]*\n", |lex| make_line_comment(lex,CommentKind::NonDocumentation,LineCommentStart::DoubleHypen),priority=5)]
    #[regex(r"-- \|[^\n]*\n", |lex| make_line_comment(lex,CommentKind::Documentation,LineCommentStart::DoubleHypen), priority = 10)]
    #[regex("//[^\n]*\n", |lex| make_line_comment(lex,CommentKind::NonDocumentation,LineCommentStart::DoubleSlash),priority=5)]
    #[regex(r"// \|[^\n]*\n", |lex| make_line_comment(lex,CommentKind::Documentation,LineCommentStart::DoubleSlash),priority=10)]
    LineComment(CommentLine),
    #[regex(r"\{-([^-]|-[^-])*-\}", |lex| make_block_comment(lex,CommentKind::Documentation,CommentBraceKind::Brace0),priority=10)]
    #[regex(r"\{--([^-]|-[^-])*--\}", |lex| make_block_comment(lex,CommentKind::Documentation,CommentBraceKind::Brace1),priority=10)]
    #[regex(r"\{---([^-]|-[^-])*---\}", |lex| make_block_comment(lex,CommentKind::Documentation,CommentBraceKind::Brace2),priority=10)]
    #[regex(r"\{----([^-]|-[^-])*----\}", |lex| make_block_comment(lex,CommentKind::Documentation,CommentBraceKind::Brace3),priority=10)]
    BlockComment(CommentBlock),
    //TODO: support for multiline_string
    /*
    #[regex(r"\"([^"]|(\\\\)\\\")\"")]
    StringLiteral(String),
    //TODO:FIXME: real char support
    #[regex(r#"'[^\n']|(\\')'"#)]
    CharacterLiteral(String),
    */
    #[regex(r#"0[0_]*|[1-9][0-9_]+"#, |x| String::from(x.slice()))]
    UintLiteral(String),
    //#[regex(r"{UINT_STRING}\\.UINT_STRING((e|E)UINT_STRING)?")]
    #[regex(r"(0[0_]*|[1-9][0-9_]+)\.(0[0_]*|[1-9][0-9_]+)((e|E)(0[0_]*|[1-9][0-9_]+))"
        , |x| String::from(x.slice()))]
    UFloatLiteral(String),
    #[regex(r"_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*"
        , make_identifier)]
    Identifier(Identifier),
    #[regex(r"`_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*`"
        , make_identifier)]
    InfixIdentifier(Identifier),
    #[regex(r"\._*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*"
        , make_identifier)]
    Selector(Identifier),
    #[regex("_")]
    AnonHole,
    //TODO:FIXME: Handle the UIntError
    //Probably we don't want to do this, instead use a String and emit a warning
    #[regex("_(0[0_]*|[1-9][0-9_]+)", |lexer| lexer.slice().parse::<u64>().ok())]
    NamedHole(u64),
    #[regex(r"(_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*::)+_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*"
        , make_module_logic_path)]
    ModuleLogicPath(ModuleLogicPath),
}

const STRING_REGEX: &'static str = r#"\"([^"\n]|((\\\\)*\\"))*""#;

pub fn logos_span_to_span(lexer: &Lexer<LogosToken>) -> Span {
    let span = lexer.span();
    Span {
        start: Position {
            source_index: span.start,
        },
        end: Position {
            source_index: span.end,
        },
    }
}

pub fn range_to_span(r: std::ops::Range<usize>) -> Span {
    Span {
        start: Position {
            source_index: r.start,
        },
        end: Position {
            source_index: r.end,
        },
    }
}

pub fn make_line_comment(
    lexer: &mut Lexer<LogosToken>,
    kind: CommentKind,
    start: LineCommentStart,
) -> Option<CommentLine> {
    let content =
        CommentLineContent::make_register(lexer.slice(), &mut lexer.extras)?;
    Some(CommentLine {
        kind,
        start,
        content,
        span: logos_span_to_span(&lexer),
    })
}

pub fn make_block_comment(
    lexer: &mut Lexer<LogosToken>,
    kind: CommentKind,
    brace: CommentBraceKind,
) -> CommentBlock {
    let span = lexer.span();

    CommentBlock::make(
        kind,
        brace,
        lexer.slice(),
        Position {
            source_index: span.start,
        },
        Position {
            source_index: span.end,
        },
        &mut lexer.extras,
    )
}

pub fn make_identifier(lexer: &mut Lexer<LogosToken>) -> Option<Identifier> {
    //TODO: HAndle the error
    Identifier::make(lexer.slice(), &mut lexer.extras).ok()
}

pub fn make_module_logic_path(
    lexer: &mut Lexer<LogosToken>,
) -> Option<ModuleLogicPath> {
    ModuleLogicPath::make(lexer.slice(), &mut lexer.extras).ok()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Interrogation(TokenInfo),
    Exclamation(TokenInfo),
    Hash(TokenInfo),
    Comma(TokenInfo),
    Colon(TokenInfo),
    StatementEnd(TokenInfo),
    Dot(TokenInfo),
    ModuleSeparator(TokenInfo),
    Minus(TokenInfo),
    CompositionRight(TokenInfo),
    CompositionLeft(TokenInfo),
    Plus(TokenInfo),
    Power(TokenInfo),
    Star(TokenInfo),
    Div(TokenInfo),
    Module(TokenInfo),
    ShiftLeft(TokenInfo),
    ShiftRigth(TokenInfo),
    Map(TokenInfo),
    MapConstRigth(TokenInfo),
    MapConstLeft(TokenInfo),
    Appliative(TokenInfo),
    ApplicativeRight(TokenInfo),
    ApplicativeLeft(TokenInfo),
    Equality(TokenInfo),
    NotEqual(TokenInfo),
    LessOrEqual(TokenInfo),
    MoreOrEqual(TokenInfo),
    LessThan(TokenInfo),
    MoreThan(TokenInfo),
    And(TokenInfo),
    Or(TokenInfo),
    ReverseAppliation(TokenInfo),
    DollarApplication(TokenInfo),
    Asignation(TokenInfo),
    At(TokenInfo),
    Pipe(TokenInfo),
    LParen(TokenInfo),
    RParen(TokenInfo),
    LBracket(TokenInfo),
    RBracket(TokenInfo),
    LBrace(TokenInfo),
    RBrace(TokenInfo),
    RightArrow(TokenInfo),
    LeftArrow(TokenInfo),
    LambdaStart(TokenInfo),
    Let(TokenInfo),
    In(TokenInfo),
    Case(TokenInfo),
    Of(TokenInfo),
    Import(TokenInfo),
    Data(TokenInfo),
    Newtype(TokenInfo),
    Class(TokenInfo),
    Instance(TokenInfo),
    Public(TokenInfo),
    Alias(TokenInfo),
    As(TokenInfo),
    Unqualified(TokenInfo),
    Forall(TokenInfo),
    Type(TokenInfo),
    U8(TokenInfo),
    U16(TokenInfo),
    U32(TokenInfo),
    U64(TokenInfo),
    I8(TokenInfo),
    I16(TokenInfo),
    I32(TokenInfo),
    I64(TokenInfo),
    F32(TokenInfo),
    F64(TokenInfo),
    CharType(TokenInfo),
    StringType(TokenInfo),
    Comment(TokenInfo, Comment),
    StringLiteral(TokenInfo, String),
    CharacterLiteral(TokenInfo, String),
    UintLiteral(TokenInfo, String),
    UFloatLiteral(TokenInfo, String),
    Identifier(TokenInfo, Identifier),
    InfixIdentifier(TokenInfo, Identifier),
    Selector(TokenInfo, Identifier),
    AnonHole(TokenInfo),
    NamedHole(TokenInfo, u64),
    ModuleLogicPath(TokenInfo, ModuleLogicPath),
    LastComments(TokenInfo, Vec<Comment>),
}

impl From<Token> for TokenInfo {
    fn from(value: Token) -> TokenInfo {
        match value {
            Token::Interrogation(info) => (info),
            Token::Exclamation(info) => (info),
            Token::Hash(info) => (info),
            Token::Comma(info) => (info),
            Token::Colon(info) => (info),
            Token::StatementEnd(info) => (info),
            Token::Dot(info) => (info),
            Token::ModuleSeparator(info) => (info),
            Token::Minus(info) => (info),
            Token::CompositionRight(info) => (info),
            Token::CompositionLeft(info) => (info),
            Token::Plus(info) => (info),
            Token::Power(info) => (info),
            Token::Star(info) => (info),
            Token::Div(info) => (info),
            Token::Module(info) => (info),
            Token::ShiftLeft(info) => (info),
            Token::ShiftRigth(info) => (info),
            Token::Map(info) => (info),
            Token::MapConstRigth(info) => (info),
            Token::MapConstLeft(info) => (info),
            Token::Appliative(info) => (info),
            Token::ApplicativeRight(info) => (info),
            Token::ApplicativeLeft(info) => (info),
            Token::Equality(info) => (info),
            Token::NotEqual(info) => (info),
            Token::LessOrEqual(info) => (info),
            Token::MoreOrEqual(info) => (info),
            Token::LessThan(info) => (info),
            Token::MoreThan(info) => (info),
            Token::And(info) => (info),
            Token::Or(info) => (info),
            Token::ReverseAppliation(info) => (info),
            Token::DollarApplication(info) => (info),
            Token::Asignation(info) => (info),
            Token::At(info) => (info),
            Token::Pipe(info) => (info),
            Token::LParen(info) => (info),
            Token::RParen(info) => (info),
            Token::LBracket(info) => (info),
            Token::RBracket(info) => (info),
            Token::LBrace(info) => (info),
            Token::RBrace(info) => (info),
            Token::RightArrow(info) => (info),
            Token::LeftArrow(info) => (info),
            Token::LambdaStart(info) => (info),
            Token::Let(info) => (info),
            Token::In(info) => (info),
            Token::Case(info) => (info),
            Token::Of(info) => (info),
            Token::Import(info) => (info),
            Token::Data(info) => (info),
            Token::Newtype(info) => (info),
            Token::Class(info) => (info),
            Token::Instance(info) => (info),
            Token::Public(info) => (info),
            Token::Alias(info) => (info),
            Token::As(info) => (info),
            Token::Unqualified(info) => (info),
            Token::Forall(info) => (info),
            Token::Type(info) => (info),
            Token::U8(info) => (info),
            Token::U16(info) => (info),
            Token::U32(info) => (info),
            Token::U64(info) => (info),
            Token::I8(info) => (info),
            Token::I16(info) => (info),
            Token::I32(info) => (info),
            Token::I64(info) => (info),
            Token::F32(info) => (info),
            Token::F64(info) => (info),
            Token::CharType(info) => (info),
            Token::StringType(info) => (info),
            Token::Comment(info, _) => info,
            //{
            //let span = match comments.get(0) {
            //    Some(s) => s.clone().get_span(),
            //    None => (usize::MAX, usize::MAX).into(),
            //};
            //TokenInfo {
            //    comments: CommentsInfo::empty(),
            //    span,
            //}
            //}
            Token::StringLiteral(info, _) => info,
            Token::CharacterLiteral(info, _) => info,
            Token::UintLiteral(info, _) => info,
            Token::UFloatLiteral(info, _) => info,
            Token::Identifier(info, _) => info,
            Token::InfixIdentifier(info, _) => info,
            Token::Selector(info, _) => info,
            Token::AnonHole(info) => info,
            Token::NamedHole(info, _) => info,
            Token::ModuleLogicPath(info, _) => info,
            Token::LastComments(info, _) => info,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexerErrorType {
    UnexpectedCharacter(String),
    UnbalancedComment,
    CantTranslateToToken(Token),
    InvaliedSequenceOfCharacters(String),
    CantConvertIdentifierTextToIdentifier(String, TokenInfo),
    CantConvertToModuleLogicPath(String, TokenInfo),
    CantParseOperator(String, TokenInfo),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexerError {
    pub error_type: LexerErrorType,
    pub position: Position,
}

macro_rules! make_lexer_token_to_token {
    ($name:tt, $output_constructor:tt, $output_type:tt) => {
        paste!{
            pub fn [< $name _token_to_token >](t:Token)->Result<base::Token<$output_type>,ParseError<Position,Token,LexerError>>{
                match t {
                    Token::$output_constructor(info,value) => Ok(base::Token{value,info}),
                    _ => Err(ParseError::User{error: LexerError{error_type:LexerErrorType::CantTranslateToToken(t.clone()),position:{let t2 : TokenInfo =t.into(); t2.span.start}} })
                }
            }
        }

    };
}
make_lexer_token_to_token!(module, ModuleLogicPath, ModuleLogicPath);
make_lexer_token_to_token!(identifier, Identifier, Identifier);
make_lexer_token_to_token!(string, StringLiteral, String);
make_lexer_token_to_token!(char, CharacterLiteral, String);
make_lexer_token_to_token!(uint, UintLiteral, String);
make_lexer_token_to_token!(ufloat, UFloatLiteral, String);
make_lexer_token_to_token!(selector, Selector, Identifier);
make_lexer_token_to_token!(named_hole, NamedHole, u64);

pub fn aux_logos_token_to_token(logos: LogosToken, info: TokenInfo) -> Token {
    match logos {
        LogosToken::Interrogation => Token::Interrogation(info),
        LogosToken::Exclamation => Token::Exclamation(info),
        LogosToken::Hash => Token::Hash(info),
        LogosToken::Comma => Token::Comma(info),
        LogosToken::Colon => Token::Colon(info),
        LogosToken::StatementEnd => Token::StatementEnd(info),
        LogosToken::Dot => Token::Dot(info),
        LogosToken::ModuleSeparator => Token::ModuleSeparator(info),
        LogosToken::Minus => Token::Minus(info),
        LogosToken::CompositionRight => Token::CompositionRight(info),
        LogosToken::CompositionLeft => Token::CompositionLeft(info),
        LogosToken::Plus => Token::Plus(info),
        LogosToken::Power => Token::Power(info),
        LogosToken::Star => Token::Star(info),
        LogosToken::Div => Token::Div(info),
        LogosToken::Module => Token::Module(info),
        LogosToken::ShiftLeft => Token::ShiftLeft(info),
        LogosToken::ShiftRigth => Token::ShiftRigth(info),
        LogosToken::Map => Token::Map(info),
        LogosToken::MapConstRigth => Token::MapConstRigth(info),
        LogosToken::MapConstLeft => Token::MapConstLeft(info),
        LogosToken::Appliative => Token::Appliative(info),
        LogosToken::ApplicativeRight => Token::ApplicativeRight(info),
        LogosToken::ApplicativeLeft => Token::ApplicativeLeft(info),
        LogosToken::Equality => Token::Equality(info),
        LogosToken::NotEqual => Token::NotEqual(info),
        LogosToken::LessOrEqual => Token::LessOrEqual(info),
        LogosToken::MoreOrEqual => Token::MoreOrEqual(info),
        LogosToken::LessThan => Token::LessThan(info),
        LogosToken::MoreThan => Token::MoreThan(info),
        LogosToken::And => Token::And(info),
        LogosToken::Or => Token::Or(info),
        LogosToken::ReverseAppliation => Token::ReverseAppliation(info),
        LogosToken::DollarApplication => Token::DollarApplication(info),
        LogosToken::Asignation => Token::Asignation(info),
        LogosToken::At => Token::At(info),
        LogosToken::Pipe => Token::Pipe(info),
        LogosToken::LParen => Token::LParen(info),
        LogosToken::RParen => Token::RParen(info),
        LogosToken::LBracket => Token::LBracket(info),
        LogosToken::RBracket => Token::RBracket(info),
        LogosToken::LBrace => Token::LBrace(info),
        LogosToken::RBrace => Token::RBrace(info),
        LogosToken::RightArrow => Token::RightArrow(info),
        LogosToken::LeftArrow => Token::LeftArrow(info),
        LogosToken::LambdaStart => Token::LambdaStart(info),
        LogosToken::Let => Token::Let(info),
        LogosToken::In => Token::In(info),
        LogosToken::Case => Token::Case(info),
        LogosToken::Of => Token::Of(info),
        LogosToken::Import => Token::Import(info),
        LogosToken::Data => Token::Data(info),
        LogosToken::Newtype => Token::Newtype(info),
        LogosToken::Class => Token::Class(info),
        LogosToken::Instance => Token::Instance(info),
        LogosToken::Public => Token::Public(info),
        LogosToken::Alias => Token::Alias(info),
        LogosToken::As => Token::As(info),
        LogosToken::Unqualified => Token::Unqualified(info),
        LogosToken::Forall => Token::Forall(info),
        LogosToken::Type => Token::Type(info),
        LogosToken::U8 => Token::U8(info),
        LogosToken::U16 => Token::U16(info),
        LogosToken::U32 => Token::U32(info),
        LogosToken::U64 => Token::U64(info),
        LogosToken::I8 => Token::I8(info),
        LogosToken::I16 => Token::I16(info),
        LogosToken::I32 => Token::I32(info),
        LogosToken::I64 => Token::I64(info),
        LogosToken::F32 => Token::F32(info),
        LogosToken::F64 => Token::F64(info),
        LogosToken::CharType => Token::CharType(info),
        LogosToken::StringType => Token::StringType(info),
        LogosToken::LineComment(c) => Token::Comment(info, Comment::Line(c)),
        LogosToken::BlockComment(c) => Token::Comment(info, Comment::Block(c)),
        LogosToken::UintLiteral(s) => Token::UintLiteral(info, s),
        LogosToken::UFloatLiteral(s) => Token::UFloatLiteral(info, s),
        LogosToken::Identifier(s) => Token::Identifier(info, s),
        LogosToken::InfixIdentifier(s) => Token::InfixIdentifier(info, s),
        LogosToken::Selector(s) => Token::Selector(info, s),
        LogosToken::AnonHole => Token::AnonHole(info),
        LogosToken::NamedHole(s) => Token::NamedHole(info, s),
        LogosToken::ModuleLogicPath(s) => Token::ModuleLogicPath(info, s),
    }
}

pub struct LexerContext<'src> {
    source: &'src str,
    line: usize,
    previous_token: Option<LogosToken>,
    lexer: &'src mut SpannedIter<'src, LogosToken>,
    found_error: bool,
}

impl<'src> LexerContext<'src> {
    fn new(
        source: &'src str,
        line: usize,
        previous_token: Option<LogosToken>,
        lexer: &'src mut SpannedIter<'src, LogosToken>,
        found_error: bool,
    ) -> Self {
        LexerContext {
            source,
            line,
            previous_token,
            lexer,
            found_error,
        }
    }
}

pub fn advance_line_count<'a>(
    token: &'a LogosToken,
    line_count: &'a mut usize,
) -> () {
    todo!()
}

pub fn acumulate_comments<'src>(
    lexer: &'src mut SpannedIter<'src, LogosToken>,
    line_count: &'src mut usize,
    acc: &mut Vec<(Span, usize, Comment)>,
) -> Result<Option<(Span, usize, LogosToken)>, ()> {
    let mut out: Result<Option<(Span, usize, LogosToken)>, ()> = Ok(None);
    loop {
        let current_line = *line_count;
        match lexer.next() {
            Some((maybe_token, logos_span)) => match maybe_token {
                Ok(token) => {
                    advance_line_count(&token, line_count);
                    match token {
                        LogosToken::LineComment(l) => {
                            acc.push((
                                range_to_span(logos_span),
                                current_line,
                                Comment::Line(l),
                            ));
                        }
                        LogosToken::BlockComment(b) => {
                            acc.push((
                                range_to_span(logos_span),
                                *line_count,
                                Comment::Block(b),
                            ));
                        }
                        _ => {
                            out = Ok(Some((
                                range_to_span(logos_span),
                                *line_count,
                                token,
                            )));
                            break;
                        }
                    }
                }
                _ => {
                    //TODO: HAndle errors
                    out = Err(());
                    break;
                }
            },
            None => break,
        }
    }
    out
}

impl<'src> Iterator for LexerContext<'src> {
    type Item = Result<Token, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        // First iter:
        // get_comments
        // get next token
        // get next token
        // if comment return a token,
        // else save new token to context and return a token
        // continue
        // Intermediate iter:
        // if saved token :
        //   get next token, if comment, return a token and clear context
        //                      else return a token and put the new token in the context
        // else :
        //  get comments
        //  get next token
        //  get next token
        match &self.previous_token {
            Some(previous) => {
                todo!()
            }
            None => {
                let mut acc = vec![];
                acumulate_comments(&mut self.lexer, &mut self.line, &mut acc);
                match self.lexer.next() {
                    Some(next_token) => {}
                    None => Ok(Token::LastComments(_, acc)),
                }
                todo!()
            }
        }
    }
}

/*
pub fn gen_tokens<'src>(
    source: &'src str,
    store: &mut Store,
) -> Result<Vec<Token>, ()> {
    let mut context = LexerContext::new(source, 0, None);
    let logos = LogosToken::lexer_with_extras(source, store);
    let mut comments_acc: Vec<Comment> = vec![];
    let mut previous_token: Option<Comment> = None;
    let mut acc: Vec<Token> = vec![];
    let iter = logos.spanned();
    while true {
        let current = logos.next();
    }
    Ok(acc)
}
*/
