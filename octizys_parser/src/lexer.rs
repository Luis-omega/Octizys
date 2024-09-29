use lalrpop_util::ParseError;
use octizys_common::identifier::Identifier;
use octizys_common::module_logic_path::ModuleLogicPath;
use std::str::CharIndices;
use std::{collections::VecDeque, rc::Rc};

use octizys_cst::cst::{
    self, Comment, CommentBlock, CommentKind, CommentLine, CommentLineContent,
    CommentsInfo, LineCommentStart, OperatorName, Position, Span, TokenInfo,
};

use paste::paste;

#[derive(Debug, PartialEq, Eq)]
pub enum LexerErrorType {
    UnexpectedCharacter,
    UnbalancedComment,
    CantTranslateToToken(Token),
    InvaliedSequenceOfCharacters(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexerError {
    pub error_type: LexerErrorType,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Comma(TokenInfo),
    Colon(TokenInfo),
    StatementEnd(TokenInfo),
    FieldAccessor(TokenInfo),
    ModuleSeparator(TokenInfo),
    CaseSeparator(TokenInfo),
    LParen(TokenInfo),
    RParen(TokenInfo),
    LBracket(TokenInfo),
    RBracket(TokenInfo),
    LBrace(TokenInfo),
    RBrace(TokenInfo),
    RightArrow(TokenInfo),
    LeftArrow(TokenInfo),
    Import(TokenInfo),
    Export(TokenInfo),
    Data(TokenInfo),
    Newtype(TokenInfo),
    Alias(TokenInfo),
    As(TokenInfo),
    Unqualified(TokenInfo),
    Forall(TokenInfo),
    Type(TokenInfo),
    Bool(TokenInfo),
    True(TokenInfo),
    False(TokenInfo),
    Unit(TokenInfo),
    U8(TokenInfo),
    U16(TokenInfo),
    U32(TokenInfo),
    U64(TokenInfo),
    I8(TokenInfo),
    I16(TokenInfo),
    I32(TokenInfo),
    I64(TokenInfo),
    F32(TokenInfo),
    F64(TokenInfo),
    LastComments(Vec<Comment>),
    StringLiteral(TokenInfo, String),
    UintLiteral(TokenInfo, String),
    UFloatLiteral(TokenInfo, String),
    Identifier(TokenInfo, Identifier),
    OperatorName(TokenInfo, OperatorName),
    ModuleLogicPath(TokenInfo, ModuleLogicPath),
}

impl From<Token> for TokenInfo {
    fn from(value: Token) -> TokenInfo {
        match value {
            Token::Comma(info) => info,
            Token::Colon(info) => info,
            Token::StatementEnd(info) => info,
            Token::FieldAccessor(info) => info,
            Token::ModuleSeparator(info) => info,
            Token::CaseSeparator(info) => info,
            Token::LParen(info) => info,
            Token::RParen(info) => info,
            Token::LBracket(info) => info,
            Token::RBracket(info) => info,
            Token::LBrace(info) => info,
            Token::RBrace(info) => info,
            Token::RightArrow(info) => info,
            Token::LeftArrow(info) => info,
            Token::Import(info) => info,
            Token::Export(info) => info,
            Token::Data(info) => info,
            Token::Newtype(info) => info,
            Token::Alias(info) => info,
            Token::As(info) => info,
            Token::Unqualified(info) => info,
            Token::Forall(info) => info,
            Token::Type(info) => info,
            Token::Bool(info) => info,
            Token::True(info) => info,
            Token::False(info) => info,
            Token::Unit(info) => info,
            Token::U8(info) => info,
            Token::U16(info) => info,
            Token::U32(info) => info,
            Token::U64(info) => info,
            Token::I8(info) => info,
            Token::I16(info) => info,
            Token::I32(info) => info,
            Token::I64(info) => info,
            Token::F32(info) => info,
            Token::F64(info) => info,
            //TODO: if there are comments, use the span of the last one
            Token::LastComments(comments) => TokenInfo {
                comments: CommentsInfo::empty(),
                span: (usize::MAX, usize::MAX).into(),
            },
            Token::StringLiteral(info, _) => info,
            Token::UintLiteral(info, _) => info,
            Token::UFloatLiteral(info, _) => info,
            Token::Identifier(info, _) => info,
            Token::OperatorName(info, _) => info,
            Token::ModuleLogicPath(info, _) => info,
        }
    }
}

macro_rules! make_lexer_token_to_token {
    ($name:tt, $output:tt) => {
        paste!{
            pub fn [< $name _token_to_token >](t:Token)->Result<cst::Token<$output>,ParseError<usize,Token,LexerError>>{
                match t {
                    Token::$output(info,value) => Ok(cst::Token{value,info}),
                    _ => Err(ParseError::User{error: LexerError{error_type:LexerErrorType::CantTranslateToToken(t.clone()),position:{let t2 : TokenInfo =t.into(); t2.span.start}} })
                }
            }
        }

    };
}

make_lexer_token_to_token!(module, ModuleLogicPath);
make_lexer_token_to_token!(identifier, Identifier);
make_lexer_token_to_token!(operator, OperatorName);

//TODO: Make those UTF8.

fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

fn is_letter(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

fn is_underscore(c: char) -> bool {
    c == '_'
}

// Non linebreak spaces
fn is_simple_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn is_linebreak(c: char) -> bool {
    c == '\n' || c == '\r'
}

// All kind of blanks
fn is_space(c: char) -> bool {
    is_simple_space(c) || is_linebreak(c)
}

fn is_identifier_start(c: char) -> bool {
    is_underscore(c) || is_letter(c)
}

fn is_identifier_body(c: char) -> bool {
    is_identifier_start(c) || is_digit(c)
}

fn is_operator_start(c: char) -> bool {
    match c {
        '+' | '*' | '/' | '^' | '%' | '=' | '&' | '~' | '#' | '!' | '$'
        | '?' => true,
        _ => false,
    }
}

fn is_operator_body(c: char) -> bool {
    is_operator_start(c) || c == ','
}

fn match_keyword(s: &str, info: TokenInfo) -> Option<Token> {
    return match s {
        "import" => Some(Token::Import(info)),
        "export" => Some(Token::Export(info)),
        "data" => Some(Token::Data(info)),
        "newtype" => Some(Token::Newtype(info)),
        "alias" => Some(Token::Alias(info)),
        "as" => Some(Token::As(info)),
        "unqualified" => Some(Token::Unqualified(info)),
        "forall" => Some(Token::Forall(info)),
        "type" => Some(Token::Type(info)),
        "Bool" => Some(Token::Bool(info)),
        "true" => Some(Token::True(info)),
        "false" => Some(Token::False(info)),
        "Unit" => Some(Token::Unit(info)),
        "U8" => Some(Token::U8(info)),
        "U16" => Some(Token::U16(info)),
        "U32" => Some(Token::U32(info)),
        "U64" => Some(Token::U64(info)),
        "I8" => Some(Token::I8(info)),
        "I16" => Some(Token::I16(info)),
        "I32" => Some(Token::I32(info)),
        "I64" => Some(Token::I64(info)),
        "F32" => Some(Token::F32(info)),
        "F64" => Some(Token::F64(info)),
        _ => None,
    };
}

#[derive(Debug)]
pub struct Lexer<'input> {
    src: &'input str,
    chars: CharIndices<'input>,
    current: Option<(usize, char)>,
    look_ahead: VecDeque<(usize, char)>,
}

impl<'input> Lexer<'input> {
    pub fn new(src: &'input str) -> Self {
        let mut chars = src.char_indices();
        let current = chars.next();
        let look_ahead = VecDeque::new();

        Lexer {
            src,
            chars,
            current,
            look_ahead,
        }
    }

    pub fn advance(&mut self) -> Option<(usize, char)> {
        match self.look_ahead.pop_front() {
            Some(current) => {
                self.current = Some(current);
            }
            None => {
                self.current = self.chars.next();
            }
        }
        let out = self.current;
        println!("advance, new: {:?}", out);
        out
    }

    pub fn restore(&mut self, acc: Vec<(usize, char)>) -> () {
        self.look_ahead.extend(acc);
    }

    pub fn satisfy(
        &mut self,
        predicate: fn(char) -> bool,
    ) -> Option<(usize, char)> {
        let (pos, c) = self.current?;
        if predicate(c) {
            Some((pos, c))
        } else {
            None
        }
    }

    //If the stream is at the end, return None.
    //If the first item don't satisfy the predicate, return None.
    //Otherwise return the start and final position in stream.
    pub fn take_while(
        &mut self,
        acc: &mut Vec<(usize, char)>,
        predicate: fn(char) -> bool,
    ) -> Option<(usize, usize)> {
        let (start_pos, initial_char) = self.satisfy(predicate)?;
        let mut end_pos = start_pos;
        acc.push((start_pos, initial_char));
        self.advance();
        loop {
            match self.satisfy(predicate) {
                Some((new_pos, c)) => {
                    acc.push((new_pos, c));
                    end_pos = new_pos;
                    self.advance();
                    continue;
                }
                None => return Some((start_pos, end_pos)),
            }
        }
    }
    pub fn discart_while(&mut self, predicate: fn(char) -> bool) {
        match self.current {
            Some((_, c)) => {
                if !predicate(c) {
                    return;
                } else {
                    ()
                }
            }
            None => return,
        }
        loop {
            match self.advance() {
                Some((_, c)) => {
                    if !predicate(c) {
                        return;
                    } else {
                        ()
                    }
                }
                None => return,
            }
        }
    }

    pub fn simple_spaces(&mut self) -> () {
        self.discart_while(is_simple_space)
    }

    pub fn all_spaces(&mut self) -> () {
        self.discart_while(is_space)
    }

    pub fn take_until_linebreak(&mut self) -> (Option<Span>, String) {
        println!("linebreak");
        let mut acc: Vec<(usize, char)> = vec![];
        self.advance();
        let span = self
            .take_while(&mut acc, |c| !is_linebreak(c))
            .map(Span::from);

        (span, acc.into_iter().map(|(x, y)| y).collect())
    }

    pub fn single_line_comment_aux(
        &mut self,
        comment_start: LineCommentStart,
    ) -> Option<CommentLine> {
        let (pos1, first) = self.current?;
        let expected_start = char::from(comment_start);
        println!("first: '{:?}' , expected '{:?}'", first, expected_start);
        if first == expected_start {
            self.advance();
            let (pos2, second) = self.current?;
            if second == expected_start {
                let (maybe_span, text) = self.take_until_linebreak();
                let mut span = maybe_span.unwrap_or(Span::from((pos1, pos2)));
                let (kind, trimed_text) = match &(text[0..2]) {
                    " |" => {
                        span.start.source_index += " |".len();
                        (CommentKind::Documentation, &text[2..])
                    }
                    _ => (CommentKind::NonDocumentation, &text[0..]),
                };
                let content =
                    CommentLineContent::decompose(&trimed_text)[0].clone();
                Some(CommentLine {
                    kind,
                    start: comment_start,
                    content,
                    span,
                })
            } else {
                self.restore(vec![(pos1, first)]);
                None
            }
        } else {
            None
        }
    }

    pub fn single_line_comment(&mut self) -> Option<CommentLine> {
        self.single_line_comment_aux(LineCommentStart::DoubleHypen)
            .or_else(|| {
                self.single_line_comment_aux(LineCommentStart::DoubleSlash)
            })
    }

    pub fn comment_block(&mut self) -> Option<CommentBlock> {
        //todo!();
        None
    }

    pub fn any_comment(&mut self) -> Option<Comment> {
        let comment = self
            .single_line_comment()
            .map(Comment::from)
            .or_else(|| self.comment_block().map(Comment::from));
        self.all_spaces();
        comment
    }

    pub fn before_comments(&mut self) -> Vec<Comment> {
        self.all_spaces();
        let mut acc = vec![];
        loop {
            match self.any_comment() {
                Some(comment) => acc.push(comment),
                None => break,
            }
        }
        self.all_spaces();
        acc
    }

    pub fn after_comments(&mut self) -> Option<Comment> {
        self.simple_spaces();
        let comment = self.single_line_comment();
        self.all_spaces();
        comment.map(Comment::from)
    }

    pub fn lex_identifier_or_hole_or_keyword(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Token> {
        let (start_pos, initial_c) = self.satisfy(is_identifier_start)?;
        self.advance();
        let mut acc = vec![(start_pos, initial_c)];
        println!("before");
        let end_pos = match self.take_while(&mut acc, is_identifier_body) {
            Some((_, end_pos)) => end_pos,
            None => start_pos,
        };
        println!("iden {:?}", acc);
        let after = self.after_comments();
        let comments = cst::CommentsInfo { before, after };
        let info = TokenInfo::make(comments, start_pos, end_pos);
        let text: String = acc.into_iter().map(|(a, b)| b).collect();
        println!("identifier: {:?}", text);
        match match_keyword(&text, info.clone()) {
            Some(tok) => Some(tok),
            None => match Identifier::make(&text) {
                Ok(identifier) => Some(Token::Identifier(info, identifier)),
                //TODO: implement holes
                Err(_) => None,
            },
        }
    }

    pub fn lex_uint_or_float(&mut self, before: Vec<Comment>) -> Option<Token> {
        println!("uint: {:?}", self.current);
        None
        //let after = self.after_comments();
        //let comments = cst::CommentsInfo { before, after };
        //let info = TokenInfo::make(comments, start_pos, end_pos);
        //let text: String = acc.into_iter().collect();
        //todo!();
    }

    pub fn lex_symbol_or_operator(
        &mut self,
        before: Vec<Comment>,
    ) -> Option<Token> {
        println!("operator: {:?}", self.current);
        None
        //let after = self.after_comments();
        //let comments = cst::CommentsInfo { before, after };
        //let info = TokenInfo::make(comments, start_pos, end_pos);
        //let text: String = acc.into_iter().collect();
        //todo!()
    }

    pub fn one_token(&mut self) -> Result<Token, Vec<Comment>> {
        let before = self.before_comments();
        //TODO: is there a way to remove the clone of before?
        //well, it can be if we return a function that consumes
        //a before and construct a token instead of the token..
        let out = self
            .lex_symbol_or_operator(before.clone())
            .or_else(|| self.lex_uint_or_float(before.clone()))
            .or_else(|| self.lex_identifier_or_hole_or_keyword(before.clone()))
            .ok_or(before);
        println!("out : {:?}", out);
        out
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(Position, Token, Position), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        };
        match self.one_token() {
            Ok(token) => {
                let info = TokenInfo::from(token.clone());
                let out = Some(Ok((info.span.start, token, info.span.start)));
                return out;
            }
            //TODO: Inject last comment as a token here
            Err(comments) => {
                let token = Token::LastComments(comments);
                let s = TokenInfo::from(token.clone()).span;
                return Some(Ok((s.start, token, s.end)));
            }
        }
    }
}

#[cfg(test)]
mod lexer_tests {

    use octizys_pretty::types::NoLineBreaksString;

    use crate::lexer::*;

    //fn generic_test_by_spaces(s: &'static str) {
    //    let lex = Lexer::new(s);
    //    let result: Vec<Result<Token, LexerError>> =
    //        lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
    //    let splitted = s.split(" ").map().collect();
    //    assert!(result == vec![])
    //}

    #[test]
    fn empty_string() {
        let input = "";
        let lex = Lexer::new(&input);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        assert!(result == vec![])
    }

    #[test]
    fn identifier() {
        let s = "helloWorld";
        let lex = Lexer::new(s);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let identifier = Identifier::make(s).unwrap();
        let info = TokenInfo {
            comments: CommentsInfo {
                before: vec![],
                after: None,
            },
            span: (0, 9).into(),
        };
        let token = Token::Identifier(info, identifier);
        let expected = vec![Ok(token)];
        println!("result: {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected);
    }

    fn make_keyword_test(
        key: &'static str,
        constructor: fn(TokenInfo) -> Token,
    ) {
        println!("beginging: ");
        let lex = Lexer::new(key);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let info = TokenInfo {
            comments: CommentsInfo {
                before: vec![],
                after: None,
            },
            span: (0, key.len() - 1).into(),
        };
        let token = constructor(info.clone());
        let expected = vec![Ok(token)];
        println!("result: {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected);
    }

    macro_rules! make_keyword_test_macro {
        ($macro_name:tt, $name:tt,$constructor_name:tt) => {
            paste! {
                #[test]
                fn [< keword_ $macro_name _test >](){
                    make_keyword_test($name,Token::$constructor_name)
                }
            }
        };
    }

    make_keyword_test_macro!("import", "import", Import);
    make_keyword_test_macro!("export", "export", Export);
    make_keyword_test_macro!("data", "data", Data);
    make_keyword_test_macro!("newtype", "newtype", Newtype);
    make_keyword_test_macro!("alias", "alias", Alias);
    make_keyword_test_macro!("as", "as", As);
    make_keyword_test_macro!("unqualified", "unqualified", Unqualified);
    make_keyword_test_macro!("forall", "forall", Forall);
    make_keyword_test_macro!("type", "type", Type);
    make_keyword_test_macro!("bool", "Bool", Bool);
    make_keyword_test_macro!("true", "true", True);
    make_keyword_test_macro!("false", "false", False);
    make_keyword_test_macro!("unit", "Unit", Unit);
    make_keyword_test_macro!("u8", "U8", U8);
    make_keyword_test_macro!("u16", "U16", U16);
    make_keyword_test_macro!("u32", "U32", U32);
    make_keyword_test_macro!("u64", "U64", U64);
    make_keyword_test_macro!("i8", "I8", I8);
    make_keyword_test_macro!("i16", "I16", I16);
    make_keyword_test_macro!("i32", "I32", I32);
    make_keyword_test_macro!("i64", "I64", I64);
    make_keyword_test_macro!("f32", "F32", F32);
    make_keyword_test_macro!("f64", "F64", F64);

    fn make_line_comment_documentation_test(
        content_string: &'static str,
        kind: CommentKind,
        start: LineCommentStart,
    ) {
        let start_str: &'static str = start.into();
        let trailing_string: String = vec![start_str, kind.into()].join("");
        let start_gap = trailing_string.len();
        let raw_string: String = trailing_string.clone() + content_string;
        let lex = Lexer::new(&raw_string);
        let result: Vec<Result<Token, LexerError>> =
            lex.into_iter().map(|x| x.map(|(_, y, _)| y)).collect();
        let content = CommentLineContent {
            content: NoLineBreaksString::make(content_string).unwrap(),
        };
        let comment_line = CommentLine {
            kind,
            start,
            content,
            // -1 here as the len give us a bigger index
            span: (start_gap, start_gap + content_string.len() - 1).into(),
        };
        let token = Token::LastComments(vec![comment_line.into()]);
        let expected = vec![Ok(token)];
        println!("result:   {:?}", result);
        println!("expected: {:?}", expected);
        assert!(result == expected)
    }

    #[test]
    fn line_comment_hypen() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#' commenting things",
            CommentKind::NonDocumentation,
            LineCommentStart::DoubleHypen,
        )
    }

    #[test]
    fn line_comment_slash() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#'  commenting things",
            CommentKind::NonDocumentation,
            LineCommentStart::DoubleSlash,
        )
    }

    #[test]
    fn line_documentation_hypen() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#'  commenting things",
            CommentKind::Documentation,
            LineCommentStart::DoubleHypen,
        )
    }

    #[test]
    fn line_documentation_slash() {
        make_line_comment_documentation_test(
            " some string ra>msf-- asfer832*cvssdfs=were#'  commenting things",
            CommentKind::Documentation,
            LineCommentStart::DoubleSlash,
        )
    }
}
