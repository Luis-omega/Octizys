use octizys_common::{
    identifier::Identifier,
    logic_path::LogicPath,
    span::{Position, Span},
};
use octizys_cst::base::{TokenInfo, TokenInfoWithPhantom};
use octizys_cst::comments::{
    Comment, CommentBlock, CommentBraceKind, CommentLine, CommentsInfo,
};
use octizys_text_store::store::Store;

use lalrpop_util::ParseError;
use paste::paste;
use regex::{Captures, Match, Regex};
use std::sync::LazyLock;

/// We lex the stream in two phases, the first one retrieve a
/// iterator of this type.
#[derive(Debug, Clone)]
pub enum BaseToken {
    Interrogation,
    Exclamation,
    Hash,
    Comma,
    Colon,
    StatementEnd,
    Dot,
    LogicPathSeparator,
    Minus,
    CompositionRight,
    CompositionLeft,
    Plus,
    Power,
    Star,
    Div,
    Module,
    ShiftLeft,
    ShiftRigth,
    Map,
    MapConstRigth,
    MapConstLeft,
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
    Alternative,
    FlippedMap,
    Annotate,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    RightArrow,
    LeftArrow,
    LambdaStart,
    Let,
    In,
    Case,
    Of,
    Import,
    Data,
    Newtype,
    Class,
    Instance,
    Public,
    Alias,
    As,
    Unqualified,
    Forall,
    Type,
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
    CharType,
    StringType,
    LineComment(CommentLine),
    BlockComment(CommentBlock),
    UintLiteral(String),
    UFloatLiteral(String),
    Identifier(Identifier),
    InfixIdentifier(Identifier),
    Selector(Identifier),
    AnonHole,
    NamedHole(u64),
}

#[derive(Debug, Clone)]
pub enum LexerError {
    UnexpectedCharacter(Position),
    /// Punctuation match matched a non know character
    /// This is a bug.
    UnexpectedPunctuationMatch(String, Span),
    Notu64NamedHole(String, Span),
    CantCreateIdentifier(String, Span),
    CantTranslateToToken(Token),
}

/*
            "\s"  // all spaces
            "\n"  // line break

            "?" => Ok(Token::Interrogation(info)),
            "#" => Ok(Token::Hash(info)),
            "," => Ok(Token::Comma(info)),
            ";" => Ok(Token::StatementEnd(info)),
            "+" => Ok(Token::Plus(info)),
            "^" => Ok(Token::Power(info)),
            "%" => Ok(Token::Module(info)),
            "@" => Ok(Token::At(info)),

            SELECTOR
            "." => Ok(Token::Dot(info)),

            ":" => Ok(Token::Colon(info)),
            "::" => Ok(Token::ModuleSeparator(info)),

            "-" => Ok(Token::Minus(info)),
            "-}" //close comment block
            "->" => Ok(Token::RightArrow(info)),
            "--" // close comment block
            "--}" //close comment block
            "---}" //close comment block
            "-- |"
            "----}" //close comment block


            "|" => Ok(Token::Pipe(info)),
            "|>" => Ok(Token::CompositionLeft(info)),
            "||" => Ok(Token::Or(info)),

            "<" => Ok(Token::LessThan(info)),
            "<|" => Ok(Token::CompositionRight(info)),
            "<|>" //TODO
            "<=" => Ok(Token::LessOrEqual(info)),
            "<-" => Ok(Token::LeftArrow(info)),
            "<$" => Ok(Token::MapConstLeft(info)),
            "<$>" => Ok(Token::Map(info)),
            "<*" => Ok(Token::ApplicativeLeft(info)),
            "<<" => ShiftLeft,
            "<*>" => Ok(Token::Appliative(info)),
            "<&>" //TODO
            "<?>" //TODO
//
            "*" => Ok(Token::Star(info)),
            "*>" => Ok(Token::ApplicativeRight(info)),

            "=" => Ok(Token::Asignation(info)),
            "==" => Ok(Token::Equality(info)),

            "!" => Ok(Token::Exclamation(info)),
            "!=" => Ok(Token::NotEqual(info)),

            ">" => Ok(Token::MoreThan(info)),
            ">>" => Ok(Token::ShiftRigth(info)),
            ">=" => Ok(Token::MoreOrEqual(info)),

            "/" => Ok(Token::Div(info)),
            "//" // comments

            "&&" => Ok(Token::And(info)),
            "&" => Ok(Token::ReverseAppliation(info)),

            "$" => Ok(Token::DollarApplication(info)),
            "$>" => Ok(Token::MapConstRigth(info)),

            "(" => Ok(Token::LParen(info)),
            ")" => Ok(Token::RParen(info)),

            "[" => Ok(Token::LBracket(info)),
            "]" => Ok(Token::RBracket(info)),


            "{" => Ok(Token::LBrace(info)),
            "{-" //comment block starts
            "{--"
            "{---"
            "{----"

            "}" => Ok(Token::RBrace(info)),


            "\\" => Ok(Token::LambdaStart(info)),

            UINT_WITH_TYPE // like 10_usize or 88_u32
            UINT
            FLOAT //shares conflict with two.
            "0(0|_)*" // Zero
            "0o[0-9]+" // octal
            "0x[0-9]+" // hex

            "'" // char?

            "\"" // string start or ends
    HOLE
    NamedHole
    Identifier  // shares start with two groups
    Ownership_variables // IDENTIFER'

            "f#\"" //interpolation string

            "r#\"" //raw string start
            "r##\""
            "r###\""
            "r####\""

            "\"#" //raw/interpolation string end
            "\"##"
            "\"###"
            "\"####"

    INFIX_IDENTIFIER


    //TODO: add pragmas syntax

    //TODO: write a spec with all the symbols and meanings
    //TODO: define unit test for all
*/

