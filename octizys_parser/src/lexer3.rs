use core::panic;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use lalrpop_util::ParseError;
use logos::{Lexer, Logos, Source, SpannedIter};
use octizys_common::span::{Position, Span};
use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath,
};
use octizys_cst::base;
use octizys_cst::base;
use octizys_cst::comments::{
    CommentBlock, CommentBraceKind, CommentKind, CommentLine,
    CommentLineContent, CommentsInfo, LineCommentStart,
};
use octizys_cst::{base::TokenInfo, comments::Comment};
use octizys_text_store::store::Store;

#[derive(Debug, Clone)]
pub struct LogosLexerContext {
    store: Rc<RefCell<Store>>,
    position: Position,
}

use paste::paste;

impl LogosLexerContext {
    pub fn new(store: Rc<RefCell<Store>>, position: Position) -> Self {
        LogosLexerContext { store, position }
    }
}

impl From<LogosLexerContext> for Position {
    fn from(value: LogosLexerContext) -> Self {
        value.position
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r]+")]
#[logos(extras = LogosLexerContext)]
#[logos(error = LexerError)]
pub enum LogosToken {
    #[regex("\n", advance_line_break)]
    NewLine,
    #[token("?", make_simple_token)]
    Interrogation(Span),
    #[token("!", make_simple_token)]
    Exclamation(Span),
    #[token("#", make_simple_token)]
    Hash(Span),
    #[token(",", make_simple_token)]
    Comma(Span),
    #[token(":", make_simple_token)]
    Colon(Span),
    #[token(";", make_simple_token)]
    StatementEnd(Span),
    #[token(".", make_simple_token)]
    Dot(Span),
    //#[token("::", make_simple_token)]
    //ModuleSeparator(Span),
    #[token("-", make_simple_token)]
    Minus(Span),
    #[token("|>", make_simple_token)]
    CompositionRight(Span),
    #[token("<|", make_simple_token)]
    CompositionLeft(Span),
    #[token("+", make_simple_token)]
    Plus(Span),
    #[token("^", make_simple_token)]
    Power(Span),
    #[token("*", make_simple_token)]
    Star(Span),
    #[token("/", make_simple_token)]
    Div(Span),
    #[token("%", make_simple_token)]
    Module(Span),
    #[token("<<", make_simple_token)]
    ShiftLeft(Span),
    #[token(">>", make_simple_token)]
    ShiftRigth(Span),
    #[token("<$>", make_simple_token)]
    Map(Span),
    #[token("$>", make_simple_token)]
    MapConstRigth(Span),
    #[token("<$", make_simple_token)]
    MapConstLeft(Span),
    #[token("<*>", make_simple_token)]
    Appliative(Span),
    #[token("*>", make_simple_token)]
    ApplicativeRight(Span),
    #[token("<*", make_simple_token)]
    ApplicativeLeft(Span),
    #[token("==", make_simple_token)]
    Equality(Span),
    #[token("!=", make_simple_token)]
    NotEqual(Span),
    #[token("<=", make_simple_token)]
    LessOrEqual(Span),
    #[token(">=", make_simple_token)]
    MoreOrEqual(Span),
    #[token("<", make_simple_token)]
    LessThan(Span),
    #[token(">", make_simple_token)]
    MoreThan(Span),
    #[token("&&", make_simple_token)]
    And(Span),
    #[token("||", make_simple_token)]
    Or(Span),
    #[token("&", make_simple_token)]
    ReverseAppliation(Span),
    #[token("$", make_simple_token)]
    DollarApplication(Span),
    #[token("=", make_simple_token)]
    Asignation(Span),
    #[token("@", make_simple_token)]
    At(Span),
    #[token("|", make_simple_token)]
    Pipe(Span),
    #[token("(", make_simple_token)]
    LParen(Span),
    #[token(")", make_simple_token)]
    RParen(Span),
    #[token("[", make_simple_token)]
    LBracket(Span),
    #[token("]", make_simple_token)]
    RBracket(Span),
    #[token("{", make_simple_token)]
    LBrace(Span),
    #[token("}", make_simple_token)]
    RBrace(Span),
    #[token("->", make_simple_token)]
    RightArrow(Span),
    #[token("<-", make_simple_token)]
    LeftArrow(Span),
    #[token("\\", make_simple_token)]
    LambdaStart(Span),
    #[token("let", make_simple_token)]
    Let(Span),
    #[token("in", make_simple_token)]
    In(Span),
    #[token("case", make_simple_token)]
    Case(Span),
    #[token("of", make_simple_token)]
    Of(Span),
    #[token("import", make_simple_token)]
    Import(Span),
    #[token("data", make_simple_token)]
    Data(Span),
    #[token("newtype", make_simple_token)]
    Newtype(Span),
    #[token("class", make_simple_token)]
    Class(Span),
    #[token("instance", make_simple_token)]
    Instance(Span),
    #[token("public", make_simple_token)]
    Public(Span),
    #[token("alias", make_simple_token)]
    Alias(Span),
    #[token("as", make_simple_token)]
    As(Span),
    #[token("unqualified", make_simple_token)]
    Unqualified(Span),
    #[token("forall", make_simple_token)]
    Forall(Span),
    #[token("type", make_simple_token)]
    Type(Span),
    #[token("U8", make_simple_token)]
    U8(Span),
    #[token("U16", make_simple_token)]
    U16(Span),
    #[token("U32", make_simple_token)]
    U32(Span),
    #[token("U64", make_simple_token)]
    U64(Span),
    #[token("I8", make_simple_token)]
    I8(Span),
    #[token("I16", make_simple_token)]
    I16(Span),
    #[token("I32", make_simple_token)]
    I32(Span),
    #[token("I64", make_simple_token)]
    I64(Span),
    #[token("F32", make_simple_token)]
    F32(Span),
    #[token("F64", make_simple_token)]
    F64(Span),
    #[regex("Char", make_simple_token)]
    CharType(Span),
    #[regex("String", make_simple_token)]
    StringType(Span),
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
    #[regex(r#"0[0_]*|[1-9][0-9_]+"#, |x| {
        (make_simple_token(x) , String::from(x.slice()))

        }
    )]
    UintLiteral((Span, String)),
    //#[regex(r"{UINT_STRING}\\.UINT_STRING((e|E)UINT_STRING)?")]
    #[regex(r"(0[0_]*|[1-9][0-9_]+)\.(0[0_]*|[1-9][0-9_]+)((e|E)(0[0_]*|[1-9][0-9_]+))"
        , |x| {(make_simple_token(x), String::from(x.slice()))}
        )
    ]
    UFloatLiteral((Span, String)),
    //#[regex(r"_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*"
    //    , make_identifier)]
    #[regex(r"[a-b]+", make_identifier)]
    Identifier((Span, Identifier)),
    //#[regex(r"`_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*`"
    //    , make_identifier)]
    #[regex(r"`[a-b]+`", make_identifier)]
    InfixIdentifier((Span, Identifier)),
    //#[regex(r"\._*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*"
    //    , make_identifier)]
    #[regex(r"\.[a-b]+", make_identifier)]
    Selector((Span, Identifier)),
    #[regex("_", make_simple_token)]
    AnonHole(Span),
    //TODO:FIXME: Handle the UIntError
    //Probably we don't want to do this, instead use a String and emit a warning
    #[regex("_(0[0_]*|[1-9][0-9_]+)", |lexer| {
        let span = make_simple_token(lexer);
        match lexer.slice().parse::<u64>(){
            Ok(v) => Ok((span,v )),
            Err(e) => {
                let error_type = LexerErrorType::CantConvertNamedHoleToU6(lexer.slice().to_string(),span);
                Err(LexerError{error_type,position:span.end})
            }
        }
    } )]
    NamedHole((Span, u64)),
    //#[regex(r"(_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*::)+"
    //    , make_module_logic_path,priority=20)]
    #[regex(r"([a-b]+::)+", make_module_logic_path, priority = 20)]
    ModuleLogicPath((Span, ModuleLogicPath)),
}

