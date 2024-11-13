use octizys_common::identifier::Identifier;
use octizys_common::module_logic_path::ModuleLogicPath;
use octizys_common::span::Span;

use crate::_type::{self as T, VariableId};

use crate::common::{Name, PathPrefix};

#[derive(Debug)]
pub enum BasicExpression {
    U64(u64),
    I64(i64),
    U32(u32),
    I32(i32),
    U16(u16),
    I16(i16),
    U8(u8),
    I8(i8),
    Float(f32),
    Double(f64),
    String(String),
    Char(char),
}

#[derive(Debug)]
pub struct BasicExpressionInfo {
    pub span: Span,
    pub term: BasicExpression,
}

#[derive(Debug)]
pub struct Function {
    pub span: Span,
    pub arguments: Vec<VariableId>,
    pub output: Box<Expression>,
}

#[derive(Debug)]
pub struct Annotation {
    pub span: Span,
    pub _type: T::Type,
}

#[derive(Debug)]
pub struct LetBinding {
    pub name: (Span, Identifier),
    pub value: Expression,
}

#[derive(Debug)]
pub struct Let {
    pub span: Span,
    pub bindings: Vec<LetBinding>,
    pub output: Box<Expression>,
}

#[derive(Debug)]
pub struct PatternApplication {
    pub span: Span,
    pub prefix: Option<ModuleLogicPath>,
    pub name: (Span, Identifier),
    pub arguments: Vec<Pattern>,
}

#[derive(Debug)]
pub enum Pattern {
    Constant(BasicExpressionInfo),
    Variable(Span, Identifier),
    Application(PatternApplication),
    Hole(Span, Identifier),
    Discard(Span),
}

#[derive(Debug)]
pub struct CaseCase {
    pub span: Span,
    pub pattern: Pattern,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct Case {
    pub span: Span,
    pub expression: Box<Expression>,
    pub cases: Vec<CaseCase>,
}

#[derive(Debug)]
pub struct Application {
    pub span: Span,
    pub head: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    BasicTerm(BasicExpressionInfo),
    LocalVariable(VariableId),
    ExternalVariable(Identifier, Option<ModuleLogicPath>),
    Function(Function),
    Annotation(Annotation),
    Let(Let),
    Case(Case),
    Application(Application),
}
