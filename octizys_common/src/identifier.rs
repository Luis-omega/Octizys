use crate::error::{error_from_pretty, Error};
use crate::newtype::Newtype;
use octizys_pretty::combinators::text;
use octizys_pretty::types::{Document, NoLineBreaksString, Pretty};

use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier<'a>(NoLineBreaksString<'a>);

#[derive(Debug)]
pub enum IdentifierError {
    ContainsInvalidCodePoint,
    EmptyIdentifier,
}

/// Keep this in sync with the lexer definition for Identifier
static IDENTIFER_LAZY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
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

impl<'a> Identifier<'a> {
    pub fn make(s: &'a str) -> Result<Identifier<'a>, IdentifierError> {
        let splitted = NoLineBreaksString::decompose(s);
        if splitted.len() == 1 {
            let zero_element = splitted[0];
            let zero_string = zero_element.extract();
            if IDENTIFER_LAZY_REGEX.is_match(zero_string) {
                Ok(Identifier(zero_element))
            } else {
                return Err(IdentifierError::ContainsInvalidCodePoint);
            }
        } else {
            if splitted.len() > 0 {
                return Err(IdentifierError::ContainsInvalidCodePoint);
            }
            {
                return Err(IdentifierError::EmptyIdentifier);
            }
        }
    }
}

impl<'a> Newtype<Identifier<'a>, NoLineBreaksString<'a>> for Identifier<'a> {
    fn extract(self) -> NoLineBreaksString<'a> {
        self.0
    }
}

impl<'a, 'b> Pretty<'a, 'b> for Identifier<'b> {
    fn to_document(&'a self) -> Document<'b> {
        text(self.0)
    }
}

impl<'a, 'b> Pretty<'a, 'b> for IdentifierError {
    fn to_document(&'a self) -> Document<'b> {
        match self {
            Self::ContainsInvalidCodePoint => {
                "The passed string is not a valid identifier, it contains invalid characters".into()
            }
            Self::EmptyIdentifier => {
                "The passed string is not a valid identifier, it is seen as a empty string".into()
            }
        }
    }
}

impl<'a, 'b> Pretty<'a, 'b> for &IdentifierError {
    fn to_document(&self) -> Document<'b> {
        (*self).to_document()
    }
}

impl<'e> Into<Error<'e>> for IdentifierError {
    fn into(self) -> Error<'e> {
        error_from_pretty(&self)
    }
}

#[cfg(test)]
mod tests {
    use crate::identifier::Identifier;
    use paste::paste;

    macro_rules! make_negative_test {
        ($name:tt,$s:expr) => {
            paste! {
                #[test]
                fn [<test_ $name _isnt_identifier>]() {
                    let s = $s;
                    let result = Identifier::make(s);
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
                    let result = Identifier::make(s);
                    match result {
                        Err(_)=>
                            assert!(false, "s = {}, result = {:?}", s, result),
                        Ok(value)=>
                            assert!(value.0.extract() == s, "s = {}, value = {:?}", s, value),
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
