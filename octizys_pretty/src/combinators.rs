use std::iter;

use crate::document::*;

pub fn empty() -> Document {
    Document::empty()
}

/// Flat mode: " "
/// Break mode : "\n"
pub fn soft_break() -> Document {
    Document::soft_break()
}

/// Flat mode: "\n"
/// Break mode : "\n"
pub fn hard_break() -> Document {
    Document::hard_break()
}

/// Flat mode: ""
/// Break mode : "\n"
pub fn empty_break() -> Document {
    Document::empty_break()
}

pub fn concat(items: Vec<Document>) -> Document {
    Document::concat(items)
}

pub fn concat_iter<T: IntoIterator<Item = Document>>(items: T) -> Document {
    let mut v: Vec<Document> = items.into_iter().collect();
    let len = v.len();
    if len == 1 {
        return v.pop().unwrap();
    }
    if len == 0 {
        return empty();
    }
    Document::concat(v)
}

pub fn internalize(
    interner: &mut Interner,
    maybe_word: &str,
) -> Option<Document> {
    Document::internalize(interner, maybe_word)
}

pub fn text(words: &str) -> Document {
    Document::text(words)
}

pub fn nest(indentation_level: u16, doc: Document) -> Document {
    Document::nest(indentation_level, doc)
}

pub fn group(doc: Document) -> Document {
    Document::group(doc)
}

pub fn intersperse<
    Doc: Into<Document>,
    Docs: IntoIterator<Item = Doc>,
    Sep: Into<Document>,
>(
    docs: Docs,
    sep: Sep,
) -> Document {
    let separator: Document = sep.into();
    let mut acc: Vec<Document> = vec![];
    for i in docs {
        acc.push(i.into());
        acc.push(separator.clone());
    }
    acc.pop();
    concat(acc)
}

pub fn intersperse_with_function<
    Doc: Into<Document>,
    Docs: IntoIterator<Item = Doc>,
    Sep: Into<Document>,
>(
    docs: Docs,
    sep: Sep,
) -> Document {
    let separator: Document = sep.into();
    let mut acc: Vec<Document> = vec![];
    for i in docs {
        acc.push(i.into());
        acc.push(separator.clone());
    }
    acc.pop();
    concat(acc)
}

pub fn repeat(doc: Document, n: usize) -> Document {
    concat(iter::repeat(doc).take(n).collect())
}