const STRING_REGEX: &'static str = r#"\"([^"\n]|((\\\\)*\\"))*""#;

fn advance_line_break(lexer: &mut Lexer<LogosToken>) -> logos::Skip {
    lexer.extras.position.source_index += 1;
    lexer.extras.position.line += 1;
    lexer.extras.position.column == 0;
    logos::Skip
}

//TODO: test this!
fn make_simple_token(lexer: &mut Lexer<LogosToken>) -> Span {
    let logos_span = lexer.span();
    let start = (&lexer.extras).position.clone();
    let end = Position {
        source_index: start.source_index + (logos_span.end - logos_span.start),
        column: start.column + logos_span.start,
        line: start.line,
    };
    lexer.extras.position = end.clone();
    Span { start, end }
}

fn make_line_comment(
    lexer: &mut Lexer<LogosToken>,
    kind: CommentKind,
    line_comment_start: LineCommentStart,
) -> Result<CommentLine, LexerError> {
    let line = lexer.extras.position.line + 1;
    let logos_span = lexer.span();
    let start = (&lexer.extras).position.clone();
    let end = Position {
        source_index: start.source_index + (logos_span.start - logos_span.end),
        column: start.column + logos_span.start,
        line: start.line,
    };
    lexer.extras.position = end.clone();
    lexer.extras.position.line += 1;
    let span = Span { start, end };
    let mut store = (*lexer.extras.store).borrow_mut();
    let maybe_content =
        CommentLineContent::make_register(lexer.slice(), &mut (*store));
    match maybe_content {
        Some(content) => Ok(CommentLine {
            kind,
            start: line_comment_start,
            content,
            span,
        }),
        None => Err(LexerError {
            error_type: LexerErrorType::CantReadLineComment(
                lexer.slice().to_string(),
                span,
            ),
            position: span.start,
        }),
    }
}

