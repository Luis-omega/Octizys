mod arguments;

use arguments::{
    DebugCommand, DebugFormatOption, FormatterConfiguration, Phase,
};
use clap::Parser;
use lalrpop_util::ParseError;
use octizys_common::report::{
    create_error_report, ReportFormat, ReportKind, ReportRequest,
    ReportSourceContext, ReportTarget,
};
use octizys_common::span::Location;
use octizys_common::{equivalence::Equivalence, span::Position};
use octizys_cst::{imports::Import, top::Top, types::Type};
use octizys_formatter::{cst::PrettyCSTConfiguration, to_document::ToDocument};
use octizys_macros::Equivalence;
use octizys_parser::{
    grammar::{import_declarationParser, topParser, type_expressionParser},
    lexer::{self, BaseLexerContext, LexerContext, Token},
    report::OctizysParserReport,
};
use octizys_pretty::{
    combinators::{external_text, foreground, static_str},
    document::Document,
    highlight::{
        base_colors::MODERATE_GREEN, EmptyRender, Highlight, HighlightRenderer,
        TerminalRender24, TerminalRender4, TerminalRender8,
    },
    store::NonLineBreakStr,
};
use octizys_text_store::store::Store;
use simplelog;
use std::{
    borrow::Borrow,
    io::{self, Write},
    path::PathBuf,
};
use std::{cell::RefCell, collections::HashSet};
use std::{collections::HashMap, fmt::Debug};
use std::{rc::Rc, result};

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
//

#[derive(Clone)]
enum OctizysError {
    LexicalError(OctizysParserReport, String),
    ParseError(
        ParseError<Position, lexer::Token, OctizysParserReport>,
        String,
    ),
    FileLoadError {
        path: PathBuf,
    },
    REPlCantReadLine {
        error: std::io::ErrorKind,
    },
}

impl OctizysError {
    fn get_source(&self) -> Option<&str> {
        match self {
            OctizysError::LexicalError(_, s) => Some(&s),
            OctizysError::ParseError(_, s) => Some(&s),
            OctizysError::FileLoadError { .. } => None,
            OctizysError::REPlCantReadLine { .. } => None,
        }
    }
}

impl<'source> ReportFormat for OctizysError {
    fn get_short_description(&self) -> octizys_pretty::store::NonLineBreakStr {
        match self {
            OctizysError::LexicalError(e, _) => {
                e.report.get_short_description()
            }
            OctizysError::ParseError(e, _) => e.get_short_description(),
            OctizysError::FileLoadError { .. } => {
                NonLineBreakStr::new("Can't open a file!")
            }
            OctizysError::REPlCantReadLine { .. } => {
                NonLineBreakStr::new("Can't read line!")
            }
        }
    }
    fn get_long_description(&self, target: &ReportTarget) -> Option<Document> {
        match self {
            OctizysError::LexicalError(e, _) => e.get_long_description(target),
            OctizysError::ParseError(e, _) => e.get_long_description(target),
            OctizysError::FileLoadError { path } => Some(external_text(
                //TODO: make it more fancy
                &format!("Couldn't open the file:{:#?}", path),
            )),
            OctizysError::REPlCantReadLine { error } => Some(external_text(
                &format!("While trying to read a line:{:}", error),
            )),
        }
    }

    fn get_report_name(&self) -> octizys_pretty::store::NonLineBreakStr {
        match self {
            OctizysError::LexicalError(e, _) => e.get_report_name(),
            OctizysError::ParseError(e, _) => e.get_report_name(),
            OctizysError::FileLoadError { path } => {
                NonLineBreakStr::new("OctizysCommandLineArgument")
            }
            OctizysError::REPlCantReadLine { error } => {
                NonLineBreakStr::new("OctizysREPL")
            }
        }
    }

    fn get_location_maybe(&self) -> Option<Location> {
        match self {
            OctizysError::LexicalError(e, _) => e.get_location_maybe(),
            OctizysError::ParseError(e, _) => e.get_location_maybe(),
            OctizysError::FileLoadError { path } => None,
            OctizysError::REPlCantReadLine { error } => None,
        }
    }

    fn get_expected(&self) -> Option<Vec<String>> {
        match self {
            OctizysError::LexicalError(e, _) => e.get_expected(),
            OctizysError::ParseError(e, _) => e.get_expected(),
            OctizysError::FileLoadError { path } => None,
            OctizysError::REPlCantReadLine { error } => None,
        }
    }
}

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
) -> Result<
    Top,
    ParseError<Position, octizys_parser::lexer::Token, OctizysParserReport>,
