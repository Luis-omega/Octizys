use octizys_pretty::combinators::{
    self, background, between_static, hard_break, intersperse, nest,
};
use octizys_pretty::document::Document;

use octizys_pretty::highlight::{
    Color, Color24Bits, Color4Bits, Color8Bits, Highlight,
};
use octizys_pretty::store::Store;
use octizys_text_store::store::NonLineBreakStr;

pub const EXPECTED_BACKGROUND_COLOR: Color = Color {
    color24: Color24Bits { r: 0, g: 117, b: 0 },
    color8: Color8Bits { color: 28 },
    color4: Color4Bits::Blue,
};

pub const ERROR_BACKGROUND_COLOR: Color = Color {
    color24: Color24Bits { r: 117, g: 0, b: 0 },
    color8: Color8Bits { color: 88 },
    color4: Color4Bits::Magenta,
};

pub fn parens<Doc: Into<Document>>(inner: Doc) -> Document {
    between_static(
        NonLineBreakStr::new("("),
        nest(2, inner.into()) + hard_break(),
        NonLineBreakStr::new(")"),
    )
}

pub trait Equivalence {
    fn equivalent(&self, other: &Self) -> bool;
    fn equivalence_or_diff(&self, other: &Self) -> Result<(), Document>;

    fn represent(&self) -> Document;
}

/// We already know that [`self`] and [`other`] aren't equal and we want
/// to represent both side by side to see the differences.
pub fn make_report<E: Equivalence>(left: &E, right: &E) -> Document {
    let left = left.represent();
    let right = right.represent();
    let report = combinators::concat(vec![
        combinators::background(EXPECTED_BACKGROUND_COLOR, parens(left)),
        combinators::hard_break(),
        combinators::background(ERROR_BACKGROUND_COLOR, parens(right)),
    ]);
    report
}

pub fn assert_equivalent<E: Equivalence>(
    l: &E,
    r: &E,
    highlight: fn(&Highlight) -> (String, String),
) -> () {
    match Equivalence::equivalence_or_diff(l, r) {
        Ok(_) => (),
        Err(report) => {
            let store = Store::default();
            let rendered = report.render_to_string(80, highlight, &store);
            panic!("Not equivalent!\n{}", rendered)
        }
    }
}

macro_rules! implement_from_eq {
    ($name:ty) => {
        impl Equivalence for $name {
            fn equivalent(&self, other: &Self) -> bool {
                self == other
            }

            fn equivalence_or_diff(
                &self,
                other: &Self,
            ) -> Result<(), Document> {
                if self == other {
                    Ok(())
                } else {
                    Err(make_report(self, other))
                }
            }
            fn represent(&self) -> Document {
                const NAME: NonLineBreakStr =
                    NonLineBreakStr::new(stringify!($name));
                combinators::static_str(NAME)
                    + combinators::external_text(&format!(" {}", self))
            }
        }
    };
}

implement_from_eq!(bool);
implement_from_eq!(u8);
implement_from_eq!(u16);
implement_from_eq!(u32);
implement_from_eq!(u64);
implement_from_eq!(u128);
implement_from_eq!(usize);
implement_from_eq!(i8);
implement_from_eq!(i16);
implement_from_eq!(i32);
implement_from_eq!(i64);
implement_from_eq!(i128);
implement_from_eq!(f32);
implement_from_eq!(f64);
implement_from_eq!(isize);
implement_from_eq!(String);
implement_from_eq!(&str);
implement_from_eq!(char);

impl<T: Equivalence> Equivalence for Option<T> {
    fn equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(s), Some(o)) => s.equivalent(o),
            _ => false,
        }
    }

    fn equivalence_or_diff(&self, other: &Self) -> Result<(), Document> {
        match (self, other) {
            (None, None) => Ok(()),
            (Some(s), Some(o)) => {
                if s.equivalent(o) {
                    Ok(())
                } else {
                    Err(make_report(self, other))
                }
            }
            _ => Err(make_report(self, other)),
        }
    }

    fn represent(&self) -> Document {
        match self {
            Some(x) => {
                combinators::static_str(NonLineBreakStr::new("Some "))
                    + nest(2, hard_break() + parens(x.represent()))
            }
            None => combinators::static_str(NonLineBreakStr::new("None")),
        }
    }
}

