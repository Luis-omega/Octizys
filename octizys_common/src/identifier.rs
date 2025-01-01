use octizys_pretty::{combinators::static_str, store::NonLineBreakStr};
use octizys_text_store::store::{approximate_string_width, Store};

use regex::Regex;
use std::{fmt::Display, sync::LazyLock};
use string_interner::DefaultSymbol;

use crate::equivalence::Equivalence;

// TODO: Document it!

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Identifier {
    symbol: DefaultSymbol,
    len: usize,
}

/// Equivalence use is to compare trees without positional information.
/// As such Identifiers are opaque!
impl Equivalence for Identifier {
    fn equivalent(&self, _other: &Self) -> bool {
        true
    }

    fn represent(&self) -> octizys_pretty::document::Document {
        const IDENT: NonLineBreakStr = NonLineBreakStr::new("Identifier");
        static_str(IDENT)
    }

    fn equivalence_or_diff(
        &self,
        _other: &Self,
    ) -> Result<(), octizys_pretty::document::Document> {
        Ok(())
    }
}

impl Identifier {
    pub fn as_tuple(&self) -> (DefaultSymbol, usize) {
        (self.symbol, self.len)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier")
    }
}

impl From<Identifier> for DefaultSymbol {
    fn from(value: Identifier) -> Self {
        value.symbol
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdentifierError {
    ContainsInvalidCodePoint(String),
    EmptyIdentifier,
}

impl Display for IdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifierError::ContainsInvalidCodePoint(s) => {
                write!(f, "invalid char in Identifier: {0}", s)
            }
            IdentifierError::EmptyIdentifier => {
                write!(f, "attempt to build empty identifier")
            }
        }
    }
}

/// Keep this in sync with the lexer definition for Identifier
pub static IDENTIFIER_LAZY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x) #ignore spaces and allow comments
        ^ #match from the begin
        # can begin with underscores, but not digits
        _*
        \p{XID_START}
        # after we found a valid Alphabetic, we can allow underscores and digits
        \p{XID_CONTINUE}*
        $ #match until the end
        ",
    )
    .unwrap()
});

impl<'a> Identifier {
    pub fn make(
        s: &str,
        store: &mut Store,
    ) -> Result<Identifier, IdentifierError> {
        if s.is_empty() {
            return Err(IdentifierError::EmptyIdentifier);
        }
        if IDENTIFIER_LAZY_REGEX.is_match(&s) {
            let symbol = store.regular.unsafe_add(s);
            let len = approximate_string_width(s);
            Ok(Identifier { symbol, len })
        } else {
            return Err(IdentifierError::ContainsInvalidCodePoint(
                String::from(s),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::identifier::Identifier;
    use octizys_text_store::store::Store;
    use paste::paste;

    macro_rules! make_negative_test {
        ($name:tt,$s:expr) => {
            paste! {
                #[test]
                fn [<test_ $name _isnt_identifier>]() {
                    let s = $s;
                    let mut store = Store::default();
                    let result = Identifier::make(s,&mut store);
                    assert!(result.is_err(), "s = {}, result = {:?}", s, result);
                }
            }
        };
    }

    make_negative_test!(empty, "");
    make_negative_test!(space, " ");
    make_negative_test!(leading_space, " asf");
    make_negative_test!(trailing_space, "asf ");
    make_negative_test!(line_break, "\n");
    make_negative_test!(underscore_alone, "_");
    make_negative_test!(multiple_underscore, "______");
    make_negative_test!(digit, "1");
    make_negative_test!(digits, "324321");
    make_negative_test!(digits_with_leters, "324asdf");
    make_negative_test!(underscore_and_numbers, "_12323");
    make_negative_test!(underscores_and_numbers, "_123_2_3");
    make_negative_test!(underscores_numbers_and_letters, "_123_2_3asf");
    make_negative_test!(non_aphabetic_symbol, "~");
    make_negative_test!(non_aphabetic_symbols_in_middle, "asfd~asdf");
    make_negative_test!(space_separation, "asdf asdf");
    make_negative_test!(space_separations, "asdf asdf d d er");
    make_negative_test!(trailing_single_quote, "asdf'");
    make_negative_test!(leading_single_quote, "'asdf");
    make_negative_test!(leading_interrogation, "?asdf");
    make_negative_test!(trailing_interrogation, "asdf?");
    make_negative_test!(with_interrogation, "asdf?asdf");

    macro_rules! make_positive_test {
        ($name:tt,$s:expr) => {
            paste! {
                #[test]
                fn [<test_ $name _is_identifier>]() {
                    let s = $s;
                    let mut store = Store::default();
                    let result = Identifier::make(s,&mut store);
                    match result {
                        Err(_)=>
                            assert!(false, "s = {}, result = {:?}", s, result),
                        Ok(value)=>
                            assert!(store.regular.resolve(value.symbol) == Some(s), "s = {}, value = {:?}", s, value),
                    }
                }
            }
        };
    }

    make_positive_test!(latin_letter, "a");
    make_positive_test!(latin_letters, "abcdefg");
    make_positive_test!(greek_letter, "λ");
    make_positive_test!(greek_letters, "λάμ");
    make_positive_test!(leading_underscore, "_a");
    make_positive_test!(leading_underscores, "____a");
    make_positive_test!(full, "__asdf__3434μ3__");
}