> {
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
    file_name: Option<String>,
    result: &Result<Top, OctizysError>,
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
            error_context.src = t.get_source().unwrap_or_else(|| &"");
            let source_name = match file_name {
                Some(name) => name,
                None => String::from("OctizysCLI"),
            };
            error_context.src_name = &source_name;
            let request = ReportRequest {
                report: t,
                source_context: error_context,
                // TODO add and option for choosing between new user and advanced.
                target: if options.use_machine_representation {
                    ReportTarget::Machine(Default::default())
                } else {
                    ReportTarget::Human(Default::default())
                },
                kind: match t {
                    OctizysError::LexicalError(r, _) => r.kind,
                    OctizysError::ParseError(p, _) => match p {
                        ParseError::User { error } => error.kind,
                        _ => ReportKind::Error,
                    },
                    OctizysError::FileLoadError { .. } => ReportKind::Error,
                    OctizysError::REPlCantReadLine { error } => {
                        ReportKind::Error
                    }
                },
            };
            let report = create_error_report(&request);
            let as_string = render_with(&report, store, &options);
            eprintln!("{}", as_string)
        }
    }
}

fn parse_string(
    string: String,
    file_path: Option<&str>,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> Result<Top, OctizysError> {
    let result = parse_string_with_options(&string, &options, store)
        .map_err(|x| OctizysError::ParseError(x, string));
    result
}

fn parse_file<'source>(
    file_path: PathBuf,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> Result<Top, OctizysError> {
    match ::std::fs::read_to_string(file_path.clone()) {
        Ok(content) => {
            let path = match std::path::absolute(file_path.clone().as_path()) {
                Ok(p) => p.to_str().map(String::from),
                Err(_) => file_path.to_str().map(String::from),
            };
            parse_string(content, path.as_deref(), options, store)
        }
        Err(_) => Err(OctizysError::FileLoadError { path: file_path }),
    }
}

fn compile_file(
    source_path: PathBuf,
    output: Option<PathBuf>,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    let result = parse_file(source_path.clone(), options, store.clone());
    let path = match std::path::absolute(source_path.clone().as_path()) {
        Ok(p) => p.to_str().map(String::from),
        Err(_) => source_path.to_str().map(String::from),
    };
    match &result {
        Err(_) => println_result(path, &result, &options, &*(*store).borrow()),
        Ok(_) => (),
    }
}

fn debug_file(
    path_or_source: Result<PathBuf, String>,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    match path_or_source {
        Ok(path) => {
            let result = parse_file(path.clone(), options, store.clone());
            let path = match std::path::absolute(path.clone().as_path()) {
                Ok(p) => p.to_str().map(String::from),
                Err(_) => path.to_str().map(String::from),
            };
            println_result(path, &result, &options, &*(*store).borrow())
        }
        Err(s) => {
            let result = parse_string(s, None, options, store.clone());
            println_result(None, &result, &options, &*(*store).borrow())
        }
    };
}

//TODO: CRITICAL:
// Put a roundtrip test and report a diff tree if it fails!
// TODO:
// use the output parameter
//
// TODO: Cleanup, remove the unused things (see Cargo.toml)
fn format_file(
    source_path: PathBuf,
    output: Option<PathBuf>,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    let result = parse_file(source_path.clone(), options, store.clone());
    let path = match std::path::absolute(source_path.clone().as_path()) {
        Ok(p) => p.to_str().map(String::from),
        Err(_) => source_path.to_str().map(String::from),
    };
    println_result(path, &result, &options, &*(*store).borrow())
}

fn repl(
    prompt: String,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    // TODO:  Add option to choose color
    // TODO: Add commands in repl (maybe use the larlpop parser for that!);
    let prompt_document = foreground(MODERATE_GREEN, external_text(&prompt));
    let rendered_prompt =
        render_with(&prompt_document, &*(*store).borrow(), options);
    loop {
        let mut buffer = String::new();
        print!("{}", rendered_prompt);
        io::stdout().flush();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => debug_file(Err(buffer), options, store.clone()),
            Err(e) => println_result(
                None,
                &Err(OctizysError::REPlCantReadLine { error: e.kind() }),
                options,
                &*(*store).borrow(),
            ),
        }
    }
}

fn main() {
    let _ = simplelog::TermLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
    let arguments = crate::arguments::Arguments::parse();
    let mut real_store = Store::default();
    let store = Rc::new(RefCell::new(real_store));
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
            compile_file(path, output, &options, store)
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
                    debug_file(path_or_source, &options, store)
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
            format_file(path, output, &options, store)
        }
        arguments::Commands::REPL { prompt } => {
            let options = GlobalOptions::from_format_configuration(
                arguments.display_format,
                &formatter_configuration,
                HashSet::new(),
            );
            repl(prompt, &options, store)
        }
    };
}
