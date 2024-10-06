use lalrpop_util::ParseError;
use octizys_common::identifier::Identifier;
use octizys_common::module_logic_path::ModuleLogicPath;
use regex::Regex;
use std::str::CharIndices;
use std::sync::LazyLock;
use std::{collections::VecDeque, rc::Rc};

use octizys_cst::cst::{
    self, Comment, CommentBlock, CommentBraceKind, CommentKind, CommentLine,
    CommentLineContent, CommentsInfo, LineCommentStart, Position, Span,
    TokenInfo,
};

use paste::paste;

#[derive(Debug, PartialEq, Eq)]
pub enum LexerErrorType {
    UnexpectedCharacter,
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Export(TokenInfo),
    Data(TokenInfo),
    Newtype(TokenInfo),
    NewInstance(TokenInfo),
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
    LastComments(Vec<Comment>, TokenInfo),
    StringLiteral(TokenInfo, String),
    CharacterLiteral(TokenInfo, char),
    UintLiteral(TokenInfo, String),
    UFloatLiteral(TokenInfo, String),
    Identifier(TokenInfo, Identifier),
    InfixIdentifier(TokenInfo, String),
    Selector(TokenInfo, String),
    AnonHole(TokenInfo, String),
    NamedHole(TokenInfo, String),
    ModuleLogicPath(TokenInfo, ModuleLogicPath),
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
            Token::Export(info) => (info),
            Token::Data(info) => (info),
            Token::Newtype(info) => (info),
            Token::NewInstance(info) => (info),
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
            Token::LastComments(_, info) => info, //{
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
            Token::AnonHole(info, _) => info,
            Token::NamedHole(info, _) => info,
            Token::ModuleLogicPath(info, _) => info,
        }
    }
}

macro_rules! make_lexer_token_to_token {
    ($name:tt, $output:tt) => {
        paste!{
            pub fn [< $name _token_to_token >](t:Token)->Result<cst::Token<$output>,ParseError<usize,Token,LexerError>>{
                match t {
                    Token::$output(info,value) => Ok(cst::Token{value,info}),
                    _ => Err(ParseError::User{error: LexerError{error_type:LexerErrorType::CantTranslateToToken(t.clone()),position:{let t2 : TokenInfo =t.into(); t2.span.start}} })
                }
            }
        }

    };
}

make_lexer_token_to_token!(module, ModuleLogicPath);
make_lexer_token_to_token!(identifier, Identifier);

fn match_keyword(s: &str, info: TokenInfo) -> Option<Token> {
    return match s {
        "let" => Some(Token::Let(info)),
        "in" => Some(Token::In(info)),
        "case" => Some(Token::Case(info)),
        "of" => Some(Token::Of(info)),
        "import" => Some(Token::Import(info)),
        "export" => Some(Token::Export(info)),
        "data" => Some(Token::Data(info)),
        "newtype" => Some(Token::Newtype(info)),
        "newinstance" => Some(Token::NewInstance(info)),
        "class" => Some(Token::Class(info)),
        "instance" => Some(Token::Instance(info)),
        "alias" => Some(Token::Alias(info)),
        "as" => Some(Token::As(info)),
        "unqualified" => Some(Token::Unqualified(info)),
        "forall" => Some(Token::Forall(info)),
        "type" => Some(Token::Type(info)),
        "U8" => Some(Token::U8(info)),
        "U16" => Some(Token::U16(info)),
        "U32" => Some(Token::U32(info)),
        "U64" => Some(Token::U64(info)),
        "I8" => Some(Token::I8(info)),
        "I16" => Some(Token::I16(info)),
        "I32" => Some(Token::I32(info)),
        "I64" => Some(Token::I64(info)),
        "F32" => Some(Token::F32(info)),
        "F64" => Some(Token::F64(info)),
        _ => None,
    };
}

