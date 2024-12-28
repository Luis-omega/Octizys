use quote::{format_ident, quote};
use syn::{
    parse::Parse, DeriveInput, GenericParam, Generics, Token, WhereClause,
};

#[derive(Debug, Clone)]
pub enum DataFieldName {
    StructNamed(syn::Ident),
    StructIndex(usize),
}

impl DataFieldName {
    fn prefixed(&self, prefix: &str) -> syn::Ident {
        match self {
            DataFieldName::StructNamed(name) => {
                format_ident!("{}_{}", prefix, name)
            }
            DataFieldName::StructIndex(number) => {
                format_ident!("{}_{}", prefix, number)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelfFieldName {
    pub field: DataFieldName,
    pub is_enum: bool,
}

impl SelfFieldName {
    pub fn access_name(&self) -> proc_macro2::TokenStream {
        if self.is_enum {
            let name = self.prefixed("self");
            return quote! {#name};
        }
        match &self.field {
            DataFieldName::StructNamed(name) => {
                quote! {self.#name}
            }
            DataFieldName::StructIndex(number) => {
                quote! {self.#number}
            }
        }
    }
    pub fn prefixed(&self, prefix: &str) -> syn::Ident {
        self.field.prefixed(prefix)
    }
}

#[derive(Debug, Clone)]
pub struct OtherFieldName {
    field: DataFieldName,
    is_enum: bool,
}

impl OtherFieldName {
    pub fn access_name(&self) -> proc_macro2::TokenStream {
        if self.is_enum {
            let name = self.prefixed("other");
            return quote! {#name};
        }
        match &self.field {
            DataFieldName::StructNamed(name) => quote! {other.#name},
            DataFieldName::StructIndex(number) => {
                quote! {other.#number}
            }
        }
    }

    pub fn prefixed(&self, prefix: &str) -> syn::Ident {
        self.field.prefixed(prefix)
    }
}

#[derive(Debug, Clone)]
pub enum StructCaseName {
    Struct(syn::Ident),
    Enum {
        struct_name: syn::Ident,
        variant_name: syn::Ident,
    },
}

impl StructCaseName {
    pub fn as_string(&self) -> String {
        match &self {
            StructCaseName::Struct(name) => format!("{}", name),
            StructCaseName::Enum {
                struct_name,
                variant_name,
            } => format!("{struct_name}::{variant_name}"),
        }
    }
}

/// The last boolean represents if the field is ignored or not.
#[derive(Debug, Clone)]
pub(crate) enum GeneratorArguments {
    Struct(Vec<(SelfFieldName, OtherFieldName, bool)>),
    Enum(Vec<(syn::Ident, Vec<(SelfFieldName, OtherFieldName, bool)>)>),
}

pub(crate) struct EquivalenceInput {
    pub struct_name: syn::Ident,
    // All not ignored parameters that are just Identifiers
    pub relevant_type_parameters: Vec<syn::Ident>,
    // All the parameters of the struct
    pub all_struct_parameters: Generics,
    pub where_clause: Option<WhereClause>,
    pub generators_arguments: GeneratorArguments,
}

fn find_simple_ignore_attribute(attributes: &Vec<syn::Attribute>) -> bool {
    for attribute in attributes {
        if attribute.path().is_ident("equivalence") {
            let out = attribute.parse_nested_meta(|meta| {
                if meta.path.is_ident("ignore") {
                    return Ok(());
                }
                match meta.path.get_ident(){
                    Some(name) =>
                        return Err(meta.error(format!("Unrecognized attribute {}, currently we only suport ignore!",name))),
                    None =>
                        return Err(meta.error("Unrecognized attribute, currently we only suport ignore!")),
                }
            });
            if out.is_ok() {
                return true;
            }
            {
                continue;
            }
        }
    }
    false
}

struct IgnoreFields {
    fields: Vec<syn::Ident>,
}

impl Parse for IgnoreFields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut acc = vec![];
        loop {
            let field: syn::Ident = input.parse()?;
            acc.push(field);
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else {
                break;
            }
        }
        Ok(IgnoreFields { fields: acc })
    }
}

struct IgnoreInput {
    _ignore: syn::Ident,
    _eq: Token![=],
    fields: IgnoreFields,
}

impl Parse for IgnoreInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let maybe_ignore: syn::Ident = input.parse()?;
        if maybe_ignore.to_string() == "ignore" {
            let eq: Token![=] = input.parse()?;
            Ok(IgnoreInput {
                _ignore: maybe_ignore,
                _eq: eq,
                fields: input.parse()?,
            })
        } else {
            Err(input.error("We Only support the parameter ignore!"))
        }
    }
}

fn parse_input_ignores(attributes: &Vec<syn::Attribute>) -> Vec<syn::Ident> {
    let mut acc = vec![];
    for attribute in attributes {
        if attribute.path().is_ident("equivalence") {
            let inputs: syn::Result<IgnoreInput> = attribute.parse_args();
            acc.extend(inputs.map_or(vec![], |x| x.fields.fields));
        }
    }
    acc
}

fn filter_non_type_param<I: IntoIterator<Item = GenericParam>>(
    i: I,
) -> Vec<syn::Ident> {
    let mut acc = vec![];
    for j in i {
        match j {
            GenericParam::Type(t) => acc.push(t.ident),
            _ => (),
        }
    }
    acc
}

fn parse_type_parameters(input: &DeriveInput) -> Vec<syn::Ident> {
    let ignores = parse_input_ignores(&input.attrs);
    filter_non_type_param(input.generics.params.clone())
        .into_iter()
        .filter(|x| !ignores.contains(x))
        .collect()
}

fn self_and_other_from_fields(
    fields: &syn::Fields,
    is_enum: bool,
) -> Vec<(SelfFieldName, OtherFieldName, bool)> {
    match fields {
        syn::Fields::Named(named_fields) => {
            let local_fields = named_fields.named.clone();
            local_fields
                .into_iter()
                .map(|f| {
                    let name = f.ident.unwrap();
                    (
                        SelfFieldName {
                            field: DataFieldName::StructNamed(name.clone()),
                            is_enum,
                        },
                        OtherFieldName {
                            field: DataFieldName::StructNamed(name),
                            is_enum,
                        },
                        find_simple_ignore_attribute(&f.attrs),
                    )
                })
                .collect()
        }
        syn::Fields::Unnamed(unnamed_fields) => {
            let local_fields = unnamed_fields.unnamed.clone();
            local_fields
                .into_iter()
                .enumerate()
                .map(|(number, field)| {
                    (
                        SelfFieldName {
                            field: DataFieldName::StructIndex(number),
                            is_enum,
                        },
                        OtherFieldName {
                            field: DataFieldName::StructIndex(number),
                            is_enum,
                        },
                        find_simple_ignore_attribute(&field.attrs),
                    )
                })
                .collect()
        }
        syn::Fields::Unit => vec![],
    }
}

fn parse_generators_arguments(data: &syn::Data) -> GeneratorArguments {
    match data {
        syn::Data::Struct(s) => GeneratorArguments::Struct(
            self_and_other_from_fields(&s.fields, false),
        ),
        syn::Data::Enum(e) => {
            let mut acc: Vec<(
                syn::Ident,
                Vec<(SelfFieldName, OtherFieldName, bool)>,
            )> = vec![];
            for variant in e.variants.iter() {
                acc.push((
                    variant.ident.clone(),
                    self_and_other_from_fields(&variant.fields, true),
                ))
            }
            GeneratorArguments::Enum(acc)
        }
        syn::Data::Union(_) => panic!("Union types are unsupported!"),
    }
}

pub fn parse_input(input: DeriveInput) -> EquivalenceInput {
    let relevant_type_parameters = parse_type_parameters(&input);
    let struct_name = input.ident;
    let all_struct_parameters = input.generics.clone();
    let where_clause = input.generics.where_clause;
    let generators_arguments = parse_generators_arguments(&input.data);

    EquivalenceInput {
        struct_name,
        relevant_type_parameters,
        all_struct_parameters,
        where_clause,
        generators_arguments,
    }
}
