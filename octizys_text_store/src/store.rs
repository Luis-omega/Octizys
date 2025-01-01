use konst::const_panic;
use konst::string;
use string_interner::DefaultStringInterner;
use string_interner::DefaultSymbol;

//TODO: change this to a better approximate using graphemes
pub fn approximate_string_width(s: &str) -> usize {
    s.chars().count()
}

///Purpose: Build non line break strings at compilation time
///Is a intermediate structure used in NonLineBreakString.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonLineBreakStr(&'static str);

impl NonLineBreakStr {
    pub const fn new(source: &'static str) -> NonLineBreakStr {
        match string::find(source, "\n") {
            Some(x) => {
                const_panic::concat_panic!(
                    false,
                    "String contains a line break! At index:",
                    x
                );
            }
            None => NonLineBreakStr(source),
        }
    }

    pub fn validate(s: &'static str) -> bool {
        string::find(s, "\n").is_none()
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonLineBreakString(String);

impl NonLineBreakString {
    pub fn make(source: &str) -> Option<NonLineBreakString> {
        if NonLineBreakString::validate(source) {
            Some(NonLineBreakString(String::from(source)))
        } else {
            None
        }
    }

    pub fn validate(s: &str) -> bool {
        s.find("\n").is_none()
    }

    /// Returns a iterator of NonLineBreakString based on the
    /// provided string. You still need to take on account the
    /// line break by yourself.
    pub fn decompose<'a>(
        source: &'a str,
    ) -> std::iter::Map<
        std::str::Split<&'a str>,
        impl FnMut(&'a str) -> NonLineBreakString,
    > {
        let s = source
            .split("\n")
            .map(|s| NonLineBreakString(String::from(s)));
        s
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<NonLineBreakStr> for NonLineBreakString {
    fn from(value: NonLineBreakStr) -> Self {
        NonLineBreakString(String::from(value.0))
    }
}

impl From<NonLineBreakString> for String {
    fn from(value: NonLineBreakString) -> Self {
        value.0
    }
}

impl<'a> From<&'a NonLineBreakString> for &'a str {
    fn from(value: &'a NonLineBreakString) -> &'a str {
        value.0.as_str()
    }
}

impl From<NonLineBreakStr> for &'static str {
    fn from(value: NonLineBreakStr) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct CommentStore {
    store: Vec<NonLineBreakString>,
}

impl CommentStore {
    pub fn new(store: Vec<NonLineBreakString>) -> CommentStore {
        CommentStore { store }
    }

    /// Store a single string without line breaks.
    pub fn add(&mut self, s: NonLineBreakString) -> usize {
        let len = self.store.len();
        self.store.push(s);
        len
    }

    /// Store a iterator of strings without line breaks,
    /// you may be interested in `NonLineBreakString::decompose`
    pub fn extend<T: Iterator<Item = NonLineBreakString>>(
        &mut self,
        s: T,
    ) -> std::ops::Range<usize> {
        let original = self.store.len();
        self.store.extend(s);
        let new = self.store.len();
        original..new
    }

    pub fn extend_and_get_lens<'a, T: Iterator<Item = NonLineBreakString>>(
        &'a mut self,
        s: T,
    ) -> std::iter::Map<
        std::ops::Range<usize>,
        impl FnMut(usize) -> (usize, usize) + 'a,
    > {
        self.extend(s).map(|i| (i, self.store[i].len()))
    }

    /// Store a single string without line breaks.
    pub fn add_str(&mut self, s: &str) -> Option<usize> {
        let len = self.store.len();
        let converted = NonLineBreakString::make(s)?;
        self.store.push(converted);
        Some(len)
    }

    pub fn resolve(&self, index: usize) -> Option<&NonLineBreakString> {
        self.store.get(index)
    }
}

#[derive(Debug, Clone)]
pub struct RegularStore {
    store: DefaultStringInterner,
}

pub type StoreSymbol = DefaultSymbol;

impl RegularStore {
    pub fn new(interner: DefaultStringInterner) -> Self {
        RegularStore { store: interner }
    }

    /// The passed string is guaranteed to be verified at
    /// compilation time tanks to NonLineBreakStr, so we
    /// can skip the verification at run time with this function.
    pub fn add_str(&mut self, s: NonLineBreakStr) -> StoreSymbol {
        self.store.get_or_intern_static(s.0)
    }

    pub fn add(&mut self, s: NonLineBreakString) -> StoreSymbol {
        self.store.get_or_intern(s.0)
    }

    pub fn try_add(&mut self, s: &str) -> Option<StoreSymbol> {
        if NonLineBreakString::validate(s) {
            Some(self.store.get_or_intern(s))
        } else {
            None
        }
    }

    /// Be sure that the string here was already processed to guaranteed
    /// that it doesn't have line breaks!
    pub fn unsafe_add(&mut self, s: &str) -> StoreSymbol {
        self.store.get_or_intern(s)
    }

    pub fn get(&self, s: &str) -> Option<StoreSymbol> {
        self.store.get(s)
    }

    pub fn resolve<'a>(&'a self, s: StoreSymbol) -> Option<&'a str> {
        self.store.resolve(s)
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub regular: RegularStore,
    pub comments: CommentStore,
}

impl Store {
    pub fn new(
        interner: DefaultStringInterner,
        comments: Vec<NonLineBreakString>,
    ) -> Self {
        Store {
            regular: RegularStore::new(interner),
            comments: CommentStore::new(comments),
        }
    }
}

impl Default for RegularStore {
    fn default() -> Self {
        RegularStore {
            store: DefaultStringInterner::new(),
        }
    }
}

impl Default for CommentStore {
    fn default() -> Self {
        CommentStore { store: Vec::new() }
    }
}

impl Default for Store {
    fn default() -> Self {
        Store {
            regular: RegularStore::default(),
            comments: CommentStore::default(),
        }
    }
}
