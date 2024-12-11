#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Color4Bits {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Default for Color4Bits {
    fn default() -> Self {
        Color4Bits::Black
    }
}

impl Color4Bits {
    // Returns the ANSI number associated with the color in the foreground.
    pub const fn as_foreground(&self) -> u8 {
        match self {
            Color4Bits::Black => 30,
            Color4Bits::Red => 31,
            Color4Bits::Green => 32,
            Color4Bits::Yellow => 33,
            Color4Bits::Blue => 34,
            Color4Bits::Magenta => 35,
            Color4Bits::Cyan => 36,
            Color4Bits::White => 37,
            Color4Bits::BrightBlack => 90,
            Color4Bits::BrightRed => 91,
            Color4Bits::BrightGreen => 92,
            Color4Bits::BrightYellow => 93,
            Color4Bits::BrightBlue => 94,
            Color4Bits::BrightMagenta => 95,
            Color4Bits::BrightCyan => 95,
            Color4Bits::BrightWhite => 96,
        }
    }

    // Returns the ANSI number associated with the color in the background.
    pub const fn as_background(&self) -> u8 {
        10 + self.as_foreground()
    }

    pub fn ansi_start(&self) -> String {
        let foreground: u8 = self.as_foreground();
        let background: u8 = self.as_background();
        format!("\x1b[{foreground};{background}m")
    }

    pub fn ansi_end() -> String {
        String::from("\x1b[0m")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Color8Bits {
    pub color: u8,
}

impl Default for Color8Bits {
    fn default() -> Self {
        Color8Bits { color: 0 }
    }
}

impl Color8Bits {
    fn ansi_start(&self) -> String {
        let value = self.color;
        format!("\x1b[38;5;{value}m")
    }
    fn ansi_end() -> String {
        format!("\x1b[0m")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Color24Bits {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color24Bits {
    fn default() -> Self {
        Color24Bits { r: 0, g: 0, b: 0 }
    }
}

impl Color24Bits {
    fn ansi_start(&self) -> String {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        format!("\x1b[38;2;;{r};{g};{b}m")
    }

    fn ansi_end() -> String {
        format!("\x1b[0m")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct Color {
    pub color4: Color4Bits,
    pub color8: Color8Bits,
    pub color24: Color24Bits,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Emphasis {
    Normal,
    Bold,
    Faint,
    Italic,
    Underline(Color),
    Invert,
}

impl Default for Emphasis {
    fn default() -> Self {
        Emphasis::Normal
    }
}

impl Emphasis {
    pub fn as_ansi_u8(&self) -> u8 {
        match self {
            Emphasis::Normal => 0,
            Emphasis::Bold => 1,
            Emphasis::Faint => 2,
            Emphasis::Italic => 3,
            Emphasis::Underline(_) => 4,
            Emphasis::Invert => 7,
        }
    }

    pub fn ansi_start(&self) -> String {
        let value = self.as_ansi_u8();
        format!("\x1b[{value}m")
    }

    pub fn ansi_end(&self) -> String {
        String::from("\x1b[0m")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub struct Highlight {
    pub color: Color,
    pub emphasis: Emphasis,
}

/// A way to translate a color to some markup.
pub trait ColorRender {
    /// The first string begin the color, the last one
    /// restores it to the default value.
    fn render_color(color: Color) -> (String, String);
}

/// A way to translate a emphasis to some markup.
pub trait EmphasisRender {
    /// The first string begin the emphasis, the last one
    /// restores it to the default value.
    fn render_emphasis(emphasis: Emphasis) -> (String, String);
}

/// A way to translate a highlight to some markup.
/// It is auto generated if you already implemented the two traits
/// [`ColorRender`] and [`EmphasisRender`].
pub trait HighlightRenderer: ColorRender + EmphasisRender {
    fn render_highlight(highlight: &Highlight) -> (String, String) {
        let (mut color_start, color_end) = Self::render_color(highlight.color);
        let (emphasis_start, mut emphasis_end) =
            Self::render_emphasis(highlight.emphasis);
        color_start.push_str(&emphasis_start);
        emphasis_end.push_str(&color_end);
        (color_start, emphasis_end)
    }
}

/// This discard all info about the highlight.
pub enum EmptyRender {}

impl ColorRender for EmptyRender {
    fn render_color(_color: Color) -> (String, String) {
        (String::new(), String::new())
    }
}
impl EmphasisRender for EmptyRender {
    fn render_emphasis(_emphasis: Emphasis) -> (String, String) {
        (String::new(), String::new())
    }
}
impl HighlightRenderer for EmptyRender {}

/// Use the 4 bits color of terminal.
pub enum TerminalRender4 {}

impl ColorRender for TerminalRender4 {
    fn render_color(color: Color) -> (String, String) {
        (color.color4.ansi_start(), Color4Bits::ansi_end())
    }
}
impl EmphasisRender for TerminalRender4 {
    fn render_emphasis(emphasis: Emphasis) -> (String, String) {
        (emphasis.ansi_start(), emphasis.ansi_end())
    }
}
impl HighlightRenderer for TerminalRender4 {}

/// Use the 8 bits color of terminal.
pub enum TerminalRender8 {}

impl ColorRender for TerminalRender8 {
    fn render_color(color: Color) -> (String, String) {
        (color.color8.ansi_start(), Color8Bits::ansi_end())
    }
}
impl EmphasisRender for TerminalRender8 {
    fn render_emphasis(emphasis: Emphasis) -> (String, String) {
        (emphasis.ansi_start(), emphasis.ansi_end())
    }
}
impl HighlightRenderer for TerminalRender8 {}

/// Use terminal true color.
pub enum TerminalRender24 {}

impl ColorRender for TerminalRender24 {
    fn render_color(color: Color) -> (String, String) {
        (color.color24.ansi_start(), Color24Bits::ansi_end())
    }
}
impl EmphasisRender for TerminalRender24 {
    fn render_emphasis(emphasis: Emphasis) -> (String, String) {
        (emphasis.ansi_start(), emphasis.ansi_end())
    }
}
impl HighlightRenderer for TerminalRender24 {}
