mod arguments;

use arguments::FormatterConfiguration;
use clap::Parser;
use octizys_common::equivalence::Equivalence;
use octizys_common::report::{
    create_error_report, IOError, ReportTarget, ReportUserKind,
};
use octizys_cst::top::Top;
use octizys_formatter::{cst::PrettyCSTConfiguration, to_document::ToDocument};
use octizys_parser::parser::{parse_file, parse_string};
use octizys_pretty::{
    combinators::{external_text, foreground},
    document::Document,
    highlight::{
        base_colors::MODERATE_GREEN, EmptyRender, Highlight, HighlightRenderer,
        TerminalRender24, TerminalRender4, TerminalRender8,
    },
};
use octizys_text_store::store::Store;
use simplelog;
use std::cell::RefCell;
use std::rc::Rc;
use std::{
    io::{self, Write},
    path::PathBuf,
};

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
struct GlobalOptions {
    column_width: usize,
    highlight: fn(&Highlight) -> (String, String),
    pretty_configuration: PrettyCSTConfiguration,
    target: ReportTarget,
}

impl From<FormatterConfiguration> for GlobalOptions {
    fn from(value: FormatterConfiguration) -> Self {
        let pretty_configuration: PrettyCSTConfiguration =
            PrettyCSTConfiguration {
                indentation_deep: value.indentation_deep,
                leading_commas: value.leading_commas,
                add_trailing_separator: value.add_trailing_separator,
                move_documentantion_before_object: value
                    .move_documentantion_before_object,
                //TODO: check the comment on Arguments for this option.
                indent_comment_blocks: false,
                //TODO: I think this option must be this kind of global.
                separe_comments_by: value.separe_by,
                compact_comments: value.compact_comments,
            };
        let mut highlight = match value.renderer {
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
        let userkind = if value.use_advanced_errors {
            ReportUserKind::Advanced
        } else {
            ReportUserKind::New
        };
        let target = if value.use_machine_representation {
            highlight = EmptyRender::render_highlight;
            ReportTarget::Machine(userkind)
        } else {
            ReportTarget::Human(userkind)
        };
        GlobalOptions {
            column_width: value.column_width,
            highlight,
            pretty_configuration,
            target,
        }
    }
}

fn render_with<'store>(
    document: &Document,
    store: Rc<RefCell<Store>>,
    options: &GlobalOptions,
) -> String
where
{
    document.render_to_string(
        options.column_width,
        options.highlight,
        &*(*store).borrow(),
    )
}

fn compile_file(
    source_path: PathBuf,
    _output: Option<PathBuf>,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    match parse_file(source_path, store.clone()) {
        Ok(_top) => (),
        Err(e) => {
            let request = e.build_report_request(
                options.target,
                String::from("OctizysCommandLine"),
                options.column_width,
            );
            let report = create_error_report(&request);
            let report_str = render_with(&report, store, options);
            eprintln!("{}", report_str)
        }
    }
}

fn check_idempotence(
    top: &Top,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> Result<Document, Document> {
    let highlight = EmptyRender::render_highlight;
    let format_options = GlobalOptions {
        highlight,
        ..options.clone()
    };
    let doc = top.to_document(&options.pretty_configuration);
    let string = render_with(&doc, store.clone(), &format_options);
    let second_result = parse_string(&string, None, store.clone());
    match second_result {
        Ok(second_top) => top.equivalence_or_diff(&second_top).map(|_| doc),
        Err(e) => {
            let request = e.build_report_request(
                options.target,
                String::from("OctizysCommandLine"),
                options.column_width,
            );
            let report = create_error_report(&request);
            Err(report)
        }
    }
}

// TODO:
// use the output parameter
fn format_file(
    source_path: PathBuf,
    _output: Option<PathBuf>,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    match parse_file(source_path, store.clone()) {
        Ok(top) => match check_idempotence(&top, options, store.clone()) {
            Ok(doc) => {
                let string = render_with(&doc, store, options);
                println!("{}", string);
            }
            Err(doc) => {
                let string = render_with(&doc, store.clone(), options);
                let origina_string = render_with(
                    &top.to_document(&options.pretty_configuration),
                    store,
                    options,
                );
                //TODO: use the error report mechanism for this!
                eprintln!(
                    "We failed to format! Couln't parse the second time, showing original resutl:\n{}",origina_string
                );
                eprintln!("{}", string);
            }
        },
        Err(e) => {
            let request = e.build_report_request(
                options.target,
                String::from("OctizysCommandLine"),
                options.column_width,
            );
            let report = create_error_report(&request);
            let report_str = render_with(&report, store, options);
            eprintln!("We failed to format!");
            eprintln!("{}", report_str)
        }
    }
}

fn repl(
    prompt: String,
    options: &GlobalOptions,
    store: Rc<RefCell<Store>>,
) -> () {
    // TODO:  Add option to choose color
    // TODO: Add commands in repl (maybe use the larlpop parser for that!);
    let prompt_document = foreground(MODERATE_GREEN, external_text(&prompt));
    let rendered_prompt = render_with(&prompt_document, store.clone(), options);
    loop {
        let mut buffer = String::new();
        print!("{}", rendered_prompt);
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => match parse_string(&buffer, None, store.clone()) {
                Ok(top) => {
                    let doc = top.to_document(&options.pretty_configuration);
                    let string = render_with(&doc, store.clone(), options);
                    println!("{}", string);
                }
                Err(e) => {
                    let request = e.build_report_request(
                        options.target,
                        String::from("OctizysCommandLine"),
                        options.column_width,
                    );
                    let report = create_error_report(&request);
                    let report_str =
                        render_with(&report, store.clone(), options);
                    eprintln!("{}", report_str)
                }
            },
            Err(e) => {
                let octizys_error =
                    IOError::REPlCantReadLine { error: e.kind() };
                let request = octizys_error.build_report_request(
                    options.target,
                    String::from("OctizysREPL"),
                    options.column_width,
                );
                let report = create_error_report(&request);
                let report_str = render_with(&report, store.clone(), options);
                eprintln!("{}", report_str)
            }
        }
    }
}

// TODO: Set the exit code error for the shell
fn main() {
    let arguments = crate::arguments::Arguments::parse();
    let debug_level = match arguments.debug_level {
        arguments::DebugLevel::Error => simplelog::LevelFilter::Error,
        arguments::DebugLevel::Info => simplelog::LevelFilter::Info,
        arguments::DebugLevel::Debug => simplelog::LevelFilter::Debug,
        arguments::DebugLevel::Trace => simplelog::LevelFilter::Trace,
    };
    let _ = simplelog::TermLogger::init(
        debug_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
    let real_store = Store::default();
    let store = Rc::new(RefCell::new(real_store));
    if arguments.show_arguments {
        println!("{:#?}", arguments);
        return;
    }
    let options = GlobalOptions::from(arguments.formatter_configuration);

    match arguments.command {
        arguments::Commands::Compile { path, output } => {
            compile_file(path, output, &options, store)
        }
        arguments::Commands::Format { path, output } => {
            format_file(path, output, &options, store)
        }
        arguments::Commands::REPL { prompt } => repl(prompt, &options, store),
    };
}
