use octizys_common::span::{Position, Span};
use octizys_pretty::{
    combinators::{
        self, concat, empty, group, hard_break, intersperse, nest, text,
    },
    document::Document,
};

use crate::pretty::{PrettyCST, PrettyCSTConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentLineContent {
    pub content: String,
}

impl CommentLineContent {
    pub fn make(value: &str) -> Result<Self, String> {
        if value.find("\n").is_some() {
            return Err(String::from(
                "CommentLines souldn't containt line breaks!",
            ));
        }
        let content = String::from(value);
        Ok(CommentLineContent { content })
    }

    pub fn decompose(value: &str) -> Vec<Self> {
        value
            .split("\n")
            .map(|x| CommentLineContent {
                content: x.to_string(),
            })
            .collect()
    }
}

impl PrettyCST for CommentLineContent {
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        text(&self.content)
    }
}

impl Into<Document> for CommentLineContent {
    fn into(self) -> Document {
        text(&self.content)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Documentation,
    NonDocumentation,
}

impl From<CommentKind> for &'static str {
    fn from(value: CommentKind) -> Self {
        match value {
            CommentKind::Documentation => " |",
            _ => "",
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
}

impl From<CommentBraceKind> for (&str, &str) {
    fn from(value: CommentBraceKind) -> Self {
        match value {
            CommentBraceKind::Brace0 => ("{-", "-}"),
            CommentBraceKind::Brace1 => ("{--", "--}"),
            CommentBraceKind::Brace2 => ("{---", "---}"),
            CommentBraceKind::Brace3 => ("{----", "----}"),
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

impl From<LineCommentStart> for char {
    fn from(value: LineCommentStart) -> Self {
        match value {
            LineCommentStart::DoubleHypen => '-',
            LineCommentStart::DoubleSlash => '/',
        }
    }
}

impl From<LineCommentStart> for &'static str {
    fn from(value: LineCommentStart) -> Self {
        match value {
            LineCommentStart::DoubleHypen => "--",
            LineCommentStart::DoubleSlash => "//",
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
    ) -> Self {
        let content = CommentLineContent::decompose(full_text);
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
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        let (block_start, block_end) = match self.brace {
            CommentBraceKind::Brace0 => match self.kind {
                CommentKind::Documentation => ("{- |", "-}"),
                CommentKind::NonDocumentation => ("{-", "-}"),
            },
            CommentBraceKind::Brace1 => match self.kind {
                CommentKind::Documentation => ("{-- |", "--}"),
                CommentKind::NonDocumentation => ("{--", "--}"),
            },
            CommentBraceKind::Brace2 => match self.kind {
                CommentKind::Documentation => ("{--- |", "---}"),
                CommentKind::NonDocumentation => ("{---", "---}"),
            },
            CommentBraceKind::Brace3 => match self.kind {
                CommentKind::Documentation => ("{---- |", "----}"),
                CommentKind::NonDocumentation => ("{----", "----}"),
            },
        };
        let content_raw = intersperse(self.content, hard_break());

        let content = if configuration.indent_comment_blocks {
            group(nest(
                configuration.indentation_deep,
                concat(vec![hard_break(), content_raw]),
            )) + hard_break()
        } else {
            content_raw
        };

        concat(vec![
            block_start.into(),
            content,
            block_end.into(),
            hard_break(),
        ])
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
    fn to_document(self, _configuration: PrettyCSTConfig) -> Document {
        let line_start = match self.start {
            LineCommentStart::DoubleHypen => match self.kind {
                CommentKind::Documentation => "-- |",
                CommentKind::NonDocumentation => "--",
            },
            LineCommentStart::DoubleSlash => match self.kind {
                CommentKind::Documentation => "// |",
                CommentKind::NonDocumentation => "//",
            },
        };
        concat(vec![line_start.into(), self.content.into(), hard_break()])
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
    fn to_document(self, configuration: PrettyCSTConfig) -> Document {
        match self {
            Comment::Line(l) => l.to_document(configuration),
            Comment::Block(l) => l.to_document(configuration),
        }
    }
}

impl Into<Comment> for CommentLine {
    fn into(self) -> Comment {
        Comment::Line(self)
    }
}

impl Into<Comment> for CommentBlock {
    fn into(self) -> Comment {
        Comment::Block(self)
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

    /// Take a CommentsInfo and transform all the contiguous occurrences
    /// of comments of the same type in a single block
    /// Example: Multiple documentation lines became a  documentation block
    /// Example: Multiple NonDocumentation lines became a block
    pub fn compact_comments(mut self) -> Self {
        todo!()
    }

    pub fn to_document(
        mut self,
        configuration: PrettyCSTConfig,
        doc: Document,
    ) -> Document {
        if configuration.move_documentantion_before_object {
            self = self.move_after_to_before();
        }
        if configuration.compact_comments {
            self = self.compact_comments();
        }
        let separe_by = usize::from(configuration.separe_comments_by);
        concat(vec![
            intersperse(
                self.before
                    .into_iter()
                    .map(|x| x.to_document(configuration)),
                combinators::repeat(hard_break(), separe_by),
            ),
            doc,
            self.after.map_or(empty(), |x| x.to_document(configuration)),
        ])
    }
}
