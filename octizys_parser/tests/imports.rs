use octizys_formatter::cst::PrettyCSTConfiguration;
use octizys_formatter::to_document::ToDocument;
use octizys_parser::grammar::import_declarationParser;
//use octizys_parser::lexer::Lexer;
use octizys_text_store::store::Store;
use std::fmt::Debug;

//fn roundtrip<
//    T: ToDocument<PrettyCSTConfiguration> + Eq + Debug + Clone,
//    E: Debug,
//>(
//    parser: impl Fn(Lexer) -> Result<T, E>,
//    source: &str,
//) {
//    let mut store = Store::default();
//    let mut lexer = Lexer::new(source, &mut store);
//    let parsed = parser(lexer).unwrap();
//    let printed = parsed
//        .to_document(&PrettyCSTConfiguration::default())
//        .render_to_string(80, &store);
//    let mut lexer2 = Lexer::new(&printed, &mut store);
//    let parsed2 = parser(lexer2).unwrap();
//    let printed2 = parsed2
//        .to_document(&PrettyCSTConfiguration::default())
//        .render_to_string(80, &store);
//    assert_eq!(parsed, parsed2);
//    assert_eq!(printed, printed2);
//    assert_eq!(source, printed);
//}
//
//#[test]
//fn imports() {
//    let input = "import some::abcdef as asfd::asdf::ss";
//    let p = import_declarationParser::new();
//    roundtrip(|x| p.parse(x), input);
//}
