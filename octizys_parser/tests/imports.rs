use octizys_common::Interner;
use octizys_cst::pretty::{PrettyCST, PrettyCSTConfig};
use octizys_parser::grammar::import_declarationParser;
use octizys_parser::lexer::Lexer;
use std::fmt::Debug;

fn roundtrip<T: PrettyCST + Eq + Debug + Clone, E: Debug>(
    parser: impl Fn(Lexer) -> Result<T, E>,
    source: &str,
) {
    let mut interner = Interner::new();
    let mut lexer = Lexer::new(source, &mut interner);
    let parsed = parser(lexer).unwrap();
    let printed = parsed
        .to_document(PrettyCSTConfig::new())
        .render_to_string(80, &interner);
    let mut lexer2 = Lexer::new(&printed, &mut interner);
    let parsed2 = parser(lexer2).unwrap();
    let printed2 = parsed2
        .to_document(PrettyCSTConfig::new())
        .render_to_string(80, &interner);
    assert_eq!(parsed, parsed2);
    assert_eq!(printed, printed2);
    assert_eq!(source, printed);
}

#[test]
fn imports() {
    let input = "import some::abcdef";
    let p = import_declarationParser::new();
    roundtrip(|x| p.parse(x), input);
}