pub fn make_block_comment(
    lexer: &mut Lexer<LogosToken>,
    kind: CommentKind,
    brace: CommentBraceKind,
) -> CommentBlock {
    let start_pos = lexer.extras.position;
    let mut store = (*lexer.extras.store).borrow_mut();
    let content = lexer.slice();

    let comment = CommentBlock::make(
        kind,
        brace,
        content,
        start_pos,
        start_pos,
        &mut (*store),
    );
    lexer.extras.position.source_index = start_pos.source_index + content.len();
    let mut as_lines: Vec<&str> = content.split('\n').collect();
    let len = as_lines.len();
    if len > 1 {
        lexer.extras.position.line += len - 1;
        let last = as_lines.pop().unwrap();
        lexer.extras.position.column = last.len();
    } else {
        lexer.extras.position.column = content.len();
    }
    comment
}

pub fn make_identifier(
    lexer: &mut Lexer<LogosToken>,
) -> Result<(Span, Identifier), LexerError> {
    let span = make_simple_token(lexer);
    let mut store = (*lexer.extras.store).borrow_mut();
    let maybe_identifier = Identifier::make(lexer.slice(), &mut (*store));
    match maybe_identifier {
        Ok(identifier) => Ok((span, identifier)),
        Err(_) => Err(LexerError {
            error_type: LexerErrorType::CantConvertIdentifierTextToIdentifier(
                lexer.slice().to_string(),
                span,
            ),
            position: span.start,
        }),
    }
}

