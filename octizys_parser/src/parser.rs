use octizys_core::error::Error;
use octizys_sast::top::Sast;
use std::path::PathBuf;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("lexer.l");
lrpar_mod!("grammar.y");

#[derive(Debug)]
pub enum ParsedFile {}

#[derive(Debug)]
pub enum ParserError {}

impl Into<Error> for ParserError {
    fn into(self) -> Error {
        todo!()
    }
}

fn parse(str: String) -> Result<Sast, ParserError> {
    let def = calc_l::lexerdef();
    todo!()
}

// TODO(optimization):
// use a Stream instead of a String, that would defeat
// the point of it being pure.
/// This don't load the file, it expect the full file in [content]
/// the [path_name] is required only for error report
pub fn parse_file(
    path_name: PathBuf,
    content: String,
) -> Result<Sast, ParserError> {
    todo!("parsing file")
}

pub fn parse_file_imports(
    path_name: PathBuf,
    content: String,
) -> Result<Sast, ParserError> {
    todo!("parsing file imports")
}
