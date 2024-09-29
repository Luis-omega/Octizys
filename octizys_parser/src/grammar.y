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
%token Dot 'Dot'
%token Newtype 'Newtype'
%token Alias 'Alias'
%token As 'As'
%token Unqualified 'Unqualified'
%token Fn 'Fn'
%token Forall 'Forall'
%token Type 'Type'
%token ModuleSeparator "ModuleSeparator"
%token LParen "LParen"
%token RParen "RParen"
%token MultiLineString "MultiLineString"
%token String "String"
%token Uint 'Uint'
%token Spaces 'Spaces'
%token LineComment "LineComment"
%token BlockComment "BlockComment0"
%token BlockComment1 "BlockComment1"
%token BlockComment2 "BlockComment2"
%token BlockComment3 "BlockComment3"
%token EOL "EOL"
%token Identifier "Identifier"
%token Operator "Operator"
%token RightArrow "RightArrow"

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

/* ';' all_spaces? */
dot -> Result<(),ParserError>:
  Dot all_spaces
  | Dot
  ;

/* LineComment all_spaces? */
line_comment ->Result<(),ParserError>:
  LineComment all_spaces
  | LineComment
  ;

block_comment -> Result<(),ParserError>:
    BlockComment all_spaces
  | BlockComment
  ;

maybe_block_comment -> Result<(),ParserError>:
    all_spaces line_comment
  | line_comment
  | all_spaces
  ;

/* line_comment | all_spaces */
maybe_line_comment ->Result<(),ParserError>:
    all_spaces line_comment
  | line_comment
  | all_spaces
  ;

comment_or_spaces -> Result<(),ParserError>:
  maybe_line_comment
  | maybe_block_comment
  ;

comments_or_spaces -> Result<(),ParserError>:
  comments_or_spaces comment_or_spaces
  | comment_or_spaces
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

operator -> a:
  Operator all_spaces
  | Operator ;

type_keyword -> a:
  Type all_spaces
  | Type ;

unqualified -> a:
  Unqualified all_spaces
  | Unqualified ;

lpar -> a:
  LParen all_spaces
  | LParen ;

rpar -> a:
  RParen all_spaces
  | RParen ;

forall -> a:
  Forall all_spaces
  | Forall ;

comma -> a:
  "," all_spaces
  | "," ;

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



as -> a :
    As
  | As maybe_line_comment
  ;


import -> a :
    Import maybe_line_comment
  | Import
  ;

imports_list_item -> Result<(),ParserError> :
    identifier
  | operator
  | type_keyword operator
  ;

imports_list_recursive -> Result<(),ParserError>:
    imports_list_recursive comma imports_list_item
  | imports_list_recursive comments_or_spaces comma imports_list_item
  | imports_list_recursive comma comments_or_spaces imports_list_item
  | imports_list_recursive comments_or_spaces comma comments_or_spaces imports_list_item
  | imports_list_item
  ;

/*TODO: fix comments, they must be abled Between items*/
imports_list -> Result<(),ParserError>:
    lpar imports_list_recursive rpar
  |  lpar imports_list_recursive comma rpar
  | lpar comma rpar
  ;

/* import unqualified? named_variable imports_list? (as identifier)? */
import_declaration -> Result<()>,ParserError> :
    import named_variable
  | import named_variable as identifier
  | import named_variable imports_list
/*  | import named_variable imports_list as identifier
  | import unqualified named_variable
  | import unqualified named_variable imports_list
  | import unqualified named_variable as identifier
  | import unqualified named_variable imports_list as identifier
*/
  ;


export -> a :
    Export maybe_line_comment
  | Export
  ;

type_variable -> a:
    named_variable
  ;

type_atom -> a :
    type_variable
  | base_type
  | lpar type_expression rpar
  ;

forall_variables -> Result<(),ParserError>:
    forall_variables identifier
  | forall_variables comments_or_spaces identifier
  | identifier
  ;

type_forall  -> Result<(),ParserError>:
    forall forall_variables dot type_expression
  ;

type_application_recusive -> Result<(),ParserError>:
  type_atom type_application_recusive
  | type_atom
  ;

type_application -> Result<(),ParserError>:
  type_expression type_application_recusive
  ;

type_expression  -> a:
    type_application
  | type_forall
  ;

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
    import_declaration colon
  | data_declaration colon
  | newtype_declaration colon
  | alias_declaration colon
  | function_definition_and_declaration colon
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
