use octizys_common::Interner;
use octizys_cst::pretty::{PrettyCST, PrettyCSTConfig};
use octizys_parser::grammar::import_declarationParser;
use octizys_parser::lexer::Lexer;
use std::fmt::Debug;
use std::io::{self, Read};

fn main() {
    let mut interner = Interner::new();
    let mut stdin = io::stdin();
    let input = &mut String::new();
    let p = import_declarationParser::new();
    let configuration = PrettyCSTConfig::new();
    loop {
        input.clear();
        stdin.read_line(input);
        input.pop();
        let mut lexer = Lexer::new(input, &mut interner);
        let parsed = p.parse(lexer);
        match parsed {
            Ok(item) => {
                println!("{:#?}", item);
                let as_doc = item.to_document(configuration);
                println!("{:?}", as_doc);
                println!(
                    "{}",
                    as_doc.render_to_string(
                        usize::from(configuration.indentation_deep),
                        &interner
                    )
                )
            }
            Err(t) => println!("Can't parse!\n{:?}", t),
        }
    }
}
