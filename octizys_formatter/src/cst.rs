use octizys_cst::{
    base::{
        Between, Delimiters, ImportedVariable, OperatorName, Separator,
        ShowableToken, Token, TokenInfo, TokenInfoWithPhantom, TrailingList,
        TrailingListItem,
    },
    comments::{
        Comment, CommentBlock, CommentBraceKind, CommentKind, CommentLine,
        CommentLineContent, CommentsInfo, LineCommentStart,
    },
    expressions::{
        ApplicationExpression, BinaryOperator, Case, CaseItem, Expression,
        ExpressionRecordItem, ExpressionSelector, LambdaExpression, Let,
        LetBinding,
    },
    imports::{AsPath, Import},
    literals::{StringLiteral, UFloatingPointLiteral, UintKind, UintLiteral},
    patterns::{PatternMatch, PatternMatchBind, PatternMatchRecordItem},
    top::{Constructor, Data, DataConstructors, Top, TopItem},
    types::{Type, TypeRecordItem},
};
use octizys_pretty::{
    combinators::{
        concat, concat_iter, empty, empty_break, external_text, group,
        hard_break, intersperse, nest, no_break_space, repeat, soft_break,
        static_str,
    },
    document::Document,
};

use crate::{keywords, to_document::ToDocument};

#[derive(Debug, Copy, Clone)]
pub struct PrettyCSTConfiguration {
    //TODO: add option to sort imports alphabetically  (Ord<String>)
    // An even better options may be to sort first by package and then sort
    // alphabetically
    pub indentation_deep: u16,
    pub leading_commas: bool,
    /// trailing commas are always preserved but
    /// if they don't exists this option as false would
    /// prevent the formatter to put a new one.
    pub add_trailing_separator: bool,
    /// In a block like:
    /// ```haskell
    /// someIdent -- | doc for this
    /// ```
    /// If this options is true, it is translated to
    /// ```haskell
    /// -- | doc for this
    /// someIdent
    /// ```
    pub move_documentantion_before_object: bool,
    pub indent_comment_blocks: bool,
    /// Contiguous but different sections of comments
    /// are going to be separated by this distance
    /// unless the compact_comments options is enabled
    pub separe_comments_by: u8,
    /// Would collapse multiples commentaries in a single block
    /// and multiple documentation commentaries to a single block
    pub compact_comments: bool,
}

impl Default for PrettyCSTConfiguration {
    fn default() -> Self {
        PrettyCSTConfiguration {
            indentation_deep: 2,
            leading_commas: true,
            add_trailing_separator: true,
            move_documentantion_before_object: true,
            indent_comment_blocks: true,
            separe_comments_by: 1,
            compact_comments: true,
        }
    }
}

pub fn indent(
    configuration: &PrettyCSTConfiguration,
    doc: Document,
) -> Document {
    group(nest(configuration.indentation_deep, doc))
}

impl ToDocument<PrettyCSTConfiguration> for CommentLineContent {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        Document::from_index_and_len(self.get_index(), self.get_len())
    }
}

impl ToDocument<PrettyCSTConfiguration> for CommentKind {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            CommentKind::Documentation => {
                Document::static_str(keywords::COMMENT_KIND)
            }
            _ => empty(),
        }
    }
}

fn comment_brace_to_documents(
    brace: &CommentBraceKind,
) -> (Document, Document) {
    let number_of_dashes = brace.len() - 1;
    let hypens =
        repeat(Document::static_str(keywords::HYPEN), number_of_dashes);
    (
        Document::from(Document::static_str(keywords::LBRACE)) + hypens.clone(),
        hypens.clone() + Document::static_str(keywords::RBRACE),
    )
}

impl ToDocument<PrettyCSTConfiguration> for LineCommentStart {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            LineCommentStart::DoubleHypen => {
                repeat(Document::static_str(keywords::HYPEN), 2)
            }
            LineCommentStart::DoubleSlash => {
                repeat(Document::static_str(keywords::SLASH), 2)
            }
        }
    }
}