impl<T: Equivalence> Equivalence for Box<T> {
    fn equivalent(&self, other: &Self) -> bool {
        Equivalence::equivalent(self.as_ref(), other.as_ref())
    }
    fn equivalence_or_diff(&self, other: &Self) -> Result<(), Document> {
        Equivalence::equivalence_or_diff(self.as_ref(), other.as_ref())
    }
    fn represent(&self) -> Document {
        Equivalence::represent(self.as_ref())
    }
}

impl<T> Equivalence for Vec<T>
where
    T: Equivalence,
{
    fn equivalent(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (l, r) in std::iter::zip(self, other) {
            if !Equivalence::equivalent(l, r) {
                return false;
            }
        }
        return true;
    }

    fn equivalence_or_diff(&self, other: &Self) -> Result<(), Document> {
        let self_len = self.len();
        let other_len = other.len();
        if self_len > other_len {
            let results = std::iter::zip(self, other).map(|(s, o)| {
                Equivalence::equivalence_or_diff(s, o)
                    .map_or_else(|report| report, |_| parens(s.represent()))
            });
            let remain = &self[other_len..];
            const VECT: NonLineBreakStr = NonLineBreakStr::new("Vec");
            Err(combinators::concat(vec![
                combinators::static_str(VECT),
                nest(
                    2,
                    hard_break()
                        + combinators::intersperse(
                            results.chain(remain.iter().map(|item| {
                                background(
                                    EXPECTED_BACKGROUND_COLOR,
                                    parens(Equivalence::represent(item)),
                                )
                            })),
                            hard_break(),
                        ),
                ),
            ]))
        } else if self_len < other_len {
            let results = std::iter::zip(self, other).map(|(s, o)| {
                Equivalence::equivalence_or_diff(s, o)
                    .map_or_else(|report| report, |_| parens(s.represent()))
            });
            let remain = &other[self_len..];
            const VECT: NonLineBreakStr = NonLineBreakStr::new("Vec");
            Err(combinators::concat(vec![
                combinators::static_str(VECT),
                nest(
                    2,
                    hard_break()
                        + combinators::intersperse(
                            results.chain(remain.iter().map(|item| {
                                background(
                                    ERROR_BACKGROUND_COLOR,
                                    parens(Equivalence::represent(item)),
                                )
                            })),
                            hard_break(),
                        ),
                ),
            ]))
        } else {
            let results: Vec<_> = std::iter::zip(self, other)
                .map(|(s, o)| {
                    Equivalence::equivalence_or_diff(s, o)
                        .map(|_| || s.represent())
                })
                .collect();
            if results.iter().all(Result::is_ok) {
                Ok(())
            } else {
                const VECT: NonLineBreakStr = NonLineBreakStr::new("Vec");
                Err(combinators::concat(vec![
                    combinators::static_str(VECT),
                    nest(
                        2,
                        combinators::concat(vec![
                            hard_break(),
                            intersperse(
                                results.into_iter().map(
                                    |result| match result {
                                        Ok(f) => parens(f()),
                                        Err(e) => e,
                                    },
                                ),
                                hard_break(),
                            ),
                        ]),
                    ),
                ]))
            }
        }
    }

    fn represent(&self) -> Document {
        const VECT: NonLineBreakStr = NonLineBreakStr::new("Vec");
        let remain = self.iter().map(|x| parens(x.represent()));
        combinators::static_str(VECT)
            + nest(2, hard_break() + intersperse(remain, hard_break()))
    }
}

#[cfg(test)]
mod equivalence_test {
    use octizys_pretty::highlight::{
        HighlightRenderer, TerminalRender24, TerminalRender4, TerminalRender8,
    };

    use super::{assert_equivalent, Equivalence};

    #[test]
    fn symmetry() {
        assert_equivalent(&true, &true, TerminalRender24::render_highlight);
        assert_equivalent(&1, &1, TerminalRender24::render_highlight);
        assert_equivalent(
            &Some('a'),
            &Some('a'),
            TerminalRender24::render_highlight,
        );
        assert_equivalent(
            &None::<u8>,
            &None::<u8>,
            TerminalRender24::render_highlight,
        );
        assert_equivalent(
            &Box::new(1),
            &Box::new(2),
            TerminalRender24::render_highlight,
        );
        assert_equivalent(
            &vec!["ab", "cd", "ef"],
            &vec!["ab", "cd", "ef"],
            TerminalRender24::render_highlight,
        );
    }
}