fn match_operator(s: &str, info: TokenInfo) -> Option<Token> {
    return match s {
        "?" => Some(Token::Interrogation(info)),
        "!" => Some(Token::Exclamation(info)),
        "#" => Some(Token::Hash(info)),
        "," => Some(Token::Comma(info)),
        ":" => Some(Token::Colon(info)),
        ";" => Some(Token::StatementEnd(info)),
        "." => Some(Token::Dot(info)),
        "::" => Some(Token::ModuleSeparator(info)),
        "-" => Some(Token::Minus(info)),
        "|>" => Some(Token::CompositionRight(info)),
        "<|" => Some(Token::CompositionLeft(info)),
        "+" => Some(Token::Plus(info)),
        "^" => Some(Token::Power(info)),
        "*" => Some(Token::Star(info)),
        "/" => Some(Token::Div(info)),
        "%" => Some(Token::Module(info)),
        "<<" => Some(Token::ShiftLeft(info)),
        ">>" => Some(Token::ShiftRigth(info)),
        "<$>" => Some(Token::Map(info)),
        "$>" => Some(Token::MapConstRigth(info)),
        "<$" => Some(Token::MapConstLeft(info)),
        "<*>" => Some(Token::Appliative(info)),
        "*>" => Some(Token::ApplicativeRight(info)),
        "<*" => Some(Token::ApplicativeLeft(info)),
        "==" => Some(Token::NotEqual(info)),
        "!=" => Some(Token::NotEqual(info)),
        "<=" => Some(Token::LessOrEqual(info)),
        "=>" => Some(Token::MoreOrEqual(info)),
        "<" => Some(Token::LessThan(info)),
        ">" => Some(Token::MoreThan(info)),
        "&&" => Some(Token::And(info)),
        "||" => Some(Token::Or(info)),
        "&" => Some(Token::ReverseAppliation(info)),
        "$" => Some(Token::DollarApplication(info)),
        "=" => Some(Token::Asignation(info)),
        "@" => Some(Token::At(info)),
        "|" => Some(Token::Pipe(info)),
        "(" => Some(Token::LParen(info)),
        ")" => Some(Token::RParen(info)),
        "{" => Some(Token::LBrace(info)),
        "}" => Some(Token::RBrace(info)),
        "[" => Some(Token::LBracket(info)),
        "]" => Some(Token::RBracket(info)),
        "->" => Some(Token::LeftArrow(info)),
        "<-" => Some(Token::RightArrow(info)),
        "\\" => Some(Token::LambdaStart(info)),
        _ => None,
    };
}

static SIMPLE_SPACES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^( |\t)+").unwrap());
static ALL_SPACES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+").unwrap());
static HYPEN_DOCUMENTATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^-- \|.*\n").unwrap());
static HYPEN_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^--.*\n").unwrap());
static SLASH_DOCUMENTATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^// \|.*\n").unwrap());
static SLASH_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^//.*\n").unwrap());
static COMMENT_BLOCK0: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{-(.|\n)*-\}").unwrap());
static COMMENT_BLOCK1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{--(.|\n)*--\}").unwrap());
static COMMENT_BLOCK2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{---(.|\n)*---\}").unwrap());
static COMMENT_BLOCK3: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{----(.|\n)*----\}").unwrap());
static COMMENT_BLOCKD0: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{- \|(.|\n)*-\}").unwrap());
static COMMENT_BLOCKD1: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{-- \|(.|\n)*--\}").unwrap());
static COMMENT_BLOCKD2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{--- \|(.|\n)*---\}").unwrap());
static COMMENT_BLOCKD3: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\{---- \|(.|\n)*----\}").unwrap());
//TODO: choose operator hierarchies
// not_alone ->
// not_alone <-
// not_alone ,
// not_alone :
// not_alone |
// not_contains ;
// TODO: we aren't going to support custom operators but instead
// would offer a fixed set of operators with know fixities like :
// +,-, <+>, <<+>>  , ++, ++++,
//  `a +> b +> c` became  `(a +> b) +> c`
//  not sure about this, maybe all of them should be left associative?
//  yes, `a - b - c` must be `(a - b) - c`  and the C language table
//  uses  all binary ops as left associative, we only need to take care
//  of the precedences.
static OPERATOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(x?:)
        \?
        |\#
        |,
        |::
        |;
        |\.
        |:
        |\|>
        |<\|
        |\+
        |\^
        |/
        |%
        |<<
        |>>
        |<$
        |$>
        |<$
        |<\*
        |\*>
        |<\*
        |\*
        |==
        |!=
        |<=
        |>=
        |<
        |>
        |!
        |&&
        |\|\|
        |&
        |\$
        |=
        |@
        |\|
        |\(
        |\)
        |\{
        |\}
        |\[
        |\]
        |->
        |<-
        |-
        |\\
        ",
    )
    .unwrap()
});
static MULTILINE_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^""".*""""#).unwrap());
static LINE_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^"([^"\n]|((\\\\)*\\\"))*""#).unwrap());
static UINT_STRING: &'static str = r"^0[0_]*|[1-9][0-9_]+";
static UINT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(UINT_STRING).unwrap());
static FLOAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^{UINT_STRING}\\.UINT_STRING((e|E)UINT_STRING)?"))
        .unwrap()
});
// For docummentation on this, see the octizys_commmon::identifier::IDENTIFER_LAZY_REGEX
static IDENTIFIER_STRING: &'static str = r"_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*";
static IDENTIFER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!("^{:}", IDENTIFIER_STRING)).unwrap());
static INFIX_IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^`{:}`", IDENTIFIER_STRING)).unwrap()
});
static SELECTOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^\\.{re}", re = IDENTIFIER_STRING)).unwrap()
});
static HOLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!("^_{re}", re = UINT_STRING)).unwrap());
static MODULE_LOGIC_PATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^(({re}::)+{re})", re = IDENTIFIER_STRING)).unwrap()
});

