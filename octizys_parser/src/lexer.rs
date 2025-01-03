use octizys_common::{
    equivalence::Equivalence,
    identifier::Identifier,
    logic_path::LogicPath,
    report::{ReportFormat, ReportKind, ReportTarget},
    span::{HasLocation, Location, Position, Span},
};
use octizys_cst::{
    base::{TokenInfo, TokenInfoWithPhantom},
    comments::{
        Comment, CommentBlock, CommentBraceKind, CommentKind, CommentLine,
        CommentLineContent, CommentsInfo, LineCommentStart,
    },
    expressions::Let,
    literals::{
        InterpolationString, StringLiteral, UFloatingPointLiteral, UintKind,
        UintLiteral,
    },
    types::{OwnershipLiteral, OwnershipVariable},
};
use octizys_macros::Equivalence;
use octizys_pretty::{
    combinators::{concat, empty, external_text, hard_break, nest, repeat},
    document::Document,
    store::NonLineBreakStr,
};
use octizys_text_store::store::{approximate_string_width, Store};

use regex::{Captures, Match, Regex};
use std::{
    borrow::BorrowMut, cell::RefCell, num::ParseIntError, rc::Rc,
    sync::LazyLock,
};

use crate::{
    report::{LexerReportKind, OctizysParserReport, ParserReport},
    tokens::aux_base_token_to_token,
};

pub use crate::tokens::{BaseToken, Token};

/// An abstraction for a [`Stream`] of characters over a [`str`].
#[derive(Debug)]
pub struct BaseLexerContext<'source> {
    /// A pointer to the original source
    source: &'source str,
    /// A pointer to the remain of the source
    index: &'source str,
    /// The source_index is the bytes position, not the number of chars!
    /// or the graphemes.
    /// The column line is the amount of bytes up to the point, not
    /// the amount of chars or graphemes.
    position: Position,
    /// Saves the line of the last token
    last_line: usize,
    /// A storage for internalization of strings.
    store: Rc<RefCell<Store>>,
}

fn make_error_report_with_span<T>(
    lexer_kind: LexerReportKind,
    s: Span,
) -> Option<Result<T, OctizysParserReport>> {
    Some(Err(OctizysParserReport {
        kind: ReportKind::Error,
        report: ParserReport::Lexer(lexer_kind),
        location: Location::Span(s),
    }))
}

fn make_error_report_with_position<T>(
    lexer_kind: LexerReportKind,
    p: Position,
) -> Option<Result<T, OctizysParserReport>> {
    Some(Err(OctizysParserReport {
        kind: ReportKind::Error,
        report: ParserReport::Lexer(lexer_kind),
        location: Location::Position(p),
    }))
}

fn make_block_comment(
    re: &Regex,
    context: &mut BaseLexerContext,
    m: Match,
    brace_kind: CommentBraceKind,
) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
    let matched = m.as_str();
    match re.captures(&context.index) {
        Some(c) => {
            let kind = match c.name("doc") {
                Some(_) => CommentKind::Documentation,
                None => CommentKind::NonDocumentation,
            };
            let span =
                context.advance_with_line_breaks(c.get(0).unwrap().as_str());
            match c.name("content") {
                Some(content) => {
                    let comment = CommentBlock::make(
                        kind,
                        brace_kind,
                        content.as_str(),
                        span.start,
                        span.end,
                        &mut *(*context.store).borrow_mut(),
                    );
                    Some(Ok((span, BaseToken::BlockComment(comment))))
                }
                //TODO: This is most likely for a non closed comment, we may look for possible
                //closings of the comment, store that in the info?
                None => make_error_report_with_span(
                    LexerReportKind::NonFinishedLineComment,
                    span,
                ),
            }
        }
        None => {
            let span = context.advance_non_line_breaks(matched);
            make_error_report_with_span(
                LexerReportKind::CouldntMatchBlockComment(brace_kind),
                span,
            )
        }
    }
}

