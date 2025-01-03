use std::{cell::RefCell, path::PathBuf, rc::Rc};

use lalrpop_util::ParseError;
use log::{debug, trace};
use octizys_common::{
    report::{
        IOError, ReportFormat, ReportKind, ReportRequest, ReportSourceContext,
        ReportTarget,
    },
    span::Position,
};
use octizys_cst::top::Top;
use octizys_pretty::store::Store;

use crate::{
    grammar::topParser,
    lexer::{BaseLexerContext, LexerContext, Token},
    report::OctizysParserReport,
};

#[derive(Debug, Clone)]
pub enum OctizysParserError {
    AtParsing {
        source: String,
        source_name: Option<PathBuf>,
        error: ParseError<Position, Token, OctizysParserReport>,
    },
    IO(IOError),
}

impl OctizysParserError {
    pub fn build_report_request(
        &self,
        target: ReportTarget,
        alternative_name: String,
        line_width: usize,
    ) -> ReportRequest<OctizysParserError> {
        match self {
            OctizysParserError::AtParsing {
                source,
                source_name,
                error,
            } => {
                let name = match source_name {
                    Some(p) => p
                        .to_str()
                        .map(String::from)
                        .unwrap_or_else(|| alternative_name),
                    None => alternative_name,
                };
                let source_context = ReportSourceContext {
                    src: &source,
                    src_name: name,
                    max_line_width: line_width,
                };
                let kind = match error {
                    ParseError::User { error } => error.kind,
                    _ => ReportKind::Error,
                };
                ReportRequest {
                    report: self,
                    source_context,
                    target,
                    kind,
                }
            }
            OctizysParserError::IO(error) => {
                let inner = error.build_report_request(
                    target,
                    alternative_name,
                    line_width,
                );
                ReportRequest {
                    report: self,
                    source_context: inner.source_context,
                    target: inner.target,
                    kind: inner.kind,
                }
            }
        }
    }
}

impl ReportFormat for OctizysParserError {
    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            OctizysParserError::AtParsing { error, .. } => error.get_expected(),
            OctizysParserError::IO(e) => e.get_expected(),
        }
    }

    fn get_report_name(&self) -> octizys_pretty::store::NonLineBreakStr {
        match self {
            OctizysParserError::AtParsing { error, .. } => {
                error.get_report_name()
            }
            OctizysParserError::IO(e) => e.get_report_name(),
        }
    }

    fn get_location_maybe(&self) -> Option<octizys_common::span::Location> {
        match self {
            OctizysParserError::AtParsing { error, .. } => {
                error.get_location_maybe()
            }
            OctizysParserError::IO(e) => e.get_location_maybe(),
        }
    }

    fn get_long_description(
        &self,
        target: &octizys_common::report::ReportTarget,
    ) -> Option<octizys_pretty::document::Document> {
        match self {
            OctizysParserError::AtParsing { error, .. } => {
                error.get_long_description(target)
            }
            OctizysParserError::IO(e) => e.get_long_description(target),
        }
    }

    fn get_short_description(&self) -> octizys_pretty::store::NonLineBreakStr {
        match self {
            OctizysParserError::AtParsing { error, .. } => {
                error.get_short_description()
            }
            OctizysParserError::IO(e) => e.get_short_description(),
        }
    }
}

pub fn parse_string(
    source: &str,
    source_name: Option<PathBuf>,
    store: Rc<RefCell<Store>>,
) -> Result<Top, OctizysParserError> {
    let mut base_context = BaseLexerContext::new(&source, store);
    let iterator = LexerContext::new(None, &mut base_context)
        .inspect(|result| trace!("{:?}", result));
    topParser::new().parse(iterator).map_err(|error| {
        OctizysParserError::AtParsing {
            source: String::from(source),
            source_name,
            error,
        }
    })
}

pub fn parse_file(
    path_name: PathBuf,
    store: Rc<RefCell<Store>>,
) -> Result<Top, OctizysParserError> {
    match ::std::fs::read_to_string(path_name.clone()) {
        Ok(content) => {
            let path = match std::path::absolute(path_name.clone().as_path()) {
                Ok(p) => p.to_str().map(String::from),
                Err(_) => path_name.to_str().map(String::from),
            };
            parse_string(&content, Some(path_name), store)
        }
        Err(_) => Err(OctizysParserError::IO(IOError::FileLoadError {
            path: path_name,
        })),
    }
}
