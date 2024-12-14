use core::panic;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, punctuated::Punctuated, token, FieldsNamed, Token};

#[derive(Debug)]
enum DataFieldName {
    StructNamed(syn::Ident),
    StructIndex(usize),
    Enum(syn::Ident),
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
            DataFieldName::Enum(name) => {
                format_ident!("{}_{}", prefix, name)
            }
        }
    }
}

struct SelfFieldName {
    field: DataFieldName,
}

impl SelfFieldName {
    fn access_name(&self) -> proc_macro2::TokenStream {
        match &self.field {
            DataFieldName::StructNamed(name) => quote! {self.#name},
            DataFieldName::StructIndex(number) => {
                quote! {self.#number}
            }
            DataFieldName::Enum(name) => quote! {#name},
        }
    }
    fn prefixed(&self, prefix: &str) -> syn::Ident {
        self.field.prefixed(prefix)
    }
}

struct OtherFieldName {
    field: DataFieldName,
}

impl OtherFieldName {
    fn access_name(&self) -> proc_macro2::TokenStream {
        match &self.field {
            DataFieldName::StructNamed(name) => quote! {other.#name},
            DataFieldName::StructIndex(number) => {
                quote! {other.#number}
            }
            DataFieldName::Enum(name) => quote! {#name},
        }
    }
}

#[derive(Debug, Clone)]
enum StructCaseName {
    Struct(syn::Ident),
    Enum {
        struct_name: syn::Ident,
        variant_name: syn::Ident,
    },
}

impl StructCaseName {
    fn as_string(&self) -> String {
        match &self {
            StructCaseName::Struct(name) => format!("{}", name),
            StructCaseName::Enum {
                struct_name,
                variant_name,
            } => format!("{struct_name}::{variant_name}"),
        }
    }
}

enum GeneratorArguments {
    Struct(SelfFieldName, OtherFieldName),
    Enum(Vec<(syn::Ident, SelfFieldName, OtherFieldName)>),
}

struct EquivalenceInput {
    struct_name: syn::Ident,
    // All not ignored parameters that are just Identifiers
    relevant_type_parameters: Vec<syn::Ident>,
    // All the parameters of the struct
    all_struct_parameters: Punctuated<syn::GenericParam, syn::Token![,]>,
    generators_arguments: GeneratorArguments,
}

fn parse_arguments(_input: TokenStream) -> EquivalenceInput {
    todo!()
}

fn generate_represent(
    name: &syn::Ident,
    arguments: &GeneratorArguments,
) -> proc_macro2::TokenStream {
    todo!()
}

fn generate_equivalence_or_diff(
    name: &syn::Ident,
    arguments: &GeneratorArguments,
) -> proc_macro2::TokenStream {
    todo!()
}

fn generate_equivalent(
    name: &syn::Ident,
    arguments: &GeneratorArguments,
) -> proc_macro2::TokenStream {
    todo!()
}

fn generate_where(arguments: &EquivalenceInput) -> proc_macro2::TokenStream {
    todo!()
}
/*
            fn equivalent(&self, other:&Self)->bool{
                #functions_imports
                #equivalence_function_body
            }

            fn equivalence_or_diff(&self, other:&Self)->std::result::Result<(),octizys_pretty::document::Document>{
                #functions_imports
                #equivalence_or_diff_body
            }

            fn represent(&self)->octizys_pretty::document::Document{
                #functions_imports
                #represent_body
            }
*/

fn generate_impl(
    parsed: &EquivalenceInput,
    equivalent: proc_macro2::TokenStream,
    equivalence_or_diff: proc_macro2::TokenStream,
    represent: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let struct_params = &parsed.all_struct_parameters;
    let struct_name = &parsed.struct_name;
    let struct_where = generate_where(parsed);
    quote! {
        impl #struct_params octizys_common::equivalence::Equivalence for #struct_name #struct_params
            #struct_where
        {
            #equivalent

            #equivalence_or_diff

            #represent
        }
    }
}

fn generate(parsed: &EquivalenceInput) -> proc_macro2::TokenStream {
    let equivalent =
        generate_equivalent(&parsed.struct_name, &parsed.generators_arguments);
    let equivalence_or_diff = generate_equivalence_or_diff(
        &parsed.struct_name,
        &parsed.generators_arguments,
    );
    let represent =
        generate_represent(&parsed.struct_name, &parsed.generators_arguments);
    generate_impl(parsed, equivalent, equivalence_or_diff, represent)
}

pub(crate) fn derive_equivalence_impl(input: TokenStream) -> TokenStream {
    let parsed = parse_arguments(input);
    generate(&parsed).into()
}