#[derive(Debug)]
pub struct Lexer<'input> {
    src: &'input str,
    offset: usize,
    found_error: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(src: &'input str) -> Self {
        Lexer {
            src,
            offset: 0,
            found_error: false,
        }
    }

    pub fn satisfy<'a>(
        self: &'a mut Lexer<'input>,
        re: &LazyLock<Regex>,
    ) -> Option<(Span, &'input str)> {
        println!("re: {:?}", re);
        let start = self.offset;
        match re.find(self.src) {
            Some(m) => {
                println!("found: {:?}", m);
                let end = m.end();
                self.src = &self.src[end..];
                self.offset += end;
                Some(((start, self.offset - 1).into(), m.as_str()))
            }
            None => None,
        }
    }

    pub fn simple_spaces(&mut self) -> () {
        self.satisfy(&SIMPLE_SPACES);
    }

    pub fn all_spaces(&mut self) -> () {
        self.satisfy(&ALL_SPACES);
    }

    pub fn single_line_comment_aux(
        &mut self,
        re: &LazyLock<Regex>,
        kind: CommentKind,
        start: LineCommentStart,
    ) -> Option<CommentLine> {
        println!("single_line with : {:?} {:?}", kind, start);
        let out = match self.satisfy(re) {
            Some((span, full_text)) => {
                let start_offset = match kind {
                    CommentKind::NonDocumentation => 2,
                    CommentKind::Documentation => 4,
                };
                let text = &full_text[start_offset..full_text.len() - 1];
                let content = CommentLineContent::decompose(&text)[0].clone();
                Some(CommentLine {
                    kind,
                    start,
                    content,
                    span,
                })
            }
            None => None,
        };
        out
    }

    pub fn single_line_comment(&mut self) -> Option<CommentLine> {
        self.single_line_comment_aux(
            &HYPEN_DOCUMENTATION,
            CommentKind::Documentation,
            LineCommentStart::DoubleHypen,
        )
        .or_else(|| {
            self.single_line_comment_aux(
                &HYPEN_COMMENT,
                CommentKind::NonDocumentation,
                LineCommentStart::DoubleHypen,
            )
        })
        .or_else(|| {
            self.single_line_comment_aux(
                &SLASH_DOCUMENTATION,
                CommentKind::Documentation,
                LineCommentStart::DoubleSlash,
            )
        })
        .or_else(|| {
            self.single_line_comment_aux(
                &SLASH_COMMENT,
                CommentKind::NonDocumentation,
                LineCommentStart::DoubleSlash,
            )
        })
    }

    pub fn comment_block(&mut self) -> Option<CommentBlock> {
        let cases = [
            (
                &COMMENT_BLOCKD3,
                CommentKind::Documentation,
                CommentBraceKind::Brace3,
            ),
            (
                &COMMENT_BLOCK3,
                CommentKind::NonDocumentation,
                CommentBraceKind::Brace3,
            ),
            (
                &COMMENT_BLOCKD2,
                CommentKind::Documentation,
                CommentBraceKind::Brace2,
            ),
            (
                &COMMENT_BLOCK2,
                CommentKind::NonDocumentation,
                CommentBraceKind::Brace2,
            ),
            (
                &COMMENT_BLOCKD1,
                CommentKind::Documentation,
                CommentBraceKind::Brace1,
            ),
            (
                &COMMENT_BLOCK1,
                CommentKind::NonDocumentation,
                CommentBraceKind::Brace1,
            ),
            (
                &COMMENT_BLOCKD0,
                CommentKind::Documentation,
                CommentBraceKind::Brace0,
            ),
            (
                &COMMENT_BLOCK0,
                CommentKind::NonDocumentation,
                CommentBraceKind::Brace0,
            ),
        ];
        let mut result: Option<((Span, &str), CommentKind, CommentBraceKind)> =
            None;
        for (re, kind, brace) in cases {
            match self.satisfy(re) {
                Some(text) => {
                    result = Some((text, kind, brace));
                    break;
                }
                None => continue,
            }
        }

        let ((span, text), kind, brace) = result?;
        let len = brace.len();
        let start_offset = len
            + match kind {
                CommentKind::Documentation => 2,
                _ => 0,
            };
        //Spans are inclusive while rust use [a,b) for ranges
        // so we need a +1
        let end_offset = span.end.source_index - len + 1;
        let block = CommentBlock {
            kind,
            brace,
            content: CommentLineContent::decompose(
                &text[start_offset..end_offset],
            ),
            span,
        };
        Some(block)
    }

    pub fn any_comment(&mut self) -> Option<Comment> {
        let comment = self
            .single_line_comment()
            .map(Comment::from)
            .or_else(|| self.comment_block().map(Comment::from));
        self.all_spaces();
        comment
    }

    pub fn before_comments(&mut self) -> Vec<Comment> {
        self.all_spaces();
        let mut acc = vec![];
        loop {
            match self.any_comment() {
                Some(comment) => acc.push(comment),
                None => break,
            }
        }
        self.all_spaces();
        acc
    }

    pub fn after_comments(&mut self) -> Option<Comment> {
        self.simple_spaces();
        let comment = self.single_line_comment();
        self.all_spaces();
        comment.map(Comment::from)
    }

    pub fn lex_with_value(
        &mut self,
        re: &LazyLock<Regex>,
        before: Vec<Comment>,
        make_token: fn(&str, TokenInfo) -> Result<Token, LexerError>,
    ) -> Option<Result<Token, LexerError>> {
        let (span, text) = self.satisfy(re)?;
        let after = self.after_comments();
        let comments = cst::CommentsInfo { before, after };
        let info = TokenInfo { comments, span };
        Some(make_token(text, info))
    }

    pub fn identifier_or_keyword(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Result<Token, LexerError>> {
        //let (span, text) = self.satisfy(&IDENTIFER)?;
        //let after = self.after_comments();
        //let comments = cst::CommentsInfo { before, after };
        //let info = TokenInfo { comments, span };
        self.lex_with_value(&IDENTIFER, before, |text, info| {
            match match_keyword(text, info.clone()){
                Some(x) => Ok(x),
                None =>{
                    match Identifier::make(text) {
                        Ok(identifier) => Ok(Token::Identifier(info, identifier)),
                        Err(_) => {let position = info.span.start; Err(LexerError {
                            error_type:
                                LexerErrorType::CantConvertIdentifierTextToIdentifier(
                                    text.into(),
                                    info,
                                ),
                            position,
                        })},
                    }
                }
            }
        })
    }

    pub fn lex_uint(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Result<Token, LexerError>> {
        self.lex_with_value(&UINT, before, |text, info| {
            //TODO: check boundaries of UINT?
            Ok(Token::UintLiteral(info, String::from(text)))
        })
    }

    pub fn lex_float(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Result<Token, LexerError>> {
        self.lex_with_value(&FLOAT, before, |text, info| {
            //TODO: check boundaries of FLOAT?
            Ok(Token::UFloatLiteral(info, String::from(text)))
        })
    }

    pub fn lex_module_logic_path(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Result<Token, LexerError>> {
        self.lex_with_value(&MODULE_LOGIC_PATH, before, |text, info| {
            match ModuleLogicPath::make(text) {
                Ok(path) => Ok(Token::ModuleLogicPath(info, path)),
                Err(_) => {
                    let position = info.span.start;
                    Err(LexerError {
                        error_type:
                            LexerErrorType::CantConvertToModuleLogicPath(
                                String::from(text),
                                info,
                            ),
                        position,
                    })
                }
            }
        })
    }

    pub fn lex_symbol_or_operator(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Result<Token, LexerError>> {
        self.lex_with_value(&OPERATOR, before, |text, info| match text {
            "?" => Ok(Token::Interrogation(info)),
            "!" => Ok(Token::Exclamation(info)),
            "#" => Ok(Token::Hash(info)),
            "," => Ok(Token::Comma(info)),
            ":" => Ok(Token::Colon(info)),
            ";" => Ok(Token::StatementEnd(info)),
            "." => Ok(Token::Dot(info)),
            "::" => Ok(Token::ModuleSeparator(info)),
            "-" => Ok(Token::Minus(info)),
            "|>" => Ok(Token::CompositionLeft(info)),
            "<|" => Ok(Token::CompositionRight(info)),
            "+" => Ok(Token::Plus(info)),
            "^" => Ok(Token::Power(info)),
            "*" => Ok(Token::Star(info)),
            "/" => Ok(Token::Div(info)),
            "%" => Ok(Token::Module(info)),
            "<<" => Ok(Token::ShiftLeft(info)),
            ">>" => Ok(Token::ShiftRigth(info)),
            //TODO: Add "<&>" = \ x y -> y $ x
            "<$>" => Ok(Token::Map(info)),
            "$>" => Ok(Token::MapConstRigth(info)),
            "<$" => Ok(Token::MapConstLeft(info)),
            //TODO: add <|> and <?>
            "<*>" => Ok(Token::Appliative(info)),
            "*>" => Ok(Token::ApplicativeRight(info)),
            "<*" => Ok(Token::ApplicativeRight(info)),
            "==" => Ok(Token::Equality(info)),
            "!=" => Ok(Token::NotEqual(info)),
            "<=" => Ok(Token::LessOrEqual(info)),
            ">=" => Ok(Token::MoreOrEqual(info)),
            "<" => Ok(Token::LessThan(info)),
            ">" => Ok(Token::MoreThan(info)),
            "&&" => Ok(Token::And(info)),
            "||" => Ok(Token::Or(info)),
            "&" => Ok(Token::ReverseAppliation(info)),
            "$" => Ok(Token::DollarApplication(info)),
            "=" => Ok(Token::Asignation(info)),
            "@" => Ok(Token::At(info)),
            "|" => Ok(Token::Pipe(info)),
            "(" => Ok(Token::LParen(info)),
            ")" => Ok(Token::RParen(info)),
            "{" => Ok(Token::LBrace(info)),
            "}" => Ok(Token::RBrace(info)),
            "[" => Ok(Token::LBracket(info)),
            "]" => Ok(Token::RBracket(info)),
            "->" => Ok(Token::RightArrow(info)),
            "<-" => Ok(Token::LeftArrow(info)),
            "\\" => Ok(Token::LambdaStart(info)),
            _ => {
                let position = info.span.start;
                Err(LexerError {
                    error_type: LexerErrorType::CantParseOperator(
                        text.into(),
                        info,
                    ),
                    position,
                })
            }
        })
    }

    pub fn one_token(
        &mut self,
    ) -> Result<Result<Token, LexerError>, Vec<Comment>> {
        let before = self.before_comments();
        //TODO: is there a way to remove the clone of before?
        //well, it can be if we return a function that consumes
        //a before and construct a token instead of the token..
        let out = self
            .identifier_or_keyword(before.clone())
            .or_else(|| self.lex_float(before.clone()))
            .or_else(|| self.lex_uint(before.clone()))
            .or_else(|| self.lex_symbol_or_operator(before.clone()))
            .ok_or(before);
        println!("out : {:?}", out);
        out
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(Position, Token, Position), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        println!("new_next : {:?}", self);
        match self.src {
            "" => return None,
            _ => (),
        }
        if self.found_error {
            return None;
        }
        match self.one_token() {
            Ok(Ok(token)) => {
                let info = TokenInfo::from(token.clone());
                let out = Some(Ok((info.span.start, token, info.span.start)));
                return out;
            }
            Ok(Err(error_token)) => {
                self.found_error = true;
                Some(Err(error_token))
            }
            Err(comments) => {
                let span = match comments.get(0) {
                    Some(s) => s.clone().get_span(),
                    None => (usize::MAX, usize::MAX).into(),
                };
                let info = TokenInfo {
                    comments: CommentsInfo::empty(),
                    span,
                };
                let token = Token::LastComments(comments, info);
                //TODO: this is a hack to make lexer to stop if we couldn't identify a token
                //To fix it, we need to get a notion of "we tried everything and we didn't find
                //anything and we still have input" and check for it
                self.found_error = true;
                return Some(Ok((span.start, token, span.end)));
            }
        }
    }
}

#[cfg(test)]
mod lexer_tests {

    use octizys_pretty::types::NoLineBreaksString;

    use crate::lexer::*;

    //fn generic_test_by_spaces(s: &'static str) {
    //    let lex = Lexer::new(s);
    //    let result: Vec<Result<Token, LexerError>> =
    //        lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
    //    let splitted = s.split(" ").map().collect();
    //    assert!(result == vec![])
    //}

    #[test]
    fn empty_string() {
        let input = "";
        let lex = Lexer::new(&input);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        assert!(result == vec![])
    }

    #[test]
    fn identifier() {
        let s = "helloWorld";
        let lex = Lexer::new(s);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let identifier = Identifier::make(s).unwrap();
        let info = TokenInfo {
            comments: CommentsInfo {
                before: vec![],
                after: None,
            },
            span: (0, 9).into(),
        };
        let token = Token::Identifier(info, identifier);
        let expected = vec![Ok(token)];
        println!("result: {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected);
    }

    fn make_keyword_test(
        key: &'static str,
        constructor: fn(TokenInfo) -> Token,
    ) {
        println!("beginging: ");
        let lex = Lexer::new(key);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let info = TokenInfo {
            comments: CommentsInfo {
                before: vec![],
                after: None,
            },
            span: (0, key.len() - 1).into(),
        };
        let token = constructor(info.clone());
        let expected = vec![Ok(token)];
        println!("result: {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected);
    }

    macro_rules! make_keyword_test_macro {
        ($macro_name:tt, $name:tt,$constructor_name:tt) => {
            paste! {
                #[test]
                fn [< keword_ $macro_name _test >](){
                    make_keyword_test($name,Token::$constructor_name)
                }
            }
        };
    }

    make_keyword_test_macro!("import", "import", Import);
    make_keyword_test_macro!("export", "export", Export);
    make_keyword_test_macro!("data", "data", Data);
    make_keyword_test_macro!("newtype", "newtype", Newtype);
    make_keyword_test_macro!("alias", "alias", Alias);
    make_keyword_test_macro!("as", "as", As);
    make_keyword_test_macro!("unqualified", "unqualified", Unqualified);
    make_keyword_test_macro!("forall", "forall", Forall);
    make_keyword_test_macro!("type", "type", Type);
    make_keyword_test_macro!("u8", "U8", U8);
    make_keyword_test_macro!("u16", "U16", U16);
    make_keyword_test_macro!("u32", "U32", U32);
    make_keyword_test_macro!("u64", "U64", U64);
    make_keyword_test_macro!("i8", "I8", I8);
    make_keyword_test_macro!("i16", "I16", I16);
    make_keyword_test_macro!("i32", "I32", I32);
    make_keyword_test_macro!("i64", "I64", I64);
    make_keyword_test_macro!("f32", "F32", F32);
    make_keyword_test_macro!("f64", "F64", F64);

    fn make_line_comment_documentation_test(
        content_string: &'static str,
        kind: CommentKind,
        start: LineCommentStart,
    ) {
        let start_str: &'static str = start.into();
        let trailing_string: String = vec![start_str, kind.into()].join("");
        let start_gap = trailing_string.len();
        let raw_string: String =
            trailing_string.clone() + content_string + "\n";
        let lex = Lexer::new(&raw_string);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let content = CommentLineContent {
            content: NoLineBreaksString::make(content_string).unwrap(),
        };
        let comment_line = CommentLine {
            kind,
            start,
            content,
            // -1 here as the len give us a bigger index
            span: (0, raw_string.len() - 1).into(),
        };
        let token = Token::LastComments(vec![comment_line.into()]);
        let expected = vec![Ok(token)];
        println!("result:   {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected)
    }

    #[test]
    fn line_comment_hypen() {
        make_line_comment_documentation_test(
            " some",
            CommentKind::NonDocumentation,
            LineCommentStart::DoubleHypen,
        )
    }

    #[test]
    fn line_comment_slash() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#'  commenting things",
            CommentKind::NonDocumentation,
            LineCommentStart::DoubleSlash,
        )
    }

    #[test]
    fn line_documentation_hypen() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#'  commenting things",
            CommentKind::Documentation,
            LineCommentStart::DoubleHypen,
        )
    }

    #[test]
    fn line_documentation_slash() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#'  commenting things",
            CommentKind::Documentation,
            LineCommentStart::DoubleSlash,
        )
    }

    fn make_block_test(
        content_string: &'static str,
        kind: CommentKind,
        brace: CommentBraceKind,
    ) {
        let kind_as_str: &str = kind.into();
        println!("kind: {:?} {:?}", kind, kind_as_str);
        let (start_str, end_str): (&'static str, &'static str) = brace.into();
        let trailing_string: String = vec![start_str, kind_as_str].join("");
        let start_gap = trailing_string.len();
        let content = CommentLineContent::decompose(content_string);
        let lexer_string = [&trailing_string, content_string, end_str].join("");
        let lex = Lexer::new(&lexer_string);
        let block = CommentBlock {
            kind,
            brace,
            content,
            span: (0, lexer_string.len() - 1).into(),
        };
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let token = Token::LastComments(vec![block.into()], todo!());
        let expected = vec![Ok(token)];
        println!("result:   {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected)
    }

    #[test]
    fn block0_comment() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::NonDocumentation,
            CommentBraceKind::Brace0,
        );
    }

    #[test]
    fn block1_comment() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::NonDocumentation,
            CommentBraceKind::Brace1,
        );
    }

    #[test]
    fn block2_comment() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::NonDocumentation,
            CommentBraceKind::Brace2,
        );
    }

    #[test]
    fn block3_comment() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::NonDocumentation,
            CommentBraceKind::Brace2,
        );
    }

    #[test]
    fn block0_documentation() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::Documentation,
            CommentBraceKind::Brace0,
        );
    }

    #[test]
    fn block1_documentation() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::Documentation,
            CommentBraceKind::Brace1,
        );
    }

    #[test]
    fn block2_documentation() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::Documentation,
            CommentBraceKind::Brace2,
        );
    }

    #[test]
    fn block3_documentation() {
        make_block_test(
            "some\ncontent\n in the \n 343 string to \n parse+sfd--asdf \n end",
            CommentKind::Documentation,
            CommentBraceKind::Brace2,
        );
    }
}