impl ToDocument<PrettyCSTConfiguration> for CommentBlock {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let (brace_start0, brace_end) = comment_brace_to_documents(&self.brace);
        let block_start = brace_start0 + self.kind.to_document(configuration);
        let block_end = self.kind.to_document(configuration) + brace_end;
        let content_raw = intersperse(
            self.content.iter().map(|x| x.to_document(configuration)),
            hard_break(),
        );

        let content = if configuration.indent_comment_blocks {
            indent(configuration, hard_break() + content_raw) + hard_break()
        } else {
            content_raw
        };

        concat(vec![block_start, content, block_end, hard_break()])
    }
}

impl ToDocument<PrettyCSTConfiguration> for CommentLine {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let line_start = self.start.to_document(configuration)
            + self.kind.to_document(configuration);
        concat(vec![
            line_start,
            self.content.to_document(configuration),
            hard_break(),
        ])
    }
}

impl ToDocument<PrettyCSTConfiguration> for Comment {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            Comment::Line(l) => l.to_document(configuration),
            Comment::Block(l) => l.to_document(configuration),
        }
    }
}

pub fn comments_info_to_document(
    comment: &CommentsInfo,
    configuration: &PrettyCSTConfiguration,
    doc: Document,
) -> Document {
    let mut out = comment.clone();
    if configuration.move_documentantion_before_object {
        out.move_after_to_before();
    }
    if configuration.compact_comments {
        out.compact_comments();
    }
    let separe_by = usize::from(configuration.separe_comments_by);
    concat(vec![
        intersperse(
            out.before.into_iter().map(|x| x.to_document(configuration)),
            repeat(hard_break(), separe_by),
        ),
        doc,
        intersperse(
            out.after.into_iter().map(|x| x.to_document(configuration)),
            external_text(" "),
        ),
    ])
}

pub fn token_info_to_document(
    info: &TokenInfo,
    configuration: &PrettyCSTConfiguration,
    doc: Document,
) -> Document {
    comments_info_to_document(&info.comments, configuration, doc)
}

impl<P> ToDocument<PrettyCSTConfiguration> for TokenInfoWithPhantom<P>
where
    P: ShowableToken,
{
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        token_info_to_document(&self.info, configuration, static_str(P::show()))
    }
}

impl<T> ToDocument<PrettyCSTConfiguration> for Token<T>
where
    T: ToDocument<PrettyCSTConfiguration>,
{
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        token_info_to_document(
            &self.info,
            configuration,
            self.value.to_document(configuration),
        )
    }
}

impl ToDocument<PrettyCSTConfiguration> for OperatorName {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        let x = match self {
            OperatorName::Interrogation => keywords::INTERROGATION,
            OperatorName::Exclamation => keywords::EXCLAMATION,
            OperatorName::Hash => keywords::HASH,
            OperatorName::Comma => keywords::COMMA,
            OperatorName::Colon => keywords::COLON,
            OperatorName::StatementEnd => keywords::SEMICOLON,
            OperatorName::Dot => keywords::DOT,
            OperatorName::ModuleSeparator => keywords::MODULE_SEPARATOR,
            OperatorName::Minus => keywords::HYPEN,
            OperatorName::CompositionLeft => keywords::COMPOSITION_LEFT,
            OperatorName::CompositionRight => keywords::COMPOSITION_RIGHT,
            OperatorName::Plus => keywords::PLUS,
            OperatorName::Power => keywords::POWER,
            OperatorName::Star => keywords::STAR,
            OperatorName::Div => keywords::DIV,
            OperatorName::Module => keywords::PERCENTAGE,
            OperatorName::ShiftLeft => keywords::SHIFT_LEFT,
            OperatorName::ShiftRigth => keywords::SHIFT_RIGTH,
            OperatorName::Map => keywords::MAP,
            OperatorName::MapConstRigth => keywords::MAP_CONST_RIGTH,
            OperatorName::MapConstLeft => keywords::MAP_CONST_LEFT,
            OperatorName::Appliative => keywords::APPLIATIVE,
            OperatorName::ApplicativeRight => keywords::APPLICATIVE_RIGHT,
            OperatorName::ApplicativeLeft => keywords::APPLICATIVE_LEFT,
            OperatorName::Equality => keywords::ASIGNATION,
            OperatorName::NotEqual => keywords::NOT_EQUAL,
            OperatorName::LessOrEqual => keywords::LESS_OR_EQUAL,
            OperatorName::MoreOrEqual => keywords::MORE_OR_EQUAL,
            OperatorName::LessThan => keywords::LESS_THAN,
            OperatorName::MoreThan => keywords::MORE_THAN,
            OperatorName::And => keywords::AND,
            OperatorName::Or => keywords::OR,
            OperatorName::ReverseAppliation => keywords::REVERSE_APPLIATION,
            OperatorName::DollarApplication => keywords::DOLLAR_APPLICATION,
            OperatorName::Asignation => keywords::ASIGNATION,
            OperatorName::At => keywords::AT,
            OperatorName::Pipe => keywords::PIPE,
            OperatorName::RightArrow => keywords::RIGHT_ARROW,
            OperatorName::LeftArrow => keywords::LEFT_ARROW,
            OperatorName::LambdaStart => keywords::LAMBDA_START,
            OperatorName::Alternative => keywords::ALTERNATIVE,
            OperatorName::FlippedMap => keywords::FLIPPEDMAP,
            OperatorName::Annotate => keywords::ANNOTATE,
        };
        Document::static_str(x)
    }
}