fn make_line_comment(
    re: &Regex,
    context: &mut BaseLexerContext,
    m: Match,
    line_start: LineCommentStart,
) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
    let matched = m.as_str();
    match SLASH_COMMENT.captures(&context.index) {
        Some(c) => {
            let kind = match c.name("doc") {
                Some(_) => CommentKind::Documentation,
                None => CommentKind::NonDocumentation,
            };
            let span =
                context.advance_with_line_breaks(c.get(0).unwrap().as_str());
            match c.name("content") {
                Some(content) => {
                    let maybe_content = CommentLineContent::make_register(
                        content.as_str(),
                        &mut *(*context.store).borrow_mut(),
                    );
                    match maybe_content {
                        Some(internalized_content) => Some(Ok((
                            span,
                            BaseToken::LineComment(CommentLine {
                                kind,
                                start: line_start,
                                content: internalized_content,
                                span,
                            }),
                        ))),
                        None => make_error_report_with_span(
                            LexerReportKind::CantCreateCommentLine,
                            span,
                        ),
                    }
                }
                None => make_error_report_with_span(
                    LexerReportKind::NonFinishedLineComment,
                    span,
                ),
            }
        }
        None => {
            let span = context.advance_non_line_breaks(matched);
            make_error_report_with_span(
                LexerReportKind::NonFinishedLineComment,
                span,
            )
        }
    }
}

