use crate::_type as T;

use crate::common::{Name, PathPrefix};

#[derive(Debug)]
pub enum BasicExpression {
    Bool(bool),
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
pub struct BasicExpressionInfo<Info> {
    info: Info,
    term: BasicExpression,
}

#[derive(Debug)]
pub struct Function<'prefix, Info> {
    info: Info,
    arguments: Vec<Name<Info>>,
    output: Box<Expression<'prefix, Info>>,
}

#[derive(Debug)]
pub struct Annotation<'prefix, Info> {
    info: Info,
    _type: T::Type<'prefix, Info>,
}

#[derive(Debug)]
pub struct LetBinding<'prefix, Info> {
    name: Name<Info>,
    value: Expression<'prefix, Info>,
}

#[derive(Debug)]
pub struct Let<'prefix, Info> {
    info: Info,
    bindings: Vec<LetBinding<'prefix, Info>>,
    output: Box<Expression<'prefix, Info>>,
}

#[derive(Debug)]
pub struct PatternApplication<'prefix, Info> {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
    name: Name<Info>,
    arguments: Vec<Pattern<'prefix, Info>>,
}

#[derive(Debug)]
pub enum Pattern<'prefix, Info> {
    Constant(BasicExpressionInfo<Info>),
    Variable(Name<Info>),
    Application(PatternApplication<'prefix, Info>),
    Hole(Name<Info>),
    Discard(Info),
}

#[derive(Debug)]
pub struct CaseCase<'prefix, Info> {
    info: Info,
    pattern: Pattern<'prefix, Info>,
    expression: Expression<'prefix, Info>,
}

#[derive(Debug)]
pub struct Case<'prefix, Info> {
    info: Info,
    expression: Box<Expression<'prefix, Info>>,
    cases: Vec<CaseCase<'prefix, Info>>,
}

#[derive(Debug)]
pub struct Application<'prefix, Info> {
    info: Info,
    head: Box<Expression<'prefix, Info>>,
    arguments: Vec<Expression<'prefix, Info>>,
}

#[derive(Debug)]
pub enum Expression<'prefix, Info> {
    BasicTerm(BasicExpressionInfo<Info>),
    Variable(Name<Info>),
    Function(Function<'prefix, Info>),
    Annotation(Annotation<'prefix, Info>),
    Let(Let<'prefix, Info>),
    Case(Case<'prefix, Info>),
    Application(Application<'prefix, Info>),
}
