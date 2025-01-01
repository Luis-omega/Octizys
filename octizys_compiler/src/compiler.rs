//! The flux of compilation is like follows:
//! At the beginning we have at least one file to be compiled.
//! We add them to the [to parse header] structure.
//! We load and parse the files one by one and put the result in the
//! [to be parsed] structure.
//! The parser used for this only look for the [Imports] and
//! forgets anything about the file except for the [Imports]
//! Additionally for every parsed file we look at the [imported] files
//! by it and add the [Logical path] of the imports to the
//! [to resolve path] structure.
//! Why do this instead of solving the structure in place to
//! maintain the previous operation pure (not input/output effects).
//!
//! We solve all the [to resolve path], register the result in
//! a [map logical paths to real paths],  find the unparsed files
//! among them and add them to the [to parse header].
//! (TODO: implement caching in the resolve path so we can avoid recompiling unneeded)
//!
//! We run this cycle until we got nothing in the [to parse header]
//! structure.
//!
//! At this point we build a [dependency graph].
//! We check the lack of cycles in this structure (mutual recursive imports
//! are forbidden!). The reported cycles should be
//!
//! Then we parse, typecheck, transform to core, optimize and build the
//! file for the different targets. We do this in a depth-first post-order traversal
//! of the dependency graph.
//!

use crate::compiler::configuration::CompilerConfiguration;
use crate::error::Error;
use crate::parser::parser::{ParsedFile, ParserError};
//use itertools::{Either, Itertools};
//

pub enum CompilerError {}

impl Into<Error> for CompilerError {
    fn into(self) -> Error {
        todo!()
    }
}

impl From<ParserError> for CompilerError {
    fn from(_: ParserError) -> CompilerError {
        todo!()
    }
}

//pub fn load_and_parse_file(path: PathBuf) -> Result<ParsedFile> {
//    "Uninplemented".into()
//}
//
//pub fn split_from_errors<T>(
//    results: Vec<Result<T>>,
//) -> (Vec<T>, Vec<anyhow::Error>) {
//    results.into_iter().partition_map(|x| match x {
//        Err(x) => Either::Left(x),
//        Ok(x) => Either::Right(x),
//    })
//}
//
pub fn compile(
    config: CompilerConfiguration,
) -> Result<ParsedFile, ParserError> {
    //config.paths_to_compile.into_iter().map(load_and_parse_file);
    dbg!(config);
    todo!();
}