/// An abstraction for a [`Stream`] of characters over a [`str`].
#[derive(Debug)]
pub struct BaseLexerContext<'store, 'source> {
    /// A pointer to the original source
    source: &'source str,
    /// A pointer to the remain of the source
    index: &'source str,
    /// The source_index is the bytes position, not the number of chars!
    /// or the graphemes.
    /// The column line is the amount of bytes up to the point, not
    /// the amount of chars or graphemes.
    position: Position,
    /// Saves the line of the last token
    last_line: usize,
    /// A storage for internalization of strings.
    store: &'store mut Store,
}

impl<'store, 'source> BaseLexerContext<'store, 'source> {
    //TODO: make this function capable to start everywere in the source
    pub fn new(source: &'source str, store: &'store mut Store) -> Self {
        BaseLexerContext {
            source,
            index: source,
            position: Position::default(),
            last_line: 0,
            store,
        }
    }

    fn advance_with_line_breaks(&mut self, s: &str) -> Span {
        println!("Advancing spaces! {s},size={:}", s.len());
        let start = self.position;
        let len = s.len();
        self.position.source_index = self.position.source_index + len;
        for c in s.chars() {
            match c {
                '\n' => {
                    self.position.line += 1;
                    self.position.column = 0;
                }
                _ => self.position.column += c.len_utf8(),
            }
        }
        self.index = &self.index[len..];
        Span {
            start,
            end: self.position,
        }
    }

    fn advance_non_line_breaks(&mut self, s: &str) -> Span {
        println!("Advancing! {:}", s);
        let start = self.position;
        let len = s.len();
        self.position.column = self.position.column + len;
        self.position.source_index = self.position.source_index + len;
        self.index = &self.index[len..];
        Span {
            start,
            end: self.position,
        }
    }

    fn consume_spaces(&mut self) -> Span {
        match SPACES_REGEX.find(&self.index) {
            Some(spaces) => self.advance_with_line_breaks(spaces.as_str()),
            None => Span::from((self.position, self.position)),
        }
    }

