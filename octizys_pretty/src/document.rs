use std::ops::Add;

use octizys_text_store::store::{
    aproximate_string_width, NonLineBreakStr, NonLineBreakString, Store,
    StoreSymbol,
};

use crate::highlight::{Color, Emphasis, Highlight};

/// Adapted from the paper Strictly Pretty, Christian Lindig.
/// Is specialized to handle the source files, that's why
/// we use internalization of strings.

#[derive(Debug, PartialEq, Eq, Clone)]
///This type is hidden as we must ensure the invariant of no "\n" in the texts
enum DocumentInternal {
    Empty,
    /// Flat mode : space
    /// Break mode : \n
    SoftBreak,
    /// Always translated to \n
    HardBreak,
    /// Flat mode : empty
    /// Break mode : \n
    EmptyBreak,
    ///Union of two or more files
    Concat(Vec<DocumentInternal>),
    ///Keywords and identifiers used in a file
    StoredRegularText(StoreSymbol, usize),
    ///All the text that isn't part of the original document and we didn't
    ///allocate in a separate structure
    ExternalText(NonLineBreakString, usize),
    StaticText(NonLineBreakStr, usize),
    /// For comments stored in a vector as String.
    /// This doesn't add a line beak!
    StoredCommentText {
        index: usize,
        len: usize,
    },
    /// Set the Indentation level for the document that it contains
    Nest(u16, Box<DocumentInternal>),
    /// Enable the option to be in flat or break mode
    Group(Box<DocumentInternal>),
    /// Establish only the background color
    Background(Color, Box<DocumentInternal>),
    /// Establish only the foreground color
    Foreground(Color, Box<DocumentInternal>),
    /// Establish only the emphasis for the document
    Emphasis(Emphasis, Box<DocumentInternal>),
    /// Use the given Highlight for the given document.
    Highlight(Highlight, Box<DocumentInternal>),
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
    highlight: Highlight,
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

    fn with_highlight(&self, highlight: Highlight) -> Self {
        FitsParam { highlight, ..*self }
    }
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

    /// Flat mode : empty doc.
    /// Break mode : a line break.
    pub fn empty_break() -> Self {
        Document(DocumentInternal::EmptyBreak)
    }

    pub fn concat(items: Vec<Document>) -> Self {
        if items.len() == 0 {
            return Document::empty();
        }
        Document(DocumentInternal::Concat(
            items.into_iter().map(|x| x.0).collect(),
        ))
    }

    pub fn try_internalize(
        store: &mut Store,
        maybe_words: &str,
    ) -> Option<Self> {
        let symbol = store.regular.try_add(maybe_words)?;
        let len = aproximate_string_width(maybe_words);
        Some(Document(DocumentInternal::StoredRegularText(symbol, len)))
    }

    /// Returns a document of a string stored in the regular store
    /// provided that we already know the symbol and len.
    /// No verification is done on the symbol or the len, is
    /// user responsibility to check that they are valid.
    pub fn from_symbol_and_len(s: StoreSymbol, len: usize) -> Self {
        Document(DocumentInternal::StoredRegularText(s, len))
    }

    pub fn internalize_non_line_break_string(
        store: &mut Store,
        s: NonLineBreakString,
    ) -> Self {
        let len = aproximate_string_width((&s).into());
        let symbol = store.regular.add(s);
        Document(DocumentInternal::StoredRegularText(symbol, len))
    }

    /// Returns a document of a string stored in the comments store
    /// provided that we already know the symbol and len.
    /// No verification is done on the index or the len, is
    /// user responsibility to check that they are valid.
    pub fn from_index_and_len(index: usize, len: usize) -> Self {
        Document(DocumentInternal::StoredCommentText { index, len })
    }

    pub fn internalize_non_line_break_str(
        store: &mut Store,
        s: NonLineBreakStr,
    ) -> Self {
        let len = aproximate_string_width(s.into());
        let symbol = store.regular.add_str(s);
        Document(DocumentInternal::StoredRegularText(symbol, len))
    }

    /// This function decomposes the given str by "\n" and returns a equivalent document
    /// This preserves line breaks and spaces as is
    /// This doesn't perform internalization of the text
    pub fn external_text(words: &str) -> Self {
        let mut acc = vec![];
        for word in NonLineBreakString::decompose(words) {
            let len = aproximate_string_width((&word).into());
            let doc = DocumentInternal::ExternalText(word, len);
            acc.push(doc);
            acc.push(DocumentInternal::HardBreak);
        }
        acc.pop();
        let len = acc.len();
        if len == 0 {
            return Document::empty();
        } else if len == 1 {
            return Document(acc.pop().unwrap());
        }
        Document(DocumentInternal::Concat(acc))
    }

