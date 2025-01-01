use octizys_common::span::{Position, Span};
use octizys_macros::Equivalence;
use octizys_text_store::store::{NonLineBreakString, Store};

/// Represents a single line of a comment without line breaks.
/// We internalize all the source text in a [`Store`] and handle comments
/// specially.
/// This structure represents a pointer to the location of the comment
/// together with the length of the comment (as understated by the
/// notion of length on the store).
/// Storing the length here, allow us to avoid access to the store.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Equivalence)]
pub struct CommentLineContent {
    // TODO: a newtype around the index type.:
    index: usize,
    len: usize,
}

impl CommentLineContent {
    /// Stores as single string as a comment string in the store.
    /// The string is supposed to be a text without line breaks, if it
    /// contains any line break the function fails.
    /// To create a [`CommentLineContent`] from an arbitrary string
    /// without failing use [`CommentLineContent::decomposoe_register`] instead.
    pub fn make_register(value: &str, store: &mut Store) -> Option<Self> {
        store.comments.add_str(value).map(|x| CommentLineContent {
            index: x,
            len: value.len(),
        })
    }

    /// Internalizes an arbitrary string by breaking it at line breaks,
    /// and returning a vector with reference to every line.
    pub fn decompose_register(value: &str, store: &mut Store) -> Vec<Self> {
        store
            .comments
            .extend_and_get_lens(NonLineBreakString::decompose(value))
            .map(|(index, len)| CommentLineContent { index, len })
            .collect()
    }

    /// Returns the length of the stored comment as is understood by the
    /// pretty printing library.
    #[inline]
    pub fn get_len(&self) -> usize {
        self.len
    }

    /// Returns a reference to a place in the Store.
    #[inline]
    pub fn get_index(&self) -> usize {
        self.index
    }
}

/// Distinguish between regular comments and documentation ones.
/// A documentation includes a space and a pipe ` |` after the
/// start of the comment.
/// In the future we may allow for any amount of spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Equivalence)]
pub enum CommentKind {
    Documentation,
    NonDocumentation,
}

/// The delimiters for a [`CommentBlock`].
/// Currently, they are four kinds of block delimiters.
/// All of them are a `{` followed by between 1 and 4
/// `-`, then the comment of the block and
/// finish with the same amount of `-` followed
/// by a `}`.
///
/// <div class="warning">
///
/// This means that a comment like:
///
/// ```txt
/// {-- some comment {--- inner comment ---} remain --}
/// ```
///
/// Would be processed as
///
/// ```txt
/// {-- some comment {--- inner comment ---}
/// ```
///
/// Causing a syntax error at `--}`.
/// To nest comments is recommended to begin using a singe hyphen
/// and continue incrementing them as needed.
///
/// </div>
///
/// We acknowledge the need for nested block comments but at
/// the same time we believe that a finite amount of them
/// is enough for most uses if not all of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Equivalence)]
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
    /// Returns the total length in bytes of the start of a block
    /// with the given kind, this means that it has the length of
    /// the full `{--`.
    pub fn len(self) -> usize {
        match self {
            Self::Brace0 => 2,
            Self::Brace1 => 3,
            Self::Brace2 => 4,
            Self::Brace3 => 5,
        }
    }
}

/// Represents the beginning of a [`CommentLine`], they
/// can be any of :
///
/// - `//`
/// - `--`
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Equivalence)]
pub enum LineCommentStart {
    // --
    DoubleHyphen,
    // //
    DoubleSlash,
}

/// A representation of a multi-line of comment in the source.
/// Every comment has the following components:
/// - A [`CommentLine::kind`] to distinguish between documentation and
///   regular comments.
/// - A [`CommentLine::start`], we support multiple brackets for
///   block comments, see [`CommentBraceKind`].
/// - A [`CommentLine::content`] it has all the content of multiple lines
///   of the comment except from the line breaks and the comment brackets.
/// - A [`CommentBlock::span`], the region on the source in which the
///   comment exists.
///
/// #Examples
///
/// ```txt
/// {- some
///    regular
///    block comment
/// -}
///
/// {- | documentation
///   block
///   comment -}
///
/// {--- another
///   regular
///   block
///   comment
///   ---}
/// ```

