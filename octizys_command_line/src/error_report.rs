use lalrpop_util::ParseError;
use octizys_common::span::{Position, Span};
use octizys_cst::base::TokenInfo;
use octizys_parser::lexer::{LexerError, Token};
use octizys_pretty::{
    combinators::{
        self, concat, emphasis, empty, empty_break, external_text, foreground,
        group, hard_break, intersperse, nest, soft_break, static_str,
    },
    document::Document,
    highlight::{
        base_colors::{CYAN, MAGENTA, MODERATE_GREEN, RED},
        Color, Emphasis,
    },
    store::NonLineBreakStr,
};
use octizys_text_store::store::approximate_string_width;

macro_rules! static_text {
    ($text:expr) => {
        static_str(NonLineBreakStr::new($text))
    };
}

#[derive(Debug)]
pub struct ReportSourceContext<'source> {
    pub src: &'source str,
    pub src_name: &'source str,
    pub max_line_width: u16,
}

impl<'a> Default for ReportSourceContext<'a> {
    fn default() -> Self {
        ReportSourceContext {
            src: &"",
            src_name: &"octizys_repl",
            max_line_width: 80,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ReportUserKind {
    /// A user that is new to programming in general and needs verbose errors
    /// with detailed explanations about basic things.
    #[default]
    New,
    /// A user with some background in programming and can understand more
    /// technical errors with fewer words.
    Advanced,
}

#[derive(Debug, Clone, Copy)]
pub enum ReportTarget {
    /// For regular compiler errors in terminal or lsp.
    Human(ReportUserKind),
    /// If the output is expected to be processed by a
    /// programming language.
    Machine(ReportUserKind),
}

impl Default for ReportTarget {
    fn default() -> Self {
        ReportTarget::Human(ReportUserKind::New)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ReportKind {
    #[default]
    Error,
    Warning,
    Info,
    Note,
}

impl ReportKind {
    fn as_str(&self) -> NonLineBreakStr {
        NonLineBreakStr::new(match self {
            ReportKind::Error => "Error",
            ReportKind::Warning => "Warning",
            ReportKind::Info => "Info",
            ReportKind::Note => "Note",
        })
    }
    fn color(&self) -> Color {
        match self {
            ReportKind::Error => RED,
            ReportKind::Warning => RED,
            ReportKind::Info => RED,
            ReportKind::Note => RED,
        }
    }
}

pub enum Location {
    Span(Span),
    Position(Position),
}

impl Location {
    fn to_document(&self) -> Document {
        match self {
            Location::Span(s) => {
                let start = position_to_document(&s.start);
                let end = position_to_document(&s.end);
                concat(vec![
                    static_text!("::From::"),
                    start,
                    static_text!("::To::"),
                    end,
                ])
            }
            Location::Position(p) => {
                let pos = position_to_document(p);
                static_text!("::At::") + pos
            }
        }
    }
}

pub trait ReportFormat {
    fn get_report_name(&self) -> NonLineBreakStr;
    fn get_short_description(&self) -> NonLineBreakStr;
    fn get_long_description(&self, target: &ReportTarget) -> Option<Document>;
    fn get_expected(&self) -> Option<Vec<String>>;
    fn get_location(&self) -> Option<Location>;
}

pub struct ReportRequest<'source, T>
where
    T: ReportFormat,
{
    pub report: &'source T,
    pub source_context: ReportSourceContext<'source>,
    pub target: ReportTarget,
    pub kind: ReportKind,
}

fn position_to_document(p: &Position) -> Document {
    concat(vec![
        static_text!("Line{"),
        external_text((1 + p.line).to_string().as_str()),
        external_text("}"),
        static_text!("::Column{"),
        external_text((1 + p.column).to_string().as_str()),
        static_text!("}"),
    ])
}

pub fn make_report_kind<R: ReportFormat>(
    report: &R,
    report_kind: &ReportKind,
    report_target: &ReportTarget,
) -> Document {
    let kind = report_kind.as_str();
    let color = report_kind.color();

    match report_target {
        ReportTarget::Human(_) => {
            let report_name = report.get_report_name();
            foreground(
                color,
                concat(vec![
                    static_str(kind),
                    static_text!("!["),
                    static_str(report_name),
                    static_text!("]: "),
                ]),
            )
        }
        ReportTarget::Machine(_) => {
            foreground(color, static_str(kind) + static_text!("!:"))
        }
    }
}

// TODO: add doc test for the examples
/// Generate the beginning of the error report (the part before we
/// show the source code signalling the error).
///
/// # Examples
///
/// ```text
/// Error![Internal::EnumName]ShortDescription
///   --> SourcePath Span
/// ```
///
/// ```text
/// Error![EnumName]ShortDescription
///   --> SourcePath Span
/// ```
///
/// ```text
/// Error![EnumName]ShortDescription
///   --> SourcePath Position
/// ```
pub fn make_report_info_start<R: ReportFormat>(
    request: &ReportRequest<R>,
) -> Document {
    let location = request.report.get_location();
    let kind = make_report_kind(request.report, &request.kind, &request.target);
    let error_name = request.report.get_report_name();
    let short_description = request.report.get_short_description();
    let location_doc = concat(vec![
        foreground(CYAN, external_text("-->")),
        foreground(
            MODERATE_GREEN,
            external_text(request.source_context.src_name),
        ),
        location.map_or_else(combinators::empty, |x| Location::to_document(&x)),
    ]);
    concat(vec![
        kind,
        static_str(short_description),
        nest(2, hard_break() + location_doc),
    ])
}

fn expected_to_document(
    expected: Option<Vec<String>>,
    target: &ReportTarget,
) -> Document {
    match expected {
        None => empty(),
        Some(v) => {
            let is_human = match target {
                ReportTarget::Human(_) => true,
                ReportTarget::Machine(_) => false,
            };

            let previous_text: Document;
            if v.len() == 0 {
                return empty();
            } else if v.len() == 1 {
                previous_text = if is_human {
                    static_text!("Expected:")
                } else {
                    static_text!("Expected one of:")
                };
            } else {
                previous_text = static_text!("Expected one of:");
            }
            let separator = if is_human {
                soft_break()
            } else {
                static_text!(" ")
            } + static_text!(",");
            previous_text
                + group(nest(
                    2,
                    if is_human {
                        empty_break()
                    } else {
                        static_text!(" ")
                    } + intersperse(
                        v.into_iter().map(|x| {
                            emphasis(
                                Emphasis::Bold,
                                foreground(CYAN, external_text(&x)),
                            )
                        }),
                        separator,
                    ),
                ))
        }
    }
}

fn make_source_error<R: ReportFormat>(
    request: &ReportRequest<R>,
    location: Location,
) -> Document {
    match location {
        Location::Span(span) => {
            //We need to get this first
            let (_, content, _) =
                span.get_text_at(request.source_context.src, None);
            if span.start.line == span.end.line {
                let remain_width =
                    match u16::try_from(approximate_string_width(content)) {
                        Ok(width) => request
                            .source_context
                            .max_line_width
                            .saturating_sub(width),
                        Err(_) => 0,
                    };
                let (before, _, after) = span.get_text_at(
                    request.source_context.src,
                    Some(usize::from(remain_width)),
                );
                let pre_spaces = approximate_string_width(before);
                let pointer = "^".repeat(approximate_string_width(content));
                concat(vec![
                    external_text(before),
                    emphasis(
                        Emphasis::Underline(RED),
                        foreground(MAGENTA, external_text(content)),
                    ),
                    external_text(after),
                    nest(
                        // This may be safe as [`before`] has to be smaller
                        // than [`remain_width`].
                        u16::try_from(pre_spaces).unwrap(),
                        concat(vec![
                            hard_break(),
                            //TODO: if the span is too large, this may be over the line width!
                            //but only the "^Expected" part!
                            foreground(RED, external_text(&pointer)),
                            foreground(
                                RED,
                                expected_to_document(
                                    request.report.get_expected(),
                                    &request.target,
                                ),
                            ),
                        ]),
                    ),
                ])
            } else {
                concat(vec![
                    emphasis(
                        Emphasis::Underline(RED),
                        foreground(MAGENTA, external_text(content)),
                    ),
                    hard_break(),
                    foreground(
                        RED,
                        expected_to_document(
                            request.report.get_expected(),
                            &request.target,
                        ),
                    ),
                ])
            }
        }
        Location::Position(position) => {
            let (before, after) =
                position.get_text_at(request.source_context.src, Some(80));
            let pre_spaces = approximate_string_width(before);
            let spaces = " ".repeat(pre_spaces) + "^";
            let mut after_iter = after.chars();
            let after_doc = match after_iter.next() {
                Some(c) => {
                    emphasis(
                        Emphasis::Underline(RED),
                        foreground(MAGENTA, external_text(&String::from(c))),
                    ) + {
                        let remain: String = after_iter.collect();
                        external_text(&remain)
                    }
                }
                None => empty(),
            };
            concat(vec![
                external_text(before),
                after_doc,
                hard_break(),
                foreground(RED, external_text(&spaces)),
                foreground(
                    RED,
                    expected_to_document(
                        request.report.get_expected(),
                        &request.target,
                    ),
                ),
            ])
        }
    }
}

pub fn create_error_report<R: ReportFormat>(
    request: &ReportRequest<R>,
) -> Document {
    let header = make_report_info_start(request);
    let location = request.report.get_location();
    let has_location = location.is_some();
    let source = location
        .map_or_else(combinators::empty, |x| make_source_error(request, x));
    let long_description =
        match request.report.get_long_description(&request.target) {
            Some(d) => d,
            None => empty(),
        };
    concat(vec![
        header,
        if has_location {
            nest(4, hard_break() + source)
        } else {
            combinators::empty()
        },
        nest(2, hard_break() + long_description),
    ])
}

impl ReportFormat for LexerError {
    fn get_report_name(&self) -> NonLineBreakStr {
        // DO NOT REMOVE THE NonLineBreakStr out of the match!
        // It performs a compile time check on the passed string,
        // it would panic at run time if moved to the top match.
        match self {
            LexerError::UnexpectedCharacter(_) => {
                NonLineBreakStr::new("UnexpectedCharacter")
            }
            LexerError::UnexpectedPunctuationMatch(_, _) => {
                NonLineBreakStr::new("Internal:UnexpectedPunctuationMatch")
            }
            LexerError::UnexpectedCommentMatch(_, _) => {
                NonLineBreakStr::new("Internal:UnexpectedCommentMatch")
            }
            LexerError::NonFinishedLineComment(_, _) => {
                NonLineBreakStr::new("Internal:NonFinishedLineComment")
            }
            LexerError::NonContentInLineComment(_, _) => {
                NonLineBreakStr::new("Internal:NonContentInLineComment")
            }
            LexerError::CantCreateCommentLine(_, _) => {
                NonLineBreakStr::new("Internal:CantCreateCommentLine")
            }
            LexerError::CouldntMatchBlockComment(_, _, _) => {
                NonLineBreakStr::new("CouldntMatchBlockComment")
            }
            LexerError::Notu64NamedHole(_, _) => {
                NonLineBreakStr::new("Notu64NamedHole")
            }
            LexerError::CantCreateIdentifier(_, _) => {
                NonLineBreakStr::new("Internal:CantCreateIdentifier")
            }
            LexerError::CantTranslateToToken(_) => {
                NonLineBreakStr::new("Internal:CantTranslateToToken")
            }
            LexerError::UnexpectedOwnershipLiteralMatch(_, _) => {
                NonLineBreakStr::new("Internal:UnexpectedOwnershipLiteralMatch")
            }
            LexerError::CantParseU64(_, _, _) => {
                NonLineBreakStr::new("Internal:CantParseU64")
            }
        }
    }
    fn get_short_description(&self) -> NonLineBreakStr {
        let common =
            NonLineBreakStr::new("This is a bug in octizys, please report it!");
        // DO NOT REMOVE THE NonLineBreakStr out of the match!
        // It performs a compile time check on the passed string,
        // it would panic at run time if moved to the top match.
        match self {
            LexerError::UnexpectedCharacter(_) => {
                NonLineBreakStr::new("The provided character doesn't correspond to a valid program.")
            }
            LexerError::UnexpectedPunctuationMatch(_, _) => common,
            LexerError::UnexpectedCommentMatch(_, _) => common,
            LexerError::NonFinishedLineComment(_, _) => common,
            LexerError::NonContentInLineComment(_, _) => common,
            LexerError::CantCreateCommentLine(_, _) => common,
            LexerError::CouldntMatchBlockComment(_,_, _) => NonLineBreakStr::new("We found the beginning of a block comment but couldn't finished it!"),
            LexerError::Notu64NamedHole(_, _) => {
                NonLineBreakStr::new("Named holes are limited to u64 integers.")
            }
            LexerError::CantCreateIdentifier(_, _) => common,
            LexerError::CantTranslateToToken(_) => common,
            LexerError::UnexpectedOwnershipLiteralMatch(_, _) => common,
            LexerError::CantParseU64(_, _,_) => common,
        }
    }
    fn get_long_description(&self, _target: &ReportTarget) -> Option<Document> {
        Some(external_text(match self {
            LexerError::UnexpectedCharacter(_) => "While reading the code, we were unable to understand this particular character.",
            LexerError::UnexpectedPunctuationMatch(_, _) => "The internal way to find punctuation marks and operators failed, it recognized a character that we didn't support!",
            LexerError::UnexpectedCommentMatch(_, _) => "The intnernal way to find a comment failed after succeeding!",
            LexerError::NonFinishedLineComment(_, _) => "We find the start of a comment but not the end for some reason (not unbalanced brackets)",
            LexerError::NonContentInLineComment(_, _) => "We find a comment but we were unable to retrieve the content",
            LexerError::CantCreateCommentLine(_, _) => "We got the content of a comment but the internalizer disagree with us that this comment has the right format!",
            LexerError::CouldntMatchBlockComment(_,_, _) => "We were looking for a matching end for the comment.\nEither we didn't find it, and we consumed all the code looking for it.\nOr something else got wrong in the search (improbable)",
            LexerError::Notu64NamedHole(_, _) => "Internally the named holes are stored as u64 integers.\nThe provided value for the hole is out of the bound for this range.\nPlease modify the hole value to something between 0 and 2^64 -1",
            LexerError::CantCreateIdentifier(_, _) => "Internally we expected something to follow the same rules as an identifier, but it didn't follow those rules",
            LexerError::CantTranslateToToken(_) => "The internal translation between simple Tokens and the CST::Tokens failed!",
            LexerError::UnexpectedOwnershipLiteralMatch(_, _) => "We find what seems to look like an ownership literal, but something unexpected passed while working with it!",
            LexerError::CantParseU64(_, _,_) => "We find what seems to look like an u64 literal, but something unexpected passed while working with it!",
        }))
    }
    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            LexerError::CouldntMatchBlockComment(_, kind, _) => {
                let hyphens = "-".repeat(kind.len() - 1);
                Some(vec![hyphens + "}"])
            }
            _ => None,
        }
    }
    fn get_location(&self) -> Option<Location> {
        Some(match self {
            LexerError::UnexpectedCharacter(p) => Location::Position(p.clone()),
            LexerError::UnexpectedPunctuationMatch(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::UnexpectedCommentMatch(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::NonFinishedLineComment(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::NonContentInLineComment(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::CantCreateCommentLine(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::CouldntMatchBlockComment(_, _, span) => {
                Location::Span(span.clone())
            }
            LexerError::Notu64NamedHole(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::CantCreateIdentifier(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::CantTranslateToToken(token) => {
                Location::Span(<&Token as Into<&TokenInfo>>::into(token).span)
            }
            LexerError::UnexpectedOwnershipLiteralMatch(_, span) => {
                Location::Span(span.clone())
            }
            LexerError::CantParseU64(_, _, span) => {
                Location::Span(span.clone())
            }
        })
    }
}

impl ReportFormat for ParseError<Position, Token, LexerError> {
    fn get_report_name(&self) -> NonLineBreakStr {
        NonLineBreakStr::new(match self {
            ParseError::InvalidToken { .. } => "Internal:InvalidToken",
            ParseError::UnrecognizedEof { .. } => "UnrecognizedEof",
            ParseError::UnrecognizedToken { .. } => "UnrecognizedToken",
            ParseError::ExtraToken { .. } => "ExtraToken",
            ParseError::User { error } => error.get_report_name().as_str(),
        })
    }
    fn get_short_description(&self) -> NonLineBreakStr {
        match self {
            ParseError::InvalidToken { .. } => NonLineBreakStr::new(
                "This shouldn't appear, if you see this, please, report it!",
            ),
            ParseError::UnrecognizedEof { .. } => {
                NonLineBreakStr::new("We expected even more code to process!")
            }
            ParseError::UnrecognizedToken { .. } => NonLineBreakStr::new(
                "There's a syntax error in the provided code.",
            ),
            ParseError::ExtraToken { .. } => NonLineBreakStr::new(
                "We expected to finish reading code, but you provide more!",
            ),
            ParseError::User { error } => error.get_short_description(),
        }
    }
    fn get_long_description(&self, target: &ReportTarget) -> Option<Document> {
        match self {
            ParseError::InvalidToken { .. } => Some(external_text("The internal library used to parse the code has this disabled by octizys.\nIf you see this, a bug in the parser generator may happen!")),
            ParseError::UnrecognizedEof { .. } =>Some(external_text("You may need to provide more code.\nWe read all what you provided, but we still couldn't understand it!")),
            ParseError::UnrecognizedToken { .. } => Some(external_text("Something is wrong with the code structure.\nThere's a chance that the error happened before this point.\nBut in such case we were able to understand (wrong) the code until we reached this place.")),
            ParseError::ExtraToken { .. } => Some(external_text("We believe that we finished reading and understanding your code before we really read everything.\nThis means that you may have other errors in the middle or that you may want to delete the excess of code.")),
            ParseError::User { error } => error.get_long_description(target),
        }
    }
    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            ParseError::InvalidToken { .. } => None,
            ParseError::UnrecognizedEof { expected, .. } => {
                Some(expected.to_owned())
            }
            ParseError::UnrecognizedToken { expected, .. } => {
                Some(expected.to_owned())
            }
            ParseError::ExtraToken { .. } => None,
            ParseError::User { error } => error.get_expected(),
        }
    }
    fn get_location(&self) -> Option<Location> {
        match self {
            ParseError::InvalidToken { location } => {
                Some(Location::Position(location.to_owned()))
            }
            ParseError::UnrecognizedEof { location, .. } => {
                Some(Location::Position(location.to_owned()))
            }
            ParseError::UnrecognizedToken {
                token: (_, token, _),
                ..
            } => Some(Location::Span(<&TokenInfo>::from(token).span)),
            ParseError::ExtraToken {
                token: (_, token, _),
            } => Some(Location::Span(<&TokenInfo>::from(token).span)),
            ParseError::User { error } => error.get_location(),
        }
    }
}
