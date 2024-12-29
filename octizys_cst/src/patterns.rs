use crate::{
    base::{
        Between, Braces, Comma, ImportedVariable, Parens, Token, TokenInfo,
        TrailingList,
    },
    literals::{StringLiteral, UFloatingPointLiteral, UintLiteral},
};
use octizys_common::identifier::Identifier;

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
    String(Token<StringLiteral>),
    Uint(Token<UintLiteral>),
    UFloat(Token<UFloatingPointLiteral>),
    AnonHole(TokenInfo),
    Tuple(Between<TrailingList<Box<PatternMatch>, Comma>, Parens>),
    Record(Between<TrailingList<PatternMatchRecordItem, Comma>, Braces>),
    Bind(PatternMatchBind),
    Application {
        start: Box<PatternMatch>,
        second: Box<PatternMatch>,
        remain: Vec<PatternMatch>,
    },
    Parens(Between<Box<PatternMatch>, Parens>),
}
