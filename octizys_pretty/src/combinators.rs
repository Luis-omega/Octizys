use std::iter;

use crate::document::*;
use octizys_text_store::store::Store;

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

/// Concatenates a vector of documents into a single one.
pub fn concat(items: Vec<Document>) -> Document {
    Document::concat(items)
}

/// Concatenates an iterator of documents into a single document.
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

/// Attempt to store a non comment string and returns a document that
/// can be used to refer to this string.
pub fn try_internalize(
    store: &mut Store,
    maybe_word: &str,
) -> Option<Document> {
    Document::try_internalize(store, maybe_word)
}

/// Takes an arbitrary string and split it to store as the internal adequate
/// representation of a document, it attempts to keep the original format of
/// the text.
pub fn external_text(words: &str) -> Document {
    Document::external_text(words)
}

/// Stores a given string as a comment in the [`Store`] and returns a
/// document that can be used to refer to it.
pub fn comment_line(
    source: &str,
    comments_accumulator: &mut Vec<String>,
) -> Document {
    Document::comment_line(source, comments_accumulator)
}

/// Augment the indentation level with respect to the current one.
/// This won't create a line break or add spaces, to see the effect
/// one must provide line breaks in the inner document.
/// The size of one level of indentation is determined at rendering time!
pub fn nest(indentation_level: u16, doc: Document) -> Document {
    Document::nest(indentation_level, doc)
}

/// Surround a document with a group.
/// This make the soft breaks inside the content of the document to possibly
/// be rendered as real line breaks or as a single space depending on the
/// available line width.
pub fn group(doc: Document) -> Document {
    Document::group(doc)
}

/// Create an iterator interspersing the provided document with
/// the elements of the provided iterator of documents.
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

/// Repeat the same document [`n`] times.
pub fn repeat(doc: Document, n: usize) -> Document {
    concat(iter::repeat(doc).take(n).collect())
}

/*
#[derive(Debug)]
struct Example<'a> {
    mutable: &'a mut u8,
    immutable: &'a u8,
}

pub fn consume<'a>(x: &'a Example<'a>) -> &'a u8 {
    x.mutable
}

pub fn main() {
    let mut x = 8;
    let y = 9;
    let mut e = Example {
        mutable: &mut x,
        immutable: &y,
    };
    let t = &x;
    let j = consume(&e);
    let w = e.mutable;
}
*/
