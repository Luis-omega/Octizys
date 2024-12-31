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
use octizys_macros::Equivalence;

#[derive(Debug, Equivalence)]
pub struct LetBinding {
    pub pattern: PatternMatch,
    #[equivalence(ignore)]
    pub equal: TokenInfo,
    pub value: Expression,
    #[equivalence(ignore)]
    pub semicolon: TokenInfo,
}

#[derive(Debug, Equivalence)]
pub struct Let {
    #[equivalence(ignore)]
    pub let_: TokenInfo,
    pub bindings: Vec<LetBinding>,
    #[equivalence(ignore)]
    pub in_: TokenInfo,
    pub expression: Box<Expression>,
}

#[derive(Debug, Equivalence)]
pub struct CaseItem {
    pub pattern: PatternMatch,
    #[equivalence(ignore)]
    pub arrow: TokenInfo,
    pub expression: Box<Expression>,
}

#[derive(Debug, Equivalence)]
pub struct Case {
    #[equivalence(ignore)]
    pub case: TokenInfo,
    pub expression: Box<Expression>,
    #[equivalence(ignore)]
    pub of: TokenInfo,
    pub cases: Between<TrailingList<CaseItem, Comma>, Braces>,
}

#[derive(Debug, Equivalence)]
pub struct BinaryOperator {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub name: Token<OperatorName>,
}

#[derive(Debug, Equivalence)]
pub struct LambdaExpression {
    pub variable: Token<Identifier>,
    pub expression: Box<Expression>,
}

#[derive(Debug, Equivalence)]
pub struct ApplicationExpression {
    pub start: Box<Expression>,
    pub remain: Vec<Expression>,
}

#[derive(Debug, Equivalence)]
pub enum ExpressionRecordItem {
    SingleVariable {
        variable: Token<Identifier>,
    },
    Assignation {
        variable: Token<Identifier>,
        #[equivalence(ignore)]
        equal: TokenInfo,
        expression: Box<Expression>,
    },
}

#[derive(Debug, Equivalence)]
pub struct ExpressionSelector {
    pub expression: Box<Expression>,
    pub accessor: Token<Identifier>,
}

#[derive(Debug, Equivalence)]
pub enum Expression {
    String(Token<StringLiteral>),
    InterpolationString(Token<InterpolationString>),
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
        #[equivalence(ignore)]
        symbol: TokenInfo,
    },
    TypeArgument {
        #[equivalence(ignore)]
        at: TokenInfo,
        type_: Type,
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

    pub fn need_parens(&self) -> bool {
        match self {
            Expression::String(_) => false,
            Expression::InterpolationString(_) => false,
            Expression::Uint(_) => false,
            Expression::UFloat(_) => false,
            Expression::LocalVariable(_) => false,
            Expression::ImportedVariable(_) => false,
            Expression::NamedHole(_) => false,
            Expression::Tuple(_) => false,
            Expression::Record(_) => false,
            Expression::Case(_) => false,
            Expression::Parens(_) => false,
            Expression::Selector(_) => false,
            Expression::Interrogation { .. } => false,
            Expression::TypeArgument { type_: _type, .. } => {
                _type.need_parens_application()
            }
            Expression::Let(_) => false,
            Expression::BinaryOperator(_) => true,
            Expression::Lambda(_) => true,
            Expression::Application(_) => true,
        }
    }
}
