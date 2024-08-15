use crate::_type;
use crate::expression;

use crate::common::{Name, PathPrefix};

#[derive(Debug)]
pub struct Sast<'prefix, Info> {
    imports: Vec<Import<'prefix, Info>>,
    exports: Vec<Export<'prefix, Info>>,
    data_types: Vec<DataType<'prefix, Info>>,
    new_types: Vec<NewType<'prefix, Info>>,
    alias: Vec<Alias<'prefix, Info>>,
    declarations: Vec<VariableDeclaration<'prefix, Info>>,
    definitions: Vec<VariableDefinition<'prefix, Info>>,
}

#[derive(Debug)]
pub struct Import<'prefix, Info> {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
}

#[derive(Debug)]
pub struct Export<'prefix, Info> {
    info: Info,
    prefix: Option<&'prefix PathPrefix>,
}

#[derive(Debug)]
pub struct DataType<'prefix, Info> {
    info: Info,
    _type: _type::SumType<'prefix, Info>,
}

#[derive(Debug)]
pub struct NewType<'prefix, Info> {
    info: Info,
    _type: _type::NewType<'prefix, Info>,
}

#[derive(Debug)]
pub struct Alias<'prefix, Info> {
    info: Info,
    _type: _type::Alias<'prefix, Info>,
}

#[derive(Debug)]
pub struct VariableDeclaration<'prefix, Info> {
    info: Info,
    name: Name<Info>,
    _type: _type::Type<'prefix, Info>,
}

#[derive(Debug)]
pub struct VariableDefinition<'prefix, Info> {
    info: Info,
    name: Name<Info>,
    function: expression::Function<'prefix, Info>,
}
