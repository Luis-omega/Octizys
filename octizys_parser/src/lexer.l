%%

U8 "U8"
U16 "U16"
U32 "U32"
U64 "U64"
I8 "I8"
I16 "I16"
I32 "I32"
I64 "I64"
F32 "F32"
F64 "F64"
Bool "Bool"
True "True"
False "False"
import "Import"
export "Export"
data "Data"
\. "Dot"
newtype "Newtype"
as "As"
unqualified "Unqualified"
alias "Alias"
fn "Fn"
forall "Forall"
type "Type"
:: "ModuleSeparator"
\( "LParen"
\) "RParen"


\"\"\".*\"\"\" "MultiLineString"
\"([^"\n]|((\\\\)*\\\"))*\" "String"

0[0_]*|[1-9][0-9_]+ "Uint"
[ \t]+ "Spaces"

(--|//)[^\n]*\n "LineComment"

\{-.*-\}  "BlockComment0"
\{--.*--\}  "BlockComment1"
\{---.*---\}  "BlockComment2"
\{----.*----\}  "BlockComment3"

\n "EOL"

; ";"
, ","
-> "RightArrow"

_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})* "Identifier"

(-|\+|\*|/|=|<|>)+ "Operator"
