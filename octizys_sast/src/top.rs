use octizys_common::module_logic_path::ModuleLogicPath;
use octizys_common::span::Span;
use octizys_core::common::Identifier;

use crate::_type;
use crate::expression;

#[derive(Debug)]
pub struct Sast {
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub data_types: Vec<DataType>,
    pub new_types: Vec<NewType>,
    pub alias: Vec<Alias>,
    pub declarations: Vec<VariableDeclaration>,
    pub definitions: Vec<VariableDefinition>,
}

#[derive(Debug)]
pub struct Import {
    pub span: Span,
    pub prefix: Option<ModuleLogicPath>,
}

#[derive(Debug)]
pub struct Export {
    pub span: Span,
    pub prefix: Option<ModuleLogicPath>,
}

#[derive(Debug)]
pub struct DataType {
    pub span: Span,
    pub _type: _type::SumType,
}

#[derive(Debug)]
pub struct NewType {
    pub span: Span,
    pub _type: _type::NewType,
}

#[derive(Debug)]
pub struct Alias {
    pub span: Span,
    pub _type: _type::Alias,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub span: Span,
    pub name: (Span, Identifier),
    pub _type: _type::Type,
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub span: Span,
    pub name: (Span, Identifier),
    pub function: expression::Function,
}