    pub fn static_str(word: NonLineBreakStr) -> Document {
        let s = word.as_str();
        let len = aproximate_string_width(s);
        Document(DocumentInternal::StaticText(word, len))
    }

    pub fn comment_line(
        source: &str,
        comments_accumulator: &mut Vec<String>,
    ) -> Document {
        let index = comments_accumulator.len();
        comments_accumulator.push(String::from(source));
        Document(DocumentInternal::StoredCommentText {
            index,
            len: aproximate_string_width(source),
        })
    }

    pub fn nest(indentation_level: u16, doc: Document) -> Document {
        Document(DocumentInternal::Nest(indentation_level, Box::new(doc.0)))
    }

    pub fn group(doc: Document) -> Document {
        Document(DocumentInternal::Group(Box::new(doc.0)))
    }

    pub fn background(color: Color, doc: Document) -> Document {
        Document(DocumentInternal::Background(color, Box::new(doc.0)))
    }

    pub fn foreground(color: Color, doc: Document) -> Document {
        Document(DocumentInternal::Foreground(color, Box::new(doc.0)))
    }

    pub fn emphasis(emphasis: Emphasis, doc: Document) -> Document {
        Document(DocumentInternal::Emphasis(emphasis, Box::new(doc.0)))
    }

    pub fn highlight(highlight: Highlight, doc: Document) -> Document {
        Document(DocumentInternal::Highlight(highlight, Box::new(doc.0)))
    }

    pub fn into_iter<'doc>(
        &'doc self,
        width: usize,
        store: &'doc Store,
        highlight: fn(&Highlight) -> (String, String),
    ) -> DocumentIterator<'doc> {
        DocumentIterator::new(self, width, highlight, store).into_iter()
    }

    pub fn render_to_string<'doc>(
        &'doc self,
        width: usize,
        highlight: fn(&Highlight) -> (String, String),
        store: &'doc Store,
    ) -> String {
        self.into_iter(width, store, highlight).collect()
    }
}

impl From<&str> for Document {
    fn from(value: &str) -> Document {
        Document::external_text(value)
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
    store: &'doc Store,
    /// A function that returns a previous text and after text for a Highlight.
    highlight_renderer: fn(&Highlight) -> (String, String),
}

impl<'doc> DocumentIterator<'doc> {
    fn new(
        doc: &'doc Document,
        width: usize,
        highlight_renderer: fn(&Highlight) -> (String, String),
        store: &'doc Store,
    ) -> Self {
        let params = FitsParam {
            ident: 0,
            mode: Mode::Flat,
            doc: &doc.0,
            highlight: Default::default(),
        };
        DocumentIterator {
            consumed_width: 0,
            line_width: width,
            stack: vec![params],
            store,
            highlight_renderer,
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
                DocumentInternal::EmptyBreak => return true,
                DocumentInternal::Concat(v) => {
                    for doc in v.into_iter().rev() {
                        stack.push(current.with_document(doc))
                    }
                }
                DocumentInternal::StoredRegularText(_, width) => {
                    if *width <= remain_width {
                        remain_width -= width;
                    } else {
                        return false;
                    }
                }
                DocumentInternal::ExternalText(_, width) => {
                    if *width <= remain_width {
                        remain_width -= width;
                    } else {
                        return false;
                    }
                }
                DocumentInternal::StaticText(_, width) => {
                    if *width <= remain_width {
                        remain_width -= width;
                    } else {
                        return false;
                    }
                }
                DocumentInternal::StoredCommentText { len, .. } => {
                    if *len <= remain_width {
                        remain_width -= len;
                    } else {
                        return false;
                    }
                }
                DocumentInternal::Nest(more_indent, doc) => stack
                    .push(current.with_document(doc).add_ident(*more_indent)),
                DocumentInternal::Group(doc) => {
                    stack.push(current.with_document(doc).as_flat())
                }
                DocumentInternal::Foreground(_, doc) => {
                    stack.push(current.with_document(doc))
                }
                DocumentInternal::Background(_, doc) => {
                    stack.push(current.with_document(doc))
                }
                DocumentInternal::Emphasis(_, doc) => {
                    stack.push(current.with_document(doc))
                }
                DocumentInternal::Highlight(_, doc) => {
                    stack.push(current.with_document(doc))
                }
            }
        }
    }
}

pub fn add_highlight<'doc>(
    highlight: &'doc Highlight,
    s: &'doc str,
    render: fn(&'doc Highlight) -> (String, String),
) -> String {
    let (start, end) = render(highlight);
    format!("{start}{s}{end}")
}

