use octizys_common::span::{Position, Span};
use octizys_parser::lexer::{LexerError, Token};
use octizys_pretty::{
    combinators::{
        concat, emphasis, empty, external_text, foreground, hard_break,
        intersperse, nest, soft_break,
    },
    document::Document,
    highlight::{
        base_colors::{CYAN, MAGENTA, MODERATE_GREEN, RED},
        Emphasis,
    },
};
use octizys_text_store::store::approximate_string_width;

use octizys_cst::base::TokenInfo;

use lalrpop_util::ParseError;

#[derive(Debug)]
pub struct ParserErrorContext<'source> {
    pub src: &'source str,
    pub src_name: &'source str,
    pub max_line_width: u16,
}

impl<'a> Default for ParserErrorContext<'a> {
    fn default() -> Self {
        ParserErrorContext {
            src: &"",
            src_name: &"octizys_repl",
            max_line_width: 80,
        }
    }
}

pub enum ErrorLocation {
    Span(Span),
    Position(Position),
}

fn position_to_document(p: &Position) -> Document {
    concat(vec![
        external_text("Line{"),
        external_text(p.line.to_string().as_str()),
        external_text("}"),
        external_text("::Column{"),
        external_text(p.column.to_string().as_str()),
        external_text("}"),
    ])
}

impl ErrorLocation {
    fn to_document(&self) -> Document {
        match self {
            ErrorLocation::Span(s) => {
                let start = position_to_document(&s.start);
                let end = position_to_document(&s.end);
                concat(vec![
                    external_text("::From::"),
                    start,
                    external_text("::To::"),
                    end,
                ])
            }
            ErrorLocation::Position(p) => {
                let pos = position_to_document(p);
                concat(vec![external_text("::At::"), pos])
            }
        }
    }
}

pub trait ParserErrorReport {
    fn get_error_name(&self) -> &str;
    // TODO: change the str for document or String?
    fn get_short_description(&self) -> &str;
    // TODO: change the str for document or String?
    // We want more dinamyc errors!
    fn get_long_description(&self) -> Option<&str>;
    /// A vector of strings describing what is expected.
    fn get_expected(&self) -> Option<Vec<String>>;
    fn get_location(&self) -> ErrorLocation;
}

/// Generate the beginning of the error report (the part before we
/// show the source code signalling the error).
pub fn make_error_info_start<E: ParserErrorReport>(
    error: &E,
    context: &ParserErrorContext,
) -> Document {
    let location = error.get_location();
    let error_name = error.get_error_name();
    let short_description = error.get_short_description();
    let location_doc = concat(vec![
        foreground(CYAN, external_text("-->")),
        foreground(MODERATE_GREEN, external_text(context.src_name)),
        location.to_document(),
    ]);
    concat(vec![
        foreground(
            RED,
            concat(vec![
                external_text("Error!["),
                external_text(error_name),
                external_text("]: "),
            ]),
        ),
        external_text(short_description),
        nest(1, hard_break() + location_doc),
    ])
}

fn expected_to_document(expected: Option<Vec<String>>) -> Document {
    match expected {
        None => empty(),
        Some(v) => {
            let previous_text: &str;
            if v.len() == 0 {
                return empty();
            } else if v.len() == 1 {
                previous_text = "Expected: ";
            } else {
                previous_text = "Expected one of: ";
            }
            external_text(previous_text)
                + intersperse(
                    v.into_iter().map(|x| {
                        emphasis(
                            Emphasis::Bold,
                            foreground(CYAN, external_text(&x)),
                        )
                    }),
                    external_text(",") + soft_break(),
                )
        }
    }
}

fn make_source_error<E: ParserErrorReport>(
    e: &E,
    context: &ParserErrorContext,
) -> Document {
    let location = e.get_location();
    match location {
        ErrorLocation::Span(span) => {
            //We need to get this first
            let (_, content, _) = span.get_text_at(context.src, None);
            if span.start.line == span.end.line {
                let remain_width =
                    match u16::try_from(approximate_string_width(content)) {
                        Ok(width) => {
                            context.max_line_width.saturating_sub(width)
                        }
                        Err(_) => 0,
                    };
                let (before, _, after) = span
                    .get_text_at(context.src, Some(usize::from(remain_width)));
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
                                expected_to_document(e.get_expected()),
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
                    foreground(RED, expected_to_document(e.get_expected())),
                ])
            }
        }
        ErrorLocation::Position(position) => {
            let (before, after) = position.get_text_at(context.src, Some(80));
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
                foreground(RED, expected_to_document(e.get_expected())),
            ])
        }
    }
}

pub fn create_error_report<E: ParserErrorReport>(
    error: &E,
    context: &ParserErrorContext,
) -> Document {
    let header = make_error_info_start(error, context);
    let source = make_source_error(error, context);
    let long_description = match error.get_long_description() {
        Some(d) => external_text(d),
        None => empty(),
    };
    concat(vec![
        header,
        nest(4, hard_break() + source),
        nest(2, hard_break() + long_description),
    ])
}

