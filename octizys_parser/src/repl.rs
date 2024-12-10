use octizys_common::span::{Position, Span};
use octizys_cst::base::TokenInfo;
use octizys_formatter::cst::PrettyCSTConfiguration;
use octizys_formatter::to_document::ToDocument;
use octizys_parser::grammar::import_declarationParser;
use octizys_parser::lexer;
use octizys_parser::lexer::{
    BaseLexerContext, BaseToken, LexerContext, LexerError, Token,
};
use octizys_pretty::{
    combinators::{
        concat, empty, external_text, group, hard_break, intersperse, nest,
        repeat,
    },
    document::Document,
};
use octizys_text_store::store::{aproximate_string_width, Store};
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{self, Read};
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
//
//
//

pub fn make_error_info_start_with_span(
    error_name: &str,
    description: &str,
    src_name: Option<&str>,
    span: &Span,
) -> Document {
    let position_start_info = concat(vec![
        external_text("Line{"),
        external_text(span.start.line.to_string().as_str()),
        external_text("}"),
        external_text("::Column{"),
        external_text(span.start.column.to_string().as_str()),
        external_text("}"),
    ]);
    let position_end_info = concat(vec![
        external_text("Line{"),
        external_text(span.end.line.to_string().as_str()),
        external_text("}"),
        external_text("::Column{"),
        external_text(span.end.column.to_string().as_str()),
        external_text("}."),
    ]);
    let location_info = match src_name {
        Some(name) => concat(vec![
            external_text("-->"),
            external_text(name),
            external_text("::From::"),
            position_start_info,
            external_text("::To::"),
            position_end_info,
        ]),
        None => concat(vec![
            external_text("-->"),
            external_text("::From::"),
            position_start_info,
            external_text("::To::"),
            position_end_info,
        ]),
    };
    let info = concat(vec![
        external_text("Error!["),
        external_text(error_name),
        external_text("]: "),
        external_text(description),
        nest(1, hard_break() + location_info),
    ]);
    info
}

pub fn render_expected(expected: Option<Vec<String>>) -> Document {
    match expected {
        None => empty(),
        Some(v) => {
            let previous_text: &str;
            if v.len() == 0 {
                return empty();
            } else if v.len() == 1 {
                previous_text = "Expected: ";
            } else {
                previous_text = "Expected one of: ";
            }
            external_text(previous_text)
                + intersperse(
                    v.into_iter().map(|x| external_text(&x)),
                    external_text(", "),
                )
        }
    }
}

pub fn create_error_report_with_token(
    token: &Token,
    expected: Option<Vec<String>>,
    src: &str,
    src_name: Option<&str>,
    error_name: &str,
    error_description: &str,
) -> Document {
    let span: Span =
        <&octizys_parser::lexer::Token as Into<&TokenInfo>>::into(token).span;
    let error_preamble = make_error_info_start_with_span(
        error_name,
        error_description,
        src_name,
        &span,
    );
    let (before, content, after) = span.get_text_at(src, Some(40));
    let pre_spaces = aproximate_string_width(before);
    let spaces = " ".repeat(pre_spaces) + "^";
    //TODO:FIXME: multi line spans won't render right!
    let error_source = concat(vec![
        external_text(before),
        external_text(content),
        external_text(after),
        hard_break(),
        external_text(&spaces),
        render_expected(expected),
    ]);
    let error_long_description = empty();

    concat(vec![
        error_preamble,
        nest(4, hard_break() + error_source),
        nest(2, hard_break() + error_long_description),
    ])
}

pub fn create_error_report_with_position(
    position: &Position,
    expected: Option<Vec<String>>,
    src: &str,
    src_name: Option<&str>,
    error_name: &str,
    error_description: &str,
) -> Document {
    let error_preamble = lexer::make_error_info_start(
        error_name,
        error_description,
        src_name,
        position,
    );
    let (before, after) = position.get_text_at(src, Some(40));
    let pre_spaces = aproximate_string_width(before);
    let spaces = " ".repeat(pre_spaces) + "^";
    let error_source = concat(vec![
        external_text(before),
        external_text(after),
        hard_break(),
        external_text(&spaces),
        render_expected(expected),
    ]);
    let error_long_description = empty();

    concat(vec![
        error_preamble,
        nest(4, hard_break() + error_source),
        nest(2, hard_break() + error_long_description),
    ])
}

fn main() {
    let mut store = Store::default();
    let configuration = PrettyCSTConfiguration::default();
    let mut stdin = io::stdin();
    let input = &mut String::new();
    let p = import_declarationParser::new();
    loop {
        input.clear();
        stdin.read_line(input);
        input.pop();
        if input == ":q" {
            break;
        }
        let mut base_context = BaseLexerContext::new(input, &mut store);
        let iterator = LexerContext::new(None, &mut base_context);
        let parsed = p.parse(iterator.map(|x| {
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
                //println!("{:?}", as_doc);
                println!("{}", as_doc.render_to_string(2, &store))
            }
            Err(t) => match t {
                lalrpop_util::ParseError::InvalidToken { location } => {
                    println!(
                        "{}",
                        create_error_report_with_position(
                            &location,
                            None,
                            &input,
                            Some("octizys_repl"),
                            "UnrecognizedEof",
                            "Expected more code to follow"
                        )
                        .render_to_string(2, &store)
                    )
                }
                lalrpop_util::ParseError::UnrecognizedEof {
                    location,
                    expected,
                } => println!(
                    "{}",
                    create_error_report_with_position(
                        &location,
                        Some(expected),
                        &input,
                        Some("octizys_repl"),
                        "UnrecognizedEof",
                        "Expected more code to follow"
                    )
                    .render_to_string(2, &store)
                ),
                lalrpop_util::ParseError::UnrecognizedToken {
                    token: (_, token, _),
                    expected,
                } => println!(
                    "{}",
                    create_error_report_with_token(
                        &token,
                        Some(expected),
                        &input,
                        Some("octizys_repl"),
                        "UnexpectedToken",
                        "There's a syntax error in the provided code",
                    )
                    .render_to_string(2, &store)
                ),
                lalrpop_util::ParseError::ExtraToken {
                    token: (_, token, _),
                } => println!(
                    "{}",
                    create_error_report_with_token(
                        &token,
                        None,
                        &input,
                        Some("octizys_repl"),
                        "ExtraToken",
                        "There's more code than what we expeted!",
                    )
                    .render_to_string(2, &store)
                ),
                lalrpop_util::ParseError::User { error } => println!(
                    "{}",
                    error
                        .to_document(&input, Some("octizys_repl"))
                        .render_to_string(2, &store)
                ),
                _ => println!("Can't parse!\n{:?}", t),
            },
        }
    }
}
