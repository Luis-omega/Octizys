use octizys_pretty::store::NonLineBreakStr;

macro_rules! make_keywords {
    ($($only_name:ident),+ | $($name:ident : $name_string:literal),+) => {
       $(pub const $only_name : NonLineBreakStr = NonLineBreakStr::new(stringify!($only_name));)+
       $(pub const $name : NonLineBreakStr = NonLineBreakStr::new(stringify!($name_string));)+
    };
}

make_keywords!(
    IMPORT
    ,UNQUALIFIED
    ,FORALL
    ,CASE
    ,OF
    |AS : "as"
    ,LET: "let"
    ,IN: "in"
    ,UNDERSCORE:"_"
    ,HYPEN:"-"
    ,SLASH:"/"
    ,LBRACE:"{"
    ,RBRACE:"}"
    ,LPAREN:"("
    ,RPAREN:")"
    ,LBRACKET:"["
    ,RBRACKET:"]"
    ,COMMENT_KIND: " |"
    ,INTERROGATION : "?"
    ,EXCLAMATION : "!"
    ,HASH : "#"
    ,COMMA : ","
    ,COLON : ":"
    ,SEMICOLON : ";"
    ,DOT : "."
    ,MODULE_SEPARATOR : "::"
    ,COMPOSITION_LEFT : "<|"
    ,COMPOSITION_RIGHT : "|>"
    ,PLUS : "+"
    ,POWER : "^"
    ,STAR : "*"
    ,DIV : "/"
    ,PERCENTAGE : "%"
    ,SHIFT_LEFT : "<<"
    ,SHIFT_RIGTH : ">>" //TODO: Add "<&>" = \ x y -> y $ x
    ,MAP : "<$>"
    ,MAP_CONST_RIGTH : "$>"
    ,MAP_CONST_LEFT : "<$" //TODO: add <|> and <?>
    ,APPLIATIVE : "<*>"
    ,APPLICATIVE_RIGHT : "*>"
    ,APPLICATIVE_LEFT : "<*"
    ,EQUALITY : "=="
    ,NOT_EQUAL : "!="
    ,LESS_OR_EQUAL : "<="
    ,MORE_OR_EQUAL : "=>"
    ,LESS_THAN : "<"
    ,MORE_THAN : ">"
    ,AND : "&&"
    ,OR : "||"
    ,REVERSE_APPLIATION : "&"
    ,DOLLAR_APPLICATION : "$"
    ,ASIGNATION : "="
    ,AT : "@"
    ,PIPE : "|"
    ,RIGHT_ARROW : "->"
    ,LEFT_ARROW : "<-"
    ,LAMBDA_START : "\\"
    ,U8  : "U8"
    ,U16 :  "U16"
    ,U32 :  "U32"
    ,U64 :  "U64"
    ,I8  : "I8"
    ,I16 :  "I16"
    ,I32 :  "I32"
    ,I64 :  "I64"
    ,F32 :  "F32"
    ,F64 :  "F64"
    ,CHAR : "Char"
    ,STRING : "String"
);