impl<T, Enclosure> ToDocument<PrettyCSTConfiguration> for Between<T, Enclosure>
where
    Enclosure: Delimiters,
    T: ToDocument<PrettyCSTConfiguration>,
{
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let (start_str, end_str) = Enclosure::to_strs();
        let (start, end) = (
            Document::static_str(start_str),
            Document::static_str(end_str),
        );
        //TODO: after the refactor, lookup if there are comments
        //inside the span of this between, if they are,
        //enforce the ")" to end in a separate line always
        //otherwise keep this.
        token_info_to_document(&self.left, configuration, start)
            + group(concat(vec![
                nest(
                    configuration.indentation_deep,
                    empty_break() + self.value.to_document(configuration),
                ),
                empty_break(),
                token_info_to_document(&self.right, configuration, end),
            ]))
    }
}

impl ToDocument<PrettyCSTConfiguration> for ImportedVariable {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        concat(vec![
            self.path.to_document(configuration),
            Document::static_str(keywords::MODULE_SEPARATOR),
            (&self.name).to_document(configuration),
        ])
    }
}

impl<T, SeparatorPhantom> ToDocument<PrettyCSTConfiguration>
    for TrailingListItem<T, SeparatorPhantom>
where
    T: ToDocument<PrettyCSTConfiguration>,
    SeparatorPhantom: Separator,
{
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let separator_doc = if configuration.leading_commas {
            empty_break()
                + Document::static_str(SeparatorPhantom::to_str())
                + no_break_space()
        } else {
            Document::static_str(SeparatorPhantom::to_str()) + soft_break()
        };
        concat(vec![
            token_info_to_document(
                &self.separator,
                configuration,
                separator_doc,
            ),
            self.item.to_document(configuration),
        ])
    }
}

impl<T, SeparatorPhantom> ToDocument<PrettyCSTConfiguration>
    for TrailingList<T, SeparatorPhantom>
where
    T: ToDocument<PrettyCSTConfiguration>,
    SeparatorPhantom: Separator,
{
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let separator_doc = if configuration.leading_commas {
            empty_break() + Document::static_str(SeparatorPhantom::to_str())
        } else {
            Document::static_str(SeparatorPhantom::to_str()) + empty_break()
        };
        let trailing = match &self.trailing_sep {
            Some(separator_info) => token_info_to_document(
                separator_info,
                configuration,
                separator_doc,
            ),
            None => {
                if configuration.add_trailing_separator {
                    separator_doc
                } else {
                    empty()
                }
            }
        };
        concat(vec![
            self.first.to_document(configuration),
            concat(
                self.items
                    .iter()
                    .map(|x| x.to_document(configuration))
                    .collect(),
            ),
            trailing,
        ])
    }
}

// --------------------------------- Literals --------------------------------

