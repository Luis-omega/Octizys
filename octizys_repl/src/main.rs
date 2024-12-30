mod arguments;
mod error_report;

use arguments::{AvailableParser, Configuration, DebugFormatOption};
use clap::Parser;
use lalrpop_util::ParseError;
use octizys_common::equivalence::Equivalence;
use octizys_cst::imports::Import;
use octizys_cst::types::Type;

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
use octizys_pretty::highlight::{
    self, EmphasisRender, EmptyRender, Highlight, TerminalRender4,
    TerminalRender8,
};
use octizys_pretty::{
    combinators::{
        concat, empty, external_text, group, hard_break, intersperse, nest,
        repeat,
    },
    document::Document,
    highlight::{HighlightRenderer, TerminalRender24},
};
use octizys_text_store::store::{aproximate_string_width, Store};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::default;
use std::fmt::Debug;
use std::io::{self, Read};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

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

enum ReplParserOutput {
    Import(Import),
    Type(Type),
}

impl ReplParserOutput {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            ReplParserOutput::Import(i) => i.to_document(configuration),
            ReplParserOutput::Type(t) => t.to_document(configuration),
        }
    }
}

#[derive(Clone)]
struct RenderInfo<Renderer>
where
    Renderer: HighlightRenderer,
{
    debug: DebugFormatOption,
    column_width: usize,
    highlight: PhantomData<Renderer>,
}

fn println_with<'store, T, Renderer>(
    value: &T,
    store: &'store Store,
    options: &RenderInfo<Renderer>,
) -> ()
where
    T: Debug + Equivalence,
    Renderer: HighlightRenderer,
{
    let document = to_document_with(value, options.debug);
    let as_string = render_with(document, store, options);
    println!("{}", as_string)
}

fn render_with<'store, Renderer>(
    document: Document,
    store: &'store Store,
    options: &RenderInfo<Renderer>,
) -> String
where
    Renderer: HighlightRenderer,
{
    document.render_to_string(
        options.column_width,
        Renderer::render_highlight,
        store,
    )
}

fn to_document_with<T>(value: &T, format: DebugFormatOption) -> Document
where
    T: Debug + Equivalence,
{
    match format {
        DebugFormatOption::PrettyDebug => {
            external_text(&format!("{:#?}", value))
        }
        DebugFormatOption::Debug => external_text(&format!("{:?}", value)),
        DebugFormatOption::EquivalenceRepresentation => value.represent(),
    }
}

macro_rules! gen_parser_option_branch {
    ($name:ident, $type:expr) => {{
        let mut base_context = BaseLexerContext::new(s, &mut store);
        let mut iterator = LexerContext::new(None, &mut base_context);
        match show_token_stream {
            Some(renderer_info) => {
                iterator = iterator.map(|x| {
                    match x.clone() {
                        Ok((_, tok, _)) => {
                            println_with(tok, &store, renderer_info)
                        }
                        Err(e) => println_with(e, &store, renderer_info),
                    }
                    x
                })
            }
            None => (),
        }
        $type::new()
            .parse(iterator)
            .map(|x| ReplParserOutput::$name(x))
    }};
}

fn parse_string_with_renderer<'store, Renderer>(
    parser_option: AvailableParser,
    s: &str,
    store: Rc<RefCell<Store>>,
    show_token_stream: Option<RenderInfo<Renderer>>,
) -> Result<
    ReplParserOutput,
    ParseError<Position, octizys_parser::lexer::Token, LexerError>,
>
where
    Renderer: HighlightRenderer,
{
    match parser_option {
        AvailableParser::Import => {
            let mut base_context = BaseLexerContext::new(s, store.clone());
            let iterator = LexerContext::new(None, &mut base_context);
            match show_token_stream {
                Some(renderer_info) => {
                    let new_iterator = iterator.map(|x| {
                        match x.clone() {
                            Ok((_, tok, _)) => println_with(
                                &tok,
                                &*(*store).borrow(),
                                &renderer_info,
                            ),
                            Err(e) => println_with(
                                &e,
                                &*(*store).borrow(),
                                &renderer_info,
                            ),
                        }
                        x
                    });
                    import_declarationParser::new()
                        .parse(new_iterator)
                        .map(|x| ReplParserOutput::Import(x))
                }
                None => import_declarationParser::new()
                    .parse(iterator)
                    .map(|x| ReplParserOutput::Import(x)),
            }
        }
        AvailableParser::Type => todo!(),
        //gen_parser_option_branch!(Type,type_expressionParser),
    }
}

