use crate::common::*;
use crate::types::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CaseAlternative {
    constructor: u64,
    arguments: u64,
    value: Term,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Unit,
    Uint(u64),
    Int(i64),
    String(String),
    Variable(u64, Type),
    NamedVariable(Identifier),
    GlobalVariable(Identifier, Type),
    Function(Box<Term>, Type),
    Application(Box<Term>, Box<Term>, Type),
    Record(Record<Term>, Type),
    Tuple(Vec<Term>, Type),
    Case(Box<Term>, Vec<CaseAlternative>, Type),
}

trait ContextInterface<contextType, valuesType> {
    fn find(&self, name: &str) -> &Vec<valuesType>;
    fn update(&mut self, name: &str, value: valuesType) -> &mut contextType;
}

pub struct LocalContext();
pub struct ExternalContext();
pub struct ModuleContext();
pub struct Context<'a> {
    local: &'a LocalContext,
    external: &'a ExternalContext,
    module: &'a ModuleContext,
}

//pub impl Context{
//    pub fn solve_name(&self,identifier:Identifier)->Result<> {
//
//    }
//}

/*
(if e then x else z):T

becomes

data Bool = True | False
(case e of
    True => x
    False => y):T

Then in core it became

Bool = Sum(0,[[],[]])

Case(e,[CaseAlternative(0,[],x),CaseAlternative(1,[],y)],T)


impl Term {
    pub fn check(
        self,
        t: Type,
        term_stack: &mut Vec<Type>,
        type_stack: &mut Vec<Type>,
    ) -> Result<(), String> {
        match self {
            Term::Uint(_) => Type::unify(t, Type::Uint),
            Term::String(_) => Type::unify(t, Type::String),
            Term::Variable(_, t2) => Type::unify(t, t2),
            Term::GlobalVariable(_, t2) => Type::unify(t, t2),
            Term::TermFunction(term, t2) => {
                let rt2 = Type::reduction(t2);
                let rt = Type::reduction(t);
                match (rt2.clone(), rt.clone()) {
                    (Type::Arrow(t3, t4), Type::Arrow(t5, t6)) => {
                        match Type::unify(*t3, (*t5).clone()).and_then(|x| {
                            Type::unify((*t4).clone(), (*t6).clone())
                        }) {
                            Ok(_) => {
                                term_stack.push(*t5);
                                Term::check(*term, *t6, term_stack, type_stack)
                            }
                            Err(msg) => Err(msg),
                        }
                    }
                    (Type::Forall(t3),Type::Forall(t4))=>
                    _ => {
                        Err(format!("Can't unify types {:?} and {:?}", rt2, rt))
                    }
                }
            }
            Term::TypeFunction(term, t2) => {
                let rt2 = Type::reduction(t2);
                let rt = Type::reduction(t);
                match (rt2.clone(), rt.clone()) {
                    (Type::Forall(t3), Type::Forall(t4)) => {
                        Type::unify((*t3).clone(), *t4).and_then(|_| {
                            type_stack.push()
                            Term::check(*term, *t3, term_stack, type_stack)
                        })
                    }
                    _ => {
                        Err(format!("Can't unify types {:?} and {:?}", rt2, rt))
                    }
                }
            }
            //Beware: stack can be mutated by the first term, so you need to avoid mutating it in
            //second term
            //Application(Box<Term>, Box<Term>, t2),
            //Record(Record<Term>, Type),
            //Tuple(Vec<Term>, Type),
            //Case(Term, Vec<CaseAlternative>, Type),
            _ => Err(String::from(format!("not implemented case {:?}", self))),
        }
    }
}
*/
