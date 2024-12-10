use octizys_common::span::Position;
use octizys_cst::base::TokenInfo;
use octizys_formatter::cst::PrettyCSTConfiguration;
use octizys_formatter::to_document::ToDocument;
use octizys_parser::grammar::import_declarationParser;
use octizys_parser::lexer::{
    BaseLexerContext, BaseToken, LexerContext, LexerError, Token,
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
            Err(t) => println!("Can't parse!\n{:?}", t),
        }
    }
}
