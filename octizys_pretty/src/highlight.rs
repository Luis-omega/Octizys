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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Highlight {
    pub background: Color,
    pub foreground: Color,
    pub emphasis: Emphasis,
}

impl Default for Highlight {
    fn default() -> Self {
        Highlight {
            background: Default::default(),
            foreground: Color {
                color4: Color4Bits::White,
                color8: Color8Bits { color: 15 },
                color24: Color24Bits {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            },
            emphasis: Default::default(),
        }
    }
}

/// A way to translate a color to some markup.
pub trait ColorRender {
    /// The first string begin the color, the last one
    /// restores it to the default value.
    fn render_color(backgrond: Color, foreground: Color) -> (String, String);
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
        let (color_start, mut color_end) =
            Self::render_color(highlight.background, highlight.foreground);
        let (mut emphasis_start, emphasis_end) =
            Self::render_emphasis(highlight.emphasis);
        emphasis_start.push_str(&color_start);
        color_end.push_str(&emphasis_end);
        (emphasis_start, color_end)
    }
}

/// This discard all info about the highlight.
pub enum EmptyRender {}

impl ColorRender for EmptyRender {
    fn render_color(_backgrond: Color, _foregrond: Color) -> (String, String) {
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
    fn render_color(background: Color, foreground: Color) -> (String, String) {
        let foreground_uint: u8 = foreground.color4.as_foreground();
        let background_uint: u8 = background.color4.as_background();
        (
            format!("\x1b[{foreground_uint};{background_uint}m"),
            Color4Bits::ansi_end(),
        )
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
    fn render_color(background: Color, foreground: Color) -> (String, String) {
        let background_uint = background.color8.color;
        let foreground_uint = foreground.color8.color;
        (
            format!("\x1b[38;5;{foreground_uint}m\x1b[48;5;{background_uint}m"),
            Color8Bits::ansi_end(),
        )
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
    fn render_color(background: Color, foreground: Color) -> (String, String) {
        let rb = background.color24.r;
        let gb = background.color24.g;
        let bb = background.color24.b;
        let rf = foreground.color24.r;
        let gf = foreground.color24.g;
        let bf = foreground.color24.b;
        (
            format!("\x1b[38;2;{rf};{gf};{bf}m\x1b[48;2;{rb};{gb};{bb}m"),
            Color24Bits::ansi_end(),
        )
    }
}
impl EmphasisRender for TerminalRender24 {
    fn render_emphasis(emphasis: Emphasis) -> (String, String) {
        (emphasis.ansi_start(), emphasis.ansi_end())
    }
}
impl HighlightRenderer for TerminalRender24 {}

pub mod base_colors {
    use super::*;

    pub const RED: Color = Color {
        color4: Color4Bits::Red,
        color8: Color8Bits { color: 160 },
        color24: Color24Bits { r: 255, g: 0, b: 0 },
    };

    pub const GREEN: Color = Color {
        color4: Color4Bits::Green,
        color8: Color8Bits { color: 28 },
        color24: Color24Bits { r: 0, g: 255, b: 0 },
    };

    pub const BLUE: Color = Color {
        color4: Color4Bits::Blue,
        color8: Color8Bits { color: 20 },
        color24: Color24Bits { r: 0, g: 0, b: 255 },
    };

    pub const BLACK: Color = Color {
        color4: Color4Bits::Black,
        color8: Color8Bits { color: 0 },
        color24: Color24Bits { r: 0, g: 0, b: 0 },
    };

    pub const WHITE: Color = Color {
        color4: Color4Bits::White,
        color8: Color8Bits { color: 15 },
        color24: Color24Bits {
            r: 255,
            g: 250,
            b: 250,
        },
    };

    pub const CYAN: Color = Color {
        color4: Color4Bits::Cyan,
        color8: Color8Bits { color: 33 },
        color24: Color24Bits {
            r: 58,
            g: 150,
            b: 21,
        },
    };

    pub const MAGENTA: Color = Color {
        color4: Color4Bits::Magenta,
        color8: Color8Bits { color: 126 },
        color24: Color24Bits {
            r: 156,
            g: 0,
            b: 156,
        },
    };
}
