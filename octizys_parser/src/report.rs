use std::num::ParseIntError;

use lalrpop_util::ParseError;
use octizys_common::{
    report::{ReportFormat, ReportKind, ReportTarget},
    span::{Location, Position},
};
use octizys_cst::comments::CommentBraceKind;
use octizys_macros::Equivalence;
use octizys_pretty::{
    combinators::external_text, document::Document, store::NonLineBreakStr,
};

use crate::tokens::Token;

#[derive(Debug, Clone, Equivalence)]
pub enum LexerReportKind {
    UnexpectedCharacter,
    /// Punctuation match matched a non know character
    /// This is a bug.
    UnexpectedPunctuationMatch,
    /// Comment match matched a non know character
    /// This is a bug.
    UnexpectedCommentMatch,
    /// A line comment pattern match failed.
    /// This is a bug.
    NonFinishedLineComment,
    /// A line comment with empty body is fine but
    /// the regex should still return a empty match.
    /// This is a bug.
    NonContentInLineComment,
    /// The internalization of comments failed!
    /// this is a bug.
    CantCreateCommentLine,
    /// We found the beginning of a block comment
    /// but the input didn't match the regex,
    /// in the user side a unbalanced bracket is what
    /// is expected.
    CouldntMatchBlockComment(CommentBraceKind),
    Notu64NamedHole,
    /// We expected a identifier and the regex should guarantee it!
    /// but if not...
    /// this is a bug.
    CantCreateIdentifier,
    /// We have some unsafe functions that translate a
    /// [`Token`] in this file to a [`octizys_cst::base::Token`],
    /// they are mainly used in the parser and shouldn't fail!
    /// this is a bug.
    CantTranslateToToken(Token),
    /// We found a match for a ownership_literal but
    /// then it wasn't a match! most probably the
    /// regex was update without updating the handler!
    /// this is a bug.
    UnexpectedOwnershipLiteralMatch,
    /// Can't parse a u64 but we already know that the string
    /// is a valid rust u64, this signals a bug!
    CantParseU64(
        //TODO: Implement Equivalence for ParseIntError
        #[equivalence(ignore)] ParseIntError,
    ),
}