impl<'doc> Iterator for DocumentIterator<'doc> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;
        match current.doc {
            DocumentInternal::Empty => Some(String::new()),
            DocumentInternal::SoftBreak => match current.mode {
                Mode::Break => Some(add_highlight(
                    &current.highlight,
                    &self.gen_line_break(current.ident),
                    self.highlight_renderer,
                )),
                Mode::Flat => Some(add_highlight(
                    &current.highlight,
                    &self.advance_width_with(String::from(" ")),
                    self.highlight_renderer,
                )),
            },
            DocumentInternal::HardBreak => Some(add_highlight(
                &current.highlight,
                &self.gen_line_break(current.ident),
                self.highlight_renderer,
            )),
            DocumentInternal::EmptyBreak => match current.mode {
                Mode::Flat => Some(String::new()),
                Mode::Break => Some(add_highlight(
                    &current.highlight,
                    &self.gen_line_break(current.ident),
                    self.highlight_renderer,
                )),
            },
            DocumentInternal::Concat(v) => {
                for doc in v.into_iter().rev() {
                    self.stack.push(current.with_document(doc))
                }
                Some(String::new())
            }
            DocumentInternal::StoredRegularText(s, _) => {
                //We abort the operation if we can't resolve a symbol
                //maybe we should instead produce a blank and continue?
                let new_str =
                    self.store.regular.resolve(*s).map(String::from)?;
                Some(add_highlight(
                    &current.highlight,
                    &self.advance_width_with(new_str),
                    self.highlight_renderer,
                ))
            }
            // What to do here? we can put s behind a Rc but
            // we want the resulting string to be fully owned by the user.
            DocumentInternal::ExternalText(s, _) => Some(add_highlight(
                &current.highlight,
                &self.advance_width_with((*s).clone().into()),
                self.highlight_renderer,
            )),
            DocumentInternal::StaticText(s, _) => Some(add_highlight(
                &current.highlight,
                &self.advance_width_with(String::from((*s).as_str())),
                self.highlight_renderer,
            )),
            DocumentInternal::StoredCommentText { index, .. } => {
                let new_str = self
                    .store
                    .comments
                    .resolve(*index)
                    .map(|x| (*x).clone().into())?;
                Some(add_highlight(
                    &current.highlight,
                    &self.advance_width_with(new_str),
                    self.highlight_renderer,
                ))
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
            DocumentInternal::Background(c, d) => {
                let new_highlight = Highlight {
                    background: *c,
                    ..current.highlight
                };
                self.stack.push(
                    current.with_document(d).with_highlight(new_highlight),
                );
                Some(String::new())
            }
            DocumentInternal::Foreground(c, d) => {
                let new_highlight = Highlight {
                    foreground: *c,
                    ..current.highlight
                };
                self.stack.push(
                    current.with_document(d).with_highlight(new_highlight),
                );
                Some(String::new())
            }
            DocumentInternal::Emphasis(e, d) => {
                let new_highlight = Highlight {
                    emphasis: *e,
                    ..current.highlight
                };
                self.stack.push(
                    current.with_document(d).with_highlight(new_highlight),
                );
                Some(String::new())
            }
            DocumentInternal::Highlight(h, d) => {
                self.stack.push(current.with_document(d).with_highlight(*h));
                Some(String::new())
            }
        }
    }
}

#[cfg(test)]
mod render_tests {
    use crate::document::*;
    use crate::highlight::{EmptyRender, HighlightRenderer};

    fn make_test(target_string: &str, doc: Document, width: usize) {
        let store = Store::default();
        let result =
            doc.render_to_string(width, EmptyRender::render_highlight, &store);
        assert_eq!(target_string, result)
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
        let document1 = Document::external_text(raw1);
        let document2 = Document::external_text(raw2);
        let document = Document::concat(vec![document1, document2]);
        make_test(&[raw1, raw2].join(""), document, 10)
    }

    #[test]
    fn text() {
        let raw = "hello world";
        let document = Document::external_text(raw);
        make_test(&raw, document, 10)
    }

    #[test]
    fn nest_break_group() {
        let document = Document::group(Document::concat(vec![
            Document::external_text("hello"),
            Document::nest(
                3,
                Document::concat(vec![
                    Document::soft_break(),
                    Document::external_text("world"),
                    Document::soft_break(),
                    Document::external_text("!"),
                ]),
            ),
        ]));
        make_test("hello\n   world\n   !", document, 2)
    }

    #[test]
    fn nest_break_group_flat() {
        let document = Document::group(Document::concat(vec![
            Document::external_text("hello"),
            Document::nest(
                3,
                Document::concat(vec![
                    Document::soft_break(),
                    Document::external_text("world"),
                    Document::soft_break(),
                    Document::external_text("!"),
                ]),
            ),
        ]));
        make_test("hello world !", document, 20)
    }
}
