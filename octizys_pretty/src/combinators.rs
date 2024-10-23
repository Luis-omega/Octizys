use crate::types::Document;
use crate::types::Document::*;
use crate::types::NoLineBreaksString;

pub fn empty() -> Document {
    Empty
}

pub fn concat(d1: Document, d2: Document) -> Document {
    Concat(Box::new(d1), Box::new(d2))
}

pub fn concat_vec(docs: Vec<Document>) -> Document {
    let mut doc = empty();
    for new_doc in docs.into_iter().rev() {
        doc = concat(new_doc, doc)
    }
    doc
}

/// concatenate a series of document using a separator.
/// the separator is always appended to the final item
/// (trailing separator)
pub fn concat_sep_by(docs: Vec<Document>, sep: Document) -> Document {
    concat_vec(docs.into_iter().map(|d| concat(d, sep.clone())).collect())
}

pub fn text(s: NoLineBreaksString) -> Document {
    Text(s)
}

pub fn nest(ident: u16, d: Document) -> Document {
    Nest(ident, Box::new(d))
}

//Rust has break keyword
pub fn break_(s: NoLineBreaksString) -> Document {
    Break(s)
}

pub fn hard_break(s: NoLineBreaksString) -> Document {
    HardBreak(s)
}

pub fn group(d: Document) -> Document {
    Group(Box::new(d))
}

pub fn from_str(s: &str) -> Document {
    let v = NoLineBreaksString::decompose(s);
    if v.len() == 0 {
        empty()
    } else {
        if v.len() == 1 {
            text(v[0].clone())
        } else {
            let mut doc = empty();
            for new_string in v.into_iter().rev() {
                doc = concat(break_(new_string), doc);
            }
            doc
        }
    }
}

impl<'d> Into<Document> for &str {
    fn into(self) -> Document {
        from_str(self)
    }
}