pub fn make_module_logic_path(
    lexer: &mut Lexer<LogosToken>,
) -> Result<(Span, ModuleLogicPath), LexerError> {
    let span = make_simple_token(lexer);
    let mut store = (*lexer.extras.store).borrow_mut();
    let maybe_path = ModuleLogicPath::make(lexer.slice(), &mut (*store));
    match maybe_path {
        Ok(path) => Ok((span, path)),
        Err(_) => Err(LexerError {
            error_type: LexerErrorType::CantConvertToModuleLogicPath(
                lexer.slice().to_string(),
                span,
            ),
            position: span.start,
        }),
    }
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
    //ModuleSeparator(TokenInfo),
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

impl LogosToken {
    fn as_span(&self) -> &Span {
        match self {
            LogosToken::NewLine => {
                panic!("This shouldn't happen! contact octizys.")
            }
            LogosToken::Interrogation(s) => s,
            LogosToken::Exclamation(s) => s,
            LogosToken::Hash(s) => s,
            LogosToken::Comma(s) => s,
            LogosToken::Colon(s) => s,
            LogosToken::StatementEnd(s) => s,
            LogosToken::Dot(s) => s,
            //            LogosToken::ModuleSeparator(s) => s,
            LogosToken::Minus(s) => s,
            LogosToken::CompositionRight(s) => s,
            LogosToken::CompositionLeft(s) => s,
            LogosToken::Plus(s) => s,
            LogosToken::Power(s) => s,
            LogosToken::Star(s) => s,
            LogosToken::Div(s) => s,
            LogosToken::Module(s) => s,
            LogosToken::ShiftLeft(s) => s,
            LogosToken::ShiftRigth(s) => s,
            LogosToken::Map(s) => s,
            LogosToken::MapConstRigth(s) => s,
            LogosToken::MapConstLeft(s) => s,
            LogosToken::Appliative(s) => s,
            LogosToken::ApplicativeRight(s) => s,
            LogosToken::ApplicativeLeft(s) => s,
            LogosToken::Equality(s) => s,
            LogosToken::NotEqual(s) => s,
            LogosToken::LessOrEqual(s) => s,
            LogosToken::MoreOrEqual(s) => s,
            LogosToken::LessThan(s) => s,
            LogosToken::MoreThan(s) => s,
            LogosToken::And(s) => s,
            LogosToken::Or(s) => s,
            LogosToken::ReverseAppliation(s) => s,
            LogosToken::DollarApplication(s) => s,
            LogosToken::Asignation(s) => s,
            LogosToken::At(s) => s,
            LogosToken::Pipe(s) => s,
            LogosToken::LParen(s) => s,
            LogosToken::RParen(s) => s,
            LogosToken::LBracket(s) => s,
            LogosToken::RBracket(s) => s,
            LogosToken::LBrace(s) => s,
            LogosToken::RBrace(s) => s,
            LogosToken::RightArrow(s) => s,
            LogosToken::LeftArrow(s) => s,
            LogosToken::LambdaStart(s) => s,
            LogosToken::Let(s) => s,
            LogosToken::In(s) => s,
            LogosToken::Case(s) => s,
            LogosToken::Of(s) => s,
            LogosToken::Import(s) => s,
            LogosToken::Data(s) => s,
            LogosToken::Newtype(s) => s,
            LogosToken::Class(s) => s,
            LogosToken::Instance(s) => s,
            LogosToken::Public(s) => s,
            LogosToken::Alias(s) => s,
            LogosToken::As(s) => s,
            LogosToken::Unqualified(s) => s,
            LogosToken::Forall(s) => s,
            LogosToken::Type(s) => s,
            LogosToken::U8(s) => s,
            LogosToken::U16(s) => s,
            LogosToken::U32(s) => s,
            LogosToken::U64(s) => s,
            LogosToken::I8(s) => s,
            LogosToken::I16(s) => s,
            LogosToken::I32(s) => s,
            LogosToken::I64(s) => s,
            LogosToken::F32(s) => s,
            LogosToken::F64(s) => s,
            LogosToken::CharType(s) => s,
            LogosToken::StringType(s) => s,
            LogosToken::LineComment(s) => &s.span,
            LogosToken::BlockComment(s) => &s.span,
            LogosToken::UintLiteral((s, _)) => s,
            LogosToken::UFloatLiteral((s, _)) => s,
            LogosToken::Identifier((s, _)) => s,
            LogosToken::InfixIdentifier((s, _)) => s,
            LogosToken::Selector((s, _)) => s,
            LogosToken::AnonHole(s) => s,
            LogosToken::NamedHole((s, _)) => s,
            LogosToken::ModuleLogicPath((s, _)) => s,
        }
    }
}

impl<'a> From<&'a LogosToken> for &'a Span {
    fn from(value: &'a LogosToken) -> Self {
        value.as_span()
    }
}

