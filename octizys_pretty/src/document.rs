use std::ops::Add;

/// Adapted from the paper Strictly Pretty, Christian Lindig.
/// Is specialized to handle the source files, that's why
/// we use internalization of strings.
use string_interner::{DefaultStringInterner, DefaultSymbol};

pub type Interner = DefaultStringInterner;

#[derive(Debug, PartialEq, Eq, Clone)]
///This type is hidden as we must ensure the invariant of no "\n" in the texts
enum DocumentInternal {
    Empty,
    // Flat mode : translated to space
    // Break mode : translated to \n
    SoftBreak,
    // Always translated to HardBreak
    HardBreak,
    //Union of two or more files
    Concat(Vec<DocumentInternal>),
    // between keywords and identifiers used in a file
    InternalizedText(DefaultSymbol, usize),
    // for things like comments
    PlainText(String, usize),
    // set the Indentation level for the document that it contains
    Nest(u16, Box<DocumentInternal>),
    // Enable the option to be in flat or break mode
    Group(Box<DocumentInternal>),
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Flat,
    Break,
}

#[derive(Debug, Clone, Copy)]
struct FitsParam<'doc> {
    ident: u16,
    mode: Mode,
    doc: &'doc DocumentInternal,
}

impl<'doc> FitsParam<'doc> {
    fn add_ident(&self, ident: u16) -> Self {
        FitsParam {
            ident: self.ident + ident,
            ..*self
        }
    }

    fn as_flat(&self) -> Self {
        FitsParam {
            mode: Mode::Flat,
            ..*self
        }
    }

    fn as_break(&self) -> Self {
        FitsParam {
            mode: Mode::Break,
            ..*self
        }
    }

    fn with_document(&self, doc: &'doc DocumentInternal) -> Self {
        FitsParam { doc, ..*self }
    }
}

//TODO: change this to a better approximate using graphemes
pub fn aproximate_string_width(s: &str) -> usize {
    s.chars().count()
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(transparent)]
pub struct Document(DocumentInternal);

impl Document {
    pub fn empty() -> Self {
        Document(DocumentInternal::Empty)
    }

    /// Flat mode : a single space.
    /// Break mode : a line break.
    pub fn soft_break() -> Self {
        Document(DocumentInternal::SoftBreak)
    }

    /// Always a line break.
    pub fn hard_break() -> Self {
        Document(DocumentInternal::HardBreak)
    }

    pub fn concat(items: Vec<Document>) -> Self {
        Document(DocumentInternal::Concat(
            items.into_iter().map(|x| x.0).collect(),
        ))
    }

    /// Is expected to be used in keywords and identifiers
    /// to save space.
    pub fn internalize(
        interner: &mut Interner,
        maybe_word: &str,
    ) -> Option<Self> {
        match maybe_word.find("\n") {
            Some(_) => None,
            None => {
                let symbol = interner.get_or_intern(maybe_word);
                Some(Document(DocumentInternal::InternalizedText(
                    symbol,
                    aproximate_string_width(maybe_word),
                )))
            }
        }
    }

    pub fn from_symbol_and_len(s: DefaultSymbol, len: usize) -> Self {
        Document(DocumentInternal::InternalizedText(s, len))
    }

    /// This function decomposes the given str by "\n" and returns a equivalent document
    /// This preserves line breaks and spaces as is
    /// This doesn't perform internalization of the text
    pub fn text(words: &str) -> Self {
        if words.find("\n").is_none() {
            return Document(DocumentInternal::PlainText(
                String::from(words),
                aproximate_string_width(words),
            ));
        }
        let mut acc = vec![];
        for line in words.split("\n") {
            acc.push(DocumentInternal::PlainText(
                String::from(line),
                aproximate_string_width(line),
            ));
            acc.push(DocumentInternal::HardBreak);
        }
        Document(DocumentInternal::Concat(acc))
    }

    pub fn nest(indentation_level: u16, doc: Document) -> Document {
        Document(DocumentInternal::Nest(indentation_level, Box::new(doc.0)))
    }

    pub fn group(doc: Document) -> Document {
        Document(DocumentInternal::Group(Box::new(doc.0)))
    }

    pub fn into_iter<'doc>(
        &'doc self,
        width: usize,
        interner: &'doc Interner,
    ) -> DocumentIterator {
        DocumentIterator::new(self, width, interner).into_iter()
    }

    pub fn render_to_string(self, width: usize, interner: &Interner) -> String {
        self.into_iter(width, interner).collect()
    }
}

impl From<&str> for Document {
    fn from(value: &str) -> Document {
        Document::text(value)
    }
}

impl Add for Document {
    type Output = Document;
    fn add(self, rhs: Self) -> Self::Output {
        Document::concat(vec![self, rhs])
    }
}

#[derive(Debug)]
pub struct DocumentIterator<'doc> {
    consumed_width: usize,
    line_width: usize,
    stack: Vec<FitsParam<'doc>>,
    interner: &'doc Interner,
}

impl<'doc> DocumentIterator<'doc> {
    fn new(
        doc: &'doc Document,
        width: usize,
        interner: &'doc Interner,
    ) -> Self {
        let params = FitsParam {
            ident: 0,
            mode: Mode::Flat,
            doc: &doc.0,
        };
        DocumentIterator {
            consumed_width: 0,
            line_width: width,
            stack: vec![params],
            interner,
        }
    }

