use crate::base::{
    Between, Braces, Comma, ImportedVariable, OperatorName, Parens, Token,
    TokenInfo, TrailingList,
};
use crate::literals::{
    InterpolationString, StringLiteral, UFloatingPointLiteral, UintLiteral,
};
use crate::patterns::PatternMatch;
use crate::types::Type;
use octizys_common::identifier::Identifier;

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
    pub cases: Between<TrailingList<CaseItem, Comma>, Braces>,
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
    String(Token<StringLiteral>),
    InterpolationString(Token<InterpolationString>),
    //TODO: make the lexer return the right type instead of string?
    //The main problem is with floats and uints, they must be in
    //the range or we should issue a warning or error about
    //maximum literal
    Uint(Token<UintLiteral>),
    UFloat(Token<UFloatingPointLiteral>),
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    NamedHole(Token<u64>),
    Tuple(Between<TrailingList<Box<Expression>, Comma>, Parens>),
    Record(Between<TrailingList<ExpressionRecordItem, Comma>, Braces>),
    Case(Case),
    Parens(Between<Box<Expression>, Parens>),
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
