use octizys_common::{
    identifier::Identifier, module_logic_path::ModuleLogicPath, Store,
    StoreSymbol,
};
use octizys_pretty::{
    combinators::{concat_iter, group, nest},
    document::{aproximate_string_width, Document},
    store::NonLineBreakStr,
};

#[derive(Debug, Copy, Clone)]
pub struct PrettyCSTConfig {
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

impl Default for PrettyCSTConfig {
    fn default() -> Self {
        PrettyCSTConfig {
            indentation_deep: 2,
            leading_commas: true,
            add_trailing_separator: false,
            move_documentantion_before_object: true,
            indent_comment_blocks: true,
            separe_comments_by: 1,
            compact_comments: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CacheItem {
    pub symbol: StoreSymbol,
    pub len: usize,
}

impl From<CacheItem> for Document {
    fn from(value: CacheItem) -> Self {
        Document::from_symbol_and_len(value.symbol, value.len)
    }
}

pub fn make_cache_item(name: &'static str, store: &mut Store) -> CacheItem {
    let s = NonLineBreakStr::new(name);
    let len = aproximate_string_width(name);
    let symbol = store.regular.add_str(s);
    CacheItem { symbol, len }
}

/// A cache for common words that are already registered in a internalizer
/// we use this for two reasons:
/// - Previously we generated a String for every keyword every time.
/// - Storing the keywords in the internalizer and just pass around a immutable
///   reference is fine, but this avoid the lookup for the symbol in the internalizer
///   and guaranties that it is already registered.
/// We use a macro for convenience
macro_rules! make_pretty_cache {
    ($($only_name:ident),+ | $($name:ident : $name_string:literal),+) => {
        #[derive(Debug, Copy, Clone)]
        pub struct PrettyCSTCache {
            $(pub $only_name: CacheItem,)+
            $(pub $name: CacheItem,)+
        }

        impl PrettyCSTCache {
            pub fn new(store: &mut Store) -> PrettyCSTCache {
                $(let $only_name = make_cache_item(stringify!($only_name), store);)+
                $(let $name = make_cache_item($name_string, store);)+
                PrettyCSTCache { $($only_name ,)+ $($name,)+ }
            }
        }

            };
}

// We generate the cache here, the elements before the `|` are
// keywords and the elements after it are symbols that need a field
// name in the cache structure or are reserved keywords and need a different
// to be a field.
make_pretty_cache!(
    import
    ,unqualified
    ,forall
    ,case
    ,of
    |_as : "as"
    ,_let: "let"
    ,_in: "in"
    ,underscore:"_"
    ,hypen:"-"
    ,slash:"/"
    ,lbrace:"{"
    ,rbrace:"}"
    ,lparen:"("
    ,rparen:")"
    ,lbracket:"["
    ,rbracket:"]"
    ,comment_kind: " |"
    ,interrogation : "?"
    ,exclamation : "!"
    ,hash : "#"
    ,comma : ","
    ,colon : ":"
    ,semicolon : ";"
    ,dot : "."
    ,module_separator : "::"
    ,composition_left : "<|"
    ,composition_right : "|>"
    ,plus : "+"
    ,power : "^"
    ,star : "*"
    ,div : "/"
    ,percentage : "%"
    ,shift_left : "<<"
    ,shift_rigth : ">>" //TODO: Add "<&>" = \ x y -> y $ x
    ,map : "<$>"
    ,map_const_rigth : "$>"
    ,map_const_left : "<$" //TODO: add <|> and <?>
    ,appliative : "<*>"
    ,applicative_right : "*>"
    ,applicative_left : "<*"
    ,equality : "=="
    ,not_equal : "!="
    ,less_or_equal : "<="
    ,more_or_equal : "=>"
    ,less_than : "<"
    ,more_than : ">"
    ,and : "&&"
    ,or : "||"
    ,reverse_appliation : "&"
    ,dollar_application : "$"
    ,asignation : "="
    ,at : "@"
    ,pipe : "|"
    ,right_arrow : "->"
    ,left_arrow : "<-"
    ,lambda_start : "\\"
    ,u8  : "U8"
    ,u16 :  "U16"
    ,u32 :  "U32"
    ,u64 :  "U64"
    ,i8  : "I8"
    ,i16 :  "I16"
    ,i32 :  "I32"
    ,i64 :  "I64"
    ,f32 :  "F32"
    ,f64 :  "F64"
    ,char : "Char"
    ,string : "String"
);

#[derive(Debug, Copy, Clone)]
pub struct PrettyCSTContext {
    pub configuration: PrettyCSTConfig,
    pub cache: PrettyCSTCache,
}

impl PrettyCSTContext {
    pub fn new(
        configuration: PrettyCSTConfig,
        cache: PrettyCSTCache,
    ) -> PrettyCSTContext {
        PrettyCSTContext {
            configuration,
            cache,
        }
    }
}

pub fn indent(context: &PrettyCSTContext, doc: Document) -> Document {
    group(nest(context.configuration.indentation_deep, doc))
}

pub trait PrettyCST {
    fn to_document(&self, context: &PrettyCSTContext) -> Document;
}

impl PrettyCST for Identifier {
    fn to_document(&self, _context: &PrettyCSTContext) -> Document {
        self.into()
    }
}

impl PrettyCST for ModuleLogicPath {
    fn to_document(&self, _context: &PrettyCSTContext) -> Document {
        self.into()
    }
}

impl<T> PrettyCST for Box<T>
where
    T: PrettyCST,
{
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        (**self).to_document(context)
    }
}

impl<T> PrettyCST for Vec<T>
where
    T: PrettyCST,
{
    fn to_document(&self, context: &PrettyCSTContext) -> Document {
        concat_iter(self.into_iter().map(|x| x.to_document(context)))
    }
}

impl PrettyCST for &str {
    fn to_document(&self, _context: &PrettyCSTContext) -> Document {
        //TODO:Correctly print strings escaping things
        todo!()
    }
}

impl PrettyCST for String {
    fn to_document(&self, _context: &PrettyCSTContext) -> Document {
        //TODO:Correctly print strings escaping things
        todo!()
    }
}

impl PrettyCST for char {
    fn to_document(&self, _context: &PrettyCSTContext) -> Document {
        //TODO:Correctly print strings escaping things
        todo!()
    }
}

mod private {
    pub trait Sealed {}
}

pub trait Separator: private::Sealed {
    fn to_document(context: &PrettyCSTContext) -> Document;
}

macro_rules! make_separator_type {
    ($name:ident, $value:ident ) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $name {}
        impl private::Sealed for $name {}
        impl Separator for $name {
            fn to_document(context: &PrettyCSTContext) -> Document {
                context.cache.$value.into()
            }
        }
    };
}

make_separator_type!(Pipe, pipe);
make_separator_type!(Comma, comma);
make_separator_type!(RightArrow, right_arrow);
make_separator_type!(Colon, colon);

pub trait Delimiters: private::Sealed {
    fn to_documents(context: &PrettyCSTContext) -> (Document, Document);
}

macro_rules! make_delimiter_type {
    ($name:ident, $left:ident,$right:ident ) => {
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        pub enum $name {}
        impl private::Sealed for $name {}
        impl Delimiters for $name {
            fn to_documents(
                context: &PrettyCSTContext,
            ) -> (Document, Document) {
                (context.cache.$left.into(), context.cache.$right.into())
            }
        }
    };
}

make_delimiter_type!(Parens, lparen, rparen);
make_delimiter_type!(Brackets, lbracket, rbracket);
make_delimiter_type!(Braces, lbrace, rbrace);
