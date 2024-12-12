// my-app-macros/src/lib.rs
extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, punctuated::Punctuated, token, Token};

fn find_ignore_attribute(attributes: &Vec<syn::Attribute>) -> bool {
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

//struct IgnoreItem{
//    name: syn::Ident,
//    sep: Token![,]
//}
//
//impl Parse for IgnoreItem {
//   fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//       Ok( IgnoreItem{
//           name: input.parse()?,
//            sep: input.parse()?,
//       }
//
//           )
//   }
//}
//
//
//struct IgnoreFields{
//    names : Vec<IgnoreItem>,
//    last : syn::Ident,
//}
//
//impl Parse for IgnoreFields {
//    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//        let content;
//        Ok(
//            IgnoreFields{
//                names : input.parse()?
//            }
//        )
//    }
//}
//

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
    ignore: syn::Ident,
    eq: Token![=],
    fields: IgnoreFields,
}

impl Parse for IgnoreInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let maybe_ignore: syn::Ident = input.parse()?;
        if maybe_ignore.to_string() == "ignore" {
            let eq: Token![=] = input.parse()?;
            Ok(IgnoreInput {
                ignore: maybe_ignore,
                eq,
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

#[proc_macro_derive(Equivalence, attributes(equivalence))]
pub fn derive_equivalence(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_name = &input.ident;
    let struct_params = &input.generics;

    let equivalence_function_body: proc_macro2::TokenStream = match &input.data
    {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(_) => {
                let mut acc: proc_macro2::TokenStream = quote! {};
                for field in fields.iter() {
                    if find_ignore_attribute(&field.attrs) {
                        continue;
                    }
                    let field_name = field.ident.as_ref().unwrap();
                    acc.extend(quote! { Equivalence::equivalent(self.#field_name, other.#field_name) & })
                }
                acc.extend(quote!(true));
                acc
            }
            syn::Fields::Unnamed(_) => {
                let mut acc: proc_macro2::TokenStream = quote! {true & };
                for (field_number, field) in fields.iter().enumerate() {
                    if find_ignore_attribute(&field.attrs) {
                        continue;
                    }
                    acc.extend(quote! { Equivalence::equivalent(self.#field_number, other.#field_number) & })
                }
                acc.extend(quote!(true));
                acc
            }
            syn::Fields::Unit => quote! { return true },
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut branches_acc: proc_macro2::TokenStream = quote! {};
            for variant in variants {
                if find_ignore_attribute(&variant.attrs) {
                    continue;
                }
                let variant_name = &variant.ident;
                let mut branch_case = quote! {};
                let mut branch_result = quote! {};
                match variant.fields {
                    syn::Fields::Named(_) => {
                        let mut branch_pattern_match_inner_self = quote! {};
                        let mut branch_pattern_match_inner_other = quote! {};
                        for field in variant.fields.iter() {
                            let name_field = field.ident.as_ref().unwrap();
                            let name_self =
                                format_ident!("selfItem{}", name_field);
                            let name_other =
                                format_ident!("otherItem{}", name_field);
                            branch_pattern_match_inner_self
                                .extend(quote! { #name_field : #name_self , });
                            branch_pattern_match_inner_other
                                .extend(quote! { #name_field: #name_other , });
                            if find_ignore_attribute(&field.attrs) {
                                ()
                            } else {
                                branch_result.extend( quote! { Equivalence::equivalent(#name_self,#name_other) & } );
                            }
                        }
                        branch_case.extend(quote! { (Self::#variant_name(#branch_pattern_match_inner_self), Self::#variant_name(#branch_pattern_match_inner_other)) => #branch_result true })
                    }
                    syn::Fields::Unnamed(_) => {
                        let mut branch_pattern_match_inner_self = quote! {};
                        let mut branch_pattern_match_inner_other = quote! {};
                        for (field_number, field) in
                            variant.fields.iter().enumerate()
                        {
                            if find_ignore_attribute(&field.attrs) {
                                branch_pattern_match_inner_self
                                    .extend(quote! { _ , });
                                branch_pattern_match_inner_other
                                    .extend(quote! { _ , });
                                // We don't need a value here, in case all the fields
                                // are ignored, we still get a true attached at the end of the
                                // branch case, is weird to have :
                                // `Name(_,_,_,_) => true`
                                // but it's good enough.
                            } else {
                                let name_self =
                                    format_ident!("selfItem{}", field_number);
                                let name_other =
                                    format_ident!("otherItem{}", field_number);
                                branch_pattern_match_inner_self
                                    .extend(quote! { #name_self , });

                                branch_pattern_match_inner_other
                                    .extend(quote! { #name_other , });

                                branch_result.extend( quote! { Equivalence::equivalent(#name_self,#name_other) & } );
                            }
                        }
                        branch_case.extend(quote! { (Self::#variant_name(#branch_pattern_match_inner_self), Self::#variant_name(#branch_pattern_match_inner_other)) => #branch_result true })
                    }
                    syn::Fields::Unit => {
                        branch_case.extend(quote! { _ => true });
                    }
                }
                branches_acc.extend(quote! {#branch_case,});
            }
            quote! {
                match (self,other) {
                    #branches_acc
                }
            }
        }
        _ => panic!("Union types are not supported!"),
    };

    let struct_ignores = parse_input_ignores(&input.attrs);
    let mut debug = vec![];

    let struct_equivalence_wheres = if struct_params.params.is_empty() {
        quote! {}
    } else {
        let mut type_constraints = quote! {};
        for param in &struct_params.params {
            match param {
                syn::GenericParam::Type(t) => {
                    if !find_ignore_attribute(&t.attrs)
                        & !struct_ignores.contains(&t.ident)
                    {
                        let name = &t.ident;
                        debug.push(name.clone());
                        type_constraints.extend(quote! { #name : Equivalence,});
                    }
                }
                _ => (),
            }
        }
        type_constraints
    };

    let struct_wheres = match struct_params.where_clause.clone() {
        Some(struct_wheres) => {
            quote! {
                #struct_wheres
                #struct_equivalence_wheres
            }
        }
        None => {
            if struct_params.params.is_empty() {
                quote! {}
            } else {
                quote! { where #struct_equivalence_wheres }
            }
        }
    };

    let out = quote! {
        impl #struct_params Equivalence for #struct_name #struct_params
            #struct_wheres
        {
            fn equivalent(self, other:Self)->bool{
                #equivalence_function_body
            }
        }

    };
    /*
    if struct_ignores.len() > 0 {
        panic!("IGNORES: {struct_ignores:?}\nTO_WHERE: {debug:?}\nFINAL_WHERES: {struct_wheres:?}\nout!: {out:#?}");
    }
    */

    out.into()
}
