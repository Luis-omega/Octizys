use octizys_common::span::{Position, Span};
use octizys_text_store::store::{NonLineBreakString, Store};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Documentation,
    NonDocumentation,
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineCommentStart {
    // --
    DoubleHypen,
    // //
    DoubleSlash,
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

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct CommentLine {
    pub kind: CommentKind,
    pub start: LineCommentStart,
    pub content: CommentLineContent,
    pub span: Span,
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
}
