use logos::Logos;
use octizys_common::span::Position;
use octizys_cst::base::TokenInfo;
use octizys_formatter::cst::PrettyCSTConfiguration;
use octizys_formatter::to_document::ToDocument;
use octizys_parser::grammar::import_declarationParser;
use octizys_parser::lexer::{
    LexerContext, LexerError, LogosLexerContext, LogosToken, Token,
};
use octizys_pretty::combinators::group;
use octizys_text_store::store::Store;
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

fn main() {
    let store = Rc::new(RefCell::new(Store::default()));
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
        let mut logos_context = RefCell::new(LogosLexerContext::new(
            store.clone(),
            Position::default(),
        ));
        let mut lexer = LogosToken::lexer_with_extras(
            input.as_str(),
            logos_context.borrow_mut().clone(),
        )
        .spanned();
        let iterator = LexerContext::new(input.as_str(), None, &mut lexer);
        let parsed = p.parse(iterator.map(|x| {
            println!("{:?}", x);
            x
        }));
        match parsed {
            Ok(item) => {
                println!("{:#?}", item);
                let as_doc = group(item.to_document(&configuration));
                println!("{:?}", as_doc);
                println!("{}", as_doc.render_to_string(2, &((*store).borrow())))
            }
            Err(t) => println!("Can't parse!\n{:?}", t),
        }
    }
}
