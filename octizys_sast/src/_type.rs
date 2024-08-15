use crate::common;
use crate::common::{Name, PathPrefix};

#[derive(Debug)]
pub enum BasicType {
    //TODO: We don't need bool here, we can apply if in any type
    //that uses two constructors only...
    Bool,
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
}

#[derive(Debug)]
pub struct BasicTypeInfo<Info> {
    info: Info,
    _type: BasicType,
}

#[derive(Debug)]
pub struct Function<'prefix, Info> {
    info: Info,
    arguments: Vec<Type<'prefix, Info>>,
    output: Box<Type<'prefix, Info>>,
}

#[derive(Debug)]
pub struct Forall<'prefix, Info> {
    info: Info,
    arguments: Vec<Name<Info>>,
    output: Box<Type<'prefix, Info>>,
}

#[derive(Debug)]
pub struct Constructor<'prefix, Info> {
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
pub enum Type<'prefix, Info> {
    BasicType(BasicTypeInfo<Info>),
    Variable(Name<Info>),
    Function(Function<'prefix, Info>),
    Forall(Forall<'prefix, Info>),
    SumType(SumType<'prefix, Info>),
    NewType(NewType<'prefix, Info>),
    Alias(Alias<'prefix, Info>),
    Record(Record<'prefix, Info>),
}
