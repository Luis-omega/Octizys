use std::{cell::RefCell, path::PathBuf, rc::Rc};

use lalrpop_util::ParseError;
use octizys_common::span::Position;
use octizys_cst::top::Top;
use octizys_pretty::store::Store;

use crate::{
    grammar::topParser,
    lexer::{BaseLexerContext, LexerContext, Token},
    report::OctizysParserReport,
};

#[derive(Debug)]
pub enum ParsedFile {}

fn parse_string(
    str: &str,
    store: Rc<RefCell<Store>>,
) -> Result<Top, ParseError<Position, Token, OctizysParserReport>> {
    let mut base_context = BaseLexerContext::new(&str, store);
    let iterator = LexerContext::new(None, &mut base_context);
    topParser::new().parse(iterator)
}

pub fn parse_file(
    path_name: PathBuf,
    store: Rc<RefCell<Store>>,
) -> Result<(), OctizysParserReport> {
    todo!("We need to think in a way to merge extraneous errors here")
    //match ::std::fs::read_to_string(path_name.clone()) {
    //    Ok(content) => {
    //        let path = match std::path::absolute(path_name.clone().as_path()) {
    //            Ok(p) => p.to_str().map(String::from),
    //            Err(_) => path_name.to_str().map(String::from),
    //        };
    //        parse_string(content, path.as_deref(), store)
    //    }
    //    Err(_) => Err(OctizysError::FileLoadError { path: file_path }),
    //}
}
