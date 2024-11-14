use crate::base::{
    Between, ImportedVariable, Token, TokenInfo, TrailingList, TrailingListItem,
};
use crate::pretty::{
    indent, Braces, Colon, Comma, Parens, PrettyCST, PrettyCSTContext,
    RightArrow,
};
use derivative::Derivative;
use octizys_common::identifier::Identifier;
use octizys_pretty::combinators::*;
use octizys_pretty::document::Document;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeBase {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    String,
}

impl PrettyCST for TypeBase {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match self {
            TypeBase::U8 => context.cache.u8,
            TypeBase::U16 => context.cache.u16,
            TypeBase::U32 => context.cache.u32,
            TypeBase::U64 => context.cache.u16,
            TypeBase::I8 => context.cache.i8,
            TypeBase::I16 => context.cache.i16,
            TypeBase::I32 => context.cache.i32,
            TypeBase::I64 => context.cache.i64,
            TypeBase::F32 => context.cache.f32,
            TypeBase::F64 => context.cache.f64,
            TypeBase::Char => context.cache.char,
            TypeBase::String => context.cache.string,
        }
        .into()
    }
}

#[derive(Debug, Derivative)]
pub struct TypeRecordItem {
    pub variable: Token<Identifier>,
    pub separator: TokenInfo,
    // This is needed as TrailingList stores a T
    // otherwise we can drop the Box, maybe put
    // the box in the TrailingList?
    pub expression: Box<Type>,
}

impl PrettyCST for TypeRecordItem {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        concat(vec![
            self.variable.to_document(context),
            self.separator.to_document(context, context.cache.colon),
            self.expression.to_document(context),
        ])
    }
}

#[derive(Debug)]
pub enum Type {
    Base(Token<TypeBase>),
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    Tuple(Between<TrailingList<Box<Type>, Comma>, Parens>),
    Record(Between<TrailingList<TypeRecordItem, Colon>, Braces>),
    Parens(Between<Box<Type>, Parens>),
    Application {
        start: Box<Type>,
        second: Box<Type>,
        remain: Vec<Type>,
    },
    Arrow {
        first: Box<Type>,
        remain: Vec<TrailingListItem<Type, RightArrow>>,
    },
    Scheme {
        forall: TokenInfo,
        first_variable: Token<Identifier>,
        remain_variables: Vec<Token<Identifier>>,
        dot: TokenInfo,
        expression: Box<Type>,
    },
}

impl Type {
    ///This function tell the pretty printer if the type needs to be
    ///surrounded by parentheses if the type is a argument in a
    ///application.
    fn need_parens_application(&self) -> bool {
        match self {
            Type::Base(_) => false,
            Type::LocalVariable(_) => false,
            Type::ImportedVariable(_) => false,
            Type::Tuple(_) => false,
            Type::Record(_) => false,
            Type::Parens(_) => false,
            Type::Application { .. } => true,
            Type::Arrow { .. } => true,
            Type::Scheme { .. } => true,
        }
    }

    ///This function tell the pretty printer if the type needs to be
    ///surrounded by parentheses if the type is a argument in a
    ///arrow.
    fn need_parens_arrow(&self) -> bool {
        match self {
            Type::Base(_) => false,
            Type::LocalVariable(_) => false,
            Type::ImportedVariable(_) => false,
            Type::Tuple(_) => false,
            Type::Record(_) => false,
            Type::Parens(_) => false,
            Type::Application { .. } => false,
            Type::Arrow { .. } => true,
            Type::Scheme { .. } => true,
        }
    }

    fn to_document_application_argument(
        &self,
        configuration: &PrettyCSTContext,
    ) -> Document {
        if self.need_parens_application() {
            concat(vec![
                "(".into(),
                soft_break(),
                self.to_document(configuration),
                soft_break(),
                ")".into(),
            ])
        } else {
            self.to_document(configuration)
        }
    }

    fn to_document_arrow_arguments(
        &self,
        configuration: &PrettyCSTContext,
    ) -> Document {
        if self.need_parens_arrow() {
            concat(vec![
                "(".into(),
                soft_break(),
                self.to_document(configuration),
                soft_break(),
                ")".into(),
            ])
        } else {
            self.to_document(configuration)
        }
    }
}

impl PrettyCST for Type {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match &self {
            Type::Base(token) => token.to_document(context),
            Type::LocalVariable(token) => token.to_document(context),
            Type::ImportedVariable(token) => token.to_document(context),
            Type::Tuple(between) => between.to_document(context),
            Type::Record(between) => between.to_document(context),
            Type::Parens(between) => between.to_document(context),
            Type::Application {
                start,
                second,
                remain,
            } => {
                start.to_document_application_argument(context)
                    + indent(
                        context,
                        concat(vec![
                            soft_break(),
                            second.to_document_application_argument(context),
                            soft_break(),
                            intersperse(
                                remain.into_iter().map(|x| {
                                    x.to_document_application_argument(context)
                                }),
                                soft_break(),
                            ),
                        ]),
                    )
            }
            Type::Arrow { first, remain } => {
                let remain_doc =
                    remain.into_iter().map(|arg| arg.to_document(context));
                first.to_document_arrow_arguments(context)
                    + concat_iter(remain_doc)
            }
            Type::Scheme {
                forall,
                first_variable,
                remain_variables,
                dot,
                expression,
            } => concat(vec![
                forall.to_document(context, context.cache.forall),
                indent(
                    context,
                    concat(vec![
                        soft_break(),
                        first_variable.to_document(context),
                        soft_break(),
                        intersperse(
                            remain_variables
                                .into_iter()
                                .map(|x| x.to_document(context)),
                            soft_break(),
                        ),
                        dot.to_document(context, context.cache.dot),
                        soft_break(),
                        indent(
                            context,
                            soft_break() + expression.to_document(context),
                        ),
                    ]),
                ),
            ]),
        }
    }
}
