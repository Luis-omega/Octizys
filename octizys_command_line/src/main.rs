mod arguments;
mod error_report;

use crate::error_report::create_error_report;
use arguments::{
    DebugCommand, DebugFormatOption, FormatterConfiguration, Phase,
};
use clap::Parser;
use error_report::{
    ReportKind, ReportRequest, ReportSourceContext, ReportTarget,
};
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
use std::rc::Rc;
use std::{borrow::Borrow, path::PathBuf};
use std::{cell::RefCell, collections::HashSet};
use std::{collections::HashMap, fmt::Debug};

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

#[derive(Clone)]
struct GlobalOptions {
    debug: DebugFormatOption,
    column_width: usize,
    highlight: fn(&Highlight) -> (String, String),
    pretty_configuration: PrettyCSTConfiguration,
    phases: HashSet<Phase>,
    use_machine_representation: bool,
}

impl GlobalOptions {
    fn from_format_configuration(
        debug: DebugFormatOption,
        configuration: &FormatterConfiguration,
        phases: HashSet<Phase>,
    ) -> GlobalOptions {
        let pretty_configuration: PrettyCSTConfiguration =
            PrettyCSTConfiguration {
                indentation_deep: configuration.indentation_deep,
                leading_commas: configuration.leading_commas,
                add_trailing_separator: configuration.add_trailing_separator,
                move_documentantion_before_object: configuration
                    .move_documentantion_before_object,
                //TODO: check the comment on Arguments for this option.
                indent_comment_blocks: false,
                //TODO: I think this option must be this kind of global.
                separe_comments_by: configuration.separe_by,
                compact_comments: configuration.compact_comments,
            };
        let mut highlight = match configuration.renderer {
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
        if configuration.use_machine_representation {
            highlight = EmptyRender::render_highlight
        }
        GlobalOptions {
            debug,
            column_width: configuration.column_width,
            highlight,
            pretty_configuration,
            phases,
            use_machine_representation: configuration
                .use_machine_representation,
        }
    }

    fn has_phase(&self, p: &Phase) -> bool {
        self.phases.contains(&p)
    }
}

fn println_with<'store, T>(
    value: &T,
    store: &'store Store,
    options: &GlobalOptions,
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
    options: &GlobalOptions,
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

fn parse_string_with_options(
    source: &str,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> Result<Top, ParseError<Position, octizys_parser::lexer::Token, LexerError>>
{
    let mut base_context = BaseLexerContext::new(source, store.clone());
    let iterator = LexerContext::new(None, &mut base_context);
    if options.has_phase(&Phase::Lexer) {
        let new_iterator = iterator.inspect(|x| match x.clone() {
            Ok((_, tok, _)) => {
                println_with(&tok, &*(*store).borrow(), &options)
            }
            Err(e) => println_with(&e, &*(*store).borrow(), &options),
        });
        topParser::new().parse(new_iterator)
    } else {
        topParser::new().parse(iterator)
    }
}

fn println_result<'s>(
    input: &'s str,
    file_name: Option<&'s str>,
    result: Result<Top, ParseError<Position, Token, LexerError>>,
    options: &'s GlobalOptions,
    store: &'s Store,
) -> () {
    match result {
        Ok(item) => {
            if options.has_phase(&Phase::Parser) {
                println_with(&item, store, options);
            }
            let as_doc = item.to_document(&options.pretty_configuration);
            if options.has_phase(&Phase::Document) {
                match options.debug {
                    DebugFormatOption::PrettyDebug => {
                        println!("{:#?}", &as_doc)
                    }
                    DebugFormatOption::Debug => println!("{:?}", as_doc),
                    DebugFormatOption::EquivalenceRepresentation => {
                        println!("{:#?}", as_doc)
                    }
                }
            }
            let as_string = render_with(&as_doc, store, options);
            println!("{}", as_string)
        }
        Err(t) => {
            let mut error_context = ReportSourceContext::default();
            // TODO: add setter to ReportSourceContext and use it instead of exposing this
            error_context.src = &input;
            match file_name {
                Some(name) => error_context.src_name = name,
                None => (),
            }
            let request = ReportRequest {
                report: &t,
                source_context: error_context,
                // TODO add and option for choosing between new user and advanced.
                target: if options.use_machine_representation {
                    ReportTarget::Machine(Default::default())
                } else {
                    ReportTarget::Human(Default::default())
                },
                // TODO: FIXME: change this! it requires major changes in the lexer
                //as we plan to change the parser error to support this!
                kind: ReportKind::Error,
            };
            let report = create_error_report(&request);
            let as_string = render_with(&report, store, &options);
            eprintln!("{}", as_string)
        }
    }
}

fn parse_string(
    string: &str,
    file_path: Option<&str>,
    store: Store,
    options: &GlobalOptions,
) -> () {
    let new_store = Rc::new(RefCell::new(store));
    let result = parse_string_with_options(string, &options, new_store.clone());
    println_result(
        string,
        file_path,
        result,
        &options,
        &*(*new_store).borrow(),
    );
}

fn parse_file(file_path: PathBuf, store: Store, options: &GlobalOptions) -> () {
    match ::std::fs::read_to_string(file_path.clone()) {
        Ok(content) => {
            let path = match std::path::absolute(file_path.clone().as_path()) {
                Ok(p) => p.to_str().map(String::from),
                Err(_) => file_path.to_str().map(String::from),
            };
            parse_string(&content, path.as_deref(), store, options)
        }
        // TODO: use the report system to report this (is an overkill but since we have it…)
        Err(_) => eprintln!("Can't read file: {file_path:?}"),
    }
}

fn compile_file(
    source_path: PathBuf,
    output: Option<PathBuf>,
    options: &GlobalOptions,
    store: &mut Store,
) -> () {
    todo!()
}

fn debug_file(
    path_or_source: Result<PathBuf, String>,
    options: &GlobalOptions,
    store: &mut Store,
) -> () {
    todo!()
}

fn format_file(
    source_path: PathBuf,
    output: Option<PathBuf>,
    options: &GlobalOptions,
    store: &mut Store,
) -> () {
    todo!()
}

fn repl(prompt: String, options: &GlobalOptions, store: &mut Store) -> () {
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
    let mut store = Store::default();
    if arguments.show_arguments {
        println!("{:#?}", arguments);
        return;
    }
    let formatter_configuration = arguments.formatter_configuration;

    match arguments.command {
        arguments::Commands::Compile { path, output } => {
            let options = GlobalOptions::from_format_configuration(
                arguments.display_format,
                &formatter_configuration,
                HashSet::new(),
            );
            compile_file(path, output, &options, &mut store)
        }
        arguments::Commands::Debug { command } => {
            let options = GlobalOptions::from_format_configuration(
                arguments.display_format,
                &formatter_configuration,
                HashSet::from_iter(command.phases),
            );
            let option_path_or_source = match command.source_string {
                Some(s) => Some(Err(s)),
                None => match command.source_path {
                    Some(p) => Some(Ok(p)),
                    None => {
                        println!("Error!: Needed source_path or string to parse!\nAborting!");
                        None
                    }
                },
            };
            match option_path_or_source {
                Some(path_or_source) => {
                    debug_file(path_or_source, &options, &mut store)
                }
                None => return (),
            }
        }
        arguments::Commands::Format { path, output } => {
            let options = GlobalOptions::from_format_configuration(
                arguments.display_format,
                &formatter_configuration,
                HashSet::new(),
            );
            format_file(path, output, &options, &mut store)
        }
        arguments::Commands::REPL { prompt } => {
            let options = GlobalOptions::from_format_configuration(
                arguments.display_format,
                &formatter_configuration,
                HashSet::new(),
            );
            repl(prompt, &options, &mut store)
        }
    };
}
