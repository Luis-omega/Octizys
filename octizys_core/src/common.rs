#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Label(String);

#[derive(Debug, PartialEq, Eq)]
pub struct MakeLabelError(pub String);

impl Label {
    pub fn make(s: String) -> Result<Label, MakeLabelError> {
        //TODO: verify the string here
        Ok(Label(s))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier(String);

pub struct MakeIdentifierError(String);

impl Identifier {
    pub fn make(s: String) -> Result<Identifier, MakeIdentifierError> {
        //TODO: verify the string here
        Ok(Identifier(s))
    }
}

pub struct MakeRecordError(String);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<T>(Vec<(Label, T)>);
impl<T> Record<T> {
    pub fn make(v: Vec<(Label, T)>) -> Result<Record<T>, MakeRecordError> {
        //TODO: Vectors should:
        //- be lexicografically sorted
        //- labels must be unique
        Ok(Record(v))
    }
}

fn unsafe_make_record<T>(v: Vec<(Label, T)>) -> Record<T> {
    Record(v)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Variable {
    Named(Identifier),
    Free(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NonEmptyVec<T>(T, Vec<T>);