    fn comment(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn punctuation_or_operator(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        Some(match matched {
            "\\" => Ok((span, BaseToken::LambdaStart)),
            "/" => Ok((span, BaseToken::Div)),
            "#" => Ok((span, BaseToken::Hash)),
            "," => Ok((span, BaseToken::Comma)),
            ";" => Ok((span, BaseToken::StatementEnd)),
            "+" => Ok((span, BaseToken::Plus)),
            "^" => Ok((span, BaseToken::Power)),
            "%" => Ok((span, BaseToken::Module)),
            "@" => Ok((span, BaseToken::At)),
            "." => Ok((span, BaseToken::Dot)),
            ":" => Ok((span, BaseToken::Colon)),
            "::" => Ok((span, BaseToken::LogicPathSeparator)),
            "-" => Ok((span, BaseToken::Minus)),
            "->" => Ok((span, BaseToken::RightArrow)),
            "|" => Ok((span, BaseToken::Pipe)),
            "<|" => Ok((span, BaseToken::CompositionRight)),
            "<|>" => Ok((span, BaseToken::Alternative)),
            "<=" => Ok((span, BaseToken::LessOrEqual)),
            "<-" => Ok((span, BaseToken::LeftArrow)),
            "<$" => Ok((span, BaseToken::MapConstLeft)),
            "<$>" => Ok((span, BaseToken::Map)),
            "<*" => Ok((span, BaseToken::ApplicativeLeft)),
            "<<" => Ok((span, BaseToken::ShiftLeft)),
            "<*>" => Ok((span, BaseToken::Appliative)),
            "<&>" => Ok((span, BaseToken::FlippedMap)),
            "<?>" => Ok((span, BaseToken::Annotate)),
            "*" => Ok((span, BaseToken::Star)),
            "*>" => Ok((span, BaseToken::ApplicativeRight)),
            "=" => Ok((span, BaseToken::Asignation)),
            "==" => Ok((span, BaseToken::Equality)),
            "!" => Ok((span, BaseToken::Exclamation)),
            "!=" => Ok((span, BaseToken::NotEqual)),
            ">" => Ok((span, BaseToken::MoreThan)),
            ">>" => Ok((span, BaseToken::ShiftRigth)),
            ">=" => Ok((span, BaseToken::MoreOrEqual)),
            "&&" => Ok((span, BaseToken::And)),
            "&" => Ok((span, BaseToken::ReverseAppliation)),
            "$" => Ok((span, BaseToken::DollarApplication)),
            "$>" => Ok((span, BaseToken::MapConstRigth)),
            _ => Err(LexerError::UnexpectedPunctuationMatch(
                matched.to_string(),
                span,
            )),
        })
    }

    fn bracket_start(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        Some(match matched {
            "(" => Ok((span, BaseToken::LParen)),
            "[" => Ok((span, BaseToken::LBracket)),
            "{" => Ok((span, BaseToken::LBrace)),
            _ => Err(LexerError::UnexpectedPunctuationMatch(
                matched.to_string(),
                span,
            )),
        })
    }

    fn bracket_end(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        Some(match matched {
            ")" => Ok((span, BaseToken::RParen)),
            "]" => Ok((span, BaseToken::RBracket)),
            "}" => Ok((span, BaseToken::RBrace)),
            _ => Err(LexerError::UnexpectedPunctuationMatch(
                matched.to_string(),
                span,
            )),
        })
    }

    fn string_start(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn named_hole(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched[1..].parse::<u64>() {
            Ok(x) => Some(Ok((span, BaseToken::NamedHole(x)))),
            Err(_) => Some(Err(LexerError::Notu64NamedHole(
                matched.to_string(),
                span,
            ))),
        }
    }

    fn identifier(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched {
            "let" => Some(Ok((span, BaseToken::Let))),
            "in" => Some(Ok((span, BaseToken::In))),
            "case" => Some(Ok((span, BaseToken::Case))),
            "of" => Some(Ok((span, BaseToken::Of))),
            "import" => Some(Ok((span, BaseToken::Import))),
            "data" => Some(Ok((span, BaseToken::Data))),
            "newtype" => Some(Ok((span, BaseToken::Newtype))),
            "class" => Some(Ok((span, BaseToken::Class))),
            "instance" => Some(Ok((span, BaseToken::Instance))),
            "public" => Some(Ok((span, BaseToken::Public))),
            "alias" => Some(Ok((span, BaseToken::Alias))),
            "As" => Some(Ok((span, BaseToken::As))),
            "unqualified" => Some(Ok((span, BaseToken::Unqualified))),
            "forall" => Some(Ok((span, BaseToken::Forall))),
            "type" => Some(Ok((span, BaseToken::Type))),
            "U8" => Some(Ok((span, BaseToken::U8))),
            "U16" => Some(Ok((span, BaseToken::U16))),
            "U32" => Some(Ok((span, BaseToken::U32))),
            "U64" => Some(Ok((span, BaseToken::U64))),
            "I8" => Some(Ok((span, BaseToken::I8))),
            "I16" => Some(Ok((span, BaseToken::I16))),
            "I32" => Some(Ok((span, BaseToken::I32))),
            "I64" => Some(Ok((span, BaseToken::I64))),
            "F32" => Some(Ok((span, BaseToken::F32))),
            "F64" => Some(Ok((span, BaseToken::F64))),
            "Char" => Some(Ok((span, BaseToken::CharType))),
            "String" => Some(Ok((span, BaseToken::StringType))),
            _ => match Identifier::make(matched, self.store) {
                Ok(iden) => Some(Ok((span, BaseToken::Identifier(iden)))),
                _ => Some(Err(LexerError::CantCreateIdentifier(
                    matched.to_string(),
                    span,
                ))),
            },
        }
    }

    fn infix_identifier(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match Identifier::make(&matched[1..(matched.len() - 1)], self.store) {
            Ok(iden) => Some(Ok((span, BaseToken::InfixIdentifier(iden)))),
            _ => Some(Err(LexerError::CantCreateIdentifier(
                matched.to_string(),
                span,
            ))),
        }
    }

    fn anon_hole(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        Some(Ok((span, BaseToken::AnonHole)))
    }

    fn character(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn ownership_literal(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn ownership_variable(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn octal(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn hex(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }

    fn numeric(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), LexerError>> {
        todo!()
    }
}

const MAIN_REGEX_STR: &'static str = r#"^((?<comment_start>//|--|\{----|\{---|\{--|\{-)|(?<punctuation_or_operator>\\|/|#|,|;|\?|\+|\^|%|\.|::|:|->|-|\|\||\|>|\||<\?>|<&>|<<|<\*>|<\*|<\$>|<\$|<-|<=|<\|>|<\||<|\*>|\*|==|=|!=|!|>=|>>|>|&&|&|\$>|\$|@)|(?<bracket_start>\(|\[|\{)|(?<bracket_end>\)|\]|\})|(?<string_start>f#"|r####"|r###"|r##"|r#"|")|(?<named_hole>_[0-9][0-9_]*)|(?<identifier>_*\p{XID_START}\p{XID_CONTINUE}*)|(?<infix_identifier>`_*\p{XID_START}\p{XID_CONTINUE}*`)|(?<anon_hole>_)|(?<char>'([^'\\]|\\'|\\\\)')|(?<ownership_literal>'(0|1|inf))|(?<ownership_variable>'_*\p{XID_START}\p{XID_CONTINUE}*)|(?<octal>0o[0-7][0-7_]*)|(?<hex>0x[0-9a-fA-F][0-9a-fA-F_]*)|(?<numeric>[0-9][0-9_]*(?<decimal_part>\.[0-9][0-9_]*(?<exponential_part>(e|E)(?<sign>\+|-)?[0-9][0-9_]*)?)?))"#;

const MAIN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(MAIN_REGEX_STR).unwrap());

const SPACES_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+"#).unwrap());

fn find_match_group<'source, 'store, 'context>(
    c: Captures,
    blc: &'context mut BaseLexerContext<'store, 'source>,
    v: Vec<(
        &'static str,
        fn(
            &'context mut BaseLexerContext<'store, 'source>,
            m: Match,
        ) -> Option<Result<(Span, BaseToken), LexerError>>,
    )>,
) -> Option<Result<(Span, BaseToken), LexerError>> {
    let mut out = Some(Err(LexerError::UnexpectedCharacter(blc.position)));
    for (name, f) in v {
        match c.name(name) {
            Some(m) => return f(blc, m),
            None => (),
        }
    }

    out
}

impl<'store, 'source> Iterator for BaseLexerContext<'store, 'source> {
    type Item = Result<(Span, BaseToken), LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.consume_spaces();
        println!("BASE_CONTEXT:{:?}", self);
        if self.index.len() == 0 {
            return None;
        }
        match MAIN_REGEX.captures(self.index) {
            Some(captures) => find_match_group(
                captures,
                self,
                vec![
                    ("comment", BaseLexerContext::comment),
                    (
                        "punctuation_or_operator",
                        BaseLexerContext::punctuation_or_operator,
                    ),
                    ("bracket_start", BaseLexerContext::bracket_start),
                    ("bracket_end", BaseLexerContext::bracket_end),
                    ("string_start", BaseLexerContext::string_start),
                    ("named_hole", BaseLexerContext::named_hole),
                    ("identifier", BaseLexerContext::identifier),
                    ("infix_identifier", BaseLexerContext::infix_identifier),
                    ("anon_hole", BaseLexerContext::anon_hole),
                    ("character", BaseLexerContext::character),
                    ("ownership_literal", BaseLexerContext::ownership_literal),
                    (
                        "ownership_variable",
                        BaseLexerContext::ownership_variable,
                    ),
                    ("octal", BaseLexerContext::octal),
                    ("hex", BaseLexerContext::hex),
                    ("numeric", BaseLexerContext::numeric),
                ],
            ),
            None => Some(Err(LexerError::UnexpectedCharacter(self.position))),
        }
    }
}

#[derive(Debug)]
pub struct LexerContext<'src, 'store> {
    previous_token: Option<Result<(Span, BaseToken), LexerError>>,
    lexer: &'src mut BaseLexerContext<'store, 'src>,
}

impl<'src, 'store> LexerContext<'src, 'store> {
    pub fn new(
        previous_token: Option<Result<(Span, BaseToken), LexerError>>,
        lexer: &'src mut BaseLexerContext<'store, 'src>,
    ) -> Self {
        LexerContext {
            previous_token,
            lexer,
        }
    }
}

pub fn accumulate_comments<
    I: Iterator<Item = Result<(Span, BaseToken), LexerError>>,
>(
    lexer: &mut I,
    acc: &mut Vec<Comment>,
) -> Option<Result<(Span, BaseToken), LexerError>> {
    let mut out: Option<Result<(Span, BaseToken), LexerError>> = None;
    loop {
        match lexer.next() {
            Some(maybe_token) => match maybe_token {
                Ok((span, token)) => match token {
                    BaseToken::LineComment(l) => {
                        acc.push(Comment::Line(l));
                    }
                    BaseToken::BlockComment(b) => {
                        acc.push(Comment::Block(b));
                    }
                    _ => {
                        out = Some(Ok((span, token)));
                        break;
                    }
                },
                Err(e) => {
                    out = Some(Err(e));
                    break;
                }
            },
            None => break,
        }
    }
    out
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
    Alternative(TokenInfo),
    FlippedMap(TokenInfo),
    Annotate(TokenInfo),
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
            Token::Alternative(info) => (info),
            Token::FlippedMap(info) => (info),
            Token::Annotate(info) => (info),
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
            Token::LastComments(info, _) => info,
        }
    }
}

macro_rules! make_lexer_token_to_token {
    ($name:tt, $output_constructor:tt, $output_type:tt) => {
        paste!{
            pub fn [< $name _token_to_token >](t:Token)->Result<octizys_cst::base::Token<$output_type>,ParseError<Position,Token,LexerError>>{
                match t {
                    Token::$output_constructor(info,value) => Ok(octizys_cst::base::Token{value,info}),
                    _ => Err(
                        ParseError::User{
                            error: LexerError::CantTranslateToToken(t.clone())
                            }
                        )
                }
            }
        }

    };
}
make_lexer_token_to_token!(identifier, Identifier, Identifier);
make_lexer_token_to_token!(string, StringLiteral, String);
make_lexer_token_to_token!(char, CharacterLiteral, String);
make_lexer_token_to_token!(uint, UintLiteral, String);
make_lexer_token_to_token!(ufloat, UFloatLiteral, String);
make_lexer_token_to_token!(selector, Selector, Identifier);
make_lexer_token_to_token!(named_hole, NamedHole, u64);

pub fn aux_base_token_to_token(
    base_token: BaseToken,
    mut info: TokenInfo,
) -> Token {
    match base_token {
        BaseToken::Interrogation => Token::Interrogation(info),
        BaseToken::Exclamation => Token::Exclamation(info),
        BaseToken::Hash => Token::Hash(info),
        BaseToken::Comma => Token::Comma(info),
        BaseToken::Colon => Token::Colon(info),
        BaseToken::StatementEnd => Token::StatementEnd(info),
        BaseToken::Dot => Token::Dot(info),
        BaseToken::LogicPathSeparator => Token::ModuleSeparator(info),
        BaseToken::Minus => Token::Minus(info),
        BaseToken::CompositionRight => Token::CompositionRight(info),
        BaseToken::CompositionLeft => Token::CompositionLeft(info),
        BaseToken::Plus => Token::Plus(info),
        BaseToken::Power => Token::Power(info),
        BaseToken::Star => Token::Star(info),
        BaseToken::Div => Token::Div(info),
        BaseToken::Module => Token::Module(info),
        BaseToken::ShiftLeft => Token::ShiftLeft(info),
        BaseToken::ShiftRigth => Token::ShiftRigth(info),
        BaseToken::Map => Token::Map(info),
        BaseToken::MapConstRigth => Token::MapConstRigth(info),
        BaseToken::MapConstLeft => Token::MapConstLeft(info),
        BaseToken::Appliative => Token::Appliative(info),
        BaseToken::ApplicativeRight => Token::ApplicativeRight(info),
        BaseToken::ApplicativeLeft => Token::ApplicativeLeft(info),
        BaseToken::Equality => Token::Equality(info),
        BaseToken::NotEqual => Token::NotEqual(info),
        BaseToken::LessOrEqual => Token::LessOrEqual(info),
        BaseToken::MoreOrEqual => Token::MoreOrEqual(info),
        BaseToken::LessThan => Token::LessThan(info),
        BaseToken::MoreThan => Token::MoreThan(info),
        BaseToken::And => Token::And(info),
        BaseToken::Or => Token::Or(info),
        BaseToken::ReverseAppliation => Token::ReverseAppliation(info),
        BaseToken::DollarApplication => Token::DollarApplication(info),
        BaseToken::Asignation => Token::Asignation(info),
        BaseToken::At => Token::At(info),
        BaseToken::Pipe => Token::Pipe(info),
        BaseToken::Alternative => Token::Alternative(info),
        BaseToken::FlippedMap => Token::FlippedMap(info),
        BaseToken::Annotate => Token::Annotate(info),
        BaseToken::LParen => Token::LParen(info),
        BaseToken::RParen => Token::RParen(info),
        BaseToken::LBracket => Token::LBracket(info),
        BaseToken::RBracket => Token::RBracket(info),
        BaseToken::LBrace => Token::LBrace(info),
        BaseToken::RBrace => Token::RBrace(info),
        BaseToken::RightArrow => Token::RightArrow(info),
        BaseToken::LeftArrow => Token::LeftArrow(info),
        BaseToken::LambdaStart => Token::LambdaStart(info),
        BaseToken::Let => Token::Let(info),
        BaseToken::In => Token::In(info),
        BaseToken::Case => Token::Case(info),
        BaseToken::Of => Token::Of(info),
        BaseToken::Import => Token::Import(info),
        BaseToken::Data => Token::Data(info),
        BaseToken::Newtype => Token::Newtype(info),
        BaseToken::Class => Token::Class(info),
        BaseToken::Instance => Token::Instance(info),
        BaseToken::Public => Token::Public(info),
        BaseToken::Alias => Token::Alias(info),
        BaseToken::As => Token::As(info),
        BaseToken::Unqualified => Token::Unqualified(info),
        BaseToken::Forall => Token::Forall(info),
        BaseToken::Type => Token::Type(info),
        BaseToken::U8 => Token::U8(info),
        BaseToken::U16 => Token::U16(info),
        BaseToken::U32 => Token::U32(info),
        BaseToken::U64 => Token::U64(info),
        BaseToken::I8 => Token::I8(info),
        BaseToken::I16 => Token::I16(info),
        BaseToken::I32 => Token::I32(info),
        BaseToken::I64 => Token::I64(info),
        BaseToken::F32 => Token::F32(info),
        BaseToken::F64 => Token::F64(info),
        BaseToken::CharType => Token::CharType(info),
        BaseToken::StringType => Token::StringType(info),
        BaseToken::LineComment(c) => Token::Comment(info, Comment::Line(c)),
        BaseToken::BlockComment(c) => Token::Comment(info, Comment::Block(c)),
        BaseToken::UintLiteral(s) => Token::UintLiteral(info, s),
        BaseToken::UFloatLiteral(s) => Token::UFloatLiteral(info, s),
        BaseToken::Identifier(s) => Token::Identifier(info, s),
        BaseToken::InfixIdentifier(s) => Token::InfixIdentifier(info, s),
        BaseToken::Selector(s) => Token::Selector(info, s),
        BaseToken::AnonHole => Token::AnonHole(info),
        BaseToken::NamedHole(s) => Token::NamedHole(info, s),
    }
}

//TODO: In case we have multiple block comments in a line
// this algorithm will store the second, third, ..., comments to the
// next token. Is this what we want? is a uncommon case but still need to be handled.
/// We already consumed all the comments before a token, and we
/// want to complete the given [`current_token`].
/// If the next token is a comment in the same line as the [`current_token`] we set the after comments.
/// If the next token is another kind of token we store it in the proviuos and emit the [`current_token`].
pub fn complete_token_or_save(
    current_token: BaseToken,
    context: &mut LexerContext,
    mut info: TokenInfo,
) -> Option<Result<(Position, Token, Position), LexerError>> {
    match context.lexer.next() {
        Some(maybe_next_base_token) => match maybe_next_base_token {
            Ok((span, next_base_token)) => match next_base_token.clone() {
                //TODO: FIXME: fix the spans
                BaseToken::LineComment(l) => {
                    let span = info.span.clone();
                    if info.span.end.line == l.span.start.line {
                        info.comments.after = Some(Comment::Line(l));
                        context.previous_token = None;
                    } else {
                        context.previous_token =
                            Some(Ok((span, next_base_token)));
                    }
                    let mut token =
                        aux_base_token_to_token(current_token, info);
                    Some(Ok((span.start, token, span.end)))
                }
                BaseToken::BlockComment(c) => {
                    let span = info.span.clone();
                    if info.span.end.line == c.span.start.line {
                        info.comments.after = Some(Comment::Block(c));
                        context.previous_token = None;
                    } else {
                        context.previous_token =
                            Some(Ok((span, next_base_token)));
                    }
                    let mut token =
                        aux_base_token_to_token(current_token, info);
                    Some(Ok((span.start, token, span.end)))
                }
                _ => {
                    context.previous_token = Some(Ok((span, next_base_token)));
                    let mut token =
                        aux_base_token_to_token(current_token, info);
                    Some(Ok((span.start, token, span.end)))
                }
            },
            Err(e) => {
                context.previous_token = Some(Err(e));
                let span = info.span;
                let mut token = aux_base_token_to_token(current_token, info);
                Some(Ok((span.start, token, span.end)))
            }
        },
        None => {
            context.previous_token = None;
            let span = info.span;
            let mut token = aux_base_token_to_token(current_token, info);
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

/// Take a vector of previously accumulated comments, lookup for the
/// rest of the comments before a token.
/// We either complete a token, emit the last comment token or have a failure.
fn accumulate_advance_with(
    context: &mut LexerContext,
    mut acc: Vec<Comment>,
) -> Option<Result<(Position, Token, Position), LexerError>> {
    let current_value = accumulate_comments(&mut context.lexer, &mut acc);
    match current_value {
        Some(Ok((span, current_token))) => {
            let mut comments_info_pre = CommentsInfo {
                before: acc,
                after: None,
            };
            let mut info =
                TokenInfo::make(comments_info_pre, span.start, span.end);
            complete_token_or_save(current_token, context, info)
        }
        //Reached eof in the lexer
        None => compact_comments_info(acc),
        Some(Err(e)) => Some(Err(e)),
    }
}

impl<'store, 'src> Iterator for LexerContext<'store, 'src> {
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
        //println!("{:?}", self);
        match &self.previous_token {
            Some(previous) => match previous {
                //TODO: catch the error if it is the default value and
                //cast to a error with position
                Err(e) => Some(Err(e.to_owned())),
                // We had a previous token that may be a comment
                // that isn't attached or any other token.
                // We Handle the comment case and delegate to complete_token_or_save
                Ok((span, BaseToken::LineComment(l))) => {
                    let mut acc = vec![Comment::Line(l.to_owned())];
                    accumulate_advance_with(self, acc)
                }
                Ok((span, BaseToken::BlockComment(c))) => {
                    let mut acc = vec![Comment::Block(c.to_owned())];
                    accumulate_advance_with(self, acc)
                }
                // We stored previously a non comment token
                // and we are going to emit it after looking up
                // for the after comment.
                Ok((span, current_base_token)) => {
                    let mut comments_info_pre = CommentsInfo {
                        before: vec![],
                        after: None,
                    };
                    let mut info = TokenInfo::make(
                        comments_info_pre,
                        span.start,
                        span.end,
                    );
                    complete_token_or_save(
                        current_base_token.to_owned(),
                        self,
                        info,
                    )
                }
            },
            None => {
                // Didn't have previous value, either we got to the
                // end of the stream or we completed a token.
                let mut acc = vec![];
                accumulate_advance_with(self, acc)
            }
        }
    }
}

#[cfg(test)]
mod test_regex {
    use super::*;

    fn main_regex_with(input: &str, capture_name: &str) {
        let result = super::MAIN_REGEX
            .captures(input)
            .expect(&format!("Can't find : {}", input));
        println!("{:?}", result);
        assert_eq!(result.get(0).unwrap().as_str(), input);
        assert_eq!(&result[capture_name], input)
    }

    #[test]
    fn comment_start() {
        main_regex_with("//", "comment_start");
        main_regex_with("--", "comment_start");
        main_regex_with("{-", "comment_start");
        main_regex_with("{--", "comment_start");
        main_regex_with("{---", "comment_start");
        main_regex_with("{----", "comment_start");
    }

    #[test]
    fn punctuation_or_operator() {
        main_regex_with("\\", "punctuation_or_operator");
        main_regex_with("/", "punctuation_or_operator");
        main_regex_with("#", "punctuation_or_operator");
        main_regex_with(",", "punctuation_or_operator");
        main_regex_with(";", "punctuation_or_operator");
        main_regex_with("+", "punctuation_or_operator");
        main_regex_with("^", "punctuation_or_operator");
        main_regex_with("%", "punctuation_or_operator");
        main_regex_with("@", "punctuation_or_operator");
        main_regex_with(".", "punctuation_or_operator");
        main_regex_with(":", "punctuation_or_operator");
        main_regex_with("::", "punctuation_or_operator");
        main_regex_with("-", "punctuation_or_operator");
        main_regex_with("->", "punctuation_or_operator");
        main_regex_with("|", "punctuation_or_operator");
        main_regex_with("<|", "punctuation_or_operator");
        main_regex_with("<|>", "punctuation_or_operator");
        main_regex_with("<=", "punctuation_or_operator");
        main_regex_with("<-", "punctuation_or_operator");
        main_regex_with("<$", "punctuation_or_operator");
        main_regex_with("<$>", "punctuation_or_operator");
        main_regex_with("<*", "punctuation_or_operator");
        main_regex_with("<<", "punctuation_or_operator");
        main_regex_with("<*>", "punctuation_or_operator");
        main_regex_with("<&>", "punctuation_or_operator");
        main_regex_with("<?>", "punctuation_or_operator");
        main_regex_with("*", "punctuation_or_operator");
        main_regex_with("*>", "punctuation_or_operator");
        main_regex_with("=", "punctuation_or_operator");
        main_regex_with("==", "punctuation_or_operator");
        main_regex_with("!", "punctuation_or_operator");
        main_regex_with("!=", "punctuation_or_operator");
        main_regex_with(">", "punctuation_or_operator");
        main_regex_with(">>", "punctuation_or_operator");
        main_regex_with(">=", "punctuation_or_operator");
        main_regex_with("&&", "punctuation_or_operator");
        main_regex_with("&", "punctuation_or_operator");
        main_regex_with("$", "punctuation_or_operator");
        main_regex_with("$>", "punctuation_or_operator");
    }

    #[test]
    fn bracket_start() {
        main_regex_with("(", "bracket_start");
        main_regex_with("{", "bracket_start");
        main_regex_with("[", "bracket_start")
    }

    #[test]
    fn bracket_end() {
        main_regex_with(")", "bracket_end");
        main_regex_with("}", "bracket_end");
        main_regex_with("]", "bracket_end")
    }

    #[test]
    fn string_start() {
        main_regex_with("\"", "string_start");
        main_regex_with("f#\"", "string_start");
        main_regex_with("r#\"", "string_start");
        main_regex_with("r##\"", "string_start");
        main_regex_with("r###\"", "string_start");
        main_regex_with("r####\"", "string_start");
    }

    #[test]
    fn named_hole() {
        main_regex_with("_0", "named_hole");
        main_regex_with("_0_0", "named_hole");
        main_regex_with("_89342", "named_hole");
    }

    #[test]
    // Sync with infix_identifier
    fn identifier() {
        main_regex_with("_a", "identifier");
        main_regex_with("ab", "identifier");
        main_regex_with("a", "identifier");
        main_regex_with("à", "identifier");
        main_regex_with("àbc", "identifier");
        main_regex_with("____a", "identifier");
        main_regex_with("_b23d", "identifier");
    }

    #[test]
    // Sync with infix_identifier
    fn infix_identifier() {
        main_regex_with("`_a`", "infix_identifier");
        main_regex_with("`ab`", "infix_identifier");
        main_regex_with("`a`", "infix_identifier");
        main_regex_with("`à`", "infix_identifier");
        main_regex_with("`àbc`", "infix_identifier");
        main_regex_with("`____a`", "infix_identifier");
        main_regex_with("`_b23d`", "infix_identifier");
    }

    #[test]
    fn anon_hole() {
        main_regex_with("_", "anon_hole");
    }

    #[test]
    fn character() {
        main_regex_with("'\\''", "char");
        main_regex_with("'\\\\'", "char");
        main_regex_with("'a'", "char");
        main_regex_with("'à'", "char");
    }

    #[test]
    fn ownership_literal() {
        main_regex_with("'0", "ownership_literal");
        main_regex_with("'1", "ownership_literal");
        main_regex_with("'inf", "ownership_literal");
    }

    #[test]
    fn ownership_variable() {
        main_regex_with("'_a", "ownership_variable");
        main_regex_with("'ab", "ownership_variable");
        main_regex_with("'a", "ownership_variable");
        main_regex_with("'à", "ownership_variable");
        main_regex_with("'àbc", "ownership_variable");
        main_regex_with("____a", "identifier");
        main_regex_with("_b23d", "identifier");
    }

    #[test]
    fn octal() {
        main_regex_with("0o0", "octal");
        main_regex_with("0o00", "octal");
        main_regex_with("0o1", "octal");
        main_regex_with("0o2_3_4", "octal");
        main_regex_with("0o1234", "octal");
    }

    #[test]
    fn hex() {
        main_regex_with("0x0", "hex");
        main_regex_with("0x00", "hex");
        main_regex_with("0x1", "hex");
        main_regex_with("0x2_3_4", "hex");
        main_regex_with("0x1234", "hex");
    }

    #[test]
    fn numeric() {
        main_regex_with("0", "numeric");
        main_regex_with("1", "numeric");
        main_regex_with("2_3_4", "numeric");
        main_regex_with("34345.384e32", "numeric");
        main_regex_with("0_89_79.52218E-32", "numeric");
        main_regex_with("839_3479.52788e+32", "numeric");
    }
}