impl<'store, 'source> BaseLexerContext<'source> {
    //TODO: make this function capable to start everywhere in the source
    pub fn new(source: &'source str, store: Rc<RefCell<Store>>) -> Self {
        BaseLexerContext {
            source,
            index: source,
            position: Position::default(),
            last_line: 0,
            store,
        }
    }

    fn advance_with_line_breaks(&mut self, s: &str) -> Span {
        //println!("Advancing spaces! {s},size={:}", s.len());
        let start = self.position;
        let len = s.len();
        self.position.source_index = self.position.source_index + len;
        for c in s.chars() {
            match c {
                '\n' => {
                    self.position.line += 1;
                    self.position.column = 0;
                }
                _ => self.position.column += c.len_utf8(),
            }
        }
        self.index = &self.index[len..];
        Span {
            start,
            end: self.position,
        }
    }

    fn advance_non_line_breaks(&mut self, s: &str) -> Span {
        //println!("Advancing! {:}", s);
        let start = self.position;
        let len = s.len();
        self.position.column = self.position.column + len;
        self.position.source_index = self.position.source_index + len;
        self.index = &self.index[len..];
        Span {
            start,
            end: self.position,
        }
    }

    fn consume_spaces(&mut self) -> Span {
        match SPACES_REGEX.find(&self.index) {
            Some(spaces) => self.advance_with_line_breaks(spaces.as_str()),
            None => Span::from((self.position, self.position)),
        }
    }

    fn comment(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        match matched {
            "--" => make_line_comment(
                &*HYPHEN_COMMENT,
                self,
                m,
                octizys_cst::comments::LineCommentStart::DoubleHyphen,
            ),
            "//" => make_line_comment(
                &*SLASH_COMMENT,
                self,
                m,
                octizys_cst::comments::LineCommentStart::DoubleSlash,
            ),
            "{-" => make_block_comment(
                &*COMMENT_BLOCK0,
                self,
                m,
                CommentBraceKind::Brace0,
            ),
            "{--" => make_block_comment(
                &*COMMENT_BLOCK1,
                self,
                m,
                CommentBraceKind::Brace1,
            ),
            "{---" => make_block_comment(
                &*COMMENT_BLOCK2,
                self,
                m,
                CommentBraceKind::Brace2,
            ),
            "{----" => make_block_comment(
                &*COMMENT_BLOCK3,
                self,
                m,
                CommentBraceKind::Brace3,
            ),
            _ => {
                let span = self.advance_non_line_breaks(matched);
                make_error_report_with_span(
                    LexerReportKind::UnexpectedCommentMatch,
                    span,
                )
            }
        }
    }

    fn punctuation_or_operator(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        Some(match matched {
            "\\" => Ok((span, BaseToken::LambdaStart)),
            "/" => Ok((span, BaseToken::Div)),
            "#" => Ok((span, BaseToken::Hash)),
            "," => Ok((span, BaseToken::Comma)),
            ";" => Ok((span, BaseToken::StatementEnd)),
            "+" => Ok((span, BaseToken::Plus)),
            "^" => Ok((span, BaseToken::Power)),
            "%" => Ok((span, BaseToken::Module)),
            "@" => Ok((span, BaseToken::At)),
            "." => Ok((span, BaseToken::Dot)),
            ":" => Ok((span, BaseToken::Colon)),
            "::" => Ok((span, BaseToken::LogicPathSeparator)),
            "-" => Ok((span, BaseToken::Minus)),
            "->" => Ok((span, BaseToken::RightArrow)),
            "|" => Ok((span, BaseToken::Pipe)),
            "<|" => Ok((span, BaseToken::CompositionRight)),
            "<|>" => Ok((span, BaseToken::Alternative)),
            "<=" => Ok((span, BaseToken::LessOrEqual)),
            "<-" => Ok((span, BaseToken::LeftArrow)),
            "<$" => Ok((span, BaseToken::MapConstLeft)),
            "<$>" => Ok((span, BaseToken::Map)),
            "<*" => Ok((span, BaseToken::ApplicativeLeft)),
            "<<" => Ok((span, BaseToken::ShiftLeft)),
            "<*>" => Ok((span, BaseToken::Appliative)),
            "<&>" => Ok((span, BaseToken::FlippedMap)),
            "<?>" => Ok((span, BaseToken::Annotate)),
            "*" => Ok((span, BaseToken::Star)),
            "*>" => Ok((span, BaseToken::ApplicativeRight)),
            "=" => Ok((span, BaseToken::Asignation)),
            "==" => Ok((span, BaseToken::Equality)),
            "!" => Ok((span, BaseToken::Exclamation)),
            "!=" => Ok((span, BaseToken::NotEqual)),
            ">" => Ok((span, BaseToken::MoreThan)),
            ">>" => Ok((span, BaseToken::ShiftRight)),
            ">=" => Ok((span, BaseToken::MoreOrEqual)),
            "&&" => Ok((span, BaseToken::And)),
            "&" => Ok((span, BaseToken::ReverseApplication)),
            "$" => Ok((span, BaseToken::DollarApplication)),
            "$>" => Ok((span, BaseToken::MapConstRight)),
            _ => Err(OctizysParserReport {
                kind: ReportKind::Error,
                report: ParserReport::Lexer(
                    LexerReportKind::UnexpectedPunctuationMatch,
                ),
                location: Location::Span(span),
            }),
        })
    }

    fn bracket_start(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched {
            "(" => Some(Ok((span, BaseToken::LParen))),
            "[" => Some(Ok((span, BaseToken::LBracket))),
            "{" => Some(Ok((span, BaseToken::LBrace))),
            _ => make_error_report_with_span(
                LexerReportKind::UnexpectedPunctuationMatch,
                span,
            ),
        }
    }

    fn bracket_end(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched {
            ")" => Some(Ok((span, BaseToken::RParen))),
            "]" => Some(Ok((span, BaseToken::RBracket))),
            "}" => Some(Ok((span, BaseToken::RBrace))),
            _ => make_error_report_with_span(
                LexerReportKind::UnexpectedPunctuationMatch,
                span,
            ),
        }
    }

    fn string_start(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        todo!()
    }

    fn named_hole(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched[1..].parse::<u64>() {
            Ok(x) => Some(Ok((span, BaseToken::NamedHole(x)))),
            Err(_) => make_error_report_with_span(
                LexerReportKind::Notu64NamedHole,
                span,
            ),
        }
    }

    fn identifier(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched {
            "let" => Some(Ok((span, BaseToken::Let))),
            "in" => Some(Ok((span, BaseToken::In))),
            "case" => Some(Ok((span, BaseToken::Case))),
            "of" => Some(Ok((span, BaseToken::Of))),
            "import" => Some(Ok((span, BaseToken::Import))),
            "data" => Some(Ok((span, BaseToken::Data))),
            "newtype" => Some(Ok((span, BaseToken::Newtype))),
            "class" => Some(Ok((span, BaseToken::Class))),
            "instance" => Some(Ok((span, BaseToken::Instance))),
            "public" => Some(Ok((span, BaseToken::Public))),
            "alias" => Some(Ok((span, BaseToken::Alias))),
            "as" => Some(Ok((span, BaseToken::As))),
            "unqualified" => Some(Ok((span, BaseToken::Unqualified))),
            "forall" => Some(Ok((span, BaseToken::Forall))),
            "type" => Some(Ok((span, BaseToken::Type))),
            _ => match Identifier::make(
                matched,
                &mut *(*self.store).borrow_mut(),
            ) {
                Ok(iden) => Some(Ok((span, BaseToken::Identifier(iden)))),
                _ => make_error_report_with_span(
                    LexerReportKind::CantCreateIdentifier,
                    span,
                ),
            },
        }
    }

    fn infix_identifier(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match Identifier::make(
            &matched[1..(matched.len() - 1)],
            &mut *(*self.store).borrow_mut(),
        ) {
            Ok(iden) => Some(Ok((span, BaseToken::InfixIdentifier(iden)))),
            _ => make_error_report_with_span(
                LexerReportKind::CantCreateIdentifier,
                span,
            ),
        }
    }

    fn anon_hole(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        Some(Ok((span, BaseToken::AnonHole)))
    }

    fn ownership_literal(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match matched {
            "'0" => Some(Ok((
                span,
                BaseToken::OwnershipLiteral(OwnershipLiteral::Zero),
            ))),
            "'1" => Some(Ok((
                span,
                BaseToken::OwnershipLiteral(OwnershipLiteral::One),
            ))),
            "'inf" => Some(Ok((
                span,
                BaseToken::OwnershipLiteral(OwnershipLiteral::Inf),
            ))),
            _ => make_error_report_with_span(
                LexerReportKind::UnexpectedOwnershipLiteralMatch,
                span,
            ),
        }
    }

    fn ownership_variable(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        //This is safe since ownership_variables start with `'`, an ascii
        //character.
        let maybe_identifier = &matched[1..];
        match Identifier::make(
            maybe_identifier,
            &mut *(*self.store).borrow_mut(),
        ) {
            Ok(idn) => Some(Ok((
                span,
                BaseToken::OwnershipVariable(OwnershipVariable {
                    variable: idn,
                }),
            ))),
            _ => make_error_report_with_span(
                LexerReportKind::CantCreateIdentifier,
                span,
            ),
        }
    }

    fn octal(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match u64::from_str_radix(&matched[2..], 8) {
            Ok(value) => Some(Ok((
                span,
                BaseToken::UintLiteral(UintLiteral {
                    kind: UintKind::Octal,
                    value,
                }),
            ))),
            Err(e) => make_error_report_with_span(
                LexerReportKind::CantParseU64(e),
                span,
            ),
        }
    }

    fn hex(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match u64::from_str_radix(&matched[2..], 16) {
            Ok(value) => Some(Ok((
                span,
                BaseToken::UintLiteral(UintLiteral {
                    kind: UintKind::Hex,
                    value,
                }),
            ))),
            Err(e) => make_error_report_with_span(
                LexerReportKind::CantParseU64(e),
                span,
            ),
        }
    }

    fn binary(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        match u64::from_str_radix(&matched[2..], 2) {
            Ok(value) => Some(Ok((
                span,
                BaseToken::UintLiteral(UintLiteral {
                    kind: UintKind::Binary,
                    value,
                }),
            ))),
            Err(e) => make_error_report_with_span(
                LexerReportKind::CantParseU64(e),
                span,
            ),
        }
    }

    //TODO: add type signatures to uint literals like `27_u8`
    fn numeric(
        &mut self,
        m: Match,
    ) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
        let matched = m.as_str();
        let span = self.advance_non_line_breaks(matched);
        let re_parsed = NUMBER.captures(matched).unwrap();
        match re_parsed.name("decimal_part") {
            Some(_) => Some(Ok((span, BaseToken::UFloatLiteral(todo!())))),
            None => match matched.parse::<u64>() {
                Ok(value) => Some(Ok((
                    span,
                    BaseToken::UintLiteral(UintLiteral {
                        kind: UintKind::Unspecified,
                        value,
                    }),
                ))),
                Err(e) => make_error_report_with_span(
                    LexerReportKind::CantParseU64(e),
                    span,
                ),
            },
        }
    }
}

