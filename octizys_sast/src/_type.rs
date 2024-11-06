use octizys_common::{identifier::Identifier, span::Span};

#[derive(Debug)]
pub enum BasicType {
    U64,
    I64,
    U32,
    I32,
    U16,
    I16,
    U8,
    I8,
    Float,
    Double,
    String,
    Char,
}

#[derive(Debug)]
pub struct Function {
    span: Span,
    arguments: Vec<Type>,
    output: Box<Type>,
}

#[derive(Debug)]
pub struct Forall {
    span: Span,
    arguments: Vec<Identifier>,
    output: Box<Type>,
}

#[derive(Debug)]
pub struct Constructor {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
    name: Name<Info>,
    _type: Box<Type<'prefix, Info>>,
}

#[derive(Debug)]
pub struct SumType<'prefix, Info> {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
    name: Name<Info>,
    bound_variables: Vec<Name<Info>>,
    constructors: Vec<Constructor<'prefix, Info>>,
}

#[derive(Debug)]
pub struct NewType<'prefix, Info> {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
    name: Name<Info>,
    bound_variables: Vec<Name<Info>>,
    constructor: Constructor<'prefix, Info>,
}

#[derive(Debug)]
pub struct Alias<'prefix, Info> {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
    name: Name<Info>,
    bound_variables: Vec<Name<Info>>,
    _type: Box<Type<'prefix, Info>>,
}

#[derive(Debug)]
pub struct Record<'prefix, Info> {
    info: Info,
    //TODO: Add row polymorphism
    record: common::Record<Type<'prefix, Info>>,
}

#[derive(Debug)]
pub enum Type {
    BasicType { _type: BasicType, span: Span },
    Variable(Name<Info>),
    Function(Function<'prefix, Info>),
    Forall(Forall<'prefix, Info>),
    SumType(SumType<'prefix, Info>),
    NewType(NewType<'prefix, Info>),
    Alias(Alias<'prefix, Info>),
    Record(Record<'prefix, Info>),
}
