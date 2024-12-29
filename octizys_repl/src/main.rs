mod error_report;
use crate::error_report::{
    create_error_report, ParserErrorContext, ParserErrorReport,
};
use octizys_common::span::{Position, Span};
use octizys_cst::base::TokenInfo;
use octizys_formatter::cst::PrettyCSTConfiguration;
use octizys_formatter::to_document::ToDocument;
use octizys_parser::grammar::{
    import_declarationParser, type_expressionParser,
};
use octizys_parser::lexer;
use octizys_parser::lexer::{
    BaseLexerContext, BaseToken, LexerContext, LexerError, Token,
};
use octizys_pretty::highlight::EmptyRender;
use octizys_pretty::{
    combinators::{
        concat, empty, external_text, group, hard_break, intersperse, nest,
        repeat,
    },
    document::Document,
    highlight::{HighlightRenderer, TerminalRender24},
};
use octizys_text_store::store::{aproximate_string_width, Store};
use std::fmt::Debug;
use std::io::{self, Read};
use std::path::Path;

//TODO:
//TEst the following :
//```
//import a::b (c,d,e,f,g,h,i -- some comment
//,j) as k::l
//```
//I believe with the right parameters we end with
//```
//import
//  a::b (c,d,e,f,g,h,i -- some comment
//  ,j) as k::l
//```
//We expected:
//```
//import
//  a::b (
//  c,d,e,f,g,h,i -- some comment
//  ,j)
//  as k::l
//```
//Or something better...
//
//

enum AvailableParser {
    Import,
    Type,
}

enum StructDisplay {
    PrettyDebug,
    Debug,
    EquivalenceRepresentation,
}

struct Configuration {
    parser: AvailableParser,
    // None means, omit
    graphics: Option<Box<Path>>,
    show_tokens: bool,
    // None means, omit
    struct_display: Option<StructDisplay>,
}

enum Action {
    ParseString(String),
    ParseFile(Box<Path>),
    REPL(),
}

struct Arguments {
    configuration: Configuration,
    action: Action,
}

fn main() {
    let mut store = Store::default();
    let configuration = PrettyCSTConfiguration::default();
    let mut stdin = io::stdin();
    let input = &mut String::new();
    let p = import_declarationParser::new();
    let p2 = type_expressionParser::new();
    loop {
        input.clear();
        stdin.read_line(input);
        input.pop();
        if input == ":q" {
            break;
        }
        let mut base_context = BaseLexerContext::new(input, &mut store);
        let iterator = LexerContext::new(None, &mut base_context);
        let parsed = p2.parse(iterator.map(|x| {
            match x.clone() {
                Ok((_, tok, _)) => println!("Ok({:?})", tok),
                Err(e) => println!("Err({:?})", e),
            }
            x
        }));
        match parsed {
            Ok(item) => {
                //println!("{:#?}", item);
                let as_doc = group(item.to_document(&configuration));
                //println!("DOC:{:#?}", as_doc);
                // TODO: make color configurable
                println!(
                    "{}",
                    as_doc.render_to_string(
                        80,
                        EmptyRender::render_highlight,
                        &store
                    )
                )
            }
            Err(t) => {
                let mut error_context = ParserErrorContext::default();
                let input_clone = input.clone();
                error_context.src = &input_clone;
                let report = create_error_report(&t, &error_context);
                let as_str = report.render_to_string(
                    80,
                    TerminalRender24::render_highlight,
                    &store,
                );
                println!("{}", as_str);
            }
        }
    }
}