const MAIN_REGEX_STR: &'static str = r#"^((?<comment_start>//|--|\{----|\{---|\{--|\{-)|(?<punctuation_or_operator>\\|/|#|,|;|\?|\+|\^|%|\.|::|:|->|-|\|\||\|>|\||<\?>|<&>|<<|<\*>|<\*|<\$>|<\$|<-|<=|<\|>|<\||<|\*>|\*|==|=|!=|!|>=|>>|>|&&|&|\$>|\$|@)|(?<bracket_start>\(|\[|\{)|(?<bracket_end>\)|\]|\})|(?<string_start>f#"|r####"|r###"|r##"|r#"|")|(?<named_hole>_[0-9][0-9_]*)|(?<identifier>_*\p{XID_START}\p{XID_CONTINUE}*)|(?<infix_identifier>`_*\p{XID_START}\p{XID_CONTINUE}*`)|(?<anon_hole>_)|(?<ownership_literal>'(0|1|inf))|(?<ownership_variable>'_*\p{XID_START}\p{XID_CONTINUE}*)|(?<octal>0o[0-7][0-7_]*)|(?<hex>0x[0-9a-fA-F][0-9a-fA-F_]*)|(?<binary>0b[01][01_]*)|(?<numeric>[0-9][0-9_]*(?<decimal_part>\.[0-9][0-9_]*(?<exponential_part>(e|E)(?<sign>\+|-)?[0-9][0-9_]*)?)?))"#;

const MAIN_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(MAIN_REGEX_STR).unwrap());

