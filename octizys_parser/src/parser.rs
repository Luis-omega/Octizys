use std::path::PathBuf;

#[derive(Debug)]
pub enum ParsedFile {}

#[derive(Debug)]
pub enum ParserError {}

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
