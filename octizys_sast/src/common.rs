use std::collections::HashMap;

use octizys_core::common::Identifier;

#[derive(Debug)]
pub struct PathPrefix {
    prefix: Vec<Identifier>,
}

#[derive(Debug)]
pub enum Name<Info> {
    NamedVariable {
        info: Info,
        name: Identifier,
    },
    // TODO: change this to Operator
    ///We consider all Operators as named
    NamedOperator {
        info: Info,
        name: Identifier,
    },
}

#[derive(Debug)]
pub struct RecordLabel(Identifier);

#[derive(Debug)]
pub struct Record<T>(HashMap<RecordLabel, T>);

pub enum ContextResult<T> {
    Found(T),
    Multiple(Vec<T>),
    NotFound,
}

pub trait ContextTrait<Key, Values> {
    fn find(&self, key: Key) -> ContextResult<Values>;
    fn fuzzy_find(&self, key: Key) -> Vec<Values>;
}