const SPACES_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+"#).unwrap());

// We need to match at the end for the EOF, otherwise the
// repl or test may fail the parse!
static HYPHEN_COMMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^-- *(?<doc>\|)?(?<content>.*)(\n|$)").unwrap()
});

static SLASH_COMMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^// *(?<doc>\|)?(?<content>.*)(\n|$)").unwrap()
});

// Remember, for the concatenation the complement of
// `ab`
// is
// `[^a]|a[^b]`
// In blocks we want to find complements of `--}`.
static COMMENT_BLOCK0: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{- *(?<doc>\|)?(?<content>([^-]|-[^\}])*)-\}").unwrap()
});
static COMMENT_BLOCK1: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{-- *(?<doc>\|)?(?<content>([^-]|-([^-]|-[^\}]))*)--\}")
        .unwrap()
});
static COMMENT_BLOCK2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\{--- *(?<doc>\|)?(?<content>([^-]|-([^-]|-([^-]|-[^\}])))*)---\}",
    )
    .unwrap()
});
static COMMENT_BLOCK3: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\{---- *(?<doc>\|)?(?<content>([^-]|-([^-]|-([^-]|-([^-]|-[^\}]))))*)----\}",
    )
    .unwrap()
});
static NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?<numeric>[0-9][0-9_]*(?<decimal_part>\.[0-9][0-9_]*(?<exponential_part>(e|E)(?<sign>\+|-)?[0-9][0-9_]*)?)?)"
    )
    .unwrap()
});
static SIMPLE_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""([^"]|[^\\](\\\\)*\\")*""#).unwrap());

fn find_match_group<'source, 'store, 'context>(
    c: Captures,
    blc: &'context mut BaseLexerContext<'source>,
    v: Vec<(
        &'static str,
        fn(
            &'context mut BaseLexerContext<'source>,
            m: Match,
        ) -> Option<Result<(Span, BaseToken), OctizysParserReport>>,
    )>,
) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
    for (name, f) in v {
        match c.name(name) {
            Some(m) => return f(blc, m),
            None => (),
        }
    }
    make_error_report_with_position(
        LexerReportKind::UnexpectedCharacter,
        blc.position,
    )
}

impl<'store, 'source> Iterator for BaseLexerContext<'source> {
    type Item = Result<(Span, BaseToken), OctizysParserReport>;
    fn next(&mut self) -> Option<Self::Item> {
        self.consume_spaces();
        //println!("BASE_CONTEXT:{:?}", self);
        if self.index.len() == 0 {
            return None;
        }
        match MAIN_REGEX.captures(self.index) {
            Some(captures) => find_match_group(
                captures,
                self,
                vec![
                    ("comment_start", BaseLexerContext::comment),
                    (
                        "punctuation_or_operator",
                        BaseLexerContext::punctuation_or_operator,
                    ),
                    ("bracket_start", BaseLexerContext::bracket_start),
                    ("bracket_end", BaseLexerContext::bracket_end),
                    ("string_start", BaseLexerContext::string_start),
                    ("named_hole", BaseLexerContext::named_hole),
                    ("identifier", BaseLexerContext::identifier),
                    ("infix_identifier", BaseLexerContext::infix_identifier),
                    ("anon_hole", BaseLexerContext::anon_hole),
                    ("ownership_literal", BaseLexerContext::ownership_literal),
                    (
                        "ownership_variable",
                        BaseLexerContext::ownership_variable,
                    ),
                    ("octal", BaseLexerContext::octal),
                    ("hex", BaseLexerContext::hex),
                    ("binary", BaseLexerContext::binary),
                    ("numeric", BaseLexerContext::numeric),
                ],
            ),
            None => make_error_report_with_position(
                LexerReportKind::UnexpectedCharacter,
                self.position,
            ),
        }
    }
}

#[derive(Debug)]
pub enum BaseOrComments {
    Base(BaseToken, TokenInfo),
    Comments(Vec<Comment>),
}

#[derive(Debug)]
pub struct LexerContext<'src> {
    previous_token: Option<
        Result<(Span, BaseOrComments), (Vec<Comment>, OctizysParserReport)>,
    >,
    lexer: &'src mut BaseLexerContext<'src>,
}

