mod arguments;
mod error_report;

use crate::error_report::create_error_report;
use arguments::{AvailableParser, Configuration, DebugFormatOption};
use clap::Parser;
use error_report::{ReportKind, ReportRequest, ReportSourceContext};
use lalrpop_util::ParseError;
use octizys_common::{equivalence::Equivalence, span::Position};
use octizys_cst::{imports::Import, top::Top, types::Type};
use octizys_formatter::{cst::PrettyCSTConfiguration, to_document::ToDocument};
use octizys_macros::Equivalence;
use octizys_parser::{
    grammar::{import_declarationParser, topParser, type_expressionParser},
    lexer::{BaseLexerContext, LexerContext, LexerError, Token},
};
use octizys_pretty::{
    combinators::external_text,
    document::Document,
    highlight::{
        EmptyRender, Highlight, HighlightRenderer, TerminalRender24,
        TerminalRender4, TerminalRender8,
    },
};
use octizys_text_store::store::Store;
use simplelog;
use std::cell::RefCell;
use std::fmt::Debug;
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

#[derive(Debug, Equivalence)]
enum ReplParserOutput {
    Import(Import),
    Type(Type),
    Top(Top),
}

impl ReplParserOutput {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            ReplParserOutput::Import(i) => i.to_document(configuration),
            ReplParserOutput::Type(t) => t.to_document(configuration),
            ReplParserOutput::Top(t) => t.to_document(configuration),
        }
    }

    fn represent(&self) -> Document {
        match self {
            ReplParserOutput::Import(i) => i.represent(),
            ReplParserOutput::Type(t) => t.represent(),
            ReplParserOutput::Top(t) => t.represent(),
        }
    }
}

#[derive(Clone)]
struct RenderInfo {
    debug: DebugFormatOption,
    column_width: usize,
    highlight: fn(&Highlight) -> (String, String),
}

fn println_with<'store, T>(
    value: &T,
    store: &'store Store,
    options: &RenderInfo,
) -> ()
where
    T: Debug + Equivalence,
{
    let document = to_document_with(value, options.debug);
    let as_string = render_with(&document, store, options);
    println!("{}", as_string)
}

