use crate::error::{error_from_document, Error};
use octizys_pretty::combinators::external_text;
use octizys_pretty::document::{aproximate_string_width, Document};
use octizys_pretty::store::{NonLineBreakStr, Store};

use regex::Regex;
use std::sync::LazyLock;
use string_interner::DefaultSymbol;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Identifier {
    symbol: DefaultSymbol,
    len: usize,
}

impl From<Identifier> for DefaultSymbol {
    fn from(value: Identifier) -> Self {
        value.symbol
    }
}

#[derive(Debug, Clone)]
pub enum IdentifierError {
    ContainsInvalidCodePoint(String),
    EmptyIdentifier,
}

/// Keep this in sync with the lexer definition for Identifier
pub static IDENTIFER_LAZY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x) #ingore spaces and allow comments
        ^ #match from the begin
        # can begin with underscores, but not digits
        _*
        #  regular \w contains digits, so, we use instead this
        (\p{Alphabetic}|\p{M}|\p{Join_Control})
        # after we found a valid Alphabetic, we can allow underscores and digits
        (_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})*
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
        if IDENTIFER_LAZY_REGEX.is_match(&s) {
            let symbol = store.regular.unsafe_add(s);
            let len = aproximate_string_width(s);
            Ok(Identifier { symbol, len })
        } else {
            return Err(IdentifierError::ContainsInvalidCodePoint(
                String::from(s),
            ));
        }
    }
}

impl From<&Identifier> for Document {
    fn from(value: &Identifier) -> Document {
        Document::from_symbol_and_len(value.symbol, value.len)
    }
}

impl From<&IdentifierError> for Document {
    fn from(value: &IdentifierError) -> Document {
        match value {
            IdentifierError::ContainsInvalidCodePoint(s)=> {
                    Document::external_non_line_break_str(NonLineBreakStr::new("The passed string is not a valid identifier, it contains invalid characters: "))
                    + external_text(&s.replace("\n","\\n"))
            }
            IdentifierError::EmptyIdentifier => {
                Document::external_non_line_break_str(NonLineBreakStr::new("The passed string is not a valid identifier, it is seen as a empty string"))
            }
        }
    }
}

impl From<IdentifierError> for Error {
    fn from(value: IdentifierError) -> Error {
        error_from_document(&value)
    }
}

#[cfg(test)]
mod tests {
    use crate::identifier::Identifier;
    use octizys_pretty::store::Store;
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