impl<'src> LexerContext<'src> {
    pub fn new(
        previous_token: Option<
            Result<(Span, BaseOrComments), (Vec<Comment>, OctizysParserReport)>,
        >,
        lexer: &'src mut BaseLexerContext<'src>,
    ) -> Self {
        LexerContext {
            previous_token,
            lexer,
        }
    }
}

// TODO: make this an iterator to consume lazily if needed
pub fn accumulate_comments<
    I: Iterator<Item = Result<(Span, BaseToken), OctizysParserReport>>,
>(
    lexer: &mut I,
    acc: &mut Vec<Comment>,
) -> Option<Result<(Span, BaseToken), OctizysParserReport>> {
    let mut out: Option<Result<(Span, BaseToken), OctizysParserReport>> = None;
    loop {
        match lexer.next() {
            Some(maybe_token) => match maybe_token {
                Ok((span, token)) => match token {
                    BaseToken::LineComment(l) => {
                        acc.push(Comment::Line(l));
                    }
                    BaseToken::BlockComment(b) => {
                        acc.push(Comment::Block(b));
                    }
                    _ => {
                        out = Some(Ok((span, token)));
                        break;
                    }
                },
                Err(e) => {
                    out = Some(Err(e));
                    break;
                }
            },
            None => break,
        }
    }
    out
}

/// Takes a vector of comments and split them according if they are
/// in the same line or not.
pub fn split_comments_by_line(
    line: usize,
    acc: Vec<Comment>,
) -> (Vec<Comment>, Vec<Comment>) {
    let mut same_line = vec![];
    let mut not_same_line = vec![];
    for i in acc.into_iter() {
        if i.get_span().start.line == line {
            same_line.push(i)
        } else {
            not_same_line.push(i)
        }
    }
    (same_line, not_same_line)
}

// TODO: In case we have multiple block comments in a line
// this algorithm will store the second, third, ..., comments to the
// next token. Is this what we want? is a uncommon case but still need to be handled.
/// We already consumed all the comments before a token, and we
/// want to complete the given [`current_token`].
/// If the next token is a comment in the same line as the [`current_token`] we set the after comments.
/// If the next token is another kind of token we store it in the previous and emit the [`current_token`].
pub fn complete_token_or_save(
    current_token: BaseToken,
    context: &mut LexerContext,
    mut info: TokenInfo,
) -> Option<
    Result<(Position, Token, Position), (Vec<Comment>, OctizysParserReport)>,
> {
    let mut acc = vec![];
    let next_token = accumulate_comments(context.lexer, &mut acc);
    let (to_attach, remain) = split_comments_by_line(info.span.end.line, acc);
    info.comments.after = to_attach;
    let span = info.span;
    match next_token {
        Some(Ok((next_span, next_base_token))) => {
            let mut comments_info_pre = CommentsInfo {
                before: remain,
                after: vec![],
            };
            let next_info = TokenInfo::make(
                comments_info_pre,
                next_span.start,
                next_span.end,
            );
            context.previous_token = Some(Ok((
                span,
                BaseOrComments::Base(next_base_token, next_info),
            )));
            let mut token = aux_base_token_to_token(current_token, info);
            Some(Ok((span.start, token, span.end)))
        }
        Some(Err(err)) => {
            context.previous_token = Some(Err((remain, err)));
            let mut token = aux_base_token_to_token(current_token, info);
            Some(Ok((span.start, token, span.end)))
        }
        None => {
            if remain.len() > 0 {
                let mut span = remain[0].get_span();
                for s in remain.iter() {
                    span = span + s.get_span();
                }
                context.previous_token =
                    Some(Ok((span, BaseOrComments::Comments(remain))));
            } else {
                context.previous_token = None;
            }
            let mut token = aux_base_token_to_token(current_token, info);
            Some(Ok((span.start, token, span.end)))
        }
    }
}

/// Given an array of comments at the end of the stream, we build a comment
/// token (if the array is empty we return None)j
fn make_last_comment_token(
    mut comments: Vec<Comment>,
) -> Option<(Position, Token, Position)> {
    let last_comment = comments.pop()?;
    let empty_comments_info = CommentsInfo::empty();
    let mut span = last_comment.get_span();
    for comment in comments.iter() {
        span = span + comment.get_span();
    }
    let info = TokenInfo::make(empty_comments_info, span.start, span.end);
    comments.push(last_comment);
    Some((span.start, Token::LastComments(info, comments), span.end))
}