fn render_with<'store>(
    document: &Document,
    store: &'store Store,
    options: &RenderInfo,
) -> String
where
{
    document.render_to_string(options.column_width, options.highlight, store)
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

macro_rules! gen_parse_function {
    ($function_name:ident, $name:ident,$parser_name:ident) => {
        fn $function_name(
            s: &str,
            store: Rc<RefCell<Store>>,
            show_token_stream: Option<RenderInfo>,
        ) -> Result<ReplParserOutput, ParseError<Position, Token, LexerError>> {
            let mut base_context = BaseLexerContext::new(s, store.clone());
            let iterator = LexerContext::new(None, &mut base_context);
            match show_token_stream {
                Some(renderer_info) => {
                    let new_iterator = iterator.inspect(|x| match x.clone() {
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
                    });
                    $parser_name::new()
                        .parse(new_iterator)
                        .map(|x| ReplParserOutput::$name(x))
                }
                None => $parser_name::new()
                    .parse(iterator)
                    .map(|x| ReplParserOutput::$name(x)),
            }
        }
    };
}

gen_parse_function!(parse_import, Import, import_declarationParser);
gen_parse_function!(parse_type, Type, type_expressionParser);
gen_parse_function!(parse_top, Top, topParser);

fn parse_string_with_renderer(
    parser_option: AvailableParser,
    s: &str,
    store: Rc<RefCell<Store>>,
    show_token_stream: Option<RenderInfo>,
) -> Result<
    ReplParserOutput,
    ParseError<Position, octizys_parser::lexer::Token, LexerError>,
> {
    match parser_option {
        AvailableParser::Import => parse_import(s, store, show_token_stream),
        AvailableParser::Type => parse_type(s, store, show_token_stream),
        AvailableParser::Top => parse_top(s, store, show_token_stream),
    }
}

fn println_result(
    input: &str,
    file_name: Option<&str>,
    result: Result<ReplParserOutput, ParseError<Position, Token, LexerError>>,
    store: &Store,
    show_cst: bool,
    show_doc: bool,
    renderer_info: &RenderInfo,
) -> () {
    //TODO: pass pretty configuration
    let pretty_configuration = PrettyCSTConfiguration::default();
    match result {
        Ok(item) => {
            if show_cst {
                println_with(&item, store, renderer_info);
            }
            let as_doc = item.to_document(&pretty_configuration);
            if show_doc {
                match renderer_info.debug {
                    DebugFormatOption::PrettyDebug => {
                        println!("{:#?}", &as_doc)
                    }
                    DebugFormatOption::Debug => println!("{:?}", as_doc),
                    DebugFormatOption::EquivalenceRepresentation => {
                        println!("{:#?}", as_doc)
                    }
                }
            }
            let as_string = render_with(&as_doc, store, renderer_info);
            println!("{}", as_string)
        }
        Err(t) => {
            let mut error_context = ReportSourceContext::default();
            // TODO: add setter to ReportSourceContext and use it instead of expossing this
            error_context.src = &input;
            match file_name {
                Some(name) => error_context.src_name = name,
                None => (),
            }
            let request = ReportRequest {
                report: &t,
                source_context: error_context,
                //TODO add option for this
                target: Default::default(),
                //TODO: FIXME: change this! it require major changes in the lexer
                //as we plan to change the parser error to support this!
                kind: ReportKind::Error,
            };
            let report = create_error_report(&request);
            let as_string = render_with(&report, store, renderer_info);
            println!("{}", as_string)
        }
    }
}

fn parse_string(
    string: &str,
    file_path: Option<&str>,
    image_path: Option<PathBuf>,
    store: Store,
    configuration: Configuration,
) -> () {
    let highlight = match configuration.renderer {
        arguments::AvailableRenderers::PlainText => {
            EmptyRender::render_highlight
        }
        arguments::AvailableRenderers::AnsiC4 => {
            TerminalRender4::render_highlight
        }
        arguments::AvailableRenderers::AnsiC8 => {
            TerminalRender8::render_highlight
        }
        arguments::AvailableRenderers::AnsiC24 => {
            TerminalRender24::render_highlight
        }
    };
    let new_store = Rc::new(RefCell::new(store));
    let renderer_info = RenderInfo {
        debug: configuration.display_format,
        column_width: configuration.column_width,
        highlight: highlight,
    };
    if configuration.show_tokens {
        let result = parse_string_with_renderer(
            configuration.parser,
            string,
            new_store.clone(),
            Some(renderer_info.clone()),
        );
        println_result(
            string,
            file_path,
            result,
            &*(*new_store).borrow(),
            configuration.show_cst,
            configuration.show_doc,
            &renderer_info,
        )
    } else {
        let result = parse_string_with_renderer(
            configuration.parser,
            string,
            new_store.clone(),
            None,
        );
        println_result(
            string,
            file_path,
            result,
            &*(*new_store).borrow(),
            configuration.show_cst,
            configuration.show_doc,
            &renderer_info,
        )
    };
}

fn parse_file(
    file_path: PathBuf,
    image_path: Option<PathBuf>,
    store: Store,
    configuration: Configuration,
) -> () {
    match ::std::fs::read_to_string(file_path.clone()) {
        Ok(content) => {
            let path = match std::path::absolute(file_path.clone().as_path()) {
                Ok(p) => p.to_str().map(String::from),
                Err(_) => file_path.to_str().map(String::from),
            };
            parse_string(
                &content,
                path.as_deref(),
                image_path,
                store,
                configuration,
            )
        }
        Err(_) => println!("Can't read file: {file_path:?}"),
    }
}

fn repl(store: Store, configuration: Configuration) -> () {
    todo!()
}

fn main() {
    let _ = simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
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
                parse_string(&string, None, image_path, store, configuration)
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
}
