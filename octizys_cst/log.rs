#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod base {
    use crate::comments::CommentsInfo;
    use octizys_common::identifier::Identifier;
    use octizys_common::logic_path::LogicPath;
    use octizys_common::span::{Position, Span};
    use octizys_macros::Equivalence;
    use octizys_text_store::store::NonLineBreakStr;
    use std::marker::PhantomData;
    mod private {
        /// We want it to create phantom types internally
        pub trait Sealed {}
    }
    /// Used to statically determine the kind of separator to use
    /// and tell rust how to represent it as string.
    /// Outside of this crate you can't create  new Separators
    /// We have defined :
    /// - [`Pipe`]
    /// - [`Comma`]
    /// - [`RightArrow`]
    /// - [`Colon`]
    /// - [`LogicPathSeparator`]
    pub trait Separator: private::Sealed {
        fn to_str() -> NonLineBreakStr;
    }
    pub enum Pipe {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Pipe {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Pipe {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Pipe {
        #[inline]
        fn eq(&self, other: &Pipe) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Pipe {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Pipe {
        #[inline]
        fn clone(&self) -> Pipe {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Pipe {}
    impl private::Sealed for Pipe {}
    impl Separator for Pipe {
        fn to_str() -> NonLineBreakStr {
            NonLineBreakStr::new("|")
        }
    }
    pub enum Comma {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Comma {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Comma {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Comma {
        #[inline]
        fn eq(&self, other: &Comma) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Comma {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comma {
        #[inline]
        fn clone(&self) -> Comma {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Comma {}
    impl private::Sealed for Comma {}
    impl Separator for Comma {
        fn to_str() -> NonLineBreakStr {
            NonLineBreakStr::new(",")
        }
    }
    pub enum RightArrow {}
    #[automatically_derived]
    impl ::core::fmt::Debug for RightArrow {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RightArrow {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RightArrow {
        #[inline]
        fn eq(&self, other: &RightArrow) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for RightArrow {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RightArrow {
        #[inline]
        fn clone(&self) -> RightArrow {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for RightArrow {}
    impl private::Sealed for RightArrow {}
    impl Separator for RightArrow {
        fn to_str() -> NonLineBreakStr {
            NonLineBreakStr::new("->")
        }
    }
    pub enum Colon {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Colon {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Colon {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Colon {
        #[inline]
        fn eq(&self, other: &Colon) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Colon {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Colon {
        #[inline]
        fn clone(&self) -> Colon {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Colon {}
    impl private::Sealed for Colon {}
    impl Separator for Colon {
        fn to_str() -> NonLineBreakStr {
            NonLineBreakStr::new(":")
        }
    }
    pub enum LogicPathSeparator {}
    #[automatically_derived]
    impl ::core::fmt::Debug for LogicPathSeparator {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for LogicPathSeparator {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LogicPathSeparator {
        #[inline]
        fn eq(&self, other: &LogicPathSeparator) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for LogicPathSeparator {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LogicPathSeparator {
        #[inline]
        fn clone(&self) -> LogicPathSeparator {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LogicPathSeparator {}
    impl private::Sealed for LogicPathSeparator {}
    impl Separator for LogicPathSeparator {
        fn to_str() -> NonLineBreakStr {
            NonLineBreakStr::new("::")
        }
    }
    /// Used to statically determine the kind of delimiters to use
    /// and tell rust how to represent it as string.
    /// Outside of this crate you can't create  new Separators
    /// We have defined:
    /// - [`Parens`]
    /// - [`Brackets`]
    /// - [`Braces`]
    pub trait Delimiters: private::Sealed {
        fn to_strs() -> (NonLineBreakStr, NonLineBreakStr);
    }
    pub enum Parens {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Parens {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Parens {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Parens {
        #[inline]
        fn eq(&self, other: &Parens) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Parens {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Parens {
        #[inline]
        fn clone(&self) -> Parens {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Parens {}
    impl private::Sealed for Parens {}
    impl Delimiters for Parens {
        fn to_strs() -> (NonLineBreakStr, NonLineBreakStr) {
            (NonLineBreakStr::new("("), NonLineBreakStr::new(")"))
        }
    }
    pub enum Brackets {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Brackets {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Brackets {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Brackets {
        #[inline]
        fn eq(&self, other: &Brackets) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Brackets {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Brackets {
        #[inline]
        fn clone(&self) -> Brackets {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Brackets {}
    impl private::Sealed for Brackets {}
    impl Delimiters for Brackets {
        fn to_strs() -> (NonLineBreakStr, NonLineBreakStr) {
            (NonLineBreakStr::new("["), NonLineBreakStr::new("]"))
        }
    }
    pub enum Braces {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Braces {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Braces {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Braces {
        #[inline]
        fn eq(&self, other: &Braces) -> bool {
            match *self {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Braces {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Braces {
        #[inline]
        fn clone(&self) -> Braces {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Braces {}
    impl private::Sealed for Braces {}
    impl Delimiters for Braces {
        fn to_strs() -> (NonLineBreakStr, NonLineBreakStr) {
            (NonLineBreakStr::new("{"), NonLineBreakStr::new("}"))
        }
    }
    /// Used to store the commentaries that belong to a token
    /// and the region of the token in the source file.
    /// See [`CommentsInfo`] for details about how the comments information
    /// is stored.
    /// See [`Span`] for details on how it is calculated.
    pub struct TokenInfo {
        pub comments: CommentsInfo,
        pub span: Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TokenInfo",
                "comments",
                &self.comments,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TokenInfo {
        #[inline]
        fn clone(&self) -> TokenInfo {
            TokenInfo {
                comments: ::core::clone::Clone::clone(&self.comments),
                span: ::core::clone::Clone::clone(&self.span),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TokenInfo {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TokenInfo {
        #[inline]
        fn eq(&self, other: &TokenInfo) -> bool {
            self.comments == other.comments && self.span == other.span
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for TokenInfo {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CommentsInfo>;
            let _: ::core::cmp::AssertParamIsEq<Span>;
        }
    }
    impl TokenInfo {
        pub fn make(
            comments_info: CommentsInfo,
            start: Position,
            end: Position,
        ) -> TokenInfo {
            TokenInfo {
                comments: comments_info,
                span: Span { start, end },
            }
        }
        /// Merges the information of other comment into the
        /// information of this comment.
        /// For details look at [`CommentsInfo::consume_info`]
        /// and [`Span::add`].
        pub fn consume_info(&mut self, other: Self) -> () {
            self.comments.consume_info(other.comments);
            self.span = self.span + other.span;
        }
    }
    /// A new type around a token Info.
    /// The principal use of this is for the grammar to type check as
    /// separators and brackets.
    pub struct TokenInfoWithPhantom<P> {
        info: TokenInfo,
        _phantom: PhantomData<P>,
    }
    impl<P> From<TokenInfoWithPhantom<P>> for TokenInfo {
        fn from(value: TokenInfoWithPhantom<P>) -> Self {
            value.info
        }
    }
    impl<P> From<TokenInfo> for TokenInfoWithPhantom<P> {
        fn from(value: TokenInfo) -> Self {
            TokenInfoWithPhantom {
                info: value,
                _phantom: Default::default(),
            }
        }
    }
    /// A Token has two pieces, a value (the content) and information
    /// like the comments around it and the source position.
    /// We never build a [`Token`] for punctuation elements or keywords,
    /// instead we build a [`TokenInfoWithPhantom`] with the appropriate
    /// phantom type.
    pub struct Token<T> {
        pub value: T,
        #[equivalence(ignore)]
        pub info: TokenInfo,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Token<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Token",
                "value",
                &self.value,
                "info",
                &&self.info,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for Token<T> {
        #[inline]
        fn clone(&self) -> Token<T> {
            Token {
                value: ::core::clone::Clone::clone(&self.value),
                info: ::core::clone::Clone::clone(&self.info),
            }
        }
    }
    impl<T> octizys_common::equivalence::Equivalence for Token<T>
    where
        T: octizys_common::equivalence::Equivalence,
    {
        fn equivalent(&self, other: &Self) -> bool {
            self.value.equivalent(&other.value)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_value = self.value.equivalence_or_diff(&other.value);
            if result_value.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr = NonLineBreakStr::new("Token");
                let doc_value = result_value
                    .map_or_else(|x| x, |_| parens(self.value.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_value]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("Token");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.value.represent()))
        }
    }
    #[automatically_derived]
    impl<T> ::core::marker::StructuralPartialEq for Token<T> {}
    #[automatically_derived]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Token<T> {
        #[inline]
        fn eq(&self, other: &Token<T>) -> bool {
            self.value == other.value && self.info == other.info
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::Eq> ::core::cmp::Eq for Token<T> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<T>;
            let _: ::core::cmp::AssertParamIsEq<TokenInfo>;
        }
    }
    impl<T> Token<T> {
        /// Creates a new token with the same info as the one provided
        /// and transform the content using the given function.
        pub fn map<Out>(self, f: fn(T) -> Out) -> Token<Out> {
            Token {
                value: f(self.value),
                info: self.info,
            }
        }
        pub fn consume_token<U>(&mut self, other: Token<U>) -> U {
            self.info.consume_info(other.info);
            other.value
        }
    }
    /// Any set of symbols that aren't identifiers, keywords or brackets, allowed
    /// inside a expression (and maybe in the future to types).
    pub enum OperatorName {
        Interrogation,
        Exclamation,
        Hash,
        Comma,
        Colon,
        StatementEnd,
        Dot,
        ModuleSeparator,
        Minus,
        CompositionLeft,
        CompositionRight,
        Plus,
        Power,
        Star,
        Div,
        Module,
        ShiftLeft,
        ShiftRigth,
        Map,
        MapConstRigth,
        MapConstLeft,
        Appliative,
        ApplicativeRight,
        ApplicativeLeft,
        Equality,
        NotEqual,
        LessOrEqual,
        MoreOrEqual,
        LessThan,
        MoreThan,
        And,
        Or,
        ReverseAppliation,
        DollarApplication,
        Asignation,
        At,
        Pipe,
        Alternative,
        FlippedMap,
        Annotate,
        RightArrow,
        LeftArrow,
        LambdaStart,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OperatorName {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    OperatorName::Interrogation => "Interrogation",
                    OperatorName::Exclamation => "Exclamation",
                    OperatorName::Hash => "Hash",
                    OperatorName::Comma => "Comma",
                    OperatorName::Colon => "Colon",
                    OperatorName::StatementEnd => "StatementEnd",
                    OperatorName::Dot => "Dot",
                    OperatorName::ModuleSeparator => "ModuleSeparator",
                    OperatorName::Minus => "Minus",
                    OperatorName::CompositionLeft => "CompositionLeft",
                    OperatorName::CompositionRight => "CompositionRight",
                    OperatorName::Plus => "Plus",
                    OperatorName::Power => "Power",
                    OperatorName::Star => "Star",
                    OperatorName::Div => "Div",
                    OperatorName::Module => "Module",
                    OperatorName::ShiftLeft => "ShiftLeft",
                    OperatorName::ShiftRigth => "ShiftRigth",
                    OperatorName::Map => "Map",
                    OperatorName::MapConstRigth => "MapConstRigth",
                    OperatorName::MapConstLeft => "MapConstLeft",
                    OperatorName::Appliative => "Appliative",
                    OperatorName::ApplicativeRight => "ApplicativeRight",
                    OperatorName::ApplicativeLeft => "ApplicativeLeft",
                    OperatorName::Equality => "Equality",
                    OperatorName::NotEqual => "NotEqual",
                    OperatorName::LessOrEqual => "LessOrEqual",
                    OperatorName::MoreOrEqual => "MoreOrEqual",
                    OperatorName::LessThan => "LessThan",
                    OperatorName::MoreThan => "MoreThan",
                    OperatorName::And => "And",
                    OperatorName::Or => "Or",
                    OperatorName::ReverseAppliation => "ReverseAppliation",
                    OperatorName::DollarApplication => "DollarApplication",
                    OperatorName::Asignation => "Asignation",
                    OperatorName::At => "At",
                    OperatorName::Pipe => "Pipe",
                    OperatorName::Alternative => "Alternative",
                    OperatorName::FlippedMap => "FlippedMap",
                    OperatorName::Annotate => "Annotate",
                    OperatorName::RightArrow => "RightArrow",
                    OperatorName::LeftArrow => "LeftArrow",
                    OperatorName::LambdaStart => "LambdaStart",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OperatorName {
        #[inline]
        fn clone(&self) -> OperatorName {
            match self {
                OperatorName::Interrogation => OperatorName::Interrogation,
                OperatorName::Exclamation => OperatorName::Exclamation,
                OperatorName::Hash => OperatorName::Hash,
                OperatorName::Comma => OperatorName::Comma,
                OperatorName::Colon => OperatorName::Colon,
                OperatorName::StatementEnd => OperatorName::StatementEnd,
                OperatorName::Dot => OperatorName::Dot,
                OperatorName::ModuleSeparator => OperatorName::ModuleSeparator,
                OperatorName::Minus => OperatorName::Minus,
                OperatorName::CompositionLeft => OperatorName::CompositionLeft,
                OperatorName::CompositionRight => {
                    OperatorName::CompositionRight
                }
                OperatorName::Plus => OperatorName::Plus,
                OperatorName::Power => OperatorName::Power,
                OperatorName::Star => OperatorName::Star,
                OperatorName::Div => OperatorName::Div,
                OperatorName::Module => OperatorName::Module,
                OperatorName::ShiftLeft => OperatorName::ShiftLeft,
                OperatorName::ShiftRigth => OperatorName::ShiftRigth,
                OperatorName::Map => OperatorName::Map,
                OperatorName::MapConstRigth => OperatorName::MapConstRigth,
                OperatorName::MapConstLeft => OperatorName::MapConstLeft,
                OperatorName::Appliative => OperatorName::Appliative,
                OperatorName::ApplicativeRight => {
                    OperatorName::ApplicativeRight
                }
                OperatorName::ApplicativeLeft => OperatorName::ApplicativeLeft,
                OperatorName::Equality => OperatorName::Equality,
                OperatorName::NotEqual => OperatorName::NotEqual,
                OperatorName::LessOrEqual => OperatorName::LessOrEqual,
                OperatorName::MoreOrEqual => OperatorName::MoreOrEqual,
                OperatorName::LessThan => OperatorName::LessThan,
                OperatorName::MoreThan => OperatorName::MoreThan,
                OperatorName::And => OperatorName::And,
                OperatorName::Or => OperatorName::Or,
                OperatorName::ReverseAppliation => {
                    OperatorName::ReverseAppliation
                }
                OperatorName::DollarApplication => {
                    OperatorName::DollarApplication
                }
                OperatorName::Asignation => OperatorName::Asignation,
                OperatorName::At => OperatorName::At,
                OperatorName::Pipe => OperatorName::Pipe,
                OperatorName::Alternative => OperatorName::Alternative,
                OperatorName::FlippedMap => OperatorName::FlippedMap,
                OperatorName::Annotate => OperatorName::Annotate,
                OperatorName::RightArrow => OperatorName::RightArrow,
                OperatorName::LeftArrow => OperatorName::LeftArrow,
                OperatorName::LambdaStart => OperatorName::LambdaStart,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for OperatorName {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for OperatorName {
        #[inline]
        fn eq(&self, other: &OperatorName) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for OperatorName {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for OperatorName {
        #[inline]
        fn partial_cmp(
            &self,
            other: &OperatorName,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for OperatorName {
        #[inline]
        fn cmp(&self, other: &OperatorName) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for OperatorName {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl octizys_common::equivalence::Equivalence for OperatorName {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Interrogation, Self::Interrogation) => true,
                (Self::Exclamation, Self::Exclamation) => true,
                (Self::Hash, Self::Hash) => true,
                (Self::Comma, Self::Comma) => true,
                (Self::Colon, Self::Colon) => true,
                (Self::StatementEnd, Self::StatementEnd) => true,
                (Self::Dot, Self::Dot) => true,
                (Self::ModuleSeparator, Self::ModuleSeparator) => true,
                (Self::Minus, Self::Minus) => true,
                (Self::CompositionLeft, Self::CompositionLeft) => true,
                (Self::CompositionRight, Self::CompositionRight) => true,
                (Self::Plus, Self::Plus) => true,
                (Self::Power, Self::Power) => true,
                (Self::Star, Self::Star) => true,
                (Self::Div, Self::Div) => true,
                (Self::Module, Self::Module) => true,
                (Self::ShiftLeft, Self::ShiftLeft) => true,
                (Self::ShiftRigth, Self::ShiftRigth) => true,
                (Self::Map, Self::Map) => true,
                (Self::MapConstRigth, Self::MapConstRigth) => true,
                (Self::MapConstLeft, Self::MapConstLeft) => true,
                (Self::Appliative, Self::Appliative) => true,
                (Self::ApplicativeRight, Self::ApplicativeRight) => true,
                (Self::ApplicativeLeft, Self::ApplicativeLeft) => true,
                (Self::Equality, Self::Equality) => true,
                (Self::NotEqual, Self::NotEqual) => true,
                (Self::LessOrEqual, Self::LessOrEqual) => true,
                (Self::MoreOrEqual, Self::MoreOrEqual) => true,
                (Self::LessThan, Self::LessThan) => true,
                (Self::MoreThan, Self::MoreThan) => true,
                (Self::And, Self::And) => true,
                (Self::Or, Self::Or) => true,
                (Self::ReverseAppliation, Self::ReverseAppliation) => true,
                (Self::DollarApplication, Self::DollarApplication) => true,
                (Self::Asignation, Self::Asignation) => true,
                (Self::At, Self::At) => true,
                (Self::Pipe, Self::Pipe) => true,
                (Self::Alternative, Self::Alternative) => true,
                (Self::FlippedMap, Self::FlippedMap) => true,
                (Self::Annotate, Self::Annotate) => true,
                (Self::RightArrow, Self::RightArrow) => true,
                (Self::LeftArrow, Self::LeftArrow) => true,
                (Self::LambdaStart, Self::LambdaStart) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Interrogation, Self::Interrogation) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Exclamation, Self::Exclamation) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Hash, Self::Hash) => ::core::result::Result::Ok(()),
                (Self::Comma, Self::Comma) => ::core::result::Result::Ok(()),
                (Self::Colon, Self::Colon) => ::core::result::Result::Ok(()),
                (Self::StatementEnd, Self::StatementEnd) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Dot, Self::Dot) => ::core::result::Result::Ok(()),
                (Self::ModuleSeparator, Self::ModuleSeparator) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Minus, Self::Minus) => ::core::result::Result::Ok(()),
                (Self::CompositionLeft, Self::CompositionLeft) => {
                    ::core::result::Result::Ok(())
                }
                (Self::CompositionRight, Self::CompositionRight) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Plus, Self::Plus) => ::core::result::Result::Ok(()),
                (Self::Power, Self::Power) => ::core::result::Result::Ok(()),
                (Self::Star, Self::Star) => ::core::result::Result::Ok(()),
                (Self::Div, Self::Div) => ::core::result::Result::Ok(()),
                (Self::Module, Self::Module) => ::core::result::Result::Ok(()),
                (Self::ShiftLeft, Self::ShiftLeft) => {
                    ::core::result::Result::Ok(())
                }
                (Self::ShiftRigth, Self::ShiftRigth) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Map, Self::Map) => ::core::result::Result::Ok(()),
                (Self::MapConstRigth, Self::MapConstRigth) => {
                    ::core::result::Result::Ok(())
                }
                (Self::MapConstLeft, Self::MapConstLeft) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Appliative, Self::Appliative) => {
                    ::core::result::Result::Ok(())
                }
                (Self::ApplicativeRight, Self::ApplicativeRight) => {
                    ::core::result::Result::Ok(())
                }
                (Self::ApplicativeLeft, Self::ApplicativeLeft) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Equality, Self::Equality) => {
                    ::core::result::Result::Ok(())
                }
                (Self::NotEqual, Self::NotEqual) => {
                    ::core::result::Result::Ok(())
                }
                (Self::LessOrEqual, Self::LessOrEqual) => {
                    ::core::result::Result::Ok(())
                }
                (Self::MoreOrEqual, Self::MoreOrEqual) => {
                    ::core::result::Result::Ok(())
                }
                (Self::LessThan, Self::LessThan) => {
                    ::core::result::Result::Ok(())
                }
                (Self::MoreThan, Self::MoreThan) => {
                    ::core::result::Result::Ok(())
                }
                (Self::And, Self::And) => ::core::result::Result::Ok(()),
                (Self::Or, Self::Or) => ::core::result::Result::Ok(()),
                (Self::ReverseAppliation, Self::ReverseAppliation) => {
                    ::core::result::Result::Ok(())
                }
                (Self::DollarApplication, Self::DollarApplication) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Asignation, Self::Asignation) => {
                    ::core::result::Result::Ok(())
                }
                (Self::At, Self::At) => ::core::result::Result::Ok(()),
                (Self::Pipe, Self::Pipe) => ::core::result::Result::Ok(()),
                (Self::Alternative, Self::Alternative) => {
                    ::core::result::Result::Ok(())
                }
                (Self::FlippedMap, Self::FlippedMap) => {
                    ::core::result::Result::Ok(())
                }
                (Self::Annotate, Self::Annotate) => {
                    ::core::result::Result::Ok(())
                }
                (Self::RightArrow, Self::RightArrow) => {
                    ::core::result::Result::Ok(())
                }
                (Self::LeftArrow, Self::LeftArrow) => {
                    ::core::result::Result::Ok(())
                }
                (Self::LambdaStart, Self::LambdaStart) => {
                    ::core::result::Result::Ok(())
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Interrogation => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Interrogation");
                    static_str(NAME)
                }
                Self::Exclamation => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Exclamation");
                    static_str(NAME)
                }
                Self::Hash => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Hash");
                    static_str(NAME)
                }
                Self::Comma => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Comma");
                    static_str(NAME)
                }
                Self::Colon => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Colon");
                    static_str(NAME)
                }
                Self::StatementEnd => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::StatementEnd");
                    static_str(NAME)
                }
                Self::Dot => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Dot");
                    static_str(NAME)
                }
                Self::ModuleSeparator => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::ModuleSeparator");
                    static_str(NAME)
                }
                Self::Minus => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Minus");
                    static_str(NAME)
                }
                Self::CompositionLeft => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::CompositionLeft");
                    static_str(NAME)
                }
                Self::CompositionRight => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::CompositionRight");
                    static_str(NAME)
                }
                Self::Plus => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Plus");
                    static_str(NAME)
                }
                Self::Power => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Power");
                    static_str(NAME)
                }
                Self::Star => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Star");
                    static_str(NAME)
                }
                Self::Div => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Div");
                    static_str(NAME)
                }
                Self::Module => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Module");
                    static_str(NAME)
                }
                Self::ShiftLeft => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::ShiftLeft");
                    static_str(NAME)
                }
                Self::ShiftRigth => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::ShiftRigth");
                    static_str(NAME)
                }
                Self::Map => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Map");
                    static_str(NAME)
                }
                Self::MapConstRigth => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::MapConstRigth");
                    static_str(NAME)
                }
                Self::MapConstLeft => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::MapConstLeft");
                    static_str(NAME)
                }
                Self::Appliative => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Appliative");
                    static_str(NAME)
                }
                Self::ApplicativeRight => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::ApplicativeRight");
                    static_str(NAME)
                }
                Self::ApplicativeLeft => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::ApplicativeLeft");
                    static_str(NAME)
                }
                Self::Equality => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Equality");
                    static_str(NAME)
                }
                Self::NotEqual => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::NotEqual");
                    static_str(NAME)
                }
                Self::LessOrEqual => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::LessOrEqual");
                    static_str(NAME)
                }
                Self::MoreOrEqual => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::MoreOrEqual");
                    static_str(NAME)
                }
                Self::LessThan => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::LessThan");
                    static_str(NAME)
                }
                Self::MoreThan => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::MoreThan");
                    static_str(NAME)
                }
                Self::And => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::And");
                    static_str(NAME)
                }
                Self::Or => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Or");
                    static_str(NAME)
                }
                Self::ReverseAppliation => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::ReverseAppliation");
                    static_str(NAME)
                }
                Self::DollarApplication => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::DollarApplication");
                    static_str(NAME)
                }
                Self::Asignation => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Asignation");
                    static_str(NAME)
                }
                Self::At => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::At");
                    static_str(NAME)
                }
                Self::Pipe => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Pipe");
                    static_str(NAME)
                }
                Self::Alternative => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Alternative");
                    static_str(NAME)
                }
                Self::FlippedMap => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::FlippedMap");
                    static_str(NAME)
                }
                Self::Annotate => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::Annotate");
                    static_str(NAME)
                }
                Self::RightArrow => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::RightArrow");
                    static_str(NAME)
                }
                Self::LeftArrow => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::LeftArrow");
                    static_str(NAME)
                }
                Self::LambdaStart => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OperatorName::LambdaStart");
                    static_str(NAME)
                }
            }
        }
    }
    /// Representation of a variable qualified by some path.
    ///
    /// # Example
    ///
    /// ```txt
    /// core::main::path
    /// ```
    pub struct ImportedVariable {
        pub path: LogicPath,
        pub name: Identifier,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ImportedVariable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ImportedVariable",
                "path",
                &self.path,
                "name",
                &&self.name,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ImportedVariable {
        #[inline]
        fn clone(&self) -> ImportedVariable {
            ImportedVariable {
                path: ::core::clone::Clone::clone(&self.path),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ImportedVariable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ImportedVariable {
        #[inline]
        fn eq(&self, other: &ImportedVariable) -> bool {
            self.path == other.path && self.name == other.name
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ImportedVariable {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<LogicPath>;
            let _: ::core::cmp::AssertParamIsEq<Identifier>;
        }
    }
    impl octizys_common::equivalence::Equivalence for ImportedVariable {
        fn equivalent(&self, other: &Self) -> bool {
            self.path.equivalent(&other.path)
                & self.name.equivalent(&other.name)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_path = self.path.equivalence_or_diff(&other.path);
            let result_name = self.name.equivalence_or_diff(&other.name);
            if result_path.is_ok() & result_name.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("ImportedVariable");
                let doc_path = result_path
                    .map_or_else(|x| x, |_| parens(self.path.represent()));
                let doc_name = result_name
                    .map_or_else(|x| x, |_| parens(self.name.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_path,
                        hard_break(),
                        doc_name,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("ImportedVariable");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.path.represent(),
                    hard_break(),
                    self.name.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    /// Some structure surrounded by delimiters like `()` and `{}`
    #[equivalence(ignore = Enclosure)]
    pub struct Between<T, Enclosure>
    where
        Enclosure: Delimiters,
    {
        #[equivalence(ignore)]
        pub left: TokenInfo,
        #[equivalence(ignore)]
        pub right: TokenInfo,
        pub value: T,
        #[equivalence(ignore)]
        pub _enclosure_phantom: PhantomData<Enclosure>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug, Enclosure: ::core::fmt::Debug>
        ::core::fmt::Debug for Between<T, Enclosure>
    where
        Enclosure: Delimiters,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Between",
                "left",
                &self.left,
                "right",
                &self.right,
                "value",
                &self.value,
                "_enclosure_phantom",
                &&self._enclosure_phantom,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone, Enclosure: ::core::clone::Clone>
        ::core::clone::Clone for Between<T, Enclosure>
    where
        Enclosure: Delimiters,
    {
        #[inline]
        fn clone(&self) -> Between<T, Enclosure> {
            Between {
                left: ::core::clone::Clone::clone(&self.left),
                right: ::core::clone::Clone::clone(&self.right),
                value: ::core::clone::Clone::clone(&self.value),
                _enclosure_phantom: ::core::clone::Clone::clone(
                    &self._enclosure_phantom,
                ),
            }
        }
    }
    impl<T, Enclosure> octizys_common::equivalence::Equivalence
        for Between<T, Enclosure>
    where
        Enclosure: Delimiters,
        T: octizys_common::equivalence::Equivalence,
    {
        fn equivalent(&self, other: &Self) -> bool {
            self.value.equivalent(&other.value)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_value = self.value.equivalence_or_diff(&other.value);
            if result_value.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr = NonLineBreakStr::new("Between");
                let doc_value = result_value
                    .map_or_else(|x| x, |_| parens(self.value.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_value]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("Between");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.value.represent()))
        }
    }
    #[automatically_derived]
    impl<T, Enclosure> ::core::marker::StructuralPartialEq for Between<T, Enclosure> where
        Enclosure: Delimiters
    {
    }
    #[automatically_derived]
    impl<T: ::core::cmp::PartialEq, Enclosure: ::core::cmp::PartialEq>
        ::core::cmp::PartialEq for Between<T, Enclosure>
    where
        Enclosure: Delimiters,
    {
        #[inline]
        fn eq(&self, other: &Between<T, Enclosure>) -> bool {
            self.left == other.left
                && self.right == other.right
                && self.value == other.value
                && self._enclosure_phantom == other._enclosure_phantom
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::Eq, Enclosure: ::core::cmp::Eq> ::core::cmp::Eq
        for Between<T, Enclosure>
    where
        Enclosure: Delimiters,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<TokenInfo>;
            let _: ::core::cmp::AssertParamIsEq<T>;
            let _: ::core::cmp::AssertParamIsEq<PhantomData<Enclosure>>;
        }
    }
    /// A item on a list of items separated by some separator like `,` or `|`.
    /// This item contains the separation comma between itself and the
    /// previous item.
    ///
    /// Example:
    ///
    /// ```txt
    /// ,b
    /// ```
    #[equivalence(ignore = SeparatorPhantom)]
    pub struct TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[equivalence(ignore)]
        pub separator: TokenInfo,
        pub item: T,
        #[equivalence(ignore)]
        pub _phantom_separator: PhantomData<SeparatorPhantom>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug, SeparatorPhantom: ::core::fmt::Debug>
        ::core::fmt::Debug for TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TrailingListItem",
                "separator",
                &self.separator,
                "item",
                &self.item,
                "_phantom_separator",
                &&self._phantom_separator,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone, SeparatorPhantom: ::core::clone::Clone>
        ::core::clone::Clone for TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        fn clone(&self) -> TrailingListItem<T, SeparatorPhantom> {
            TrailingListItem {
                separator: ::core::clone::Clone::clone(&self.separator),
                item: ::core::clone::Clone::clone(&self.item),
                _phantom_separator: ::core::clone::Clone::clone(
                    &self._phantom_separator,
                ),
            }
        }
    }
    #[automatically_derived]
    impl<T, SeparatorPhantom> ::core::marker::StructuralPartialEq
        for TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
    }
    #[automatically_derived]
    impl<
            T: ::core::cmp::PartialEq,
            SeparatorPhantom: ::core::cmp::PartialEq,
        > ::core::cmp::PartialEq for TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        fn eq(&self, other: &TrailingListItem<T, SeparatorPhantom>) -> bool {
            self.separator == other.separator
                && self.item == other.item
                && self._phantom_separator == other._phantom_separator
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::Eq, SeparatorPhantom: ::core::cmp::Eq> ::core::cmp::Eq
        for TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<TokenInfo>;
            let _: ::core::cmp::AssertParamIsEq<T>;
            let _: ::core::cmp::AssertParamIsEq<PhantomData<SeparatorPhantom>>;
        }
    }
    impl<T, SeparatorPhantom> octizys_common::equivalence::Equivalence
        for TrailingListItem<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
        T: octizys_common::equivalence::Equivalence,
    {
        fn equivalent(&self, other: &Self) -> bool {
            self.item.equivalent(&other.item)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_item = self.item.equivalence_or_diff(&other.item);
            if result_item.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("TrailingListItem");
                let doc_item = result_item
                    .map_or_else(|x| x, |_| parens(self.item.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_item]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("TrailingListItem");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.item.represent()))
        }
    }
    /// A list of items separated by some separator like `,` and `|`
    /// that allow optional final separator without item.
    ///
    /// Example
    ///
    /// ```txt
    /// a , b, c, d,
    /// ```
    ///
    /// The last `,` is optional.
    #[equivalence(ignore = SeparatorPhantom)]
    pub struct TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        pub first: T,
        pub items: Vec<TrailingListItem<T, SeparatorPhantom>>,
        #[equivalence(ignore)]
        pub trailing_sep: Option<TokenInfo>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug, SeparatorPhantom: ::core::fmt::Debug>
        ::core::fmt::Debug for TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TrailingList",
                "first",
                &self.first,
                "items",
                &self.items,
                "trailing_sep",
                &&self.trailing_sep,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone, SeparatorPhantom: ::core::clone::Clone>
        ::core::clone::Clone for TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        fn clone(&self) -> TrailingList<T, SeparatorPhantom> {
            TrailingList {
                first: ::core::clone::Clone::clone(&self.first),
                items: ::core::clone::Clone::clone(&self.items),
                trailing_sep: ::core::clone::Clone::clone(&self.trailing_sep),
            }
        }
    }
    #[automatically_derived]
    impl<T, SeparatorPhantom> ::core::marker::StructuralPartialEq
        for TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
    }
    #[automatically_derived]
    impl<
            T: ::core::cmp::PartialEq,
            SeparatorPhantom: ::core::cmp::PartialEq,
        > ::core::cmp::PartialEq for TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        fn eq(&self, other: &TrailingList<T, SeparatorPhantom>) -> bool {
            self.first == other.first
                && self.items == other.items
                && self.trailing_sep == other.trailing_sep
        }
    }
    #[automatically_derived]
    impl<T: ::core::cmp::Eq, SeparatorPhantom: ::core::cmp::Eq> ::core::cmp::Eq
        for TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
    {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<T>;
            let _: ::core::cmp::AssertParamIsEq<
                Vec<TrailingListItem<T, SeparatorPhantom>>,
            >;
            let _: ::core::cmp::AssertParamIsEq<Option<TokenInfo>>;
        }
    }
    impl<T, SeparatorPhantom> octizys_common::equivalence::Equivalence
        for TrailingList<T, SeparatorPhantom>
    where
        SeparatorPhantom: Separator,
        T: octizys_common::equivalence::Equivalence,
    {
        fn equivalent(&self, other: &Self) -> bool {
            self.first.equivalent(&other.first)
                & self.items.equivalent(&other.items)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_first = self.first.equivalence_or_diff(&other.first);
            let result_items = self.items.equivalence_or_diff(&other.items);
            if result_first.is_ok() & result_items.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("TrailingList");
                let doc_first = result_first
                    .map_or_else(|x| x, |_| parens(self.first.represent()));
                let doc_items = result_items
                    .map_or_else(|x| x, |_| parens(self.items.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_first,
                        hard_break(),
                        doc_items,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("TrailingList");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.first.represent(),
                    hard_break(),
                    self.items.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    impl<T, ToInfo, SeparatorPhantom>
        From<(T, Vec<(ToInfo, T)>, Option<ToInfo>)>
        for TrailingList<T, SeparatorPhantom>
    where
        ToInfo: Into<TokenInfo>,
        SeparatorPhantom: Separator,
    {
        fn from(
            value: (T, Vec<(ToInfo, T)>, Option<ToInfo>),
        ) -> TrailingList<T, SeparatorPhantom> {
            let items = value
                .1
                .into_iter()
                .map(|(separator, item)| TrailingListItem {
                    separator: separator.into(),
                    item,
                    _phantom_separator: Default::default(),
                })
                .collect();
            let first = value.0;
            let trailing_sep = value.2;
            TrailingList {
                first,
                items,
                trailing_sep: trailing_sep.map(|x| x.into()),
            }
        }
    }
    impl From<TrailingList<Token<Identifier>, LogicPathSeparator>>
        for Token<LogicPath>
    {
        fn from(
            value: TrailingList<Token<Identifier>, LogicPathSeparator>,
        ) -> Self {
            let mut info = value.first.info;
            let mut acc = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([value.first.value]),
            );
            for i in value.items {
                acc.push(i.item.value);
                info.comments.consume_info(i.item.info.comments)
            }
            Token {
                value: acc.try_into().unwrap(),
                info,
            }
        }
    }
}
pub mod comments {
    use octizys_common::span::{Position, Span};
    use octizys_macros::Equivalence;
    use octizys_text_store::store::{NonLineBreakString, Store};
    /// Represents a single line of a comment without line breaks.
    /// We internalize all the source text in a [`Store`] and handle comments
    /// specially.
    /// This structure represents a pointer to the location of the comment
    /// together with the length of the comment (as understated by the
    /// notion of length on the store).
    /// Storing the length here, allow us to avoid access to the store.
    pub struct CommentLineContent {
        index: usize,
        len: usize,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentLineContent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CommentLineContent",
                "index",
                &self.index,
                "len",
                &&self.len,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommentLineContent {
        #[inline]
        fn clone(&self) -> CommentLineContent {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentLineContent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentLineContent {
        #[inline]
        fn eq(&self, other: &CommentLineContent) -> bool {
            self.index == other.index && self.len == other.len
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CommentLineContent {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CommentLineContent {}
    impl octizys_common::equivalence::Equivalence for CommentLineContent {
        fn equivalent(&self, other: &Self) -> bool {
            self.index.equivalent(&other.index)
                & self.len.equivalent(&other.len)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_index = self.index.equivalence_or_diff(&other.index);
            let result_len = self.len.equivalence_or_diff(&other.len);
            if result_index.is_ok() & result_len.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("CommentLineContent");
                let doc_index = result_index
                    .map_or_else(|x| x, |_| parens(self.index.represent()));
                let doc_len = result_len
                    .map_or_else(|x| x, |_| parens(self.len.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_index,
                        hard_break(),
                        doc_len,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("CommentLineContent");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.index.represent(),
                    hard_break(),
                    self.len.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    impl CommentLineContent {
        /// Stores as single string as a comment string in the store.
        /// The string is supposed to be a text without line breaks, if it
        /// contains any line break the function fails.
        /// To create a [`CommentLineContent`] from an arbitrary string
        /// without failing use [`CommentLineContent::decomposoe_register`] instead.
        pub fn make_register(value: &str, store: &mut Store) -> Option<Self> {
            store.comments.add_str(value).map(|x| CommentLineContent {
                index: x,
                len: value.len(),
            })
        }
        /// Internalizes an arbitrary string by breaking it at line breaks,
        /// and returning a vector with reference to every line.
        pub fn decompose_register(value: &str, store: &mut Store) -> Vec<Self> {
            store
                .comments
                .extend_and_get_lens(NonLineBreakString::decompose(value))
                .map(|(index, len)| CommentLineContent { index, len })
                .collect()
        }
        /// Returns the length of the stored comment as is understood by the
        /// pretty printing library.
        #[inline]
        pub fn get_len(&self) -> usize {
            self.len
        }
        /// Returns a reference to a place in the Store.
        #[inline]
        pub fn get_index(&self) -> usize {
            self.index
        }
    }
    /// Distinguish between regular comments and documentation ones.
    /// A documentation includes a space and a pipe ` |` after the
    /// start of the comment.
    /// In the future we may allow for any amount of spaces.
    pub enum CommentKind {
        Documentation,
        NonDocumentation,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentKind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    CommentKind::Documentation => "Documentation",
                    CommentKind::NonDocumentation => "NonDocumentation",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommentKind {
        #[inline]
        fn clone(&self) -> CommentKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CommentKind {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentKind {
        #[inline]
        fn eq(&self, other: &CommentKind) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CommentKind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl octizys_common::equivalence::Equivalence for CommentKind {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Documentation, Self::Documentation) => true,
                (Self::NonDocumentation, Self::NonDocumentation) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Documentation, Self::Documentation) => {
                    ::core::result::Result::Ok(())
                }
                (Self::NonDocumentation, Self::NonDocumentation) => {
                    ::core::result::Result::Ok(())
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Documentation => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("CommentKind::Documentation");
                    static_str(NAME)
                }
                Self::NonDocumentation => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("CommentKind::NonDocumentation");
                    static_str(NAME)
                }
            }
        }
    }
    /// The delimiters for a [`CommentBlock`].
    /// Currently, they are four kinds of block delimiters.
    /// All of them are a `{` followed by between 1 and 4
    /// `-`, then the comment of the block and
    /// finish with the same amount of `-` followed
    /// by a `}`.
    ///
    /// <div class="warning">
    ///
    /// This means that a comment like:
    ///
    /// ```txt
    /// {-- some comment {--- inner comment ---} remain --}
    /// ```
    ///
    /// Would be processed as
    ///
    /// ```txt
    /// {-- some comment {--- inner comment ---}
    /// ```
    ///
    /// Causing a syntax error at `--}`.
    /// To nest comments is recommended to begin using a singe hyphen
    /// and continue incrementing them as needed.
    ///
    /// </div>
    ///
    /// We acknowledge the need for nested block comments but at
    /// the same time we believe that a finite amount of them
    /// is enough for most uses if not all of them.
    pub enum CommentBraceKind {
        Brace0,
        Brace1,
        Brace2,
        Brace3,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentBraceKind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    CommentBraceKind::Brace0 => "Brace0",
                    CommentBraceKind::Brace1 => "Brace1",
                    CommentBraceKind::Brace2 => "Brace2",
                    CommentBraceKind::Brace3 => "Brace3",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommentBraceKind {
        #[inline]
        fn clone(&self) -> CommentBraceKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CommentBraceKind {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentBraceKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentBraceKind {
        #[inline]
        fn eq(&self, other: &CommentBraceKind) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CommentBraceKind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl octizys_common::equivalence::Equivalence for CommentBraceKind {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Brace0, Self::Brace0) => true,
                (Self::Brace1, Self::Brace1) => true,
                (Self::Brace2, Self::Brace2) => true,
                (Self::Brace3, Self::Brace3) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Brace0, Self::Brace0) => ::core::result::Result::Ok(()),
                (Self::Brace1, Self::Brace1) => ::core::result::Result::Ok(()),
                (Self::Brace2, Self::Brace2) => ::core::result::Result::Ok(()),
                (Self::Brace3, Self::Brace3) => ::core::result::Result::Ok(()),
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Brace0 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("CommentBraceKind::Brace0");
                    static_str(NAME)
                }
                Self::Brace1 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("CommentBraceKind::Brace1");
                    static_str(NAME)
                }
                Self::Brace2 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("CommentBraceKind::Brace2");
                    static_str(NAME)
                }
                Self::Brace3 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("CommentBraceKind::Brace3");
                    static_str(NAME)
                }
            }
        }
    }
    impl CommentBraceKind {
        /// Returns the total length in bytes of the start of a block
        /// with the given kind, this means that it has the length of
        /// the full `{--`.
        pub fn len(self) -> usize {
            match self {
                Self::Brace0 => 2,
                Self::Brace1 => 3,
                Self::Brace2 => 4,
                Self::Brace3 => 5,
            }
        }
    }
    /// Represents the beginning of a [`CommentLine`], they
    /// can be any of :
    ///
    /// - `//`
    /// - `--`
    ///
    pub enum LineCommentStart {
        DoubleHypen,
        DoubleSlash,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LineCommentStart {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    LineCommentStart::DoubleHypen => "DoubleHypen",
                    LineCommentStart::DoubleSlash => "DoubleSlash",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LineCommentStart {}
    #[automatically_derived]
    impl ::core::clone::Clone for LineCommentStart {
        #[inline]
        fn clone(&self) -> LineCommentStart {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for LineCommentStart {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LineCommentStart {
        #[inline]
        fn eq(&self, other: &LineCommentStart) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for LineCommentStart {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl octizys_common::equivalence::Equivalence for LineCommentStart {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::DoubleHypen, Self::DoubleHypen) => true,
                (Self::DoubleSlash, Self::DoubleSlash) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::DoubleHypen, Self::DoubleHypen) => {
                    ::core::result::Result::Ok(())
                }
                (Self::DoubleSlash, Self::DoubleSlash) => {
                    ::core::result::Result::Ok(())
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::DoubleHypen => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("LineCommentStart::DoubleHypen");
                    static_str(NAME)
                }
                Self::DoubleSlash => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("LineCommentStart::DoubleSlash");
                    static_str(NAME)
                }
            }
        }
    }
    /// A representation of a multi-line of comment in the source.
    /// Every comment has the following components:
    /// - A [`CommentLine::kind`] to distinguish between documentation and
    ///   regular comments.
    /// - A [`CommentLine::start`], we support multiple brackets for
    ///   block comments, see [`CommentBraceKind`].
    /// - A [`CommentLine::content`] it has all the content of multiple lines
    ///   of the comment except from the line breaks and the comment brackets.
    /// - A [`CommentBlock::span`], the region on the source in which the
    ///   comment exists.
    ///
    /// #Examples
    ///
    /// ```txt
    /// {- some
    ///    regular
    ///    block comment
    /// -}
    ///
    /// {- | documentation
    ///   block
    ///   comment -}
    ///
    /// {--- another
    ///   regular
    ///   block
    ///   comment
    ///   ---}
    /// ```
    pub struct CommentBlock {
        pub kind: CommentKind,
        pub brace: CommentBraceKind,
        pub content: Vec<CommentLineContent>,
        #[equivalence(ignore)]
        pub span: Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentBlock {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "CommentBlock",
                "kind",
                &self.kind,
                "brace",
                &self.brace,
                "content",
                &self.content,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommentBlock {
        #[inline]
        fn clone(&self) -> CommentBlock {
            CommentBlock {
                kind: ::core::clone::Clone::clone(&self.kind),
                brace: ::core::clone::Clone::clone(&self.brace),
                content: ::core::clone::Clone::clone(&self.content),
                span: ::core::clone::Clone::clone(&self.span),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentBlock {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentBlock {
        #[inline]
        fn eq(&self, other: &CommentBlock) -> bool {
            self.kind == other.kind
                && self.brace == other.brace
                && self.content == other.content
                && self.span == other.span
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CommentBlock {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CommentKind>;
            let _: ::core::cmp::AssertParamIsEq<CommentBraceKind>;
            let _: ::core::cmp::AssertParamIsEq<Vec<CommentLineContent>>;
            let _: ::core::cmp::AssertParamIsEq<Span>;
        }
    }
    impl octizys_common::equivalence::Equivalence for CommentBlock {
        fn equivalent(&self, other: &Self) -> bool {
            self.kind.equivalent(&other.kind)
                & self.brace.equivalent(&other.brace)
                & self.content.equivalent(&other.content)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_kind = self.kind.equivalence_or_diff(&other.kind);
            let result_brace = self.brace.equivalence_or_diff(&other.brace);
            let result_content =
                self.content.equivalence_or_diff(&other.content);
            if result_kind.is_ok()
                & result_brace.is_ok()
                & result_content.is_ok()
                & true
            {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("CommentBlock");
                let doc_kind = result_kind
                    .map_or_else(|x| x, |_| parens(self.kind.represent()));
                let doc_brace = result_brace
                    .map_or_else(|x| x, |_| parens(self.brace.represent()));
                let doc_content = result_content
                    .map_or_else(|x| x, |_| parens(self.content.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_kind,
                        hard_break(),
                        doc_brace,
                        hard_break(),
                        doc_content,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("CommentBlock");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.kind.represent(),
                    hard_break(),
                    self.brace.represent(),
                    hard_break(),
                    self.content.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    impl CommentBlock {
        pub fn make(
            kind: CommentKind,
            brace: CommentBraceKind,
            full_text: &str,
            start_pos: Position,
            end_pos: Position,
            store: &mut Store,
        ) -> Self {
            let content =
                CommentLineContent::decompose_register(full_text, store);
            CommentBlock {
                kind,
                brace,
                content,
                span: Span {
                    start: start_pos,
                    end: end_pos,
                },
            }
        }
    }
    /// A representation of a single line of comments in the source.
    /// Every line comment has the following components:
    /// - A [`CommentLine::kind`] to distinguish between documentation and
    ///   regular comments.
    /// - A [`CommentLine::start`], we support two ways to begin a comment
    ///   in a line, by using either `//` or `--`.
    /// - A [`CommentLine::content`] it has all the content of a line
    ///   comment after the start delimiter except from the line break.
    /// - A [`CommentLine::span`], the region on the source in which the comment exists.
    ///
    /// #Examples
    /// ```txt
    /// -- some regular line comment.
    /// // another line comment comment.
    /// // | a documentation line comment.
    /// -- | another documentation line comment.
    /// ```
    pub struct CommentLine {
        pub kind: CommentKind,
        pub start: LineCommentStart,
        pub content: CommentLineContent,
        #[equivalence(ignore)]
        pub span: Span,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentLine {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "CommentLine",
                "kind",
                &self.kind,
                "start",
                &self.start,
                "content",
                &self.content,
                "span",
                &&self.span,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommentLine {
        #[inline]
        fn clone(&self) -> CommentLine {
            let _: ::core::clone::AssertParamIsClone<CommentKind>;
            let _: ::core::clone::AssertParamIsClone<LineCommentStart>;
            let _: ::core::clone::AssertParamIsClone<CommentLineContent>;
            let _: ::core::clone::AssertParamIsClone<Span>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentLine {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentLine {
        #[inline]
        fn eq(&self, other: &CommentLine) -> bool {
            self.kind == other.kind
                && self.start == other.start
                && self.content == other.content
                && self.span == other.span
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CommentLine {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CommentKind>;
            let _: ::core::cmp::AssertParamIsEq<LineCommentStart>;
            let _: ::core::cmp::AssertParamIsEq<CommentLineContent>;
            let _: ::core::cmp::AssertParamIsEq<Span>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CommentLine {}
    impl octizys_common::equivalence::Equivalence for CommentLine {
        fn equivalent(&self, other: &Self) -> bool {
            self.kind.equivalent(&other.kind)
                & self.start.equivalent(&other.start)
                & self.content.equivalent(&other.content)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_kind = self.kind.equivalence_or_diff(&other.kind);
            let result_start = self.start.equivalence_or_diff(&other.start);
            let result_content =
                self.content.equivalence_or_diff(&other.content);
            if result_kind.is_ok()
                & result_start.is_ok()
                & result_content.is_ok()
                & true
            {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("CommentLine");
                let doc_kind = result_kind
                    .map_or_else(|x| x, |_| parens(self.kind.represent()));
                let doc_start = result_start
                    .map_or_else(|x| x, |_| parens(self.start.represent()));
                let doc_content = result_content
                    .map_or_else(|x| x, |_| parens(self.content.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_kind,
                        hard_break(),
                        doc_start,
                        hard_break(),
                        doc_content,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("CommentLine");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.kind.represent(),
                    hard_break(),
                    self.start.represent(),
                    hard_break(),
                    self.content.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    /// We can categorize the comments in two types:
    /// - [`Comment::Line`] comments that began in a line and ends
    ///   at the end of line.
    /// - [`Comment::Block`] multi-line commentaries.
    ///
    /// <div class="warning">Documentation comments can be in any format, but the
    /// official documentation generator is going to use common Markdown
    /// or some other dialect of it.</div>
    ///
    /// We may represent in the future additional kind of comments like:
    ///
    /// - `Todo:`
    /// - `Warning:`
    /// - `Note:`
    ///
    /// At the current time we don't know if this is going to be useful.
    pub enum Comment {
        Line(CommentLine),
        Block(CommentBlock),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Comment::Line(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Line", &__self_0,
                    )
                }
                Comment::Block(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Block", &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comment {
        #[inline]
        fn clone(&self) -> Comment {
            match self {
                Comment::Line(__self_0) => {
                    Comment::Line(::core::clone::Clone::clone(__self_0))
                }
                Comment::Block(__self_0) => {
                    Comment::Block(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Comment {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Comment {
        #[inline]
        fn eq(&self, other: &Comment) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (Comment::Line(__self_0), Comment::Line(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (Comment::Block(__self_0), Comment::Block(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Comment {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CommentLine>;
            let _: ::core::cmp::AssertParamIsEq<CommentBlock>;
        }
    }
    impl octizys_common::equivalence::Equivalence for Comment {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Line(self_0), Self::Line(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (Self::Block(self_0), Self::Block(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Line(self_0), Self::Line(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Comment::Line");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (Self::Block(self_0), Self::Block(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Comment::Block");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Line(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Comment::Line");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Block(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Comment::Block");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
            }
        }
    }
    impl Comment {
        pub fn get_span(&self) -> Span {
            match self {
                Self::Line(CommentLine { span, .. }) => span.to_owned(),
                Self::Block(CommentBlock { span, .. }) => span.to_owned(),
            }
        }
    }
    impl From<CommentLine> for Comment {
        fn from(value: CommentLine) -> Comment {
            Comment::Line(value)
        }
    }
    impl From<CommentBlock> for Comment {
        fn from(value: CommentBlock) -> Comment {
            Comment::Block(value)
        }
    }
    /// Every token in a file can have any kind of comment above it and
    /// can have a single line comment in the same line after the token.
    ///
    /// #Example
    ///
    /// ```txt
    /// -- some line comment
    /// // other comment
    /// -- | Documentation line comment
    /// token  // More comments
    /// ```
    /// This example has 3 comments above the token
    /// and a comment after it.
    pub struct CommentsInfo {
        pub before: Vec<Comment>,
        pub after: Vec<Comment>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommentsInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CommentsInfo",
                "before",
                &self.before,
                "after",
                &&self.after,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommentsInfo {
        #[inline]
        fn clone(&self) -> CommentsInfo {
            CommentsInfo {
                before: ::core::clone::Clone::clone(&self.before),
                after: ::core::clone::Clone::clone(&self.after),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CommentsInfo {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CommentsInfo {
        #[inline]
        fn eq(&self, other: &CommentsInfo) -> bool {
            self.before == other.before && self.after == other.after
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CommentsInfo {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<Comment>>;
            let _: ::core::cmp::AssertParamIsEq<Vec<Comment>>;
        }
    }
    impl Default for CommentsInfo {
        fn default() -> Self {
            CommentsInfo {
                before: ::alloc::vec::Vec::new(),
                after: ::alloc::vec::Vec::new(),
            }
        }
    }
    impl CommentsInfo {
        /// A commentaries' information that doesn't have any information inside.:
        /// This is useful if we need to attach information to a node before
        /// we have the information available.
        pub fn empty() -> Self {
            Self::default()
        }
        /// Add the elements of an iterator of comments to the comments
        /// in the before field.
        /// The comments are added after all the original comments in
        /// the before field.
        pub fn extend<T>(&mut self, remmain: T) -> ()
        where
            T: IntoIterator<Item = Comment>,
        {
            self.before.extend(remmain)
        }
        /// Add a single comment to the `before` field.
        pub fn push(&mut self, new: Comment) -> () {
            self.before.push(new)
        }
        /// Move the comments after a token to before it.
        pub fn move_after_to_before(&mut self) -> () {
            self.before
                .extend(self.after.iter().rev().map(|x| x.clone()));
            self.after = ::alloc::vec::Vec::new();
        }
        /// Takes another comment info (maybe from another token)
        /// and combines it with the current one.
        /// Not every comment before or after a token can made sense,
        /// and we may choose to move some comments from its original place
        /// to another one where it made more sense.
        ///
        /// # Examples
        ///
        /// In the following block:
        ///
        /// ```txt
        ///  -- | Some comment about `a`
        /// let a =
        ///   (
        ///     -- | The first before comment of `1`
        ///     1, -- | The before comment of `,`, going to be moved to the before comments of `1`
        ///     2,
        ///     3, 4
        ///   )
        /// ```
        ///
        /// The third comment is attached to the comma as an after comment.
        /// The comment can be referring to the second item of the tuple
        /// or the third. If we decide to move it to the second item
        /// the end structure is the one described in the comments.
        pub fn consume_info(&mut self, other: CommentsInfo) -> () {
            self.move_after_to_before();
            let CommentsInfo { before, after } = other;
            self.extend(before.into_iter());
            self.extend(after.into_iter());
        }
        /// Take a CommentsInfo and transform all the contiguous occurrences
        /// of comments of the same type in a single block
        ///
        /// #Example
        ///
        /// This
        ///
        /// ```txt
        /// -- | 1
        /// -- | 2
        /// -- | 3
        /// // | 4
        /// // | 6
        /// -- 8
        /// {- 9 -}
        /// // | 10
        /// // | 11
        /// ```
        ///
        /// Becomes
        ///
        /// ```txt
        /// {- | 1
        ///      2
        ///      3
        ///      4
        ///      6 -}
        /// {- 8
        ///    9 -}
        /// {- | 10
        ///      11
        /// -}
        /// ```
        pub fn compact_comments(&mut self) -> () {}
    }
}
pub mod expressions {
    use crate::base::{
        Between, Braces, Comma, ImportedVariable, OperatorName, Parens, Token,
        TokenInfo, TrailingList,
    };
    use crate::literals::{
        InterpolationString, StringLiteral, UFloatingPointLiteral, UintLiteral,
    };
    use crate::patterns::PatternMatch;
    use crate::types::Type;
    use octizys_common::identifier::Identifier;
    pub struct LetBinding {
        pub pattern: PatternMatch,
        pub equal: TokenInfo,
        pub value: Expression,
        pub semicolon: TokenInfo,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LetBinding {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "LetBinding",
                "pattern",
                &self.pattern,
                "equal",
                &self.equal,
                "value",
                &self.value,
                "semicolon",
                &&self.semicolon,
            )
        }
    }
    pub struct Let {
        pub let_: TokenInfo,
        pub bindings: Vec<LetBinding>,
        pub in_: TokenInfo,
        pub expression: Box<Expression>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Let {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Let",
                "let_",
                &self.let_,
                "bindings",
                &self.bindings,
                "in_",
                &self.in_,
                "expression",
                &&self.expression,
            )
        }
    }
    pub struct CaseItem {
        pub pattern: PatternMatch,
        pub arrow: TokenInfo,
        pub expression: Box<Expression>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CaseItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "CaseItem",
                "pattern",
                &self.pattern,
                "arrow",
                &self.arrow,
                "expression",
                &&self.expression,
            )
        }
    }
    pub struct Case {
        pub case: TokenInfo,
        pub expression: Box<Expression>,
        pub of: TokenInfo,
        pub cases: Between<TrailingList<CaseItem, Comma>, Braces>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Case {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Case",
                "case",
                &self.case,
                "expression",
                &self.expression,
                "of",
                &self.of,
                "cases",
                &&self.cases,
            )
        }
    }
    pub struct BinaryOperator {
        pub left: Box<Expression>,
        pub right: Box<Expression>,
        pub name: Token<OperatorName>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BinaryOperator {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "BinaryOperator",
                "left",
                &self.left,
                "right",
                &self.right,
                "name",
                &&self.name,
            )
        }
    }
    pub struct LambdaExpression {
        pub variable: Token<Identifier>,
        pub expression: Box<Expression>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LambdaExpression {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "LambdaExpression",
                "variable",
                &self.variable,
                "expression",
                &&self.expression,
            )
        }
    }
    pub struct ApplicationExpression {
        pub start: Box<Expression>,
        pub remain: Vec<Expression>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ApplicationExpression {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ApplicationExpression",
                "start",
                &self.start,
                "remain",
                &&self.remain,
            )
        }
    }
    pub enum ExpressionRecordItem {
        SingleVariable {
            variable: Token<Identifier>,
        },
        Assignation {
            variable: Token<Identifier>,
            equal: TokenInfo,
            expression: Box<Expression>,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ExpressionRecordItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ExpressionRecordItem::SingleVariable { variable: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "SingleVariable",
                        "variable",
                        &__self_0,
                    )
                }
                ExpressionRecordItem::Assignation {
                    variable: __self_0,
                    equal: __self_1,
                    expression: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Assignation",
                    "variable",
                    __self_0,
                    "equal",
                    __self_1,
                    "expression",
                    &__self_2,
                ),
            }
        }
    }
    pub struct ExpressionSelector {
        pub expression: Box<Expression>,
        pub accessor: Token<Identifier>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ExpressionSelector {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ExpressionSelector",
                "expression",
                &self.expression,
                "accessor",
                &&self.accessor,
            )
        }
    }
    pub enum Expression {
        String(Token<StringLiteral>),
        InterpolationString(Token<InterpolationString>),
        Uint(Token<UintLiteral>),
        UFloat(Token<UFloatingPointLiteral>),
        LocalVariable(Token<Identifier>),
        ImportedVariable(Token<ImportedVariable>),
        NamedHole(Token<u64>),
        Tuple(Between<TrailingList<Box<Expression>, Comma>, Parens>),
        Record(Between<TrailingList<ExpressionRecordItem, Comma>, Braces>),
        Case(Case),
        Parens(Between<Box<Expression>, Parens>),
        Selector(ExpressionSelector),
        Interrogation {
            expression: Box<Expression>,
            symbol: TokenInfo,
        },
        TypeArgument {
            at: TokenInfo,
            _type: Type,
        },
        Let(Let),
        BinaryOperator(BinaryOperator),
        Lambda(LambdaExpression),
        Application(ApplicationExpression),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expression {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expression::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "String", &__self_0,
                    )
                }
                Expression::InterpolationString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InterpolationString",
                        &__self_0,
                    )
                }
                Expression::Uint(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Uint", &__self_0,
                    )
                }
                Expression::UFloat(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "UFloat", &__self_0,
                    )
                }
                Expression::LocalVariable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LocalVariable",
                        &__self_0,
                    )
                }
                Expression::ImportedVariable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ImportedVariable",
                        &__self_0,
                    )
                }
                Expression::NamedHole(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NamedHole",
                        &__self_0,
                    )
                }
                Expression::Tuple(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Tuple", &__self_0,
                    )
                }
                Expression::Record(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Record", &__self_0,
                    )
                }
                Expression::Case(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Case", &__self_0,
                    )
                }
                Expression::Parens(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Parens", &__self_0,
                    )
                }
                Expression::Selector(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Selector", &__self_0,
                    )
                }
                Expression::Interrogation {
                    expression: __self_0,
                    symbol: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Interrogation",
                    "expression",
                    __self_0,
                    "symbol",
                    &__self_1,
                ),
                Expression::TypeArgument {
                    at: __self_0,
                    _type: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TypeArgument",
                    "at",
                    __self_0,
                    "_type",
                    &__self_1,
                ),
                Expression::Let(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Let", &__self_0,
                    )
                }
                Expression::BinaryOperator(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BinaryOperator",
                        &__self_0,
                    )
                }
                Expression::Lambda(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Lambda", &__self_0,
                    )
                }
                Expression::Application(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Application",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl Expression {
        pub fn selector_from_args(
            e: Box<Self>,
            s: Token<Identifier>,
            symbol: Option<TokenInfo>,
        ) -> Self {
            let selector = Expression::Selector(ExpressionSelector {
                expression: e,
                accessor: s,
            });
            match symbol {
                Some(info) => Expression::Interrogation {
                    expression: Box::new(selector),
                    symbol: info,
                },
                None => selector,
            }
        }
    }
}
pub mod imports {
    use crate::base::{Between, Comma, Parens, Token, TokenInfo, TrailingList};
    use octizys_common::{identifier::Identifier, logic_path::LogicPath};
    use octizys_macros::Equivalence;
    pub struct AsPath {
        #[equivalence(ignore)]
        pub _as: TokenInfo,
        pub path: Token<LogicPath>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AsPath {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AsPath",
                "_as",
                &self._as,
                "path",
                &&self.path,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AsPath {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AsPath {
        #[inline]
        fn eq(&self, other: &AsPath) -> bool {
            self._as == other._as && self.path == other.path
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AsPath {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<TokenInfo>;
            let _: ::core::cmp::AssertParamIsEq<Token<LogicPath>>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AsPath {
        #[inline]
        fn clone(&self) -> AsPath {
            AsPath {
                _as: ::core::clone::Clone::clone(&self._as),
                path: ::core::clone::Clone::clone(&self.path),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for AsPath {
        fn equivalent(&self, other: &Self) -> bool {
            self.path.equivalent(&other.path)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_path = self.path.equivalence_or_diff(&other.path);
            if result_path.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr = NonLineBreakStr::new("AsPath");
                let doc_path = result_path
                    .map_or_else(|x| x, |_| parens(self.path.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_path]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("AsPath");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.path.represent()))
        }
    }
    pub struct Import {
        #[equivalence(ignore)]
        pub import: TokenInfo,
        #[equivalence(ignore)]
        pub unqualified: Option<TokenInfo>,
        pub logic_path: Token<LogicPath>,
        pub import_list:
            Option<Between<TrailingList<Token<Identifier>, Comma>, Parens>>,
        pub qualified_path: Option<AsPath>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Import {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Import",
                "import",
                &self.import,
                "unqualified",
                &self.unqualified,
                "logic_path",
                &self.logic_path,
                "import_list",
                &self.import_list,
                "qualified_path",
                &&self.qualified_path,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Import {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Import {
        #[inline]
        fn eq(&self, other: &Import) -> bool {
            self.import == other.import
                && self.unqualified == other.unqualified
                && self.logic_path == other.logic_path
                && self.import_list == other.import_list
                && self.qualified_path == other.qualified_path
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Import {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<TokenInfo>;
            let _: ::core::cmp::AssertParamIsEq<Option<TokenInfo>>;
            let _: ::core::cmp::AssertParamIsEq<Token<LogicPath>>;
            let _: ::core::cmp::AssertParamIsEq<
                Option<Between<TrailingList<Token<Identifier>, Comma>, Parens>>,
            >;
            let _: ::core::cmp::AssertParamIsEq<Option<AsPath>>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Import {
        #[inline]
        fn clone(&self) -> Import {
            Import {
                import: ::core::clone::Clone::clone(&self.import),
                unqualified: ::core::clone::Clone::clone(&self.unqualified),
                logic_path: ::core::clone::Clone::clone(&self.logic_path),
                import_list: ::core::clone::Clone::clone(&self.import_list),
                qualified_path: ::core::clone::Clone::clone(
                    &self.qualified_path,
                ),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for Import {
        fn equivalent(&self, other: &Self) -> bool {
            self.logic_path.equivalent(&other.logic_path)
                & self.import_list.equivalent(&other.import_list)
                & self.qualified_path.equivalent(&other.qualified_path)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_logic_path =
                self.logic_path.equivalence_or_diff(&other.logic_path);
            let result_import_list =
                self.import_list.equivalence_or_diff(&other.import_list);
            let result_qualified_path = self
                .qualified_path
                .equivalence_or_diff(&other.qualified_path);
            if result_logic_path.is_ok()
                & result_import_list.is_ok()
                & result_qualified_path.is_ok()
                & true
            {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr = NonLineBreakStr::new("Import");
                let doc_logic_path = result_logic_path.map_or_else(
                    |x| x,
                    |_| parens(self.logic_path.represent()),
                );
                let doc_import_list = result_import_list.map_or_else(
                    |x| x,
                    |_| parens(self.import_list.represent()),
                );
                let doc_qualified_path = result_qualified_path.map_or_else(
                    |x| x,
                    |_| parens(self.qualified_path.represent()),
                );
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_logic_path,
                        hard_break(),
                        doc_import_list,
                        hard_break(),
                        doc_qualified_path,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("Import");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.logic_path.represent(),
                    hard_break(),
                    self.import_list.represent(),
                    hard_break(),
                    self.qualified_path.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
}
pub mod literals {
    use octizys_common::{identifier::Identifier, span::Span};
    use octizys_macros::Equivalence;
    pub enum StringKind {
        Raw0,
        Raw1,
        Raw2,
        Raw3,
        #[default]
        Normal,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StringKind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    StringKind::Raw0 => "Raw0",
                    StringKind::Raw1 => "Raw1",
                    StringKind::Raw2 => "Raw2",
                    StringKind::Raw3 => "Raw3",
                    StringKind::Normal => "Normal",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StringKind {
        #[inline]
        fn clone(&self) -> StringKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StringKind {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StringKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StringKind {
        #[inline]
        fn eq(&self, other: &StringKind) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StringKind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StringKind {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StringKind,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StringKind {
        #[inline]
        fn cmp(&self, other: &StringKind) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StringKind {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for StringKind {
        #[inline]
        fn default() -> StringKind {
            Self::Normal
        }
    }
    impl octizys_common::equivalence::Equivalence for StringKind {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Raw0, Self::Raw0) => true,
                (Self::Raw1, Self::Raw1) => true,
                (Self::Raw2, Self::Raw2) => true,
                (Self::Raw3, Self::Raw3) => true,
                (Self::Normal, Self::Normal) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Raw0, Self::Raw0) => ::core::result::Result::Ok(()),
                (Self::Raw1, Self::Raw1) => ::core::result::Result::Ok(()),
                (Self::Raw2, Self::Raw2) => ::core::result::Result::Ok(()),
                (Self::Raw3, Self::Raw3) => ::core::result::Result::Ok(()),
                (Self::Normal, Self::Normal) => ::core::result::Result::Ok(()),
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Raw0 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringKind::Raw0");
                    static_str(NAME)
                }
                Self::Raw1 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringKind::Raw1");
                    static_str(NAME)
                }
                Self::Raw2 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringKind::Raw2");
                    static_str(NAME)
                }
                Self::Raw3 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringKind::Raw3");
                    static_str(NAME)
                }
                Self::Normal => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringKind::Normal");
                    static_str(NAME)
                }
            }
        }
    }
    pub enum StringComponent {
        RegularString(String),
        Scaped(String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StringComponent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                StringComponent::RegularString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RegularString",
                        &__self_0,
                    )
                }
                StringComponent::Scaped(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Scaped", &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StringComponent {
        #[inline]
        fn clone(&self) -> StringComponent {
            match self {
                StringComponent::RegularString(__self_0) => {
                    StringComponent::RegularString(::core::clone::Clone::clone(
                        __self_0,
                    ))
                }
                StringComponent::Scaped(__self_0) => StringComponent::Scaped(
                    ::core::clone::Clone::clone(__self_0),
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StringComponent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StringComponent {
        #[inline]
        fn eq(&self, other: &StringComponent) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        StringComponent::RegularString(__self_0),
                        StringComponent::RegularString(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        StringComponent::Scaped(__self_0),
                        StringComponent::Scaped(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StringComponent {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StringComponent {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StringComponent,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (
                    StringComponent::RegularString(__self_0),
                    StringComponent::RegularString(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (
                    StringComponent::Scaped(__self_0),
                    StringComponent::Scaped(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                _ => ::core::cmp::PartialOrd::partial_cmp(
                    &__self_discr,
                    &__arg1_discr,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StringComponent {
        #[inline]
        fn cmp(&self, other: &StringComponent) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            match ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr) {
                ::core::cmp::Ordering::Equal => match (self, other) {
                    (
                        StringComponent::RegularString(__self_0),
                        StringComponent::RegularString(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (
                        StringComponent::Scaped(__self_0),
                        StringComponent::Scaped(__arg1_0),
                    ) => ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                },
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StringComponent {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                StringComponent::RegularString(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                StringComponent::Scaped(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for StringComponent {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::RegularString(self_0), Self::RegularString(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (Self::Scaped(self_0), Self::Scaped(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::RegularString(self_0), Self::RegularString(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr = NonLineBreakStr::new(
                            "StringComponent::RegularString",
                        );
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (Self::Scaped(self_0), Self::Scaped(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("StringComponent::Scaped");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::RegularString(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringComponent::RegularString");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Scaped(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("StringComponent::Scaped");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
            }
        }
    }
    impl Default for StringComponent {
        fn default() -> Self {
            Self::RegularString(String::new())
        }
    }
    pub struct StringLiteral {
        pub kind: StringKind,
        pub value: Vec<StringComponent>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StringLiteral {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "StringLiteral",
                "kind",
                &self.kind,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StringLiteral {
        #[inline]
        fn clone(&self) -> StringLiteral {
            StringLiteral {
                kind: ::core::clone::Clone::clone(&self.kind),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StringLiteral {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StringLiteral {
        #[inline]
        fn eq(&self, other: &StringLiteral) -> bool {
            self.kind == other.kind && self.value == other.value
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for StringLiteral {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<StringKind>;
            let _: ::core::cmp::AssertParamIsEq<Vec<StringComponent>>;
        }
    }
    impl octizys_common::equivalence::Equivalence for StringLiteral {
        fn equivalent(&self, other: &Self) -> bool {
            self.kind.equivalent(&other.kind)
                & self.value.equivalent(&other.value)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_kind = self.kind.equivalence_or_diff(&other.kind);
            let result_value = self.value.equivalence_or_diff(&other.value);
            if result_kind.is_ok() & result_value.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("StringLiteral");
                let doc_kind = result_kind
                    .map_or_else(|x| x, |_| parens(self.kind.represent()));
                let doc_value = result_value
                    .map_or_else(|x| x, |_| parens(self.value.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_kind,
                        hard_break(),
                        doc_value,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("StringLiteral");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.kind.represent(),
                    hard_break(),
                    self.value.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StringLiteral {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.kind, state);
            ::core::hash::Hash::hash(&self.value, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for StringLiteral {
        #[inline]
        fn partial_cmp(
            &self,
            other: &StringLiteral,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.kind, &other.kind)
            {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(
                        &self.value,
                        &other.value,
                    )
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for StringLiteral {
        #[inline]
        fn cmp(&self, other: &StringLiteral) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.kind, &other.kind) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.value, &other.value)
                }
                cmp => cmp,
            }
        }
    }
    pub struct InterpolationOptions {
        pub is_debug: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InterpolationOptions {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "InterpolationOptions",
                "is_debug",
                &&self.is_debug,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InterpolationOptions {
        #[inline]
        fn clone(&self) -> InterpolationOptions {
            let _: ::core::clone::AssertParamIsClone<bool>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for InterpolationOptions {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InterpolationOptions {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InterpolationOptions {
        #[inline]
        fn eq(&self, other: &InterpolationOptions) -> bool {
            self.is_debug == other.is_debug
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for InterpolationOptions {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for InterpolationOptions {
        #[inline]
        fn partial_cmp(
            &self,
            other: &InterpolationOptions,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(
                &self.is_debug,
                &other.is_debug,
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for InterpolationOptions {
        #[inline]
        fn cmp(&self, other: &InterpolationOptions) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.is_debug, &other.is_debug)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for InterpolationOptions {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.is_debug, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for InterpolationOptions {
        #[inline]
        fn default() -> InterpolationOptions {
            InterpolationOptions {
                is_debug: ::core::default::Default::default(),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for InterpolationOptions {
        fn equivalent(&self, other: &Self) -> bool {
            self.is_debug.equivalent(&other.is_debug)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_is_debug =
                self.is_debug.equivalence_or_diff(&other.is_debug);
            if result_is_debug.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("InterpolationOptions");
                let doc_is_debug = result_is_debug
                    .map_or_else(|x| x, |_| parens(self.is_debug.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_is_debug]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("InterpolationOptions");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.is_debug.represent()))
        }
    }
    pub enum InterpolationComponent {
        RegularString(String),
        Scaped(char),
        Interpolation {
            variable: Identifier,
            #[equivalence(ignore)]
            span: Span,
            options: InterpolationOptions,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InterpolationComponent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                InterpolationComponent::RegularString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RegularString",
                        &__self_0,
                    )
                }
                InterpolationComponent::Scaped(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Scaped", &__self_0,
                    )
                }
                InterpolationComponent::Interpolation {
                    variable: __self_0,
                    span: __self_1,
                    options: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Interpolation",
                    "variable",
                    __self_0,
                    "span",
                    __self_1,
                    "options",
                    &__self_2,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InterpolationComponent {
        #[inline]
        fn clone(&self) -> InterpolationComponent {
            match self {
                InterpolationComponent::RegularString(__self_0) => {
                    InterpolationComponent::RegularString(
                        ::core::clone::Clone::clone(__self_0),
                    )
                }
                InterpolationComponent::Scaped(__self_0) => {
                    InterpolationComponent::Scaped(::core::clone::Clone::clone(
                        __self_0,
                    ))
                }
                InterpolationComponent::Interpolation {
                    variable: __self_0,
                    span: __self_1,
                    options: __self_2,
                } => InterpolationComponent::Interpolation {
                    variable: ::core::clone::Clone::clone(__self_0),
                    span: ::core::clone::Clone::clone(__self_1),
                    options: ::core::clone::Clone::clone(__self_2),
                },
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InterpolationComponent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InterpolationComponent {
        #[inline]
        fn eq(&self, other: &InterpolationComponent) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        InterpolationComponent::RegularString(__self_0),
                        InterpolationComponent::RegularString(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        InterpolationComponent::Scaped(__self_0),
                        InterpolationComponent::Scaped(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    (
                        InterpolationComponent::Interpolation {
                            variable: __self_0,
                            span: __self_1,
                            options: __self_2,
                        },
                        InterpolationComponent::Interpolation {
                            variable: __arg1_0,
                            span: __arg1_1,
                            options: __arg1_2,
                        },
                    ) => {
                        __self_0 == __arg1_0
                            && __self_1 == __arg1_1
                            && __self_2 == __arg1_2
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for InterpolationComponent {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<char>;
            let _: ::core::cmp::AssertParamIsEq<Identifier>;
            let _: ::core::cmp::AssertParamIsEq<Span>;
            let _: ::core::cmp::AssertParamIsEq<InterpolationOptions>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for InterpolationComponent {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                InterpolationComponent::RegularString(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                InterpolationComponent::Scaped(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                InterpolationComponent::Interpolation {
                    variable: __self_0,
                    span: __self_1,
                    options: __self_2,
                } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state);
                    ::core::hash::Hash::hash(__self_2, state)
                }
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for InterpolationComponent {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::RegularString(self_0), Self::RegularString(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (Self::Scaped(self_0), Self::Scaped(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (
                    Self::Interpolation {
                        variable: self_variable,
                        options: self_options,
                        ..
                    },
                    Self::Interpolation {
                        variable: other_variable,
                        options: other_options,
                        ..
                    },
                ) => {
                    self_variable.equivalent(&other_variable)
                        & self_options.equivalent(&other_options)
                }
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::RegularString(self_0), Self::RegularString(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr = NonLineBreakStr::new(
                            "InterpolationComponent::RegularString",
                        );
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (Self::Scaped(self_0), Self::Scaped(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr = NonLineBreakStr::new(
                            "InterpolationComponent::Scaped",
                        );
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (
                    Self::Interpolation {
                        variable: self_variable,
                        options: self_options,
                        ..
                    },
                    Self::Interpolation {
                        variable: other_variable,
                        options: other_options,
                        ..
                    },
                ) => {
                    let result_variable =
                        self_variable.equivalence_or_diff(&other_variable);
                    let result_options =
                        self_options.equivalence_or_diff(&other_options);
                    if result_variable.is_ok() & result_options.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr = NonLineBreakStr::new(
                            "InterpolationComponent::Interpolation",
                        );
                        let doc_variable = result_variable.map_or_else(
                            |x| x,
                            |_| parens(self_variable.represent()),
                        );
                        let doc_options = result_options.map_or_else(
                            |x| x,
                            |_| parens(self_options.represent()),
                        );
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                hard_break(),
                                doc_variable,
                                hard_break(),
                                doc_options,
                            ]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::RegularString(self_0) => {
                    const NAME: NonLineBreakStr = NonLineBreakStr::new(
                        "InterpolationComponent::RegularString",
                    );
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Scaped(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("InterpolationComponent::Scaped");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Interpolation {
                    variable: self_variable,
                    options: self_options,
                    ..
                } => {
                    const NAME: NonLineBreakStr = NonLineBreakStr::new(
                        "InterpolationComponent::Interpolation",
                    );
                    let children_representation = concat(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            self_variable.represent(),
                            hard_break(),
                            self_options.represent(),
                        ]),
                    ));
                    static_str(NAME)
                        + nest(2, hard_break() + children_representation)
                }
            }
        }
    }
    pub struct InterpolationString {
        pub value: Vec<InterpolationComponent>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InterpolationString {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "InterpolationString",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InterpolationString {
        #[inline]
        fn clone(&self) -> InterpolationString {
            InterpolationString {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InterpolationString {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InterpolationString {
        #[inline]
        fn eq(&self, other: &InterpolationString) -> bool {
            self.value == other.value
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for InterpolationString {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<InterpolationComponent>>;
        }
    }
    impl octizys_common::equivalence::Equivalence for InterpolationString {
        fn equivalent(&self, other: &Self) -> bool {
            self.value.equivalent(&other.value)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_value = self.value.equivalence_or_diff(&other.value);
            if result_value.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("InterpolationString");
                let doc_value = result_value
                    .map_or_else(|x| x, |_| parens(self.value.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_value]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("InterpolationString");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.value.represent()))
        }
    }
    pub enum UintKind {
        Hex,
        Octal,
        Binary,
        U8,
        U16,
        U32,
        U64,
        /// This means a regular literal (not hex or octal)
        /// that has no suffix for the type
        #[default]
        Unspecified,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UintKind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    UintKind::Hex => "Hex",
                    UintKind::Octal => "Octal",
                    UintKind::Binary => "Binary",
                    UintKind::U8 => "U8",
                    UintKind::U16 => "U16",
                    UintKind::U32 => "U32",
                    UintKind::U64 => "U64",
                    UintKind::Unspecified => "Unspecified",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UintKind {
        #[inline]
        fn clone(&self) -> UintKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for UintKind {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UintKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UintKind {
        #[inline]
        fn eq(&self, other: &UintKind) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for UintKind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for UintKind {
        #[inline]
        fn partial_cmp(
            &self,
            other: &UintKind,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for UintKind {
        #[inline]
        fn cmp(&self, other: &UintKind) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for UintKind {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UintKind {
        #[inline]
        fn default() -> UintKind {
            Self::Unspecified
        }
    }
    impl octizys_common::equivalence::Equivalence for UintKind {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Hex, Self::Hex) => true,
                (Self::Octal, Self::Octal) => true,
                (Self::Binary, Self::Binary) => true,
                (Self::U8, Self::U8) => true,
                (Self::U16, Self::U16) => true,
                (Self::U32, Self::U32) => true,
                (Self::U64, Self::U64) => true,
                (Self::Unspecified, Self::Unspecified) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Hex, Self::Hex) => ::core::result::Result::Ok(()),
                (Self::Octal, Self::Octal) => ::core::result::Result::Ok(()),
                (Self::Binary, Self::Binary) => ::core::result::Result::Ok(()),
                (Self::U8, Self::U8) => ::core::result::Result::Ok(()),
                (Self::U16, Self::U16) => ::core::result::Result::Ok(()),
                (Self::U32, Self::U32) => ::core::result::Result::Ok(()),
                (Self::U64, Self::U64) => ::core::result::Result::Ok(()),
                (Self::Unspecified, Self::Unspecified) => {
                    ::core::result::Result::Ok(())
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Hex => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::Hex");
                    static_str(NAME)
                }
                Self::Octal => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::Octal");
                    static_str(NAME)
                }
                Self::Binary => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::Binary");
                    static_str(NAME)
                }
                Self::U8 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::U8");
                    static_str(NAME)
                }
                Self::U16 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::U16");
                    static_str(NAME)
                }
                Self::U32 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::U32");
                    static_str(NAME)
                }
                Self::U64 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::U64");
                    static_str(NAME)
                }
                Self::Unspecified => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UintKind::Unspecified");
                    static_str(NAME)
                }
            }
        }
    }
    pub struct UintLiteral {
        pub kind: UintKind,
        pub value: u64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UintLiteral {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "UintLiteral",
                "kind",
                &self.kind,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UintLiteral {
        #[inline]
        fn clone(&self) -> UintLiteral {
            let _: ::core::clone::AssertParamIsClone<UintKind>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for UintLiteral {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UintLiteral {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UintLiteral {
        #[inline]
        fn eq(&self, other: &UintLiteral) -> bool {
            self.kind == other.kind && self.value == other.value
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for UintLiteral {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<UintKind>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for UintLiteral {
        #[inline]
        fn partial_cmp(
            &self,
            other: &UintLiteral,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.kind, &other.kind)
            {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(
                        &self.value,
                        &other.value,
                    )
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for UintLiteral {
        #[inline]
        fn cmp(&self, other: &UintLiteral) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.kind, &other.kind) {
                ::core::cmp::Ordering::Equal => {
                    ::core::cmp::Ord::cmp(&self.value, &other.value)
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for UintLiteral {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.kind, state);
            ::core::hash::Hash::hash(&self.value, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UintLiteral {
        #[inline]
        fn default() -> UintLiteral {
            UintLiteral {
                kind: ::core::default::Default::default(),
                value: ::core::default::Default::default(),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for UintLiteral {
        fn equivalent(&self, other: &Self) -> bool {
            self.kind.equivalent(&other.kind)
                & self.value.equivalent(&other.value)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_kind = self.kind.equivalence_or_diff(&other.kind);
            let result_value = self.value.equivalence_or_diff(&other.value);
            if result_kind.is_ok() & result_value.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("UintLiteral");
                let doc_kind = result_kind
                    .map_or_else(|x| x, |_| parens(self.kind.represent()));
                let doc_value = result_value
                    .map_or_else(|x| x, |_| parens(self.value.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_kind,
                        hard_break(),
                        doc_value,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr = NonLineBreakStr::new("UintLiteral");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.kind.represent(),
                    hard_break(),
                    self.value.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
    pub enum UFloatingPointKind {
        F32,
        #[default]
        F64,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UFloatingPointKind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    UFloatingPointKind::F32 => "F32",
                    UFloatingPointKind::F64 => "F64",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UFloatingPointKind {
        #[inline]
        fn clone(&self) -> UFloatingPointKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for UFloatingPointKind {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UFloatingPointKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UFloatingPointKind {
        #[inline]
        fn eq(&self, other: &UFloatingPointKind) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for UFloatingPointKind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for UFloatingPointKind {
        #[inline]
        fn partial_cmp(
            &self,
            other: &UFloatingPointKind,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for UFloatingPointKind {
        #[inline]
        fn cmp(&self, other: &UFloatingPointKind) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for UFloatingPointKind {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UFloatingPointKind {
        #[inline]
        fn default() -> UFloatingPointKind {
            Self::F64
        }
    }
    impl octizys_common::equivalence::Equivalence for UFloatingPointKind {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::F32, Self::F32) => true,
                (Self::F64, Self::F64) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::F32, Self::F32) => ::core::result::Result::Ok(()),
                (Self::F64, Self::F64) => ::core::result::Result::Ok(()),
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::F32 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UFloatingPointKind::F32");
                    static_str(NAME)
                }
                Self::F64 => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("UFloatingPointKind::F64");
                    static_str(NAME)
                }
            }
        }
    }
    pub struct UFloatingPointLiteral {
        pub kind: UFloatingPointKind,
        pub integral_part: u64,
        pub exponent: u8,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UFloatingPointLiteral {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "UFloatingPointLiteral",
                "kind",
                &self.kind,
                "integral_part",
                &self.integral_part,
                "exponent",
                &&self.exponent,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UFloatingPointLiteral {
        #[inline]
        fn clone(&self) -> UFloatingPointLiteral {
            let _: ::core::clone::AssertParamIsClone<UFloatingPointKind>;
            let _: ::core::clone::AssertParamIsClone<u64>;
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for UFloatingPointLiteral {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for UFloatingPointLiteral {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for UFloatingPointLiteral {
        #[inline]
        fn eq(&self, other: &UFloatingPointLiteral) -> bool {
            self.kind == other.kind
                && self.integral_part == other.integral_part
                && self.exponent == other.exponent
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for UFloatingPointLiteral {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<UFloatingPointKind>;
            let _: ::core::cmp::AssertParamIsEq<u64>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for UFloatingPointLiteral {
        #[inline]
        fn partial_cmp(
            &self,
            other: &UFloatingPointLiteral,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.kind, &other.kind)
            {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.integral_part,
                        &other.integral_part,
                    ) {
                        ::core::option::Option::Some(
                            ::core::cmp::Ordering::Equal,
                        ) => ::core::cmp::PartialOrd::partial_cmp(
                            &self.exponent,
                            &other.exponent,
                        ),
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for UFloatingPointLiteral {
        #[inline]
        fn cmp(&self, other: &UFloatingPointLiteral) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.kind, &other.kind) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(
                        &self.integral_part,
                        &other.integral_part,
                    ) {
                        ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(
                            &self.exponent,
                            &other.exponent,
                        ),
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for UFloatingPointLiteral {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.kind, state);
            ::core::hash::Hash::hash(&self.integral_part, state);
            ::core::hash::Hash::hash(&self.exponent, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UFloatingPointLiteral {
        #[inline]
        fn default() -> UFloatingPointLiteral {
            UFloatingPointLiteral {
                kind: ::core::default::Default::default(),
                integral_part: ::core::default::Default::default(),
                exponent: ::core::default::Default::default(),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for UFloatingPointLiteral {
        fn equivalent(&self, other: &Self) -> bool {
            self.kind.equivalent(&other.kind)
                & self.integral_part.equivalent(&other.integral_part)
                & self.exponent.equivalent(&other.exponent)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_kind = self.kind.equivalence_or_diff(&other.kind);
            let result_integral_part =
                self.integral_part.equivalence_or_diff(&other.integral_part);
            let result_exponent =
                self.exponent.equivalence_or_diff(&other.exponent);
            if result_kind.is_ok()
                & result_integral_part.is_ok()
                & result_exponent.is_ok()
                & true
            {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("UFloatingPointLiteral");
                let doc_kind = result_kind
                    .map_or_else(|x| x, |_| parens(self.kind.represent()));
                let doc_integral_part = result_integral_part.map_or_else(
                    |x| x,
                    |_| parens(self.integral_part.represent()),
                );
                let doc_exponent = result_exponent
                    .map_or_else(|x| x, |_| parens(self.exponent.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        hard_break(),
                        doc_kind,
                        hard_break(),
                        doc_integral_part,
                        hard_break(),
                        doc_exponent,
                    ]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("UFloatingPointLiteral");
            let children_representation = concat(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    self.kind.represent(),
                    hard_break(),
                    self.integral_part.represent(),
                    hard_break(),
                    self.exponent.represent(),
                ]),
            ));
            static_str(NAME) + nest(2, hard_break() + children_representation)
        }
    }
}
pub mod patterns {
    use crate::{
        base::{
            Between, Braces, Comma, ImportedVariable, Parens, Token, TokenInfo,
            TrailingList,
        },
        literals::{StringLiteral, UFloatingPointLiteral, UintLiteral},
    };
    use octizys_common::identifier::Identifier;
    pub enum PatternMatchRecordItem {
        OnlyVariable {
            variable: Token<Identifier>,
        },
        WithPattern {
            variable: Token<Identifier>,
            separator: TokenInfo,
            pattern: Box<PatternMatch>,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatternMatchRecordItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PatternMatchRecordItem::OnlyVariable { variable: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "OnlyVariable",
                        "variable",
                        &__self_0,
                    )
                }
                PatternMatchRecordItem::WithPattern {
                    variable: __self_0,
                    separator: __self_1,
                    pattern: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "WithPattern",
                    "variable",
                    __self_0,
                    "separator",
                    __self_1,
                    "pattern",
                    &__self_2,
                ),
            }
        }
    }
    pub struct PatternMatchBind {
        pub variable: Token<Identifier>,
        pub at: TokenInfo,
        pub pattern: Box<PatternMatch>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatternMatchBind {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "PatternMatchBind",
                "variable",
                &self.variable,
                "at",
                &self.at,
                "pattern",
                &&self.pattern,
            )
        }
    }
    pub enum PatternMatch {
        LocalVariable(Token<Identifier>),
        ImportedVariable(Token<ImportedVariable>),
        String(Token<StringLiteral>),
        Uint(Token<UintLiteral>),
        UFloat(Token<UFloatingPointLiteral>),
        AnonHole(TokenInfo),
        Tuple(Between<TrailingList<Box<PatternMatch>, Comma>, Parens>),
        Record(Between<TrailingList<PatternMatchRecordItem, Comma>, Braces>),
        Bind(PatternMatchBind),
        Application {
            start: Box<PatternMatch>,
            second: Box<PatternMatch>,
            remain: Vec<PatternMatch>,
        },
        Parens(Between<Box<PatternMatch>, Parens>),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PatternMatch {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PatternMatch::LocalVariable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LocalVariable",
                        &__self_0,
                    )
                }
                PatternMatch::ImportedVariable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ImportedVariable",
                        &__self_0,
                    )
                }
                PatternMatch::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "String", &__self_0,
                    )
                }
                PatternMatch::Uint(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Uint", &__self_0,
                    )
                }
                PatternMatch::UFloat(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "UFloat", &__self_0,
                    )
                }
                PatternMatch::AnonHole(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "AnonHole", &__self_0,
                    )
                }
                PatternMatch::Tuple(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Tuple", &__self_0,
                    )
                }
                PatternMatch::Record(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Record", &__self_0,
                    )
                }
                PatternMatch::Bind(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Bind", &__self_0,
                    )
                }
                PatternMatch::Application {
                    start: __self_0,
                    second: __self_1,
                    remain: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Application",
                    "start",
                    __self_0,
                    "second",
                    __self_1,
                    "remain",
                    &__self_2,
                ),
                PatternMatch::Parens(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Parens", &__self_0,
                    )
                }
            }
        }
    }
}
pub mod pretty {}
pub mod top {}
pub mod types {
    use crate::base::{
        Between, Braces, Comma, ImportedVariable, Parens, RightArrow, Token,
        TokenInfo, TrailingList, TrailingListItem,
    };
    use octizys_common::identifier::Identifier;
    use octizys_macros::Equivalence;
    pub enum OwnershipLiteral {
        #[default]
        Zero,
        One,
        Inf,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OwnershipLiteral {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    OwnershipLiteral::Zero => "Zero",
                    OwnershipLiteral::One => "One",
                    OwnershipLiteral::Inf => "Inf",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for OwnershipLiteral {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for OwnershipLiteral {
        #[inline]
        fn eq(&self, other: &OwnershipLiteral) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for OwnershipLiteral {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for OwnershipLiteral {
        #[inline]
        fn partial_cmp(
            &self,
            other: &OwnershipLiteral,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for OwnershipLiteral {
        #[inline]
        fn cmp(&self, other: &OwnershipLiteral) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for OwnershipLiteral {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for OwnershipLiteral {
        #[inline]
        fn default() -> OwnershipLiteral {
            Self::Zero
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OwnershipLiteral {
        #[inline]
        fn clone(&self) -> OwnershipLiteral {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OwnershipLiteral {}
    impl octizys_common::equivalence::Equivalence for OwnershipLiteral {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Zero, Self::Zero) => true,
                (Self::One, Self::One) => true,
                (Self::Inf, Self::Inf) => true,
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::Zero, Self::Zero) => ::core::result::Result::Ok(()),
                (Self::One, Self::One) => ::core::result::Result::Ok(()),
                (Self::Inf, Self::Inf) => ::core::result::Result::Ok(()),
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::Zero => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OwnershipLiteral::Zero");
                    static_str(NAME)
                }
                Self::One => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OwnershipLiteral::One");
                    static_str(NAME)
                }
                Self::Inf => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("OwnershipLiteral::Inf");
                    static_str(NAME)
                }
            }
        }
    }
    pub struct OwnershipVariable {
        pub variable: Identifier,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OwnershipVariable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "OwnershipVariable",
                "variable",
                &&self.variable,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for OwnershipVariable {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for OwnershipVariable {
        #[inline]
        fn eq(&self, other: &OwnershipVariable) -> bool {
            self.variable == other.variable
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for OwnershipVariable {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Identifier>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for OwnershipVariable {
        #[inline]
        fn partial_cmp(
            &self,
            other: &OwnershipVariable,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(
                &self.variable,
                &other.variable,
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for OwnershipVariable {
        #[inline]
        fn cmp(&self, other: &OwnershipVariable) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.variable, &other.variable)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for OwnershipVariable {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.variable, state)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OwnershipVariable {
        #[inline]
        fn clone(&self) -> OwnershipVariable {
            OwnershipVariable {
                variable: ::core::clone::Clone::clone(&self.variable),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for OwnershipVariable {
        fn equivalent(&self, other: &Self) -> bool {
            self.variable.equivalent(&other.variable)
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            let result_variable =
                self.variable.equivalence_or_diff(&other.variable);
            if result_variable.is_ok() & true {
                ::core::result::Result::Ok(())
            } else {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new("OwnershipVariable");
                let doc_variable = result_variable
                    .map_or_else(|x| x, |_| parens(self.variable.represent()));
                let children = concat(<[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([hard_break(), doc_variable]),
                ));
                ::core::result::Result::Err(
                    static_str(NAME) + nest(2, children),
                )
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            const NAME: NonLineBreakStr =
                NonLineBreakStr::new("OwnershipVariable");
            static_str(NAME)
                + nest(2, hard_break() + parens(self.variable.represent()))
        }
    }
    pub struct TypeRecordItem {
        pub variable: Token<Identifier>,
        pub separator: TokenInfo,
        pub expression: Box<Type>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TypeRecordItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TypeRecordItem",
                "variable",
                &self.variable,
                "separator",
                &self.separator,
                "expression",
                &&self.expression,
            )
        }
    }
    pub enum Type {
        LocalVariable(Token<Identifier>),
        ImportedVariable(Token<ImportedVariable>),
        Tuple(Between<TrailingList<Box<Type>, Comma>, Parens>),
        Record(Between<TrailingList<TypeRecordItem, Comma>, Braces>),
        Parens(Between<Box<Type>, Parens>),
        Application {
            start: Box<Type>,
            second: Box<Type>,
            remain: Vec<Type>,
        },
        Arrow {
            first: Box<Type>,
            remain: Vec<TrailingListItem<Type, RightArrow>>,
        },
        Scheme {
            forall: TokenInfo,
            first_variable: Token<Identifier>,
            remain_variables: Vec<Token<Identifier>>,
            #[equivalence(ignore)]
            dot: TokenInfo,
            expression: Box<Type>,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Type {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Type::LocalVariable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LocalVariable",
                        &__self_0,
                    )
                }
                Type::ImportedVariable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ImportedVariable",
                        &__self_0,
                    )
                }
                Type::Tuple(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Tuple", &__self_0,
                    )
                }
                Type::Record(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Record", &__self_0,
                    )
                }
                Type::Parens(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f, "Parens", &__self_0,
                    )
                }
                Type::Application {
                    start: __self_0,
                    second: __self_1,
                    remain: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Application",
                    "start",
                    __self_0,
                    "second",
                    __self_1,
                    "remain",
                    &__self_2,
                ),
                Type::Arrow {
                    first: __self_0,
                    remain: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f, "Arrow", "first", __self_0, "remain", &__self_1,
                ),
                Type::Scheme {
                    forall: __self_0,
                    first_variable: __self_1,
                    remain_variables: __self_2,
                    dot: __self_3,
                    expression: __self_4,
                } => ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Scheme",
                    "forall",
                    __self_0,
                    "first_variable",
                    __self_1,
                    "remain_variables",
                    __self_2,
                    "dot",
                    __self_3,
                    "expression",
                    &__self_4,
                ),
            }
        }
    }
    impl octizys_common::equivalence::Equivalence for Type {
        fn equivalent(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::LocalVariable(self_0), Self::LocalVariable(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (
                    Self::ImportedVariable(self_0),
                    Self::ImportedVariable(other_0),
                ) => self_0.equivalent(&other_0),
                (Self::Tuple(self_0), Self::Tuple(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (Self::Record(self_0), Self::Record(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (Self::Parens(self_0), Self::Parens(other_0)) => {
                    self_0.equivalent(&other_0)
                }
                (
                    Self::Application {
                        start: self_start,
                        second: self_second,
                        remain: self_remain,
                        ..
                    },
                    Self::Application {
                        start: other_start,
                        second: other_second,
                        remain: other_remain,
                        ..
                    },
                ) => {
                    self_start.equivalent(&other_start)
                        & self_second.equivalent(&other_second)
                        & self_remain.equivalent(&other_remain)
                }
                (
                    Self::Arrow {
                        first: self_first,
                        remain: self_remain,
                        ..
                    },
                    Self::Arrow {
                        first: other_first,
                        remain: other_remain,
                        ..
                    },
                ) => {
                    self_first.equivalent(&other_first)
                        & self_remain.equivalent(&other_remain)
                }
                (
                    Self::Scheme {
                        forall: self_forall,
                        first_variable: self_first_variable,
                        remain_variables: self_remain_variables,
                        expression: self_expression,
                        ..
                    },
                    Self::Scheme {
                        forall: other_forall,
                        first_variable: other_first_variable,
                        remain_variables: other_remain_variables,
                        expression: other_expression,
                        ..
                    },
                ) => {
                    self_forall.equivalent(&other_forall)
                        & self_first_variable.equivalent(&other_first_variable)
                        & self_remain_variables
                            .equivalent(&other_remain_variables)
                        & self_expression.equivalent(&other_expression)
                }
                (_, _) => false,
            }
        }
        fn equivalence_or_diff(
            &self,
            other: &Self,
        ) -> ::core::result::Result<(), ::octizys_pretty::document::Document>
        {
            use ::octizys_common::equivalence::{make_report, parens};
            use ::octizys_pretty::combinators::{
                concat, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match (self, other) {
                (Self::LocalVariable(self_0), Self::LocalVariable(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::LocalVariable");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (
                    Self::ImportedVariable(self_0),
                    Self::ImportedVariable(other_0),
                ) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::ImportedVariable");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (Self::Tuple(self_0), Self::Tuple(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::Tuple");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (Self::Record(self_0), Self::Record(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::Record");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (Self::Parens(self_0), Self::Parens(other_0)) => {
                    let result_0 = self_0.equivalence_or_diff(&other_0);
                    if result_0.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::Parens");
                        let doc_0 = result_0
                            .map_or_else(|x| x, |_| parens(self_0.represent()));
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([hard_break(), doc_0]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (
                    Self::Application {
                        start: self_start,
                        second: self_second,
                        remain: self_remain,
                        ..
                    },
                    Self::Application {
                        start: other_start,
                        second: other_second,
                        remain: other_remain,
                        ..
                    },
                ) => {
                    let result_start =
                        self_start.equivalence_or_diff(&other_start);
                    let result_second =
                        self_second.equivalence_or_diff(&other_second);
                    let result_remain =
                        self_remain.equivalence_or_diff(&other_remain);
                    if result_start.is_ok()
                        & result_second.is_ok()
                        & result_remain.is_ok()
                        & true
                    {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::Application");
                        let doc_start = result_start.map_or_else(
                            |x| x,
                            |_| parens(self_start.represent()),
                        );
                        let doc_second = result_second.map_or_else(
                            |x| x,
                            |_| parens(self_second.represent()),
                        );
                        let doc_remain = result_remain.map_or_else(
                            |x| x,
                            |_| parens(self_remain.represent()),
                        );
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                hard_break(),
                                doc_start,
                                hard_break(),
                                doc_second,
                                hard_break(),
                                doc_remain,
                            ]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (
                    Self::Arrow {
                        first: self_first,
                        remain: self_remain,
                        ..
                    },
                    Self::Arrow {
                        first: other_first,
                        remain: other_remain,
                        ..
                    },
                ) => {
                    let result_first =
                        self_first.equivalence_or_diff(&other_first);
                    let result_remain =
                        self_remain.equivalence_or_diff(&other_remain);
                    if result_first.is_ok() & result_remain.is_ok() & true {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::Arrow");
                        let doc_first = result_first.map_or_else(
                            |x| x,
                            |_| parens(self_first.represent()),
                        );
                        let doc_remain = result_remain.map_or_else(
                            |x| x,
                            |_| parens(self_remain.represent()),
                        );
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                hard_break(),
                                doc_first,
                                hard_break(),
                                doc_remain,
                            ]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (
                    Self::Scheme {
                        forall: self_forall,
                        first_variable: self_first_variable,
                        remain_variables: self_remain_variables,
                        expression: self_expression,
                        ..
                    },
                    Self::Scheme {
                        forall: other_forall,
                        first_variable: other_first_variable,
                        remain_variables: other_remain_variables,
                        expression: other_expression,
                        ..
                    },
                ) => {
                    let result_forall =
                        self_forall.equivalence_or_diff(&other_forall);
                    let result_first_variable = self_first_variable
                        .equivalence_or_diff(&other_first_variable);
                    let result_remain_variables = self_remain_variables
                        .equivalence_or_diff(&other_remain_variables);
                    let result_expression =
                        self_expression.equivalence_or_diff(&other_expression);
                    if result_forall.is_ok()
                        & result_first_variable.is_ok()
                        & result_remain_variables.is_ok()
                        & result_expression.is_ok()
                        & true
                    {
                        ::core::result::Result::Ok(())
                    } else {
                        const NAME: NonLineBreakStr =
                            NonLineBreakStr::new("Type::Scheme");
                        let doc_forall = result_forall.map_or_else(
                            |x| x,
                            |_| parens(self_forall.represent()),
                        );
                        let doc_first_variable = result_first_variable
                            .map_or_else(
                                |x| x,
                                |_| parens(self_first_variable.represent()),
                            );
                        let doc_remain_variables = result_remain_variables
                            .map_or_else(
                                |x| x,
                                |_| parens(self_remain_variables.represent()),
                            );
                        let doc_expression = result_expression.map_or_else(
                            |x| x,
                            |_| parens(self_expression.represent()),
                        );
                        let children = concat(<[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                hard_break(),
                                doc_forall,
                                hard_break(),
                                doc_first_variable,
                                hard_break(),
                                doc_remain_variables,
                                hard_break(),
                                doc_expression,
                            ]),
                        ));
                        ::core::result::Result::Err(
                            static_str(NAME) + nest(2, children),
                        )
                    }
                }
                (_, _) => ::core::result::Result::Err(make_report(self, other)),
            }
        }
        fn represent(&self) -> octizys_pretty::document::Document {
            use ::octizys_common::equivalence::parens;
            use ::octizys_pretty::combinators::{
                concat, empty, hard_break, nest, static_str,
            };
            use ::octizys_text_store::store::NonLineBreakStr;
            match self {
                Self::LocalVariable(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::LocalVariable");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::ImportedVariable(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::ImportedVariable");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Tuple(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::Tuple");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Record(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::Record");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Parens(self_0) => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::Parens");
                    static_str(NAME)
                        + nest(2, hard_break() + parens(self_0.represent()))
                }
                Self::Application {
                    start: self_start,
                    second: self_second,
                    remain: self_remain,
                    ..
                } => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::Application");
                    let children_representation = concat(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            self_start.represent(),
                            hard_break(),
                            self_second.represent(),
                            hard_break(),
                            self_remain.represent(),
                        ]),
                    ));
                    static_str(NAME)
                        + nest(2, hard_break() + children_representation)
                }
                Self::Arrow {
                    first: self_first,
                    remain: self_remain,
                    ..
                } => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::Arrow");
                    let children_representation = concat(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            self_first.represent(),
                            hard_break(),
                            self_remain.represent(),
                        ]),
                    ));
                    static_str(NAME)
                        + nest(2, hard_break() + children_representation)
                }
                Self::Scheme {
                    forall: self_forall,
                    first_variable: self_first_variable,
                    remain_variables: self_remain_variables,
                    expression: self_expression,
                    ..
                } => {
                    const NAME: NonLineBreakStr =
                        NonLineBreakStr::new("Type::Scheme");
                    let children_representation = concat(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            self_forall.represent(),
                            hard_break(),
                            self_first_variable.represent(),
                            hard_break(),
                            self_remain_variables.represent(),
                            hard_break(),
                            self_expression.represent(),
                        ]),
                    ));
                    static_str(NAME)
                        + nest(2, hard_break() + children_representation)
                }
            }
        }
    }
    impl Type {
        ///This function tell the pretty printer if the type needs to be
        ///surrounded by parentheses if the type is a argument in a
        ///application.
        pub fn need_parens_application(&self) -> bool {
            match self {
                Type::LocalVariable(_) => false,
                Type::ImportedVariable(_) => false,
                Type::Tuple(_) => false,
                Type::Record(_) => false,
                Type::Parens(_) => false,
                Type::Application { .. } => true,
                Type::Arrow { .. } => true,
                Type::Scheme { .. } => true,
            }
        }
        ///This function tell the pretty printer if the type needs to be
        ///surrounded by parentheses if the type is a argument in a
        ///arrow.
        pub fn need_parens_arrow(&self) -> bool {
            match self {
                Type::LocalVariable(_) => false,
                Type::ImportedVariable(_) => false,
                Type::Tuple(_) => false,
                Type::Record(_) => false,
                Type::Parens(_) => false,
                Type::Application { .. } => false,
                Type::Arrow { .. } => true,
                Type::Scheme { .. } => true,
            }
        }
    }
}
