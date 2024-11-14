use octizys_common::{
    span::{Position, Span},
    Store,
};
use octizys_pretty::{
    combinators::{self, concat, empty, hard_break, intersperse, repeat},
    document::Document,
    store::NonLineBreakString,
};

use crate::pretty::{indent, PrettyCST, PrettyCSTContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLineContent {
    pub index: usize,
    pub len: usize,
}

impl CommentLineContent {
    //Register a single line without line breaks
    pub fn make_register(value: &str, store: &mut Store) -> Option<Self> {
        store.comments.add_str(value).map(|x| CommentLineContent {
            index: x,
            len: value.len(),
        })
    }

    pub fn decompose_register(value: &str, store: &mut Store) -> Vec<Self> {
        store
            .comments
            .extend_and_get_lens(NonLineBreakString::decompose(value))
            .map(|(index, len)| CommentLineContent { index, len })
            .collect()
    }
}

impl PrettyCST for CommentLineContent {
    fn to_document(&self, _context: &PrettyCSTContext) -> Document {
        self.into()
    }
}

impl From<&CommentLineContent> for Document {
    fn from(value: &CommentLineContent) -> Document {
        Document::from_index_and_len(value.index, value.len)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Documentation,
    NonDocumentation,
}

impl PrettyCST for CommentKind {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match self {
            CommentKind::Documentation => context.cache.comment_kind.into(),
            _ => empty(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentBraceKind {
    // "{- asdf -}"
    Brace0,
    // "{-- asdf --}"
    Brace1,
    // "{--- asdf ---}"
    Brace2,
    // "{---- asdf ----}"
    Brace3,
}
impl CommentBraceKind {
    pub fn len(self) -> usize {
        match self {
            Self::Brace0 => 2,
            Self::Brace1 => 3,
            Self::Brace2 => 4,
            Self::Brace3 => 5,
        }
    }
    pub fn to_documents(
        &self,
        context: &PrettyCSTContext,
    ) -> (Document, Document) {
        let number_of_dashes = self.len() - 1;
        let hypens = repeat(context.cache.hypen.into(), number_of_dashes);
        (
            Document::from(context.cache.lbrace) + hypens.clone(),
            hypens.clone() + context.cache.rbrace.into(),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineCommentStart {
    // --
    DoubleHypen,
    // //
    DoubleSlash,
}

impl PrettyCST for LineCommentStart {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match self {
            LineCommentStart::DoubleHypen => {
                repeat(context.cache.hypen.into(), 2)
            }
            LineCommentStart::DoubleSlash => {
                repeat(context.cache.slash.into(), 2)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentBlock {
    pub kind: CommentKind,
    pub brace: CommentBraceKind,
    pub content: Vec<CommentLineContent>,
    pub span: Span,
}
impl CommentBlock {
    pub fn make(
        kind: CommentKind,
        brace: CommentBraceKind,
        full_text: &str,
        start_pos: Position,
        end_pos: Position,
        store: &mut Store,
    ) -> Self {
        let content = CommentLineContent::decompose_register(full_text, store);
        CommentBlock {
            kind,
            brace,
            content,
            span: Span {
                start: start_pos,
                end: end_pos,
            },
        }
    }
}

impl PrettyCST for CommentBlock {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let (brace_start0, brace_end) = self.brace.to_documents(context);
        let block_start = brace_start0 + self.kind.to_document(context);
        let block_end = self.kind.to_document(context) + brace_end;
        let content_raw = intersperse(&self.content, hard_break());

        let content = if context.configuration.indent_comment_blocks {
            indent(context, hard_break() + content_raw) + hard_break()
        } else {
            content_raw
        };

        concat(vec![block_start, content, block_end, hard_break()])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLine {
    pub kind: CommentKind,
    pub start: LineCommentStart,
    pub content: CommentLineContent,
    pub span: Span,
}

impl PrettyCST for CommentLine {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        let line_start =
            self.start.to_document(context) + self.kind.to_document(context);
        concat(vec![
            line_start,
            Document::from(&self.content),
            hard_break(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
    Line(CommentLine),
    Block(CommentBlock),
}

impl Comment {
    pub fn get_span(self) -> Span {
        match self {
            Self::Line(CommentLine { span, .. }) => span,
            Self::Block(CommentBlock { span, .. }) => span,
        }
    }
}

impl PrettyCST for Comment {
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        match self {
            Comment::Line(l) => l.to_document(context),
            Comment::Block(l) => l.to_document(context),
        }
    }
}

impl From<CommentLine> for Comment {
    fn from(value: CommentLine) -> Comment {
        Comment::Line(value)
    }
}

impl From<CommentBlock> for Comment {
    fn from(value: CommentBlock) -> Comment {
        Comment::Block(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentsInfo {
    pub before: Vec<Comment>,
    pub after: Option<Comment>,
}

impl CommentsInfo {
    pub fn empty() -> Self {
        CommentsInfo {
            before: vec![],
            after: None,
        }
    }

    pub fn extend<T>(&mut self, remmain: T) -> ()
    where
        T: Iterator<Item = Comment>,
    {
        self.before.extend(remmain)
    }

    pub fn push(&mut self, new: Comment) -> () {
        self.before.push(new)
    }

    pub fn move_after_to_before(mut self) -> Self {
        match self.after {
            Some(c) => {
                self.before.push(c);
                self.after = None;
                return self;
            }
            None => return self,
        }
    }
    //TODO: this is wrong, check the docummentation on grammar about
    //the expected behaviour
    pub fn absorb_info(&mut self, other: CommentsInfo) -> () {
        let CommentsInfo { before, after } = other;
        self.extend(before.into_iter());
        match after {
            Some(c) => self.push(c),
            None => (),
        }
    }

    //FIXME:
    /// Take a CommentsInfo and transform all the contiguous occurrences
    /// of comments of the same type in a single block
    /// Example: Multiple documentation lines became a  documentation block
    /// Example: Multiple NonDocumentation lines became a block
    pub fn compact_comments(mut self) -> Self {
        self
    }

    pub fn to_document(
        &self,
        context: &PrettyCSTContext,
        doc: Document,
    ) -> Document {
        let mut out = self.clone();
        if context.configuration.move_documentantion_before_object {
            out = out.move_after_to_before();
        }
        if context.configuration.compact_comments {
            out = out.compact_comments();
        }
        let separe_by = usize::from(context.configuration.separe_comments_by);
        concat(vec![
            intersperse(
                out.before.into_iter().map(|x| x.to_document(context)),
                combinators::repeat(hard_break(), separe_by),
            ),
            doc,
            out.after.map_or(empty(), |x| x.to_document(context)),
        ])
    }
}
