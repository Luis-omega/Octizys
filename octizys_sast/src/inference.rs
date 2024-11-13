use std::collections::HashMap;

use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath, span::Span,
};

use crate::{
    _type::{self, Type, VariableId},
    expression::{BasicExpression, Expression, Function},
};

#[derive(Debug)]
pub enum TableValue {
    Type(Identifier, Span, Type),
    Term(Identifier, Span, Expression, Type),
    Both(Identifier, Span, Type, (Expression, Type)),
}

#[derive(Debug)]
pub struct Table {
    table: HashMap<VariableId, TableValue>,
}

#[derive(Debug)]
pub enum InferenceError {
    CantFindVariable(VariableId),
    CantFindExternalVariable(Identifier, Option<ModuleLogicPath>),
}

pub fn infer<Context>(
    context: Context,
    local_table: Table,
    expression: Expression,
) -> Result<Type, InferenceError> {
    todo!()
}

pub fn unify(t1: Type, t2: Type) -> Option<InferenceError> {
    todo!()
}

const DEFAULT_SPAN: Span = Span {
    start: octizys_common::span::Position { source_index: 0 },
    end: octizys_common::span::Position { source_index: 0 },
};

pub fn check<Context>(
    context: Context,
    local_table: Table,
    expression: Expression,
    t: Type,
) -> Option<InferenceError> {
    match expression {
        Expression::BasicTerm(e) => {
            fn compare_basic(
                t1: Type,
                t2: crate::_type::BasicType,
            ) -> Option<InferenceError> {
                unify(
                    t1,
                    crate::_type::Type::BasicType {
                        _type: t2,
                        span: DEFAULT_SPAN,
                    },
                )
            }
            match e.term {
                BasicExpression::U64(_) => {
                    compare_basic(t, crate::_type::BasicType::U64)
                }
                BasicExpression::U32(_) => {
                    compare_basic(t, crate::_type::BasicType::U32)
                }
                BasicExpression::U16(_) => {
                    compare_basic(t, crate::_type::BasicType::U16)
                }
                BasicExpression::U8(_) => {
                    compare_basic(t, crate::_type::BasicType::U8)
                }
                BasicExpression::I64(_) => {
                    compare_basic(t, crate::_type::BasicType::I64)
                }
                BasicExpression::I32(_) => {
                    compare_basic(t, crate::_type::BasicType::I32)
                }
                BasicExpression::I16(_) => {
                    compare_basic(t, crate::_type::BasicType::I16)
                }
                BasicExpression::I8(_) => {
                    compare_basic(t, crate::_type::BasicType::I8)
                }
                BasicExpression::Float(_) => {
                    compare_basic(t, crate::_type::BasicType::Float)
                }
                BasicExpression::Double(_) => {
                    compare_basic(t, crate::_type::BasicType::Double)
                }
                BasicExpression::String(_) => {
                    compare_basic(t, crate::_type::BasicType::String)
                }
                BasicExpression::Char(_) => {
                    compare_basic(t, crate::_type::BasicType::Char)
                }
            }
        }
        Expression::LocalVariable(vid) => match local_table.find(vid) {
            Some(t2) => unify(t, t2),
            None => Some(InferenceError::CantFindVariable(vid)),
        },
        Expression::ExternalVariable(name, path) => match context
            .find(name, path)
        {
            Some(t2) => unify(t, t2.into()),
            None => Some(InferenceError::CantFindExternalVariable(name, path)),
        },

        Expression::Function(Function {
            span,
            arguments,
            output,
        }) => {
            let len = arguments.len();
            match solve_up_to_arrow(t) {
                Some(_type::Function {
                    arguments: args2,
                    output: output2,
                    ..
                }) => unify(t, args2),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
