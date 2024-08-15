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
newtype "Newtype"
alias "Alias"
fn "Fn"
:: "ModuleSeparator"


\"\"\".*\"\"\" "MultiLineString"
\"([^"\n]|((\\\\)*\\\"))*\" "String"

0[0_]*|[1-9][0-9_]+ "Uint"
[ \t]+ "Spaces"

(--|//)[^\n]*\n "LineComment"

\n "EOL"

; ";"

_*(\p{Alphabetic}|\p{M}|\p{Join_Control})(_|\d|\p{Alphabetic}|\p{M}|\p{Join_Control})* "Identifier" 