impl ToDocument<PrettyCSTConfiguration> for UintKind {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for UintLiteral {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for StringLiteral {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for UFloatingPointLiteral {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

// --------------------------------- Imports --------------------------------

impl ToDocument<PrettyCSTConfiguration> for AsPath {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        soft_break()
            + concat(vec![
                token_info_to_document(
                    &self._as,
                    configuration,
                    Document::static_str(keywords::AS),
                ),
                indent(
                    configuration,
                    soft_break() + self.path.to_document(configuration),
                ),
            ])
    }
}

impl ToDocument<PrettyCSTConfiguration> for Import {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let import = token_info_to_document(
            &self.import,
            configuration,
            Document::static_str(keywords::IMPORT),
        );
        let unqualified: Document = match &self.unqualified {
            Some(x) => soft_break() + x.to_document(configuration),
            None => empty(),
        };

        let path = soft_break() + self.logic_path.to_document(configuration);
        let imports = match &self.import_list {
            Some(x) => x.to_document(configuration),
            None => empty(),
        };
        let _as = self
            .qualified_path
            .as_ref()
            .map_or_else(empty, |x| x.to_document(configuration));
        import
            + indent(
                configuration,
                concat(vec![unqualified, path, imports, _as]),
            )
    }
}

impl ToDocument<PrettyCSTConfiguration> for PatternMatchRecordItem {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            PatternMatchRecordItem::OnlyVariable { variable } => {
                variable.to_document(configuration)
            }
            PatternMatchRecordItem::WithPattern {
                variable,
                separator,
                pattern,
            } => concat(vec![
                variable.to_document(configuration),
                token_info_to_document(
                    separator,
                    configuration,
                    Document::static_str(keywords::COLON),
                ),
                pattern.to_document(configuration),
            ]),
        }
    }
}

impl ToDocument<PrettyCSTConfiguration> for PatternMatchBind {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        concat(vec![
            self.variable.to_document(configuration),
            token_info_to_document(
                &self.at,
                configuration,
                Document::static_str(keywords::AT),
            ),
            self.pattern.to_document(configuration),
        ])
    }
}

fn to_document_pattern_application_argument(
    pattern: &PatternMatch,
    configuration: &PrettyCSTConfiguration,
) -> Document {
    match &pattern {
        PatternMatch::Bind(b) => match *b.pattern {
            PatternMatch::Application { .. } => concat(vec![
                Document::static_str(keywords::LPAREN),
                soft_break(),
                pattern.to_document(configuration),
                soft_break(),
                Document::static_str(keywords::RPAREN),
            ]),
            _ => pattern.to_document(configuration),
        },
        PatternMatch::Application { .. } => concat(vec![
            Document::static_str(keywords::LPAREN),
            soft_break(),
            pattern.to_document(configuration),
            soft_break(),
            Document::static_str(keywords::RPAREN),
        ]),
        _ => pattern.to_document(configuration),
    }
}

impl ToDocument<PrettyCSTConfiguration> for PatternMatch {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            PatternMatch::LocalVariable(tok) => tok.to_document(configuration),
            PatternMatch::ImportedVariable(tok) => {
                tok.to_document(configuration)
            }
            PatternMatch::String(_tok) => {
                todo!("Strings are delayed for now in the pretty printer")
            } //tok.to_document(configuration),
            PatternMatch::UFloat(_tok) => {
                todo!("Strings are delayed for now in the pretty printer")
            } //tok.to_document(configuration),
            PatternMatch::Uint(_tok) => {
                todo!("Strings are delayed for now in the pretty printer")
            } //tok.to_document(configuration),
            PatternMatch::AnonHole(info) => token_info_to_document(
                info,
                configuration,
                Document::static_str(keywords::UNDERSCORE),
            ),
            PatternMatch::Tuple(t) => t.to_document(configuration),
            PatternMatch::Record(r) => r.to_document(configuration),
            PatternMatch::Bind(b) => b.to_document(configuration),
            PatternMatch::Application {
                start,
                second,
                remain,
            } => concat(vec![
                to_document_pattern_application_argument(start, configuration),
                to_document_pattern_application_argument(second, configuration),
                concat_iter(remain.into_iter().map(|x| {
                    to_document_pattern_application_argument(x, configuration)
                })),
            ]),
            PatternMatch::Parens(x) => x.to_document(configuration),
        }
    }
}