#[derive(Debug, Clone, PartialEq, Eq, Equivalence)]
pub struct CommentBlock {
    pub kind: CommentKind,
    pub brace: CommentBraceKind,
    pub content: Vec<CommentLineContent>,
    #[equivalence(ignore)]
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

/// A representation of a single line of comments in the source.
/// Every line comment has the following components:
/// - A [`CommentLine::kind`] to distinguish between documentation and
///   regular comments.
/// - A [`CommentLine::start`], we support two ways to begin a comment
///   in a line, by using either `//` or `--`.
/// - A [`CommentLine::content`] it has all the content of a line
///   comment after the start delimiter except from the line break.
/// - A [`CommentLine::span`], the region on the source in which the comment exists.
///
/// #Examples
/// ```txt
/// -- some regular line comment.
/// // another line comment comment.
/// // | a documentation line comment.
/// -- | another documentation line comment.
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Copy, Equivalence)]
pub struct CommentLine {
    pub kind: CommentKind,
    pub start: LineCommentStart,
    pub content: CommentLineContent,
    #[equivalence(ignore)]
    pub span: Span,
}

/// We can categorize the comments in two types:
/// - [`Comment::Line`] comments that began in a line and ends
///   at the end of line.
/// - [`Comment::Block`] multi-line commentaries.
///
/// <div class="warning">Documentation comments can be in any format, but the
/// official documentation generator is going to use common Markdown
/// or some other dialect of it.</div>
///
/// We may represent in the future additional kind of comments like:
///
/// - `Todo:`
/// - `Warning:`
/// - `Note:`
///
/// At the current time we don't know if this is going to be useful.
#[derive(Debug, Clone, PartialEq, Eq, Equivalence)]
pub enum Comment {
    Line(CommentLine),
    Block(CommentBlock),
}

impl Comment {
    pub fn get_span(&self) -> Span {
        match self {
            // Cloning spans is cheap!
            Self::Line(CommentLine { span, .. }) => span.to_owned(),
            Self::Block(CommentBlock { span, .. }) => span.to_owned(),
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

/// Every token in a file can have any kind of comment above it and
/// can have a single line comment in the same line after the token.
///
/// #Example
///
/// ```txt
/// -- some line comment
/// // other comment
/// -- | Documentation line comment
/// token  // More comments
/// ```
/// This example has 3 comments above the token
/// and a comment after it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentsInfo {
    pub before: Vec<Comment>,
    pub after: Vec<Comment>,
}

impl Default for CommentsInfo {
    fn default() -> Self {
        CommentsInfo {
            before: vec![],
            after: vec![],
        }
    }
}

impl CommentsInfo {
    /// A commentaries' information that doesn't have any information inside.:
    /// This is useful if we need to attach information to a node before
    /// we have the information available.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Add the elements of an iterator of comments to the comments
    /// in the before field.
    /// The comments are added after all the original comments in
    /// the before field.
    pub fn extend<T>(&mut self, remmain: T) -> ()
    where
        T: IntoIterator<Item = Comment>,
    {
        self.before.extend(remmain)
    }

    /// Add a single comment to the `before` field.
    pub fn push(&mut self, new: Comment) -> () {
        self.before.push(new)
    }

    /// Move the comments after a token to before it.
    pub fn move_after_to_before(&mut self) -> () {
        // We made sure that cloning comments is acceptable
        // by storing the comments in a separate place.
        self.before
            .extend(self.after.iter().rev().map(|x| x.clone()));
        self.after = vec![];
    }

    /// Takes another comment info (maybe from another token)
    /// and combines it with the current one.
    /// Not every comment before or after a token can made sense,
    /// and we may choose to move some comments from its original place
    /// to another one where it made more sense.
    ///
    /// # Examples
    ///
    /// In the following block:
    ///
    /// ```txt
    ///  -- | Some comment about `a`
    /// let a =
    ///   (
    ///     -- | The first before comment of `1`
    ///     1, -- | The before comment of `,`, going to be moved to the before comments of `1`
    ///     2,
    ///     3, 4
    ///   )
    /// ```
    ///
    /// The third comment is attached to the comma as an after comment.
    /// The comment can be referring to the second item of the tuple
    /// or the third. If we decide to move it to the second item
    /// the end structure is the one described in the comments.
    pub fn consume_info(&mut self, other: CommentsInfo) -> () {
        self.move_after_to_before();
        let CommentsInfo { before, after } = other;
        self.extend(before.into_iter());
        self.extend(after.into_iter());
    }

    /// Take a CommentsInfo and transform all the contiguous occurrences
    /// of comments of the same type in a single block
    ///
    /// #Example
    ///
    /// This
    ///
    /// ```txt
    /// -- | 1
    /// -- | 2
    /// -- | 3
    /// // | 4
    /// // | 6
    /// -- 8
    /// {- 9 -}
    /// // | 10
    /// // | 11
    /// ```
    ///
    /// Becomes
    ///
    /// ```txt
    /// {- | 1
    ///      2
    ///      3
    ///      4
    ///      6 -}
    /// {- 8
    ///    9 -}
    /// {- | 10
    ///      11
    /// -}
    /// ```
    //FIXME:
    pub fn compact_comments(&mut self) -> () {}
}