/// To call at the beginning of lexing.
fn parse_token(
    context: &mut LexerContext,
) -> Option<
    Result<(Position, Token, Position), (Vec<Comment>, OctizysParserReport)>,
> {
    let mut acc = vec![];
    let current_value = accumulate_comments(&mut context.lexer, &mut acc);
    match current_value {
        Some(Ok((span, current_token))) => {
            let mut comments_info_pre = CommentsInfo {
                before: acc,
                after: vec![],
            };
            let mut info =
                TokenInfo::make(comments_info_pre, span.start, span.end);
            complete_token_or_save(current_token, context, info)
        }
        //Reached eof in the lexer
        None => make_last_comment_token(acc).map(|x| Ok(x)),
        Some(Err(e)) => Some(Err((vec![], e))),
    }
}

impl<'store, 'src> Iterator for LexerContext<'src> {
    type Item = Result<(Position, Token, Position), OctizysParserReport>;
    fn next(&mut self) -> Option<Self::Item> {
        //println!("{:?}", self);
        match &self.previous_token {
            Some(previous) => match previous {
                /// An error happened while looking for the after comments
                // TODO: We are ignoring the comments associated with a erroneous token!
                Err((_, e)) => {
                    let e2 = e.clone();
                    self.previous_token = None;
                    Some(Err(e2))
                }
                // We reached the end of the stream with remain comments
                Ok((span, BaseOrComments::Comments(comments))) => {
                    let comments2 = comments.clone();
                    self.previous_token = None;
                    make_last_comment_token(comments2).map(|x| Ok(x))
                }
                // We stored previously a non comment token
                // and we are going to emit it after looking up
                // for the after comment.
                Ok((span, BaseOrComments::Base(base_token, info))) => {
                    complete_token_or_save(
                        base_token.clone(),
                        self,
                        info.clone(),
                    )
                    // TODO: We are ignoring the comments associated with a erroneous token!
                    .map(|x| x.map_err(|(_, z)| z))
                }
            },
            None => {
                // Didn't have previous value, either we are at the beginning
                // of the process or we reached the end of the stream
                // TODO: We are ignoring the comments associated with a erroneous token!
                parse_token(self).map(|x| x.map_err(|(_, z)| z))
            }
        }
    }
}

#[cfg(test)]
mod test_regex {
    use super::*;

    fn main_regex_with(input: &str, capture_name: &str) {
        let result = super::MAIN_REGEX
            .captures(input)
            .expect(&format!("Can't find : {}", input));
        println!("{:?}", result);
        assert_eq!(result.get(0).unwrap().as_str(), input);
        assert_eq!(&result[capture_name], input)
    }

    #[test]
    fn comment_start() {
        main_regex_with("//", "comment_start");
        main_regex_with("--", "comment_start");
        main_regex_with("{-", "comment_start");
        main_regex_with("{--", "comment_start");
        main_regex_with("{---", "comment_start");
        main_regex_with("{----", "comment_start");
    }

    #[test]
    fn punctuation_or_operator() {
        main_regex_with("\\", "punctuation_or_operator");
        main_regex_with("/", "punctuation_or_operator");
        main_regex_with("#", "punctuation_or_operator");
        main_regex_with(",", "punctuation_or_operator");
        main_regex_with(";", "punctuation_or_operator");
        main_regex_with("+", "punctuation_or_operator");
        main_regex_with("^", "punctuation_or_operator");
        main_regex_with("%", "punctuation_or_operator");
        main_regex_with("@", "punctuation_or_operator");
        main_regex_with(".", "punctuation_or_operator");
        main_regex_with(":", "punctuation_or_operator");
        main_regex_with("::", "punctuation_or_operator");
        main_regex_with("-", "punctuation_or_operator");
        main_regex_with("->", "punctuation_or_operator");
        main_regex_with("|", "punctuation_or_operator");
        main_regex_with("<|", "punctuation_or_operator");
        main_regex_with("<|>", "punctuation_or_operator");
        main_regex_with("<=", "punctuation_or_operator");
        main_regex_with("<-", "punctuation_or_operator");
        main_regex_with("<$", "punctuation_or_operator");
        main_regex_with("<$>", "punctuation_or_operator");
        main_regex_with("<*", "punctuation_or_operator");
        main_regex_with("<<", "punctuation_or_operator");
        main_regex_with("<*>", "punctuation_or_operator");
        main_regex_with("<&>", "punctuation_or_operator");
        main_regex_with("<?>", "punctuation_or_operator");
        main_regex_with("*", "punctuation_or_operator");
        main_regex_with("*>", "punctuation_or_operator");
        main_regex_with("=", "punctuation_or_operator");
        main_regex_with("==", "punctuation_or_operator");
        main_regex_with("!", "punctuation_or_operator");
        main_regex_with("!=", "punctuation_or_operator");
        main_regex_with(">", "punctuation_or_operator");
        main_regex_with(">>", "punctuation_or_operator");
        main_regex_with(">=", "punctuation_or_operator");
        main_regex_with("&&", "punctuation_or_operator");
        main_regex_with("&", "punctuation_or_operator");
        main_regex_with("$", "punctuation_or_operator");
        main_regex_with("$>", "punctuation_or_operator");
    }

