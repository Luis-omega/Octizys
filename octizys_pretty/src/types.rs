use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NoLineBreaksString(Rc<str>);

impl From<NoLineBreaksString> for Rc<str> {
    fn from(value: NoLineBreaksString) -> Self {
        value.0.clone()
    }
}

impl NoLineBreaksString {
    pub fn make(s: &str) -> Result<NoLineBreaksString, String> {
        if s.find("\n").is_some() {
            Err(String::from("Malformed string"))
        } else {
            Ok(NoLineBreaksString(Rc::from(s)))
        }
    }

    pub fn make_from_rc(s: Rc<str>) -> Result<NoLineBreaksString, String> {
        if s.find("\n").is_some() {
            Err(String::from("Malformed string"))
        } else {
            Ok(NoLineBreaksString(s))
        }
    }

    pub fn decompose(s: &str) -> Vec<NoLineBreaksString> {
        s.split('\n')
            .map(|x| NoLineBreaksString(Rc::from(x)))
            .collect()
    }

    pub fn extract(self) -> Rc<str> {
        self.0
    }
}

#[cfg(test)]
mod no_line_breaks_string_test {
    use crate::types::NoLineBreaksString;

    #[test]
    fn make_fails() {
        assert_eq!(NoLineBreaksString::make(&"aome\nadsf").is_ok(), false);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Document {
    Empty,
    Concat(Box<Document>, Box<Document>),
    Text(NoLineBreaksString),
    Nest(u16, Box<Document>),
    Break(NoLineBreaksString),
    Group(Box<Document>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SimpleDocument {
    Empty,
    Text(NoLineBreaksString, Box<SimpleDocument>),
    Line(u16, Box<SimpleDocument>),
}

pub trait Pretty {
    fn to_document(&self) -> Document;
}