impl ToDocument<PrettyCSTConfiguration> for TypeRecordItem {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        concat(vec![
            self.variable.to_document(configuration),
            token_info_to_document(
                &self.separator,
                configuration,
                Document::static_str(keywords::COLON),
            ),
            self.expression.to_document(configuration),
        ])
    }
}

fn to_document_type_application_argument(
    t: &Type,
    configuration: &PrettyCSTConfiguration,
) -> Document {
    if t.need_parens_application() {
        concat(vec![
            "(".into(),
            soft_break(),
            t.to_document(configuration),
            soft_break(),
            ")".into(),
        ])
    } else {
        t.to_document(configuration)
    }
}

fn to_document_type_arrow_arguments(
    t: &Type,
    configuration: &PrettyCSTConfiguration,
) -> Document {
    if t.need_parens_arrow() {
        concat(vec![
            "(".into(),
            soft_break(),
            t.to_document(configuration),
            soft_break(),
            ")".into(),
        ])
    } else {
        t.to_document(configuration)
    }
}

impl ToDocument<PrettyCSTConfiguration> for Type {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match &self {
            Type::LocalVariable(token) => token.to_document(configuration),
            Type::ImportedVariable(token) => token.to_document(configuration),
            Type::Tuple(between) => between.to_document(configuration),
            Type::Record(between) => between.to_document(configuration),
            Type::Parens(between) => between.to_document(configuration),
            Type::Application {
                start,
                second,
                remain,
            } => {
                to_document_type_application_argument(start, configuration)
                    + indent(
                        configuration,
                        concat(vec![
                            soft_break(),
                            to_document_type_application_argument(
                                second,
                                configuration,
                            ),
                            soft_break(),
                            intersperse(
                                remain.into_iter().map(|x| {
                                    to_document_type_application_argument(
                                        x,
                                        configuration,
                                    )
                                }),
                                soft_break(),
                            ),
                        ]),
                    )
            }
            Type::Arrow { first, remain } => {
                let remain_doc = remain
                    .into_iter()
                    .map(|arg| arg.to_document(configuration));
                to_document_type_arrow_arguments(
                    (*first).as_ref(),
                    configuration,
                ) + concat_iter(remain_doc)
            }
            Type::Scheme {
                forall,
                first_variable,
                remain_variables,
                dot,
                expression,
            } => concat(vec![
                token_info_to_document(
                    forall,
                    configuration,
                    Document::static_str(keywords::FORALL),
                ),
                indent(
                    configuration,
                    concat(vec![
                        soft_break(),
                        first_variable.to_document(configuration),
                        soft_break(),
                        intersperse(
                            remain_variables
                                .into_iter()
                                .map(|x| x.to_document(configuration)),
                            soft_break(),
                        ),
                        token_info_to_document(
                            dot,
                            configuration,
                            Document::static_str(keywords::DOT),
                        ),
                        soft_break(),
                        indent(
                            configuration,
                            soft_break()
                                + expression.to_document(configuration),
                        ),
                    ]),
                ),
            ]),
        }
    }
}

impl ToDocument<PrettyCSTConfiguration> for LetBinding {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        self.pattern.to_document(configuration)
            + indent(
                configuration,
                concat(vec![
                    soft_break(),
                    token_info_to_document(
                        &self.equal,
                        configuration,
                        Document::static_str(keywords::ASIGNATION),
                    ),
                    soft_break(),
                    self.value.to_document(configuration),
                    token_info_to_document(
                        &self.semicolon,
                        configuration,
                        Document::static_str(keywords::SEMICOLON),
                    ),
                ]),
            )
    }
}