impl<'a> From<&'a Token> for &'a TokenInfo {
    fn from(value: &'a Token) -> &'a TokenInfo {
        match value {
            Token::Interrogation(info) => (info),
            Token::Exclamation(info) => (info),
            Token::Hash(info) => (info),
            Token::Comma(info) => (info),
            Token::Colon(info) => (info),
            Token::StatementEnd(info) => (info),
            Token::Dot(info) => (info),
            //            Token::ModuleSeparator(info) => (info),
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
            //            Token::ModuleSeparator(info) => (info),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexerErrorType {
    InvalidCharacter(String, Position),
    CantTranslateToToken(Token),
    CantConvertIdentifierTextToIdentifier(String, Span),
    CantConvertToModuleLogicPath(String, Span),
    CantParseOperator(String, TokenInfo),
    CantConvertNamedHoleToU6(String, Span),
    CantReadLineComment(String, Span),
    LogosMandatoryDeafult,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LexerError {
    pub error_type: LexerErrorType,
    pub position: Position,
}

impl Default for LexerError {
    fn default() -> Self {
        LexerError {
            error_type: LexerErrorType::LogosMandatoryDeafult,
            position: Position {
                source_index: 0,
                line: 0,
                column: 0,
            },
        }
    }
}

macro_rules! make_lexer_token_to_token {
    ($name:tt, $output_constructor:tt, $output_type:tt) => {
        paste!{
            pub fn [< $name _token_to_token >](t:Token)->Result<base::Token<$output_type>,ParseError<Position,Token,LexerError>>{
                match t {
                    Token::$output_constructor(info,value) => Ok(base::Token{value,info}),
                    _ => Err(
                        ParseError::User{
                            error: LexerError{
                                error_type:LexerErrorType::CantTranslateToToken(t.clone())
                                ,position:{let t2 : TokenInfo =(t).into(); t2.span.start}
                            }
                        }
                    )
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

pub fn aux_logos_token_to_token(
    logos: LogosToken,
    mut info: TokenInfo,
) -> Token {
    match logos {
        LogosToken::Interrogation(_) => Token::Interrogation(info),
        LogosToken::Exclamation(_) => Token::Exclamation(info),
        LogosToken::Hash(_) => Token::Hash(info),
        LogosToken::Comma(_) => Token::Comma(info),
        LogosToken::Colon(_) => Token::Colon(info),
        LogosToken::StatementEnd(_) => Token::StatementEnd(info),
        LogosToken::Dot(_) => Token::Dot(info),
        //        LogosToken::ModuleSeparator(_) => Token::ModuleSeparator(info),
        LogosToken::Minus(_) => Token::Minus(info),
        LogosToken::CompositionRight(_) => Token::CompositionRight(info),
        LogosToken::CompositionLeft(_) => Token::CompositionLeft(info),
        LogosToken::Plus(_) => Token::Plus(info),
        LogosToken::Power(_) => Token::Power(info),
        LogosToken::Star(_) => Token::Star(info),
        LogosToken::Div(_) => Token::Div(info),
        LogosToken::Module(_) => Token::Module(info),
        LogosToken::ShiftLeft(_) => Token::ShiftLeft(info),
        LogosToken::ShiftRigth(_) => Token::ShiftRigth(info),
        LogosToken::Map(_) => Token::Map(info),
        LogosToken::MapConstRigth(_) => Token::MapConstRigth(info),
        LogosToken::MapConstLeft(_) => Token::MapConstLeft(info),
        LogosToken::Appliative(_) => Token::Appliative(info),
        LogosToken::ApplicativeRight(_) => Token::ApplicativeRight(info),
        LogosToken::ApplicativeLeft(_) => Token::ApplicativeLeft(info),
        LogosToken::Equality(_) => Token::Equality(info),
        LogosToken::NotEqual(_) => Token::NotEqual(info),
        LogosToken::LessOrEqual(_) => Token::LessOrEqual(info),
        LogosToken::MoreOrEqual(_) => Token::MoreOrEqual(info),
        LogosToken::LessThan(_) => Token::LessThan(info),
        LogosToken::MoreThan(_) => Token::MoreThan(info),
        LogosToken::And(_) => Token::And(info),
        LogosToken::Or(_) => Token::Or(info),
        LogosToken::ReverseAppliation(_) => Token::ReverseAppliation(info),
        LogosToken::DollarApplication(_) => Token::DollarApplication(info),
        LogosToken::Asignation(_) => Token::Asignation(info),
        LogosToken::At(_) => Token::At(info),
        LogosToken::Pipe(_) => Token::Pipe(info),
        LogosToken::LParen(_) => Token::LParen(info),
        LogosToken::RParen(_) => Token::RParen(info),
        LogosToken::LBracket(_) => Token::LBracket(info),
        LogosToken::RBracket(_) => Token::RBracket(info),
        LogosToken::LBrace(_) => Token::LBrace(info),
        LogosToken::RBrace(_) => Token::RBrace(info),
        LogosToken::RightArrow(_) => Token::RightArrow(info),
        LogosToken::LeftArrow(_) => Token::LeftArrow(info),
        LogosToken::LambdaStart(_) => Token::LambdaStart(info),
        LogosToken::Let(_) => Token::Let(info),
        LogosToken::In(_) => Token::In(info),
        LogosToken::Case(_) => Token::Case(info),
        LogosToken::Of(_) => Token::Of(info),
        LogosToken::Import(_) => Token::Import(info),
        LogosToken::Data(_) => Token::Data(info),
        LogosToken::Newtype(_) => Token::Newtype(info),
        LogosToken::Class(_) => Token::Class(info),
        LogosToken::Instance(_) => Token::Instance(info),
        LogosToken::Public(_) => Token::Public(info),
        LogosToken::Alias(_) => Token::Alias(info),
        LogosToken::As(_) => Token::As(info),
        LogosToken::Unqualified(_) => Token::Unqualified(info),
        LogosToken::Forall(_) => Token::Forall(info),
        LogosToken::Type(_) => Token::Type(info),
        LogosToken::U8(_) => Token::U8(info),
        LogosToken::U16(_) => Token::U16(info),
        LogosToken::U32(_) => Token::U32(info),
        LogosToken::U64(_) => Token::U64(info),
        LogosToken::I8(_) => Token::I8(info),
        LogosToken::I16(_) => Token::I16(info),
        LogosToken::I32(_) => Token::I32(info),
        LogosToken::I64(_) => Token::I64(info),
        LogosToken::F32(_) => Token::F32(info),
        LogosToken::F64(_) => Token::F64(info),
        LogosToken::CharType(_) => Token::CharType(info),
        LogosToken::StringType(_) => Token::StringType(info),
        LogosToken::LineComment(c) => Token::Comment(info, Comment::Line(c)),
        LogosToken::BlockComment(c) => Token::Comment(info, Comment::Block(c)),
        LogosToken::UintLiteral((_, s)) => Token::UintLiteral(info, s),
        LogosToken::UFloatLiteral((_, s)) => Token::UFloatLiteral(info, s),
        LogosToken::Identifier((_, s)) => Token::Identifier(info, s),
        LogosToken::InfixIdentifier((_, s)) => Token::InfixIdentifier(info, s),
        LogosToken::Selector((_, s)) => Token::Selector(info, s),
        LogosToken::AnonHole(_) => Token::AnonHole(info),
        LogosToken::NamedHole((_, s)) => Token::NamedHole(info, s),
        LogosToken::ModuleLogicPath((_, s)) => Token::ModuleLogicPath(info, s),
        LogosToken::NewLine => panic!(
            "This shouldn't happen, something is wrong with the octizys lexer"
        ),
    }
}

#[derive(Debug)]
pub struct LexerContext<'src> {
    source: &'src str,
    previous_token: Option<Result<LogosToken, LexerError>>,
    lexer: &'src mut Lexer<'src, LogosToken>,
}

impl<'src> LexerContext<'src> {
    pub fn new(
        source: &'src str,
        previous_token: Option<Result<LogosToken, LexerError>>,
        lexer: &'src mut Lexer<'src, LogosToken>,
    ) -> Self {
        LexerContext {
            source,
            previous_token,
            lexer,
        }
    }
}

pub fn acumulate_comments<'iterator, 'src>(
    lexer: &'iterator mut Lexer<'src, LogosToken>,
    acc: &mut Vec<Comment>,
) -> Result<Option<LogosToken>, LexerError> {
    let mut out: Result<Option<LogosToken>, LexerError> = Ok(None);
    loop {
        match lexer.next() {
            Some(maybe_token) => match maybe_token {
                Ok(token) => match token {
                    LogosToken::LineComment(l) => {
                        acc.push(Comment::Line(l));
                    }
                    LogosToken::BlockComment(b) => {
                        acc.push(Comment::Block(b));
                    }
                    _ => {
                        out = Ok(Some(token));
                        break;
                    }
                },
                Err(e) => {
                    out = Err(e);
                    break;
                }
            },
            None => break,
        }
    }
    out
}

pub fn complete_token_or_save(
    current_token: LogosToken,
    context: &mut LexerContext,
    mut info: TokenInfo,
) -> Option<Result<(Position, Token, Position), LexerError>> {
    match context.lexer.next() {
        Some(maybe_next_logos_token) => {
            match maybe_next_logos_token {
                Ok(logos_token) =>
                //TODO: FIXME: remove the clone
                {
                    match logos_token.clone() {
                        LogosToken::LineComment(l) => {
                            let span = info.span.clone();
                            if info.span.end.line == l.span.start.line {
                                info.comments.after = Some(Comment::Line(l));
                            } else {
                                context.previous_token = Some(Ok(logos_token));
                            }
                            let mut token =
                                aux_logos_token_to_token(current_token, info);
                            Some(Ok((span.start, token, span.end)))
                        }
                        LogosToken::BlockComment(c) => {
                            let span = info.span.clone();
                            if info.span.end.line == c.span.start.line {
                                info.comments.after = Some(Comment::Block(c));
                            } else {
                                context.previous_token = Some(Ok(logos_token));
                            }
                            let mut token =
                                aux_logos_token_to_token(current_token, info);
                            Some(Ok((span.start, token, span.end)))
                        }
                        _ => {
                            context.previous_token = Some(Ok(logos_token));
                            let span = current_token.as_span().to_owned();
                            let mut token =
                                aux_logos_token_to_token(current_token, info);
                            Some(Ok((span.start, token, span.end)))
                        }
                    }
                }
                Err(e) => {
                    context.previous_token = Some(Err(e));
                    let span = current_token.as_span().to_owned();
                    let mut token =
                        aux_logos_token_to_token(current_token, info);
                    Some(Ok((span.start, token, span.end)))
                }
            }
        }
        None => {
            context.previous_token = None;
            let span = current_token.as_span().to_owned();
            let mut token = aux_logos_token_to_token(current_token, info);
            Some(Ok((span.start, token, span.end)))
        }
    }
}

/// Given an array of comments at the end of the stream, we build a comment
/// token (if the array is empty we return None)j
fn compact_comments_info(
    comments: Vec<Comment>,
) -> Option<Result<(Position, Token, Position), LexerError>> {
    if comments.len() > 0 {
        todo!()
        //let token_info: TokenInfo = todo!();
        //Some(Ok(Token::LastComments(token_info, acc)))
    } else {
        None
    }
}

impl<'src> Iterator for LexerContext<'src> {
    type Item = Result<(Position, Token, Position), LexerError>;
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
        println!("{:?}", self);
        match &self.previous_token {
            Some(previous) => match previous {
                //TODO: catch the error if it is the default value and
                //cast to a error with position
                Err(e) => match e.error_type {
                    LexerErrorType::LogosMandatoryDeafult => {
                        println!("{:?}", self.lexer.span());
                        println!("{:?}", self.lexer.slice());
                        let real_type = LexerErrorType::InvalidCharacter(
                            self.lexer.slice().to_string(),
                            self.lexer.extras.position,
                        );
                        Some(Err(LexerError {
                            error_type: real_type,
                            position: self.lexer.extras.position,
                        }))
                    }
                    _ => Some(Err(e.to_owned())),
                },
                Ok(current_logos_token) => {
                    let mut comments_info_pre = CommentsInfo {
                        before: match current_logos_token {
                            LogosToken::LineComment(l) => {
                                vec![Comment::Line(l.to_owned())]
                            }
                            LogosToken::BlockComment(c) => {
                                vec![Comment::Block(c.to_owned())]
                            }
                            _ => vec![],
                        },
                        after: None,
                    };
                    let span_ref: &Span = (current_logos_token).into();
                    let span = span_ref.to_owned();
                    let mut info = TokenInfo::make(
                        comments_info_pre,
                        span.start,
                        span.end,
                    );
                    complete_token_or_save(
                        //TODO:FIXME remove the clone
                        current_logos_token.clone(),
                        self,
                        info,
                    )
                }
            },
            None => {
                // Didn't have previous value, either we got to the
                // end of the stream or we completed a token.
                let mut acc = vec![];
                let current_value =
                    acumulate_comments(&mut self.lexer, &mut acc);
                match current_value {
                    Ok(Some(current_token)) => {
                        let mut comments_info_pre = CommentsInfo {
                            before: acc,
                            after: None,
                        };
                        let span_ref: &Span = (&current_token).into();
                        let span = span_ref.to_owned();
                        let mut info = TokenInfo::make(
                            comments_info_pre,
                            span.start,
                            span.end,
                        );
                        complete_token_or_save(current_token, self, info)
                    }
                    Ok(None) => compact_comments_info(acc),
                    Err(e) => Some(Err(e)),
                }
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
