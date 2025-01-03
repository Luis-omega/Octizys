use octizys_common::equivalence::assert_equivalent;
use octizys_common::equivalence::Equivalence;
use octizys_common::report::create_error_report;
use octizys_common::span::{Position, Span};
use octizys_cst::imports::Import;
use octizys_formatter::cst::PrettyCSTConfiguration;
use octizys_formatter::to_document::ToDocument;
use octizys_parser::grammar::import_declarationParser;
use octizys_parser::lexer::{BaseLexerContext, BaseToken, LexerContext, Token};
use octizys_parser::report::OctizysParserReport;
use octizys_pretty::highlight::{
    EmptyRender, HighlightRenderer, TerminalRender24,
};
use octizys_text_store::store::Store;
use std::fmt::Debug;

use lalrpop_util::ParseError;

fn parse<T: ToDocument<PrettyCSTConfiguration> + Clone>(
    source: &str,
    parser: fn(
        LexerContext,
    )
        -> Result<T, ParseError<Position, Token, OctizysParserReport>>,
) -> (T, String) {
    let mut store = Store::default();
    let configuration = PrettyCSTConfiguration::default();
    let mut base_context = BaseLexerContext::new(source, &mut store);
    let lexer = LexerContext::new(None, &mut base_context);
    match parser(lexer) {
        Ok(x) => (
            x.clone(),
            x.to_document(&configuration).render_to_string(
                80,
                EmptyRender::render_highlight,
                &store,
            ),
        ),
        Err(e) => {
            let mut error_context = ParserErrorContext::default();
            error_context.src = &source;
            let report = create_error_report(&e, &error_context);
            let as_str = report.render_to_string(
                80,
                TerminalRender24::render_highlight,
                &store,
            );
            panic!("{}", as_str);
        }
    }
}

fn roundtrip<
    T: ToDocument<PrettyCSTConfiguration> + Equivalence + Clone + Debug,
>(
    source: &str,
    parser: fn(
        LexerContext,
    )
        -> Result<T, ParseError<Position, Token, OctizysParserReport>>,
) {
    println!("ORIGINAL:{}", source.replace("\n", "\\n"));
    let (result1, source2) = parse(source, parser);
    println!("RESULT1 :{}", source2);
    let (result2, source3) = parse(&source2, parser);
    println!("RESULT2 :{}", source3);
    assert_equivalent(&result1, &result2, TerminalRender24::render_highlight);
    assert_equivalent(&source2, &source3, TerminalRender24::render_highlight);
}

fn parse_import(
    context: LexerContext,
) -> Result<Import, ParseError<Position, Token, OctizysParserReport>> {
    let p = import_declarationParser::new();
    p.parse(context)
}

#[test]
fn simple() {
    let input = "import some::";
    roundtrip(input, parse_import)
}

#[test]
fn simple_values() {
    let input = "import some::(a,b,c,d)";
    roundtrip(input, parse_import)
}

#[test]
fn import_as() {
    let input = "import some::abcdef:: as asfd::asdf::";
    roundtrip(input, parse_import)
}

#[test]
fn import_unqualified() {
    let input = "import unqualified some::abcdef::";
    roundtrip(input, parse_import)
}

#[test]
fn import_full() {
    let input = "import unqualified some::abcdef::jkl::(er,be,wewer,iouueor,wwer)\n as j::t::k::";
    roundtrip(input, parse_import)
}
