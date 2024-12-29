use octizys_common::{identifier::Identifier, span::Span};
use octizys_macros::Equivalence;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Equivalence,
)]
pub enum StringKind {
    Raw0, //r#
    Raw1, //r##
    Raw2, //r###
    Raw3, //r####
    #[default]
    Normal, //"
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Equivalence)]
pub enum StringComponent {
    RegularString(String),
    Scaped(String),
}

impl Default for StringComponent {
    fn default() -> Self {
        Self::RegularString(String::new())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Equivalence, Hash, PartialOrd, Ord)]
pub struct StringLiteral {
    pub kind: StringKind,
    pub value: Vec<StringComponent>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Equivalence,
)]
pub struct InterpolationOptions {
    pub is_debug: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Equivalence)]
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

#[derive(Debug, Clone, PartialEq, Eq, Equivalence)]
pub struct InterpolationString {
    pub value: Vec<InterpolationComponent>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Equivalence,
)]
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

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Equivalence,
)]
pub struct UintLiteral {
    pub kind: UintKind,
    pub value: u64,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Equivalence,
)]
pub enum UFloatingPointKind {
    F32,
    #[default]
    F64,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Equivalence,
)]
pub struct UFloatingPointLiteral {
    pub kind: UFloatingPointKind,
    pub integral_part: u64,
    pub exponent: u8,
}
