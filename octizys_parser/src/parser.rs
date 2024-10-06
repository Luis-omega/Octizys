use lalrpop_util::lalrpop_mod;
use octizys_common::error::Error;
use octizys_cst::cst;
use std::path::PathBuf;

lalrpop_mod!(grammar);

#[derive(Debug)]
pub enum ParsedFile {}

#[derive(Debug)]
pub enum ParserError {}

impl Into<Error> for ParserError {
    fn into(self) -> Error {
        todo!()
    }
}

fn parse(str: String) -> Result<(), ParserError> {
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
) -> Result<(), ParserError> {
    todo!("parsing file")
}

pub fn parse_file_imports(
    path_name: PathBuf,
    content: String,
) -> Result<(), ParserError> {
    todo!("parsing file imports")
}
