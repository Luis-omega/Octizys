use crate::report::{LexerReportKind, OctizysParserReport, ParserReport};
use lalrpop_util::ParseError;
use octizys_common::span::Position;
use octizys_common::{
    identifier::Identifier,
    report::ReportKind,
    span::{HasLocation, Location},
};
use octizys_cst::{
    base::TokenInfo,
    comments::{Comment, CommentBlock, CommentLine},
    literals::{
        InterpolationString, StringLiteral, UFloatingPointLiteral, UintLiteral,
    },
    types::{OwnershipLiteral, OwnershipVariable},
};
use octizys_macros::Equivalence;
use paste::paste;

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
    ShiftRight,
    Map,
    MapConstRight,
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
    ReverseApplication,
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
    LineComment(CommentLine),
    BlockComment(CommentBlock),
    UintLiteral(UintLiteral),
    UFloatLiteral(UFloatingPointLiteral),
    StringLiteral(StringLiteral),
    StringInterpolation(InterpolationString),
    Identifier(Identifier),
    InfixIdentifier(Identifier),
    Selector(Identifier),
    AnonHole,
    NamedHole(u64),
    OwnershipLiteral(OwnershipLiteral),
    OwnershipVariable(OwnershipVariable),
}

#[derive(Debug, PartialEq, Eq, Clone, Equivalence)]
pub enum Token {
    Interrogation(#[equivalence(ignore)] TokenInfo),
    Exclamation(#[equivalence(ignore)] TokenInfo),
    Hash(#[equivalence(ignore)] TokenInfo),
    Comma(#[equivalence(ignore)] TokenInfo),
    Colon(#[equivalence(ignore)] TokenInfo),
    StatementEnd(#[equivalence(ignore)] TokenInfo),
    Dot(#[equivalence(ignore)] TokenInfo),
    ModuleSeparator(#[equivalence(ignore)] TokenInfo),
    Minus(#[equivalence(ignore)] TokenInfo),
    CompositionRight(#[equivalence(ignore)] TokenInfo),
    CompositionLeft(#[equivalence(ignore)] TokenInfo),
    Plus(#[equivalence(ignore)] TokenInfo),
    Power(#[equivalence(ignore)] TokenInfo),
    Star(#[equivalence(ignore)] TokenInfo),
    Div(#[equivalence(ignore)] TokenInfo),
    Module(#[equivalence(ignore)] TokenInfo),
    ShiftLeft(#[equivalence(ignore)] TokenInfo),
    ShiftRight(#[equivalence(ignore)] TokenInfo),
    Map(#[equivalence(ignore)] TokenInfo),
    MapConstRight(#[equivalence(ignore)] TokenInfo),
    MapConstLeft(#[equivalence(ignore)] TokenInfo),
    Appliative(#[equivalence(ignore)] TokenInfo),
    ApplicativeRight(#[equivalence(ignore)] TokenInfo),
    ApplicativeLeft(#[equivalence(ignore)] TokenInfo),
    Equality(#[equivalence(ignore)] TokenInfo),
    NotEqual(#[equivalence(ignore)] TokenInfo),
    LessOrEqual(#[equivalence(ignore)] TokenInfo),
    MoreOrEqual(#[equivalence(ignore)] TokenInfo),
    LessThan(#[equivalence(ignore)] TokenInfo),
    MoreThan(#[equivalence(ignore)] TokenInfo),
    And(#[equivalence(ignore)] TokenInfo),
    Or(#[equivalence(ignore)] TokenInfo),
    ReverseApplication(#[equivalence(ignore)] TokenInfo),
    DollarApplication(#[equivalence(ignore)] TokenInfo),
    Asignation(#[equivalence(ignore)] TokenInfo),
    At(#[equivalence(ignore)] TokenInfo),
    Pipe(#[equivalence(ignore)] TokenInfo),
    Alternative(#[equivalence(ignore)] TokenInfo),
    FlippedMap(#[equivalence(ignore)] TokenInfo),
    Annotate(#[equivalence(ignore)] TokenInfo),
    LParen(#[equivalence(ignore)] TokenInfo),
    RParen(#[equivalence(ignore)] TokenInfo),
    LBracket(#[equivalence(ignore)] TokenInfo),
    RBracket(#[equivalence(ignore)] TokenInfo),
    LBrace(#[equivalence(ignore)] TokenInfo),
    RBrace(#[equivalence(ignore)] TokenInfo),
    RightArrow(#[equivalence(ignore)] TokenInfo),
    LeftArrow(#[equivalence(ignore)] TokenInfo),
    LambdaStart(#[equivalence(ignore)] TokenInfo),
    Let(#[equivalence(ignore)] TokenInfo),
    In(#[equivalence(ignore)] TokenInfo),
    Case(#[equivalence(ignore)] TokenInfo),
    Of(#[equivalence(ignore)] TokenInfo),
    Import(#[equivalence(ignore)] TokenInfo),
    Data(#[equivalence(ignore)] TokenInfo),
    Newtype(#[equivalence(ignore)] TokenInfo),
    Class(#[equivalence(ignore)] TokenInfo),
    Instance(#[equivalence(ignore)] TokenInfo),
    Public(#[equivalence(ignore)] TokenInfo),
    Alias(#[equivalence(ignore)] TokenInfo),
    As(#[equivalence(ignore)] TokenInfo),
    Unqualified(#[equivalence(ignore)] TokenInfo),
    Forall(#[equivalence(ignore)] TokenInfo),
    Type(#[equivalence(ignore)] TokenInfo),
    U8(#[equivalence(ignore)] TokenInfo),
    U16(#[equivalence(ignore)] TokenInfo),
    U32(#[equivalence(ignore)] TokenInfo),
    U64(#[equivalence(ignore)] TokenInfo),
    I8(#[equivalence(ignore)] TokenInfo),
    I16(#[equivalence(ignore)] TokenInfo),
    I32(#[equivalence(ignore)] TokenInfo),
    I64(#[equivalence(ignore)] TokenInfo),
    F32(#[equivalence(ignore)] TokenInfo),
    F64(#[equivalence(ignore)] TokenInfo),
    CharType(#[equivalence(ignore)] TokenInfo),
    StringType(#[equivalence(ignore)] TokenInfo),
    Comment(#[equivalence(ignore)] TokenInfo, Comment),
    StringLiteral(#[equivalence(ignore)] TokenInfo, StringLiteral),
    StringInterpolation(#[equivalence(ignore)] TokenInfo, InterpolationString),
    UintLiteral(#[equivalence(ignore)] TokenInfo, UintLiteral),
    UFloatLiteral(#[equivalence(ignore)] TokenInfo, UFloatingPointLiteral),
    Identifier(#[equivalence(ignore)] TokenInfo, Identifier),
    InfixIdentifier(#[equivalence(ignore)] TokenInfo, Identifier),
    Selector(#[equivalence(ignore)] TokenInfo, Identifier),
    AnonHole(#[equivalence(ignore)] TokenInfo),
    NamedHole(#[equivalence(ignore)] TokenInfo, u64),
    LastComments(#[equivalence(ignore)] TokenInfo, Vec<Comment>),
    OwnershipLiteral(#[equivalence(ignore)] TokenInfo, OwnershipLiteral),
    OwnershipVariable(#[equivalence(ignore)] TokenInfo, OwnershipVariable),
}

impl HasLocation for Token {
    fn get_location(&self) -> Location {
        <&TokenInfo>::from(self).get_location()
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
            Token::ShiftRight(info) => (info),
            Token::Map(info) => (info),
            Token::MapConstRight(info) => (info),
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
            Token::ReverseApplication(info) => (info),
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
            Token::StringInterpolation(info, _) => info,
            Token::UintLiteral(info, _) => info,
            Token::UFloatLiteral(info, _) => info,
            Token::Identifier(info, _) => info,
            Token::InfixIdentifier(info, _) => info,
            Token::Selector(info, _) => info,
            Token::AnonHole(info) => info,
            Token::NamedHole(info, _) => info,
            Token::LastComments(info, _) => info,
            Token::OwnershipLiteral(info, _) => info,
            Token::OwnershipVariable(info, _) => info,
        }
    }
}

impl<'a> From<&'a Token> for &'a TokenInfo {
    fn from(value: &'a Token) -> &'a TokenInfo {
        match &value {
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
            Token::ShiftRight(info) => (info),
            Token::Map(info) => (info),
            Token::MapConstRight(info) => (info),
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
            Token::ReverseApplication(info) => (info),
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
            Token::StringInterpolation(info, _) => info,
            Token::UintLiteral(info, _) => info,
            Token::UFloatLiteral(info, _) => info,
            Token::Identifier(info, _) => info,
            Token::InfixIdentifier(info, _) => info,
            Token::Selector(info, _) => info,
            Token::AnonHole(info) => info,
            Token::NamedHole(info, _) => info,
            Token::LastComments(info, _) => info,
            Token::OwnershipLiteral(info, _) => info,
            Token::OwnershipVariable(info, _) => info,
        }
    }
}

macro_rules! make_lexer_token_to_token {
    ($name:tt, $output_constructor:tt, $output_type:tt) => {
        paste!{
            pub fn [< $name _token_to_token >](t:Token)->Result<octizys_cst::base::Token<$output_type>,ParseError<Position,Token,OctizysParserReport>>{
                match t {
                    Token::$output_constructor(info,value) => Ok(octizys_cst::base::Token{value,info}),
                    _ => Err(
                        ParseError::User{
                            error: OctizysParserReport{
                                kind: ReportKind::Error,
                                report:ParserReport::Lexer(LexerReportKind::CantTranslateToToken(t.clone())),
                                location: t.get_location()
                            },
                        }
                    )
                }
            }
        }

    };
}
make_lexer_token_to_token!(identifier, Identifier, Identifier);
make_lexer_token_to_token!(string, StringLiteral, StringLiteral);
make_lexer_token_to_token!(uint, UintLiteral, UintLiteral);
make_lexer_token_to_token!(
    interpolation,
    StringInterpolation,
    InterpolationString
);
make_lexer_token_to_token!(ufloat, UFloatLiteral, UFloatingPointLiteral);
make_lexer_token_to_token!(selector, Selector, Identifier);
make_lexer_token_to_token!(named_hole, NamedHole, u64);
make_lexer_token_to_token!(last_comment, Comment, Comment);

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
        BaseToken::ShiftRight => Token::ShiftRight(info),
        BaseToken::Map => Token::Map(info),
        BaseToken::MapConstRight => Token::MapConstRight(info),
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
        BaseToken::ReverseApplication => Token::ReverseApplication(info),
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
        BaseToken::LineComment(c) => Token::Comment(info, Comment::Line(c)),
        BaseToken::BlockComment(c) => Token::Comment(info, Comment::Block(c)),
        BaseToken::StringLiteral(s) => Token::StringLiteral(info, s),
        BaseToken::StringInterpolation(s) => {
            Token::StringInterpolation(info, s)
        }
        BaseToken::UintLiteral(s) => Token::UintLiteral(info, s),
        BaseToken::UFloatLiteral(s) => Token::UFloatLiteral(info, s),
        BaseToken::Identifier(s) => Token::Identifier(info, s),
        BaseToken::InfixIdentifier(s) => Token::InfixIdentifier(info, s),
        BaseToken::Selector(s) => Token::Selector(info, s),
        BaseToken::AnonHole => Token::AnonHole(info),
        BaseToken::NamedHole(s) => Token::NamedHole(info, s),
        BaseToken::OwnershipLiteral(s) => Token::OwnershipLiteral(info, s),
        BaseToken::OwnershipVariable(s) => Token::OwnershipVariable(info, s),
    }
}
