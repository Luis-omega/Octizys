/*use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("grammar.y")
                .unwrap()
        })
        .lexer_in_src_dir("lexer.l")
        .unwrap()
        .build()
        .unwrap();
}
*/
fn main() {
    lalrpop::process_root().unwrap();
}
