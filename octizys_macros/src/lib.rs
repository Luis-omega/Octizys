// my-app-macros/src/lib.rs
extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, punctuated::Punctuated, token, FieldsNamed, Token};

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

fn generate_structure_equivalent_body(
    fields: Vec<syn::Ident>,
) -> proc_macro2::TokenStream {
    let mut results: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(|field_name| {
            quote! {self.#field_name.equivalent(&other.#field_name)}
        })
        .collect();

    if results.len() == 0 {
        quote! {true}
    } else {
        let last = results.pop().unwrap();
        let code = results.into_iter().map(|x| {
            quote! { #x & }
        });
        code.chain(vec![last]).collect()
    }
}

fn generate_structure_equivalence_or_diff_body(
    struct_name: syn::Ident,
    fields: Vec<syn::Ident>,
) -> proc_macro2::TokenStream {
    let params = fields
        .into_iter()
        .map(|field_name| {
            let field_result_name = format_ident!("result_{}",field_name);
            let field_doc_name = format_ident!("doc_{}",field_name);
            (quote! {
                let #field_result_name = self.#field_name.equivalence_or_diff(&other.#field_name);
            },
            quote! {
                #field_result_name.is_ok() &
            },
            quote! {
                let #field_doc_name  = #field_result_name.map_or_else(|x| x, |_| self.#field_name.represent());
            },
            quote! {
                #field_doc_name ,
            }
            )
        });

    let mut results_lets = quote! {};
    let mut results_check = quote! {};
    let mut documents_let = quote! {};
    let mut document_final_build = quote! {};

    for (l, c, dl, df) in params {
        results_lets.extend(l);
        results_check.extend(c);
        documents_let.extend(dl);
        document_final_build.extend(df);
    }

    results_check.extend(quote! {true});

    quote! {
        #results_lets

        if #results_check {
            Ok(())
        } else {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(stringify!(#struct_name));
            #documents_let
            let children =
                        hard_break() +
                        intersperse( [#document_final_build]
                            ,
                            hard_break()
                        )
                    ;
            Err(
                static_str(NAME)
                +
                nest(
                    2,
                    children
                )
            )
        }
    }
}

fn generate_structure_represent_body(
    struct_name: syn::Ident,
    mut identifiers: Vec<syn::Ident>,
) -> proc_macro2::TokenStream {
    if identifiers.len() == 0 {
        quote! { empty() }
    } else if identifiers.len() == 1 {
        let one = identifiers.pop().unwrap();
        quote! {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(stringify!(#struct_name));
            static_str(NAME)
                + nest(2, hard_break() +
                        parens(self.#one.represent())
                )
        }
    } else {
        let last = identifiers.pop().unwrap();
        let children: proc_macro2::TokenStream = identifiers
            .into_iter()
            .map(|iden| {
                quote! {
                    self.#iden.represent(), hard_break(), sep.clone(),
                }
            })
            .chain(vec![quote! {self.#last.represent()}])
            .collect();
        //panic!("CHILDREN!:{:#?}", children);
        quote! {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(stringify!(#struct_name));
            const SEP : NonLineBreakStr = NonLineBreakStr::new(",");
            let sep = static_str(SEP);
            let children_representation = concat(vec![#children]);

            static_str(NAME)
                + nest(2, hard_break() +
                    children_representation
                )
        }
    }
}

#[proc_macro_derive(Equivalence, attributes(equivalence))]
pub fn derive_equivalence(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_name = &input.ident;
    let struct_params = &input.generics;

    let functions_imports = quote! {
        use octizys_pretty::combinators::{
            background, between_static, hard_break, intersperse, nest,
            empty,concat,static_str
        };
        use octizys_text_store::store::NonLineBreakStr;
        use octizys_common::equivalence::{EXPECTED_BACKGROUND_COLOR,ERROR_BACKGROUND_COLOR,parens};

    };

    let (equivalence_function_body,equivalence_or_diff_body, represent_body): (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ) = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            syn::Fields::Named(named_fields) => {

            let local_fields = named_fields.named.clone();
            let fields : Vec<syn::Ident> = local_fields.into_iter().filter(|f| !find_ignore_attribute(&f.attrs)).map(|f| f.ident.unwrap()).collect();
            let equivalent = generate_structure_equivalent_body(fields.clone());
            let equivalence_or_diff = generate_structure_equivalence_or_diff_body(struct_name.to_owned(),fields.clone());
            let represent = generate_structure_represent_body(struct_name.to_owned(),fields.clone());
            (equivalent,equivalence_or_diff,represent)
            },
            syn::Fields::Unnamed(_) => {
                let mut equivalent_acc: proc_macro2::TokenStream =
                    quote! {true & };
                for (field_number, field) in fields.iter().enumerate() {
                    if find_ignore_attribute(&field.attrs) {
                        continue;
                    }
                    equivalent_acc.extend(quote! { Equivalence::equivalent(self.#field_number, other.#field_number) & })
                }
                equivalent_acc.extend(quote!(true));
                (equivalent_acc,quote! {todo!()},quote!{todo!()})
            }
            syn::Fields::Unit => {
                let equivalence = quote! { self.0.equivalent(other.0) };
                let equivalence_or_diff = quote! {
                    match self.0.equivalence_or_diff(other.0) {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            const NAME: NonLineBreakStr = NonLineBreakStr::new(stringify!(#struct_name));
                            static_str(NAME)
                                + nest(2, hard_break() + e
                                )
                        }
                    }
                };
                let represent = quote! {
                    const NAME: NonLineBreakStr = NonLineBreakStr::new(stringify!(#struct_name));
                    static_str(NAME)
                        + nest(2, hard_break() +
                                parens(self.0.represent())
                        )
                };
                (equivalence,equivalence_or_diff,represent)
            }
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let mut equivalent_branches_acc: proc_macro2::TokenStream =
                quote! {};
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
                equivalent_branches_acc.extend(quote! {#branch_case,});
            }
            (quote! {
                match (self,other) {
                    #equivalent_branches_acc
                }
            },quote! {todo!()}, quote! {todo!()})
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
                        type_constraints.extend(quote! { #name : octizys_common::equivalence::Equivalence,});
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
        impl #struct_params octizys_common::equivalence::Equivalence for #struct_name #struct_params
            #struct_wheres
        {
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
        }

    };
    /*
    if struct_ignores.len() > 0 {
        panic!("IGNORES: {struct_ignores:?}\nTO_WHERE: {debug:?}\nFINAL_WHERES: {struct_wheres:?}\nout!: {out:#?}");
    }
    */

    //panic!("{:#?}", out);

    out.into()
}