impl ParserErrorReport for LexerError {
    fn get_error_name(&self) -> &str {
        match self {
            LexerError::UnexpectedCharacter(_) => "UnexpectedCharacter",
            LexerError::UnexpectedPunctuationMatch(_, _) => {
                "Internal:UnexpectedPunctuationMatch"
            }
            LexerError::UnexpectedCommentMatch(_, _) => {
                "Internal:UnexpectedCommentMatch"
            }
            LexerError::NonFinishedLineComment(_, _) => {
                "Internal:NonFinishedLineComment"
            }
            LexerError::NonContentInLineComment(_, _) => {
                "Internal:NonContentInLineComment"
            }
            LexerError::CantCreateCommentLine(_, _) => {
                "Internal:CantCreateCommentLine"
            }
            LexerError::CouldntMatchBlockComment(_, _, _) => {
                "CouldntMatchBlockComment"
            }
            LexerError::Notu64NamedHole(_, _) => "Notu64NamedHole",
            LexerError::CantCreateIdentifier(_, _) => {
                "Internal:CantCreateIdentifier"
            }
            LexerError::CantTranslateToToken(_) => {
                "Internal:CantTranslateToToken"
            }
            LexerError::UnexpectedOwnershipLiteralMatch(_, _) => {
                "Internal:UnexpectedOwnershipLiteralMatch"
            }
            LexerError::CantParseU64(_, _, _) => "Internal:CantParseU64",
        }
    }
    fn get_short_description(&self) -> &str {
        let common = "This is a bug in octizys, please report it!";
        match self {
            LexerError::UnexpectedCharacter(_) => {
                "The provided character doesn't correspond to a valid program."
            }
            LexerError::UnexpectedPunctuationMatch(_, _) => common,
            LexerError::UnexpectedCommentMatch(_, _) => common,
            LexerError::NonFinishedLineComment(_, _) => common,
            LexerError::NonContentInLineComment(_, _) => common,
            LexerError::CantCreateCommentLine(_, _) => common,
            LexerError::CouldntMatchBlockComment(_,_, _) => "We found the beginning of a block comment but couldn't finished it!",
            LexerError::Notu64NamedHole(_, _) => {
                "Named holes are limited to u64 integers."
            }
            LexerError::CantCreateIdentifier(_, _) => common,
            LexerError::CantTranslateToToken(_) => common,
            LexerError::UnexpectedOwnershipLiteralMatch(_, _) => common,
            LexerError::CantParseU64(_, _,_) => common,
        }
    }
    fn get_long_description(&self) -> Option<&str> {
        Some(match self {
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
        })
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
    fn get_location(&self) -> ErrorLocation {
        match self {
            LexerError::UnexpectedCharacter(p) => {
                ErrorLocation::Position(p.clone())
            }
            LexerError::UnexpectedPunctuationMatch(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::UnexpectedCommentMatch(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::NonFinishedLineComment(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::NonContentInLineComment(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::CantCreateCommentLine(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::CouldntMatchBlockComment(_, _, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::Notu64NamedHole(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::CantCreateIdentifier(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::CantTranslateToToken(token) => ErrorLocation::Span(
                <&Token as Into<&TokenInfo>>::into(token).span,
            ),
            LexerError::UnexpectedOwnershipLiteralMatch(_, span) => {
                ErrorLocation::Span(span.clone())
            }
            LexerError::CantParseU64(_, _, span) => {
                ErrorLocation::Span(span.clone())
            }
        }
    }
}

impl ParserErrorReport for ParseError<Position, Token, LexerError> {
    fn get_error_name(&self) -> &str {
        match self {
            ParseError::InvalidToken { .. } => "Internal:InvalidToken",
            ParseError::UnrecognizedEof { .. } => "UnrecognizedEof",
            ParseError::UnrecognizedToken { .. } => "UnrecognizedToken",
            ParseError::ExtraToken { .. } => "ExtraToken",
            ParseError::User { error } => error.get_error_name(),
        }
    }
    fn get_short_description(&self) -> &str {
        match self {
            ParseError::InvalidToken { .. } => {
                "This shouldn't appear, if you see this, please, report it!"
            }
            ParseError::UnrecognizedEof { .. } => {
                "We expected even more code to process!"
            }
            ParseError::UnrecognizedToken { .. } => {
                "There's a syntax error in the provided code."
            }
            ParseError::ExtraToken { .. } => {
                "We expected to finish reading code, but you provide more!"
            }
            ParseError::User { error } => error.get_short_description(),
        }
    }
    fn get_long_description(&self) -> Option<&str> {
        match self {
            ParseError::InvalidToken { .. } => Some("The internal library used to parse the code has this disabled by octizys.\nIf you see this, a bug in the parser generator may happen!"),
            ParseError::UnrecognizedEof { .. } =>Some("You may need to provide more code.\nWe read all what you provided, but we still couldn't understand it!"),
            ParseError::UnrecognizedToken { .. } => Some("Something is wrong with the code structure.\nThere's a chance that the error happened before this point.\nBut in such case we were able to understand (wrong) the code until we reached this place."),
            ParseError::ExtraToken { .. } => Some("We believe that we finished reading and understanding your code before we really read everything.\nThis means that you may have other errors in the middle or that you may want to delete the excess of code."),
            ParseError::User { error } => error.get_long_description(),
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
    fn get_location(&self) -> ErrorLocation {
        match self {
            ParseError::InvalidToken { location } => {
                ErrorLocation::Position(location.to_owned())
            }
            ParseError::UnrecognizedEof { location, .. } => {
                ErrorLocation::Position(location.to_owned())
            }
            ParseError::UnrecognizedToken {
                token: (_, token, _),
                ..
            } => ErrorLocation::Span(<&TokenInfo>::from(token).span),
            ParseError::ExtraToken {
                token: (_, token, _),
            } => ErrorLocation::Span(<&TokenInfo>::from(token).span),
            ParseError::User { error } => error.get_location(),
        }
    }
}
