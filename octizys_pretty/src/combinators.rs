use crate::document::*;

pub fn empty() -> Document {
    Document::empty()
}

pub fn soft_break() -> Document {
    Document::soft_break()
}

pub fn hard_break() -> Document {
    Document::hard_break()
}

pub fn concat(items: Vec<Document>) -> Document {
    Document::concat(items)
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
