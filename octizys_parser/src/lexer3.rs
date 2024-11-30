use logos::{Lexer, Logos, Source, SpannedIter};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r]+")]
pub enum Token {
    #[regex("a+", |x| String::from(x.slice()),priority=40)]
    Ident(String),
    #[regex("(a+::)+", |x| String::from(x.slice()))]
    Path(String),
}

#[cfg(test)]
mod t_logos {
    use super::*;
    use logos::Logos;

    #[test]
    fn main() {
        let input = "a::a";

        let mut lexer = Token::lexer(input);
        while let Some(token) = lexer.next() {
            println!("{:?}", token);
            println!(
                "lexer_span:{0:?},lexer_slice:{1:?}",
                lexer.span(),
                lexer.slice()
            )
        }
        panic!("fails");
    }
}