impl ReportFormat for LexerReportKind {
    fn get_report_name(&self) -> NonLineBreakStr {
        // DO NOT REMOVE THE NonLineBreakStr out of the match!
        // It performs a compile time check on the passed string,
        // it would panic at run time if moved to the top match.
        match self {
            LexerReportKind::UnexpectedCharacter => {
                NonLineBreakStr::new("UnexpectedCharacter")
            }
            LexerReportKind::UnexpectedPunctuationMatch => {
                NonLineBreakStr::new("Internal:UnexpectedPunctuationMatch")
            }
            LexerReportKind::UnexpectedCommentMatch => {
                NonLineBreakStr::new("Internal:UnexpectedCommentMatch")
            }
            LexerReportKind::NonFinishedLineComment => {
                NonLineBreakStr::new("Internal:NonFinishedLineComment")
            }
            LexerReportKind::NonContentInLineComment => {
                NonLineBreakStr::new("Internal:NonContentInLineComment")
            }
            LexerReportKind::CantCreateCommentLine => {
                NonLineBreakStr::new("Internal:CantCreateCommentLine")
            }
            LexerReportKind::CouldntMatchBlockComment(_) => {
                NonLineBreakStr::new("CouldntMatchBlockComment")
            }
            LexerReportKind::Notu64NamedHole => {
                NonLineBreakStr::new("Notu64NamedHole")
            }
            LexerReportKind::CantCreateIdentifier => {
                NonLineBreakStr::new("Internal:CantCreateIdentifier")
            }
            LexerReportKind::CantTranslateToToken(_) => {
                NonLineBreakStr::new("Internal:CantTranslateToToken")
            }
            LexerReportKind::UnexpectedOwnershipLiteralMatch => {
                NonLineBreakStr::new("Internal:UnexpectedOwnershipLiteralMatch")
            }
            LexerReportKind::CantParseU64(_) => {
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
            LexerReportKind::UnexpectedCharacter => {
                NonLineBreakStr::new("The provided character doesn't correspond to a valid program.")
            }
            LexerReportKind::UnexpectedPunctuationMatch => common,
            LexerReportKind::UnexpectedCommentMatch => common,
            LexerReportKind::NonFinishedLineComment => common,
            LexerReportKind::NonContentInLineComment => common,
            LexerReportKind::CantCreateCommentLine => common,
            LexerReportKind::CouldntMatchBlockComment(_)=> NonLineBreakStr::new("We found the beginning of a block comment but couldn't finished it!"),
            LexerReportKind::Notu64NamedHole => {
                NonLineBreakStr::new("Named holes are limited to u64 integers.")
            }
            LexerReportKind::CantCreateIdentifier => common,
            LexerReportKind::CantTranslateToToken(_) => common,
            LexerReportKind::UnexpectedOwnershipLiteralMatch => common,
            LexerReportKind::CantParseU64(_)=> common,
        }
    }
    fn get_long_description(&self, _target: &ReportTarget) -> Option<Document> {
        Some(external_text(match self {
            LexerReportKind::UnexpectedCharacter => "While reading the code, we were unable to understand this particular character.",
            LexerReportKind::UnexpectedPunctuationMatch => "The internal way to find punctuation marks and operators failed, it recognized a character that we didn't support!",
            LexerReportKind::UnexpectedCommentMatch => "The intnernal way to find a comment failed after succeeding!",
            LexerReportKind::NonFinishedLineComment => "We find the start of a comment but not the end for some reason (not unbalanced brackets)",
            LexerReportKind::NonContentInLineComment => "We find a comment but we were unable to retrieve the content",
            LexerReportKind::CantCreateCommentLine => "We got the content of a comment but the internalizer disagree with us that this comment has the right format!",
            LexerReportKind::CouldntMatchBlockComment(_)=> "We were looking for a matching end for the comment.\nEither we didn't find it, and we consumed all the code looking for it.\nOr something else got wrong in the search (improbable)",
            LexerReportKind::Notu64NamedHole => "Internally the named holes are stored as u64 integers.\nThe provided value for the hole is out of the bound for this range.\nPlease modify the hole value to something between 0 and 2^64 -1",
            LexerReportKind::CantCreateIdentifier => "Internally we expected something to follow the same rules as an identifier, but it didn't follow those rules",
            LexerReportKind::CantTranslateToToken(_) => "The internal translation between simple Tokens and the CST::Tokens failed!",
            LexerReportKind::UnexpectedOwnershipLiteralMatch => "We find what seems to look like an ownership literal, but something unexpected passed while working with it!",
            LexerReportKind::CantParseU64(_)=> "We find what seems to look like an u64 literal, but something unexpected passed while working with it!",
        }))
    }
    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            LexerReportKind::CouldntMatchBlockComment(kind) => {
                let hyphens = "-".repeat(kind.len() - 1);
                Some(vec![hyphens + "}"])
            }
            _ => None,
        }
    }
    fn get_location_maybe(&self) -> Option<Location> {
        None
    }
}

#[derive(Debug, Clone, Equivalence)]
pub enum ParserReport {
    Lexer(LexerReportKind),
}

#[derive(Debug, Clone, Equivalence)]
pub struct OctizysParserReport {
    pub kind: ReportKind,
    pub report: ParserReport,
    #[equivalence(ignore)]
    pub location: Location,
}

impl ReportFormat for ParserReport {
    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            ParserReport::Lexer(e) => e.get_expected(),
        }
    }
    fn get_report_name(&self) -> NonLineBreakStr {
        match self {
            ParserReport::Lexer(e) => e.get_report_name(),
        }
    }
    fn get_location_maybe(&self) -> Option<Location> {
        match self {
            ParserReport::Lexer(e) => e.get_location_maybe(),
        }
    }
    fn get_long_description(&self, target: &ReportTarget) -> Option<Document> {
        match self {
            ParserReport::Lexer(e) => e.get_long_description(target),
        }
    }
    fn get_short_description(&self) -> NonLineBreakStr {
        match self {
            ParserReport::Lexer(e) => e.get_short_description(),
        }
    }
}

impl ReportFormat for OctizysParserReport {
    fn get_expected(&self) -> Option<Vec<String>> {
        self.report.get_expected()
    }
    fn get_report_name(&self) -> NonLineBreakStr {
        self.report.get_report_name()
    }
    fn get_location_maybe(&self) -> Option<Location> {
        Some(self.location)
    }
    fn get_long_description(&self, target: &ReportTarget) -> Option<Document> {
        self.report.get_long_description(target)
    }
    fn get_short_description(&self) -> NonLineBreakStr {
        self.report.get_short_description()
    }
}
