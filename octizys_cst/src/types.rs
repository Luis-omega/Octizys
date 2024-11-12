use crate::base::{
    Between, Enclosures, ImportedVariable, ItemSeparator, Token, TokenInfo,
    TrailingList, TrailingListItem,
};
use crate::pretty::{PrettyCST, PrettyCSTConfig};
use octizys_common::identifier::Identifier;
use octizys_pretty::combinators::*;
use octizys_pretty::document::Document;

#[derive(Debug)]
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
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        match self {
            TypeBase::U8 => "U8".into(),
            TypeBase::U16 => "U16".into(),
            TypeBase::U32 => "U32".into(),
            TypeBase::U64 => "U64".into(),
            TypeBase::I8 => "I8".into(),
            TypeBase::I16 => "I16".into(),
            TypeBase::I32 => "I32".into(),
            TypeBase::I64 => "I64".into(),
            TypeBase::F32 => "F32".into(),
            TypeBase::F64 => "F64".into(),
            TypeBase::Char => "Char".into(),
            TypeBase::String => "String".into(),
        }
    }
}

#[derive(Debug)]
pub struct TypeRecordItem {
    pub variable: Token<Identifier>,
    pub separator: TokenInfo,
    // This is needed as TrailingList stores a T
    // otherwise we can drop the Box, maybe put
    // the box in the TrailingList?
    pub expression: Box<Type>,
}

impl PrettyCST for TypeRecordItem {
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        concat(vec![
            self.variable
                .info
                .to_document(configuration, self.variable.value.into()),
            self.separator.to_document(configuration, ":".into()),
            self.expression.to_document(configuration),
        ])
    }
}

fn pretty_betwee_trailing<T: PrettyCST>(
    between: Between<TrailingList<T>>,
    configuration: PrettyCSTConfig,
    sep: ItemSeparator,
    enclosure: Enclosures,
) -> Document {
    between.to_document(configuration, enclosure, |l, c| l.to_document(c, sep))
}

#[derive(Debug)]
pub enum Type {
    Base(Token<TypeBase>),
    LocalVariable(Token<Identifier>),
    ImportedVariable(Token<ImportedVariable>),
    Tuple(Between<TrailingList<Box<Type>>>),
    Record(Between<TrailingList<TypeRecordItem>>),
    Parens(Between<Box<Type>>),
    Application {
        start: Box<Type>,
        second: Box<Type>,
        remain: Vec<Type>,
    },
    Arrow {
        first: Box<Type>,
        remain: Vec<TrailingListItem<Type>>,
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
        self,
        configuration: PrettyCSTConfig,
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
        self,
        configuration: PrettyCSTConfig,
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
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        match self {
            Type::Base(token) => token.to_document(configuration),
            Type::LocalVariable(token) => {
                token.info.to_document(configuration, token.value.into())
            }
            Type::ImportedVariable(token) => token.to_document(configuration),
            Type::Tuple(between) => pretty_betwee_trailing(
                between,
                configuration,
                ItemSeparator::Comma,
                Enclosures::Parens,
            ),
            Type::Record(between) => pretty_betwee_trailing(
                between,
                configuration,
                ItemSeparator::Comma,
                Enclosures::Braces,
            ),
            Type::Parens(between) => between.to_document(
                configuration,
                Enclosures::Parens,
                |t, c| t.to_document(c),
            ),
            Type::Application {
                start,
                second,
                remain,
            } => group(nest(
                configuration.indentation_deep,
                concat(vec![
                    start.to_document_application_argument(configuration),
                    soft_break(),
                    second.to_document_application_argument(configuration),
                    soft_break(),
                    intersperse(
                        remain.into_iter().map(|x| {
                            x.to_document_application_argument(configuration)
                        }),
                        soft_break(),
                    ),
                ]),
            )),
            Type::Arrow { first, remain } => {
                let remain_doc = remain.into_iter().map(|arg| {
                    arg.to_document(configuration, ItemSeparator::Arrow)
                });
                first.to_document_arrow_arguments(configuration)
                    + concat_iter(remain_doc)
            }
            Type::Scheme {
                forall,
                first_variable,
                remain_variables,
                dot,
                expression,
            } => concat(vec![
                forall.to_document(configuration, "forall".into()),
                group(nest(
                    configuration.indentation_deep,
                    concat(vec![
                        soft_break(),
                        first_variable.to_document(configuration),
                        soft_break(),
                        intersperse(
                            remain_variables
                                .into_iter()
                                .map(|x| x.to_document(configuration)),
                            soft_break(),
                        ),
                        dot.to_document(configuration, ".".into()),
                        soft_break(),
                        group(nest(
                            configuration.indentation_deep,
                            soft_break()
                                + expression.to_document(configuration),
                        )),
                    ]),
                )),
            ]),
        }
    }
}
