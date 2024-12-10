use octizys_common::{
    identifier::Identifier, module_logic_path::LogicPath, span::Span,
};

#[derive(Debug)]
pub struct VariableId(pub u32);

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
    pub span: Span,
    pub arguments: Vec<Span>,
    pub output: Box<Type>,
}

#[derive(Debug)]
pub struct Forall {
    pub span: Span,
    pub arguments: Vec<Span>,
    pub output: Box<Type>,
}

#[derive(Debug)]
pub struct Constructor {
    pub span: Span,
    pub prefix: Option<LogicPath>,
    pub name: (Span, Variable),
    pub _type: Box<Type>,
}

#[derive(Debug)]
pub struct SumType {
    pub span: Span,
    pub prefix: Option<LogicPath>,
    pub name: (Span, Identifier),
    pub bound_variables: Vec<(Span, Identifier)>,
    pub constructors: Vec<Constructor>,
}

#[derive(Debug)]
pub struct NewType {
    pub span: Span,
    pub prefix: Option<LogicPath>,
    pub name: (Span, Identifier),
    pub bound_variables: Vec<(Span, Identifier)>,
    pub constructor: Constructor,
}

#[derive(Debug)]
pub struct Alias {
    pub span: Span,
    pub prefix: Option<LogicPath>,
    pub name: (Span, Identifier),
    pub bound_variables: Vec<(Span, Identifier)>,
    pub _type: Box<Type>,
}

/*
#[derive(Debug)]
pub struct Record {
    span: Span,
    //TODO: Add row polymorphism
    record: octizys_common::Record<Type>,
}
*/

#[derive(Debug)]
pub enum Type {
    BasicType { _type: BasicType, span: Span },
    LocalVariable(VariableId),
    ExternalVariable(Identifier, Option<LogicPath>),
    InferenceVariable(VariableId),
    Function(Function),
    Forall(Forall),
    SumType(SumType),
    NewType(NewType),
    Alias(Alias),
    //Record(Record<'prefix, Info>),
}
