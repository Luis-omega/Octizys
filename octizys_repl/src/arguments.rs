use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(
    ValueEnum, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum AvailableParser {
    #[clap(name = "import", help = "Use the parser for import statements.")]
    Import,
    #[clap(name = "type", help = "Use the parser for the type expressions.")]
    Type,
    #[clap(name = "top", help = "Use the parser of a whole file.")]
    Top,
}

#[derive(
    ValueEnum, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum AvailableRenderers {
    #[clap(
        name = "plain",
        help = "Plain text, don't use colors or bold or italic or any thing that isn't text."
    )]
    PlainText,
    #[clap(
        name = "ansi4",
        help = "For terminals that support the 4 bits color scheme."
    )]
    AnsiC4,
    #[clap(
        name = "ansi8",
        help = "For terminals that support the 8 bits color scheme."
    )]
    AnsiC8,
    #[clap(
        name = "ansi24",
        help = "For terminals that support the 24 bits (true color) color scheme."
    )]
    AnsiC24,
}

#[derive(
    ValueEnum, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum DebugFormatOption {
    #[clap(
        name = "pretty",
        help = "Use the rust pretty print debug option {:#?}."
    )]
    PrettyDebug,
    #[clap(name = "normal", help = "Use the rust regular debug option {:?}.")]
    Debug,
    #[clap(
        name = "equiv",
        help = "Use the Equivalence trait (defined in octizys) to create a S-Expression."
    )]
    EquivalenceRepresentation,
}

#[derive(
    ValueEnum, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Phase {
    #[clap(name = "lexer", help = "Show the lexer token stream.")]
    Lexer,
    #[clap(name = "parser", help = "Show the parsing result (currently CST).")]
    Parser,
    #[clap(
        name = "documents",
        help = "Show the `Document` generated for structures before rendering."
    )]
    Document,
    #[clap(name = "ast", help = "Show the AST.")]
    Ast,
    #[clap(name = "typed-ast", help = "Show the AST after type inference.")]
    FullyTypedAST,
    #[clap(name = "core", help = "Show the desugared code `Core`.")]
    Core,
}

#[derive(Parser, Clone, Debug)]
#[group(required = true, multiple = false)]
pub struct DebugCommand {
    pub source_string: Option<String>,
    pub source_path: Option<PathBuf>,
    #[arg(
        short = 'p',
        long = "parser",
        default_value = "top",
        help = "The parser to use on the input."
    )]
    pub parser: AvailableParser,
    #[arg(long = "phase", help = "Show the result of the specific phase.")]
    pub phases: Vec<Phase>,
}

#[derive(Parser, Clone, Debug)]
pub struct FormatterConfiguration {
    #[arg(
        short = 'i',
        long = "indentation",
        name = "INDENTATION",
        default_value = "2",
        help = "The amount of character to use for indentation, if 0 we would use 2."
    )]
    pub indentation_deep: u16,
    #[arg(
        long = "leading-commas",
        name = "LEADING_COMMAS",
        default_value = "true",
        help = "If enabled, the formatter puts delimiters like commas after line breaks, ie at the beginning of a line."
    )]
    pub leading_commas: bool,
    #[arg(
        long = "add-trailing-separator",
        name = "ADD_TRAILING_SEPARATOR",
        default_value = "false",
        help = "If enabled, put a terminated delimiter (like comma) at the end of every structure that supports it.
Disabling this won't delete the commas that you already put."
    )]
    pub add_trailing_separator: bool,
    #[arg(
        long = "move-documentation",
        name = "MOVE_DOCUMENTATION",
        default_value = "true",
        help = "Move the documentation put in front of a item to be above the item.
Formatter would do it anyways if the item is not docummentable, but this option forces it to keep it in place if it can."
    )]
    pub move_documentantion_before_object: bool,
    //TODO: is this a good idea? I see a future where
    // we have a bug with this that adds more indentation every time
    // a formatter is run.
    //pub indent_comment_blocks: bool,
    #[arg(
        long = "separe-by",
        name = "SEPARE_BY",
        default_value = "2",
        help = "The amount of lines to put between top level items like imports, data definitions and function definitions"
    )]
    pub separe_by: u8,
    #[arg(
        long = "compact-comments",
        name = "COMPACT_COMMENTS",
        default_value = "true",
        help = "If the formatter see multiple comments of the same kind (documentation or not documentation) together in the same line or without at least one line break of separation we merge them in a single block comment.
Example:

// a b c
// def
{- w -} {- j -}

becomes
{- a b c
 def
 w
 j
-}
"
    )]
    pub compact_comments: bool,
    #[arg(
        short = 'm',
        long = "machine",
        help = "Set it if you want error messages in a format that can be parsed with a regular expression (mostly).
This also disables the color output."
    )]
    pub use_machine_representation: bool,
    #[arg(
        short = 'r',
        long = "renderer",
        default_value = "ansi24",
        help = "
Define the renderer backend to use to pretty print the generated CST (Concrete Syntax Tree).
This affects the available colors and styles.
In modern terminals you may want ansi24.
If you want to disable colors at all use: plain."
    )]
    pub renderer: AvailableRenderers,
    #[arg(
        short = 'c',
        long = "column-width",
        default_value = "80",
        help = "The maximum amount of columns to use for formatting.
Formatter will try to respect this as much as we can."
    )]
    pub column_width: usize,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "compile")]
    #[command(about = "Compile the given file.")]
    Compile {
        #[arg(short = 'p', long = "path", help = "A file to be compiled.")]
        path: PathBuf,
        #[arg(
            short = 'p',
            long = "path",
            default_value = "a.out",
            help = "A file to place the compilation output."
        )]
        output: Option<PathBuf>,
    },
    #[command(name = "debug")]
    #[command(
        about = "Run the compiler cycle but print to standard output all intermediate results."
    )]
    Debug {
        #[command(flatten)]
        command: DebugCommand,
    },
    #[command(name = "format")]
    // TODO: allow format of multiple directories and files.
    #[command(about = "Format a file")]
    Format {
        #[arg(short = 'p', long = "path", help = "A file to be formatted.")]
        path: PathBuf,
        // TODO: Alternatively an output directory.
        #[arg(
            short = 'o',
            long = "output",
            help = "A file to place the formatting output, if not provided we use standard out."
        )]
        output: Option<PathBuf>,
    },
    #[command(name = "repl")]
    #[command(about = "Start a REPL (Read Eval Print Loop) for octizys")]
    REPL {
        // TODO: Add other commands like:
        // - Load file
        // - History (with maximum history and path to store it.)
        #[arg(
            short = 'p',
            long = "prompt",
            default_value = "octizys_repl>",
            help = "The text that appears at the beginning of every line in the repl."
        )]
        prompt: String,
    },
}

#[derive(Parser, Debug)]
#[command(version = "1.0")]
#[command(about = "Octizys command line utility", long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(
        short = 'a',
        long = "show_arguments",
        help = "Show the passed arguments and exit."
    )]
    pub show_arguments: bool,
    //TODO: handle it
    #[arg(
        long = "debug-format",
        name = "DEBUG_FORMAT",
        default_value = "normal",
        help = "The option used inside format to show intermediate structures"
    )]
    pub display_format: DebugFormatOption,
    #[command(flatten)]
    pub formatter_configuration: FormatterConfiguration,
}