fn println_result(
    input: &str,
    result: Result<ReplParserOutput, ParseError<Position, Token, LexerError>>,
    store: &Store,
    highlight: fn(&Highlight) -> (String, String),
) -> () {
    let pretty_configuration = PrettyCSTConfiguration::default();
    match result {
        Ok(item) => {
            let as_doc = group(item.to_document(&pretty_configuration));

            println!("{}", as_doc.render_to_string(80, highlight, store))
        }
        Err(t) => {
            let mut error_context = ParserErrorContext::default();
            error_context.src = &input;
            let report = create_error_report(&t, &error_context);
            let as_str = report.render_to_string(80, highlight, store);
            println!("{}", as_str);
        }
    }
}

fn parse_string(
    string: &str,
    image_path: Option<PathBuf>,
    store: Store,
    configuration: Configuration,
) -> () {
    let new_store = Rc::new(RefCell::new(store));
    let result = if configuration.show_tokens {
        match configuration.renderer {
            arguments::AvailableRenderers::PlainText => {
                let renderer_info: RenderInfo<EmptyRender> = RenderInfo {
                    debug: configuration.display_format,
                    column_width: configuration.column_width,
                    highlight: Default::default(),
                };
                let result = parse_string_with_renderer(
                    configuration.parser,
                    string,
                    new_store.clone(),
                    Some(renderer_info),
                );
                println_result(
                    string,
                    result,
                    &*(*new_store).borrow(),
                    EmptyRender::render_highlight,
                )
            }
            arguments::AvailableRenderers::AnsiC4 => {
                let renderer_info: RenderInfo<TerminalRender4> = RenderInfo {
                    debug: configuration.display_format,
                    column_width: configuration.column_width,
                    highlight: Default::default(),
                };
                let result = parse_string_with_renderer(
                    configuration.parser,
                    string,
                    new_store.clone(),
                    Some(renderer_info),
                );
                println_result(
                    string,
                    result,
                    &*(*new_store).borrow(),
                    TerminalRender4::render_highlight,
                )
            }
            arguments::AvailableRenderers::AnsiC8 => {
                let renderer_info: RenderInfo<TerminalRender8> = RenderInfo {
                    debug: configuration.display_format,
                    column_width: configuration.column_width,
                    highlight: Default::default(),
                };
                let result = parse_string_with_renderer(
                    configuration.parser,
                    string,
                    new_store.clone(),
                    Some(renderer_info),
                );
                println_result(
                    string,
                    result,
                    &*(*new_store).borrow(),
                    TerminalRender8::render_highlight,
                )
            }
            arguments::AvailableRenderers::AnsiC24 => {
                let renderer_info: RenderInfo<TerminalRender24> = RenderInfo {
                    debug: configuration.display_format,
                    column_width: configuration.column_width,
                    highlight: Default::default(),
                };
                let result = parse_string_with_renderer(
                    configuration.parser,
                    string,
                    new_store.clone(),
                    Some(renderer_info),
                );
                println_result(
                    string,
                    result,
                    &*(*new_store).borrow(),
                    TerminalRender24::render_highlight,
                )
            }
        }
    } else {
        let renderer: Option<RenderInfo<TerminalRender24>> = None;
        let result = parse_string_with_renderer(
            configuration.parser,
            string,
            new_store.clone(),
            renderer,
        );
        println_result(
            string,
            result,
            &*(*new_store).borrow(),
            TerminalRender24::render_highlight,
        )
    };
}

fn parse_file(
    path_name: PathBuf,
    image_path: Option<PathBuf>,
    store: Store,
    configuration: Configuration,
) -> () {
    todo!()
}

fn repl(store: Store, configuration: Configuration) -> () {
    todo!()
}

fn main() {
    let arguments = crate::arguments::Arguments::parse();
    let store = Store::default();
    match arguments.command {
        arguments::Commands::ParseString {
            string,
            configuration,
            image_path,
        } => {
            if configuration.show_arguments {
                println!("{:#?}", configuration)
            } else {
                parse_string(&string, image_path, store, configuration)
            }
        }
        arguments::Commands::ParseFile {
            path,
            configuration,
            image_path,
        } => {
            if configuration.show_arguments {
                println!("{:#?}", configuration)
            } else {
                parse_file(path, image_path, store, configuration)
            }
        }
        arguments::Commands::REPL { configuration } => {
            repl(store, configuration)
        }
    };
    /*
    let mut store = Rc::new(RefCell::new(Store::default()));
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
        let mut base_context = BaseLexerContext::new(input, store.clone());
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
                        &*(*store).borrow()
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
                    &*(*store).borrow(),
                );
                println!("{}", as_str);
            }
        }
    }
    */
}
