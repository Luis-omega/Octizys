use crate::base::{
    Between, ImportedVariable, OperatorName, Token, TokenInfo, TrailingList,
};
use crate::patterns::PatternMatch;
use crate::pretty::{indent, PrettyCST, PrettyCSTConfig};
use crate::types::Type;
use octizys_common::identifier::Identifier;
use octizys_pretty::combinators::{
    concat, concat_iter, empty_break, hard_break, soft_break,
};
use octizys_pretty::document::Document;

#[derive(Debug)]
pub struct LetBinding {
    pub pattern: PatternMatch,
    pub equal: TokenInfo,
    pub value: Expression,
    pub semicolon: TokenInfo,
}

impl PrettyCST for LetBinding {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        self.pattern.to_document(configuration)
            + indent(
                configuration,
                concat(vec![
                    soft_break(),
                    self.equal.to_document(configuration, "=".into()),
                    soft_break(),
                    self.value.to_document(configuration),
                    self.semicolon.to_document(configuration, ";".into()),
                ]),
            )
    }
}

#[derive(Debug)]
pub struct Let {
    pub let_: TokenInfo,
    pub bindings: Vec<LetBinding>,
    pub in_: TokenInfo,
    pub expression: Box<Expression>,
}

impl PrettyCST for Let {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.let_.to_document(configuration, "let".into()),
            indent(
                configuration,
                soft_break()
                    + concat_iter(
                        self.bindings
                            .iter()
                            .map(|x| x.to_document(configuration)),
                    ),
            ),
            soft_break(),
            self.in_.to_document(configuration, "in".into()),
            indent(
                configuration,
                soft_break() + self.expression.to_document(configuration),
            ),
        ])
    }
}

#[derive(Debug)]
pub struct CaseItem {
    pub pattern: PatternMatch,
    pub arrow: TokenInfo,
    pub expression: Box<Expression>,
}

impl PrettyCST for CaseItem {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.pattern.to_document(configuration),
            self.arrow.to_document(configuration, " =>".into()),
            indent(
                configuration,
                soft_break() + self.expression.to_document(configuration),
            ),
        ])
    }
}

#[derive(Debug)]
pub struct Case {
    pub case: TokenInfo,
    pub expression: Box<Expression>,
    pub of: TokenInfo,
    pub cases: Between<TrailingList<CaseItem>>,
}

impl PrettyCST for Case {
    fn to_document(&self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.case.to_document(configuration, "case".into()),
            indent(
                configuration,
                soft_break() + self.expression.to_document(configuration),
            ),
            self.of.to_document(configuration, "of".into()),
            self.cases.left.to_document(configuration, "{".into()),
            //TODO: finish this
        ])
    }
}

#[derive(Debug)]
pub struct BinaryOperator {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub name: Token<OperatorName>,
}

impl PrettyCST for BinaryOperator {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        todo!()
    }
}

#[derive(Debug)]
pub struct LambdaExpression {
    pub variable: Token<Identifier>,
    pub expression: Box<Expression>,
}

impl PrettyCST for LambdaExpression {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        todo!()
    }
}

#[derive(Debug)]
pub struct ApplicationExpression {
    pub start: Box<Expression>,
    pub remain: Vec<Expression>,
}

impl PrettyCST for ApplicationExpression {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        todo!()
    }
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

impl PrettyCST for ExpressionRecordItem {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        todo!()
    }
}

#[derive(Debug)]
pub struct ExpressionSelector {
    pub expression: Box<Expression>,
    pub accessor: Token<Identifier>,
}

impl PrettyCST for ExpressionSelector {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        todo!()
    }
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

impl PrettyCST for Expression {
    fn to_document(&self, _configuration: PrettyCSTConfig) -> Document {
        todo!()
    }
}
