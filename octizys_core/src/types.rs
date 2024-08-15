use std::fmt::format;

use crate::common::{Identifier, NonEmptyVec, Record, Variable};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Unit,
    Uint,
    Int,
    String,
    Variable(Variable),
    // forall a b c . a -> (a-> b) -> c
    // Forall(3, Arrow(2,Arrow(Arrow(2,1), 0)))
    // Forall (0,t) means just t without binded variables
    Forall(u64, Box<Type>),
    Arrow(Box<Type>, Box<Type>),
    Record(Record<Type>),
    // data T a = (a,T)
    // Recursive(1, Forall (1, Tuple(0,1)))
    // Recursive(0,t) is just t
    Recursive(u64, Box<Type>),
    Application(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    //The constructor name is the index of the type in the sum
    //data w a b c = S1 | S2 a | S3 b a c
    //NewType(Identifier(w),Forall(3,Sum([(Sum([])),[2],[1,2,0]])))
    Sum(Identifier, Vec<Type>),
    NewType(Identifier, Box<Type>),
    Alias(Identifier, Box<Type>),
}

/*
impl Type {
    fn shift(t:Self,amount:u64, nest_level:u64)->Self{
        match t {
            Type::Uint=> t,
            Type::String=>t,
            Type::Variable(x)=> if x >= nest_level {Type::Variable(x+amount)} else {t},
            Type::Forall(new_levels,t2) => shift(),
            Type::Arrow(Box<Type>, Box<Type>),
            Type::Record(Record<Type>),
            Type::RecursiveType(Box<Type>),
            Type::Application(Box<Type>, Box<Type>),
            Type::Tuple(Vec<Type>),
            Type::Sum(u64, Vec<Vec<Type>>),
        }
    }


    fn substitute_type_var(
        type1: Type,
        variable_number: u64,
        type2: Type,
    ) -> Type {
        match type1 {
            Type::Uint => type1,
            Type::String => type1,
            Type::Variable(x) => {
                if x == variable_number {
                    type2
                } else {
                    type1
                }
            }
            Type::Forall(t1) => {
                Type::substitute_type_var(*t1, variable_number + 1, type2)
            }
            Type::Arrow(t1, t2) => Type::Arrow(
                Box::from(Type::substitute_type_var(
                    *t1,
                    variable_number,
                    type2.clone(),
                )),
                Box::from(Type::substitute_type_var(
                    *t2,
                    variable_number,
                    type2,
                )),
            ),
            Type::Record(Record(v)) => Type::Record(unsafe_make_record(
                v.iter()
                    .map(|(label, t)| {
                        (
                            (*label).clone(),
                            Type::substitute_type_var(
                                (*t).clone(),
                                variable_number,
                                type2.clone(),
                            ),
                        )
                    })
                    .collect(),
            )),
            Type::RecursiveType(t1) => Type::RecursiveType(Box::from(
                Type::substitute_type_var(*t1, variable_number + 1, type2),
            )),
            Type::Application(t1, t2) => Type::Application(
                Box::from(Type::substitute_type_var(
                    *t1,
                    variable_number,
                    type2.clone(),
                )),
                Box::from(Type::substitute_type_var(
                    *t2,
                    variable_number,
                    type2,
                )),
            ),
            Type::Tuple(v) => Type::Tuple(
                v.iter()
                    .map(|t| {
                        Type::substitute_type_var(
                            (*t).clone(),
                            variable_number,
                            type2.clone(),
                        )
                    })
                    .collect(),
            ),
            Type::Sum(binded_vars, v1) => Type::Sum(
                binded_vars,
                v1.iter()
                    .map(|v2| {
                        v2.iter()
                            .map(|t| {
                                Type::substitute_type_var(
                                    (*t).clone(),
                                    variable_number + binded_vars,
                                    type2.clone(),
                                )
                            })
                            .collect()
                    })
                    .collect(),
            ),
        }
    }

    fn reduction(type_: Type) -> Type {
        match type_ {
            Type::Uint => type_,
            Type::String => type_,
            Type::Variable(_) => type_,
            Type::Forall(t1) => Type::Forall(Box::from(Type::reduction(*t1))),
            Type::Arrow(t1, t2) => Type::Arrow(
                Box::from(Type::reduction(*t1)),
                Box::from(Type::reduction(*t2)),
            ),
            Type::Record(Record(v)) => Type::Record(unsafe_make_record(
                v.iter()
                    .map(|(label, t)| {
                        ((*label).clone(), Type::reduction((*t).clone()))
                    })
                    .collect(),
            )),
            Type::RecursiveType(t) => {
                Type::RecursiveType(Box::from(Type::reduction(*t)))
            }
            Type::Application(t1, t2) => match *t1 {
                Type::Forall(t3) => Type::substitute_type_var(*t3, 0, *t2),
                _ => Type::Application(
                    Box::from(Type::reduction(*t1)),
                    Box::from(Type::reduction(*t2)),
                ),
            },
            Type::Tuple(v) => Type::Tuple(
                v.iter().map(|t| Type::reduction((*t).clone())).collect(),
            ),
            Type::Sum(binded_vars, v1) => Type::Sum(
                binded_vars,
                v1.iter()
                    .map(|v2| {
                        v2.iter()
                            .map(|t| Type::reduction((*t).clone()))
                            .collect()
                    })
                    .collect(),
            ),
        }
    }
    fn unify(type1: Type, type2: Type) -> Result<(), String> {
        let simplified1 = Type::reduction(type1.clone());
        let simplified2 = Type::reduction(type2.clone());
        if simplified1 == simplified2 {
            Ok(())
        } else {
            Err(format!("Can't unify types {:?} and {:?}", type1, type2))
        }
    }
}
*/