    fn advance_width_with(&mut self, new_str: String) -> String {
        self.consumed_width += aproximate_string_width(&new_str);
        new_str
    }

    fn gen_line_break(&mut self, indentation: u16) -> String {
        let new_str =
            [String::from("\n"), " ".repeat(usize::from(indentation))].join("");
        self.consumed_width = aproximate_string_width(&new_str);
        new_str
    }

    fn fits(&self, param: FitsParam) -> bool {
        if self.line_width <= self.consumed_width {
            return false;
        }
        let mut remain_width = self.line_width - self.consumed_width;

        let mut stack = vec![param];

        let mut state_stack = &self.stack as &[FitsParam];

        loop {
            let current = match stack.pop() {
                Some(c) => c,
                None => match state_stack.split_last() {
                    None => return true,
                    Some((last, remain)) => {
                        state_stack = remain;
                        *last
                    }
                },
            };
            match current.doc {
                DocumentInternal::Empty => continue,
                DocumentInternal::SoftBreak => match current.mode {
                    Mode::Flat => {
                        if 1 <= remain_width {
                            remain_width -= 1;
                        } else {
                            return false;
                        }
                    }
                    Mode::Break => return true,
                },
                DocumentInternal::HardBreak => return true,
                DocumentInternal::Concat(v) => {
                    for doc in v.into_iter().rev() {
                        stack.push(current.with_document(doc))
                    }
                }
                DocumentInternal::InternalizedText(_, width) => {
                    if *width <= remain_width {
                        remain_width -= width;
                    } else {
                        return false;
                    }
                }
                DocumentInternal::PlainText(_, width) => {
                    if *width <= remain_width {
                        remain_width -= width;
                    } else {
                        return false;
                    }
                }
                DocumentInternal::Nest(more_indent, doc) => stack
                    .push(current.with_document(doc).add_ident(*more_indent)),
                DocumentInternal::Group(doc) => {
                    stack.push(current.with_document(doc).as_flat())
                }
            }
        }
    }
}

impl<'doc> Iterator for DocumentIterator<'doc> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;
        match current.doc {
            DocumentInternal::Empty => Some(String::new()),
            DocumentInternal::SoftBreak => match current.mode {
                Mode::Break => Some(self.gen_line_break(current.ident)),
                Mode::Flat => Some(self.advance_width_with(String::from(" "))),
            },
            DocumentInternal::HardBreak => {
                Some(self.gen_line_break(current.ident))
            }
            DocumentInternal::Concat(v) => {
                for doc in v.into_iter().rev() {
                    self.stack.push(current.with_document(doc))
                }
                Some(String::new())
            }
            DocumentInternal::InternalizedText(s, _) => {
                //We abort the operation if we can't resolve a symbol
                let new_str = self.interner.resolve(*s).map(String::from)?;
                Some(self.advance_width_with(new_str))
            }
            // What to do here? we can put s behind a Rc but
            // we want the resulting string to be fully owned by the user.
            DocumentInternal::PlainText(s, _) => {
                Some(self.advance_width_with(s.clone()))
            }
            //TODO: move this function to a one that can handle this
            // without returning a empty string.
            // This is acceptable for now as a new empty string is low cost.
            DocumentInternal::Nest(more_indent, doc) => {
                self.stack
                    .push(current.with_document(doc).add_ident(*more_indent));
                Some(String::new())
            }
            DocumentInternal::Group(doc) => {
                if self.fits(current.with_document(doc).as_flat()) {
                    self.stack.push(current.with_document(doc).as_flat())
                } else {
                    self.stack.push(current.with_document(doc).as_break())
                }
                Some(String::new())
            }
        }
    }
}

#[cfg(test)]
mod render_tests {
    use crate::document::*;

    fn make_test(target_string: &str, doc: Document, width: usize) {
        let interner = Interner::new();
        assert_eq!(target_string, doc.render_to_string(width, &interner))
    }

    #[test]
    fn empty() {
        let doc = Document::empty();
        make_test("", doc, 10)
    }

    #[test]
    fn concat() {
        let raw1 = "hello world";
        let raw2 = " bye world";
        let document1 = Document::text(raw1);
        let document2 = Document::text(raw2);
        let document = Document::concat(vec![document1, document2]);
        make_test(&[raw1, raw2].join(""), document, 10)
    }

    #[test]
    fn text() {
        let raw = "hello world";
        let document = Document::text(raw);
        make_test(&raw, document, 10)
    }

    #[test]
    fn nest_break_group() {
        let document = Document::group(Document::concat(vec![
            Document::text("hello"),
            Document::nest(
                3,
                Document::concat(vec![
                    Document::soft_break(),
                    Document::text("world"),
                ]),
            ),
        ]));
        make_test("hello\n   world", document, 2)
    }

    #[test]
    fn nest_break_group_flat() {
        let document = Document::group(Document::concat(vec![
            Document::text("hello"),
            Document::nest(
                3,
                Document::concat(vec![
                    Document::soft_break(),
                    Document::text("world"),
                ]),
            ),
        ]));
        make_test("hello world", document, 20)
    }
}