    #[test]
    fn bracket_start() {
        main_regex_with("(", "bracket_start");
        main_regex_with("{", "bracket_start");
        main_regex_with("[", "bracket_start")
    }

    #[test]
    fn bracket_end() {
        main_regex_with(")", "bracket_end");
        main_regex_with("}", "bracket_end");
        main_regex_with("]", "bracket_end")
    }

    #[test]
    fn string_start() {
        main_regex_with("\"", "string_start");
        main_regex_with("f#\"", "string_start");
        main_regex_with("r#\"", "string_start");
        main_regex_with("r##\"", "string_start");
        main_regex_with("r###\"", "string_start");
        main_regex_with("r####\"", "string_start");
    }

    #[test]
    fn named_hole() {
        main_regex_with("_0", "named_hole");
        main_regex_with("_0_0", "named_hole");
        main_regex_with("_89342", "named_hole");
    }

    #[test]
    // Sync with infix_identifier
    fn identifier() {
        main_regex_with("_a", "identifier");
        main_regex_with("ab", "identifier");
        main_regex_with("a", "identifier");
        main_regex_with("à", "identifier");
        main_regex_with("àbc", "identifier");
        main_regex_with("____a", "identifier");
        main_regex_with("_b23d", "identifier");
    }

    #[test]
    // Sync with infix_identifier
    fn infix_identifier() {
        main_regex_with("`_a`", "infix_identifier");
        main_regex_with("`ab`", "infix_identifier");
        main_regex_with("`a`", "infix_identifier");
        main_regex_with("`à`", "infix_identifier");
        main_regex_with("`àbc`", "infix_identifier");
        main_regex_with("`____a`", "infix_identifier");
        main_regex_with("`_b23d`", "infix_identifier");
    }

    #[test]
    fn anon_hole() {
        main_regex_with("_", "anon_hole");
    }

    #[test]
    fn ownership_literal() {
        main_regex_with("'0", "ownership_literal");
        main_regex_with("'1", "ownership_literal");
        main_regex_with("'inf", "ownership_literal");
    }

    #[test]
    fn ownership_variable() {
        main_regex_with("'_a", "ownership_variable");
        main_regex_with("'ab", "ownership_variable");
        main_regex_with("'a", "ownership_variable");
        main_regex_with("'à", "ownership_variable");
        main_regex_with("'àbc", "ownership_variable");
        main_regex_with("____a", "identifier");
        main_regex_with("_b23d", "identifier");
    }

    #[test]
    fn octal() {
        main_regex_with("0o0", "octal");
        main_regex_with("0o00", "octal");
        main_regex_with("0o1", "octal");
        main_regex_with("0o2_3_4", "octal");
        main_regex_with("0o1234", "octal");
    }

    #[test]
    fn hex() {
        main_regex_with("0x0", "hex");
        main_regex_with("0x00", "hex");
        main_regex_with("0x1", "hex");
        main_regex_with("0x2_3_4", "hex");
        main_regex_with("0x1234", "hex");
    }

    #[test]
    fn binary() {
        main_regex_with("0b0", "binary");
        main_regex_with("0b00", "binary");
        main_regex_with("0b1", "binary");
        main_regex_with("0b101", "binary");
        main_regex_with("0b001010_1_0100_001", "binary");
    }

    #[test]
    fn numeric() {
        main_regex_with("0", "numeric");
        main_regex_with("1", "numeric");
        main_regex_with("2_3_4", "numeric");
        main_regex_with("34345.384e32", "numeric");
        main_regex_with("0_89_79.52218E-32", "numeric");
        main_regex_with("839_3479.52788e+32", "numeric");
    }
}
