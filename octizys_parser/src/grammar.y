%start top
%token U8 'U8'
%token U16 'U16'
%token U32 'U32'
%token U64 'U64'
%token I8 'I8'
%token I16 'I16'
%token I32 'I32'
%token I64 'I64'
%token F32 'F32'
%token F64 'F64'
%token Bool 'Bool'
%token True 'True'
%token False 'False'
%token Import 'Import'
%token Export 'Export'
%token Data 'Data'
%token Newtype 'Newtype'
%token Alias 'Alias'
%token Fn 'Fn'
%token ModuleSeparator "ModuleSeparator"
%token MultiLineString "MultiLineString"
%token String "String"
%token Uint 'Uint'
%token Spaces 'Spaces'
%token LineComment "LineComment"
%token EOL "EOL"
%token Identifier "Identifier"

%%

/* (spaces | EOL)+ */
all_spaces -> Result<(),ParserError>: 
    Spaces
  | EOL
  | Spaces all_spaces
  | EOL all_spaces
  ;

/* ';' all_spaces? */
colon -> Result<(),ParserError>:
  ';' all_spaces 
  | ';' 
  ;

/* LineComment all_spaces? */
line_comment ->Result<(),ParserError>:
  LineComment all_spaces
  | LineComment
  ;

/* line_comment | all_spaces */
maybe_line_comment ->Result<(),ParserError>:
    all_spaces line_comment
  | line_comment
  | all_spaces
  ;

/* Uint line_comment?  */
uint -> Result<u64,ParserError> :
 Uint maybe_line_comment {
  let value = $1.map_err(|_| ())?;
  parse_int($lexer.span_str(value.span()))
  }
  | Uint
  {
  let value = $1.map_err(|_| ())?;
  parse_int($lexer.span_str(value.span()))
  }
 ;

identifier -> a: 
  Identifier all_spaces
  | Identifier ;

string ->a :
    String all_spaces
  | String
  | MultiLineString
  | MultiLineString all_spaces
  ;

base_type_raw -> Result<BaseType,ParserError> : 
  U8 {U8} 
  | U16 {U16}
  | U32 {U32}
  | U64 {U64}  
  | I8  {I8}
  | I16 {I16}
  | I32 {I32}
  | I64 {I64}
  | F32 {F32}
  | F64 {F64}
  | Bool {Bool}
  ;

base_type -> Result<(),ParserError>:
    base_type_raw maybe_line_comment
  | base_type_raw 
  ;


bool_value_raw -> Result<(),ParserError>:
    True {true}
  | False {false}
  ;

bool_value -> Result<(),ParserError>:
    bool_value_raw  maybe_line_comment
  | bool_value_raw
  ;


module_logical_path_raw -> a :
    module_logical_path_raw ModuleSeparator Identifier
  | Identifier
  ;

named_variable_with_path_raw -> a:
  module_logical_path_raw ModuleSeparator Identifier
  ;

named_variable_alone_raw -> a:
  Identifier 
  ;

named_variable_raw -> a :
    named_variable_alone_raw
  | named_variable_with_path_raw
  ;

named_variable -> a:
    named_variable_raw maybe_line_comment
  | named_variable_raw 
  ;
   



import -> a :
    Import maybe_line_comment
  | Import
  ;


import_declaration -> Result<()>,ParserError> :
    import named_variable
  ;





export -> a :
    Export maybe_line_comment
  | Export
  ;

export_declaration -> Result<()>,ParserError> : export;



data -> a :
    Data maybe_line_comment
  | Data
  ;
data_declaration -> Result<()>,ParserError> : data;


newtype ->a :
    Newtype maybe_line_comment
  | Newtype
  ;

newtype_declaration -> Result<()>,ParserError> : newtype;



alias ->a :
    Alias maybe_line_comment
  | Alias
  ;

alias_declaration -> Result<()>,ParserError> : alias;


fn -> a :
    Fn maybe_line_comment
  | Fn 
  ;

function_definition_and_declaration -> Result<()>,ParserError> : fn;



top_element -> a :
    import_declaration
  | export_declaration
  | data_declaration
  | newtype_declaration
  | alias_declaration
  | function_definition_and_declaration
  ;

/*TODO: change this*/
comment -> a :
  maybe_line_comment
  ;

non_empty_top -> a: top_element
  | non_empty_top top_element
  ;

top-> a : non_empty_top
  | comment
  ;


%%

pub struct ParserError{};

fn parse_int(s: &str) -> Result<u64, ()> {
    match s.parse::<u64>() {
        Ok(val) => Ok(val),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}
