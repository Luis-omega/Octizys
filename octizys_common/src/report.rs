use std::path::PathBuf;

#[cfg(feature = "lalrpop")]
use crate::span::{HasLocation, Position};
#[cfg(feature = "lalrpop")]
use lalrpop_util::ParseError;

use crate::span::Location;
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
    static_text,
    store::NonLineBreakStr,
};
use octizys_text_store::store::approximate_string_width;

#[derive(Debug)]
pub struct ReportSourceContext<'source> {
    pub src: &'source str,
    pub src_name: String,
    pub max_line_width: u16,
}

impl<'a> Default for ReportSourceContext<'a> {
    fn default() -> Self {
        ReportSourceContext {
            src: &"",
            src_name: String::from("octizys_repl"),
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

pub trait ReportFormat {
    fn get_report_name(&self) -> NonLineBreakStr;
    fn get_short_description(&self) -> NonLineBreakStr;
    fn get_long_description(&self, target: &ReportTarget) -> Option<Document>;
    fn get_expected(&self) -> Option<Vec<String>>;
    fn get_location_maybe(&self) -> Option<Location>;
}

pub struct ReportRequest<'source, T>
where
    T: ReportFormat,
{
    pub report: &'source T,
    // If you want to pass them empty, the ReportFormat
    // trait must return None on error location.
    pub source_context: ReportSourceContext<'source>,
    pub target: ReportTarget,
    pub kind: ReportKind,
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
    let location = request.report.get_location_maybe();
    let kind = make_report_kind(request.report, &request.kind, &request.target);
    let short_description = request.report.get_short_description();
    let location_doc = concat(vec![
        foreground(CYAN, external_text("-->")),
        foreground(
            MODERATE_GREEN,
            external_text(&request.source_context.src_name),
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
    let location = request.report.get_location_maybe();
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

#[cfg(feature = "lalrpop")]
impl<T, E> ReportFormat for ParseError<Position, T, E>
where
    E: ReportFormat,
    T: HasLocation,
{
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
    fn get_location_maybe(&self) -> Option<Location> {
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
            } => Some(token.get_location()),
            ParseError::ExtraToken {
                token: (_, token, _),
            } => Some(token.get_location()),
            ParseError::User { error } => error.get_location_maybe(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IOError {
    FileLoadError { path: PathBuf },
    REPlCantReadLine { error: std::io::ErrorKind },
}

impl IOError {
    pub fn build_report_request(
        &self,
        target: ReportTarget,
        alternative_name: String,
        line_width: u16,
    ) -> ReportRequest<IOError> {
        let mut source_context: ReportSourceContext = Default::default();
        source_context.max_line_width = line_width;
        let kind = ReportKind::Error;
        match self {
            IOError::FileLoadError { path } => {
                let name = path
                    .to_str()
                    .map(String::from)
                    .unwrap_or_else(|| alternative_name);
                source_context.src_name = name;
                ReportRequest {
                    report: self,
                    source_context,
                    target,
                    kind,
                }
            }
            IOError::REPlCantReadLine { .. } => {
                source_context.src_name = alternative_name;
                ReportRequest {
                    report: self,
                    source_context,
                    target,
                    kind,
                }
            }
        }
    }
}

impl ReportFormat for IOError {
    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            IOError::FileLoadError { .. } => None,
            IOError::REPlCantReadLine { .. } => None,
        }
    }
    fn get_report_name(&self) -> NonLineBreakStr {
        match self {
            IOError::FileLoadError { .. } => {
                NonLineBreakStr::new("OctizysCommandLineArgument")
            }
            IOError::REPlCantReadLine { .. } => {
                NonLineBreakStr::new("OctizysREPL")
            }
        }
    }
    fn get_location_maybe(&self) -> Option<Location> {
        match self {
            IOError::FileLoadError { .. } => None,
            IOError::REPlCantReadLine { .. } => None,
        }
    }
    fn get_long_description(&self, _target: &ReportTarget) -> Option<Document> {
        match self {
            IOError::FileLoadError { path } => Some(external_text(
                //TODO: make it more fancy
                &format!("Couldn't open the file:{:#?}", path),
            )),
            IOError::REPlCantReadLine { error } => Some(external_text(
                &format!("While trying to read a line:{:}", error),
            )),
        }
    }
    fn get_short_description(&self) -> NonLineBreakStr {
        match self {
            IOError::FileLoadError { .. } => {
                NonLineBreakStr::new("Can't open a file!")
            }
            IOError::REPlCantReadLine { .. } => {
                NonLineBreakStr::new("Can't read line!")
            }
        }
    }
}