impl ToDocument<PrettyCSTConfiguration> for CaseItem {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        concat(vec![
            self.pattern.to_document(configuration),
            token_info_to_document(
                &self.arrow,
                configuration,
                Document::static_str(keywords::MORE_OR_EQUAL),
            ),
            indent(
                configuration,
                soft_break() + self.expression.to_document(configuration),
            ),
        ])
    }
}
impl ToDocument<PrettyCSTConfiguration> for Let {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        concat(vec![
            token_info_to_document(
                &self.let_,
                configuration,
                Document::static_str(keywords::LET),
            ),
            indent(
                configuration,
                soft_break()
                    + concat_iter(
                        self.bindings
                            .iter()
                            .map(|x| x.to_document(configuration)),
                    ),
            ),
            soft_break(),
            token_info_to_document(
                &self.in_,
                configuration,
                Document::static_str(keywords::IN),
            ),
            indent(
                configuration,
                soft_break() + self.expression.to_document(configuration),
            ),
        ])
    }
}

impl ToDocument<PrettyCSTConfiguration> for Case {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        concat(vec![
            token_info_to_document(
                &self.case,
                configuration,
                Document::static_str(keywords::CASE),
            ),
            indent(
                configuration,
                soft_break() + self.expression.to_document(configuration),
            ),
            token_info_to_document(
                &self.of,
                configuration,
                Document::static_str(keywords::OF),
            ),
            //TODO: finish this, we need a cases especific to_document instead of the default for
            //between
            self.cases.to_document(configuration),
        ])
    }
}

impl ToDocument<PrettyCSTConfiguration> for BinaryOperator {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}
impl ToDocument<PrettyCSTConfiguration> for LambdaExpression {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for ApplicationExpression {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for ExpressionRecordItem {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for Expression {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}
impl ToDocument<PrettyCSTConfiguration> for ExpressionSelector {
    fn to_document(&self, _configuration: &PrettyCSTConfiguration) -> Document {
        todo!()
    }
}

impl ToDocument<PrettyCSTConfiguration> for Constructor {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match &self.type_ {
            Some(x) => {
                self.name.to_document(configuration)
                    + nest(
                        configuration.indentation_deep,
                        soft_break() + x.to_document(configuration),
                    )
            }
            None => self.name.to_document(configuration),
        }
    }
}

impl ToDocument<PrettyCSTConfiguration> for DataConstructors {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        token_info_to_document(
            &self.eq,
            configuration,
            static_str(keywords::ASIGNATION),
        ) + soft_break()
            + self.constructors.to_document(configuration)
    }
}

impl ToDocument<PrettyCSTConfiguration> for Data {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let public = self
            .public
            .as_ref()
            .map(|_| static_str(keywords::PUBLIC) + soft_break())
            .unwrap_or_else(empty);
        let data = token_info_to_document(
            &self.data,
            configuration,
            static_str(keywords::DATA),
        );
        let name = self.name.to_document(configuration);
        let variables = if self.variables.len() > 0 {
            intersperse(
                self.variables.iter().map(|x| x.to_document(configuration)),
                soft_break(),
            )
        } else {
            empty()
        };
        let constructors = self
            .constructors
            .as_ref()
            .map(|x| ToDocument::to_document(x, configuration))
            .unwrap_or_else(empty);

        concat(vec![
            public,
            data,
            soft_break(),
            nest(
                configuration.indentation_deep,
                name + soft_break()
                    + nest(configuration.indentation_deep, variables)
                    + soft_break()
                    + group(constructors),
            ),
        ])
    }
}

impl ToDocument<PrettyCSTConfiguration> for TopItem {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        match self {
            TopItem::Data(d) => d.to_document(configuration),
        }
    }
}

impl ToDocument<PrettyCSTConfiguration> for Top {
    fn to_document(&self, configuration: &PrettyCSTConfiguration) -> Document {
        let items = self
            .items
            .as_ref()
            .map(|x| x.to_document(configuration))
            .unwrap_or_else(empty);
        let separation = repeat(
            hard_break(),
            usize::from(1 + configuration.separe_comments_by),
        );
        match &self.imports {
            Some(imports) => match &self.last_comment {
                Some(last_comment) => {
                    imports.to_document(configuration)
                        + separation
                        + items
                        + last_comment.to_document(configuration)
                }
                None => imports.to_document(configuration) + separation + items,
            },
            None => match &self.last_comment {
                Some(last_comment) => {
                    items + separation + last_comment.to_document(configuration)
                }
                None => items,
            },
        }
    }
}
