use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::equivalence_parser::{
    parse_input, DataFieldName, EquivalenceInput, GeneratorArguments,
    OtherFieldName, SelfFieldName, StructCaseName,
};

fn generate_equivalent_body(
    names: &Vec<(SelfFieldName, OtherFieldName)>,
) -> proc_macro2::TokenStream {
    let mut results: Vec<proc_macro2::TokenStream> = names
        .into_iter()
        .map(|(self_encapsulate, other_encapsulate)| {
            let self_name = self_encapsulate.access_name();
            let other_name = other_encapsulate.access_name();
            quote! {#self_name.equivalent(&#other_name)}
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

fn generate_represent_body(
    struct_name: StructCaseName,
    mut identifiers: Vec<SelfFieldName>,
) -> proc_macro2::TokenStream {
    if identifiers.len() == 0 {
        let name_str = struct_name.as_string();
        let a = &name_str;
        quote! {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(#a);
            static_str(NAME)
        }
    } else if identifiers.len() == 1 {
        let one = identifiers.pop().unwrap().access_name();
        let name_str = struct_name.as_string();
        let a = &name_str;
        quote! {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(#a);
            static_str(NAME)
                + nest(2, hard_break() +
                        parens(#one.represent())
                )
        }
    } else {
        let last = identifiers.pop().unwrap().access_name();
        let children: proc_macro2::TokenStream = identifiers
            .into_iter()
            .map(|iden| {
                let name_access = iden.access_name();
                quote! {
                    parens(#name_access.represent()), hard_break(),
                }
            })
            .chain(vec![quote! {#last.represent()}])
            .collect();
        let name_str = struct_name.as_string();
        let a = &name_str;
        quote! {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(#a);
            let children_representation = concat(vec![#children]);

            static_str(NAME)
                + nest(2, hard_break() +
                    children_representation
                )
        }
    }
}

fn generate_equivalence_or_diff_body(
    struct_name: StructCaseName,
    fields: &Vec<(SelfFieldName, OtherFieldName)>,
) -> proc_macro2::TokenStream {
    let params = fields
        .into_iter()
        .map(|(self_name,other_name)| {
            let self_access = self_name.access_name();
            let other_access = other_name.access_name();
            let field_result_name = self_name.prefixed("result");
            let field_doc_name = self_name.prefixed("doc");
            (quote! {
                let #field_result_name = #self_access.equivalence_or_diff(&#other_access);
            },
            quote! {
                #field_result_name.is_ok() &
            },
            quote! {
                let #field_doc_name  = #field_result_name.map_or_else(|x| x, |_| parens(#self_access.represent()));
            },
            quote! {
                hard_break(), #field_doc_name ,
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
    let name_value = struct_name.as_string();
    let name_ref = &name_value;
    quote! {
        #results_lets

        if #results_check {
            ::core::result::Result::Ok(())
        } else {
            const NAME: NonLineBreakStr = NonLineBreakStr::new(#name_ref);
            #documents_let
            let children =
                        concat(vec![#document_final_build])
                    ;
            ::core::result::Result::Err(
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

fn generate_represent(
    name: &syn::Ident,
    arguments: &GeneratorArguments,
) -> proc_macro2::TokenStream {
    let body = match arguments {
        GeneratorArguments::Struct(names) => generate_represent_body(
            StructCaseName::Struct(name.to_owned()),
            names
                .to_owned()
                .into_iter()
                .filter(|(_, _, is_ignored)| !is_ignored)
                .map(|(s, _, _)| s)
                .collect(),
        ),
        GeneratorArguments::Enum(branches) => {
            let mut branches_acc = quote! {};
            for (branch_name, names) in branches {
                let body = generate_represent_body(
                    StructCaseName::Enum {
                        struct_name: name.to_owned(),
                        variant_name: branch_name.clone(),
                    },
                    names
                        .to_owned()
                        .into_iter()
                        .filter(|(_, _, is_ignored)| !is_ignored)
                        .map(|(s, _, _)| s)
                        .collect(),
                );
                let mut pattern = quote! {};
                let mut self_acc = quote! {};
                for (name_self, _, is_ignored) in names {
                    match &name_self.field {
                        crate::equivalence_parser::DataFieldName::StructNamed(ident) => {
                            if !is_ignored{
                                let field_name = ident;
                                let pattern_name = name_self.access_name();
                                self_acc.extend(quote! {
                                   #field_name :  #pattern_name,
                                });
                            }
                        },
                        crate::equivalence_parser::DataFieldName::StructIndex(_) => {
                            if !is_ignored{
                                let pattern_name = name_self.access_name();
                                self_acc.extend(quote! {
                                   #pattern_name,
                                });
                            }else{
                               self_acc.extend(quote! { _,} );

                            }
                        }
                    }
                }
                if names.len() > 0 {
                    let (first, _, _) = names
                        .get(0)
                        .expect("first item of a enum, the enum was empty!");
                    pattern.extend(match first.field {
                        DataFieldName::StructNamed(_) => {
                            self_acc.extend(quote! { .. });
                            quote! {Self::#branch_name{#self_acc} => {#body}}
                        }
                        DataFieldName::StructIndex(_) => {
                            quote! {Self::#branch_name(#self_acc) => {#body}}
                        }
                    });
                    branches_acc.extend(quote! {#pattern,})
                } else {
                    let name = StructCaseName::Enum {
                        struct_name: name.to_owned(),
                        variant_name: branch_name.clone(),
                    }
                    .as_string();
                    pattern.extend(quote! {Self::#branch_name => {
                        const NAME: NonLineBreakStr = NonLineBreakStr::new(#name);
                        static_str(NAME)

                    }});
                    branches_acc.extend(quote! {#pattern,})
                }
            }
            quote! {match self {#branches_acc}}
        }
    };

    quote! {
            fn represent(&self)->octizys_pretty::document::Document{
                use ::octizys_text_store::store::NonLineBreakStr;
                use ::octizys_pretty::combinators::{concat,nest,hard_break,static_str,empty};
                use ::octizys_common::equivalence::parens;
                #body
            }
    }
    .into()
}

fn generate_equivalence_or_diff(
    name: &syn::Ident,
    arguments: &GeneratorArguments,
) -> proc_macro2::TokenStream {
    let body = match arguments {
        GeneratorArguments::Struct(names) => generate_equivalence_or_diff_body(
            StructCaseName::Struct(name.to_owned()),
            &from_generict_param_to_vec(names),
        ),
        GeneratorArguments::Enum(branches) => {
            let mut branches_acc = quote! {};
            for (branch_name, names) in branches {
                let body = generate_equivalence_or_diff_body(
                    StructCaseName::Enum {
                        struct_name: name.to_owned(),
                        variant_name: branch_name.clone(),
                    },
                    &from_generict_param_to_vec(names),
                );
                let mut pattern = quote! {};
                let mut self_acc = quote! {};
                let mut other_acc = quote! {};
                for (name_self, name_other, is_ignored) in names {
                    match &name_self.field {
                        crate::equivalence_parser::DataFieldName::StructNamed(ident) => {
                            if !is_ignored{
                                let field_name = ident;
                                let pattern_name = name_self.access_name();
                                let other_pattern_name = name_other.access_name();
                                self_acc.extend(quote! {
                                   #field_name :  #pattern_name,
                                });
                                other_acc.extend(quote! {
                                   #field_name :  #other_pattern_name,
                                })
                            }
                        },
                        crate::equivalence_parser::DataFieldName::StructIndex(_) => {
                            if !is_ignored{
                                let pattern_name = name_self.access_name();
                                let other_pattern_name = name_other.access_name();
                                self_acc.extend(quote! {
                                   #pattern_name,
                                });
                                other_acc.extend(quote! {
                                   #other_pattern_name,
                                })
                            }else{
                               self_acc.extend(quote! { _,} );
                               other_acc.extend(quote! { _,} )

                            }
                        }
                    }
                }
                if names.len() > 0 {
                    let (first, _, _) = names
                        .get(0)
                        .expect("first item of a enum, the enum was empty!");
                    pattern.extend(match first.field {
                        DataFieldName::StructNamed(_) => {
                            self_acc.extend(quote! { .. });
                            other_acc.extend(quote! { .. });
                            quote! {(Self::#branch_name{#self_acc},Self::#branch_name{#other_acc}) => {#body}}
                        }
                        DataFieldName::StructIndex(_) => {
                            quote! {(Self::#branch_name(#self_acc),Self::#branch_name(#other_acc)) => {#body}}
                        }
                    });
                    branches_acc.extend(quote! {#pattern,})
                } else {
                    pattern.extend(
                        quote! {(Self::#branch_name,Self::#branch_name) => {
                            ::core::result::Result::Ok(())
                        }},
                    );
                    branches_acc.extend(quote! {#pattern,})
                }
            }
            quote! {match (self,other) {#branches_acc
                (_,_) =>{
                        ::core::result::Result::Err(
                            make_report(self,other)
                        )
                    }
                }
            }
        }
    };
    quote! {
            fn equivalence_or_diff(&self, other:&Self)->::core::result::Result<(),::octizys_pretty::document::Document>{
                use ::octizys_text_store::store::NonLineBreakStr;
                use ::octizys_pretty::combinators::{concat,nest,hard_break,static_str};
                use ::octizys_common::equivalence::{make_report,parens};
                #body
            }
    }
    .into()
}

fn from_generict_param_to_vec(
    v: &Vec<(SelfFieldName, OtherFieldName, bool)>,
) -> Vec<(SelfFieldName, OtherFieldName)> {
    let mut out: Vec<(SelfFieldName, OtherFieldName)> =
        Vec::with_capacity(v.len());
    for (s, o, is_ignored) in v {
        if !is_ignored {
            let s2: SelfFieldName = s.clone();
            let o2: OtherFieldName = o.clone();
            out.push((s2, o2))
        }
    }
    out
}

fn generate_equivalent(
    _name: &syn::Ident,
    arguments: &GeneratorArguments,
) -> proc_macro2::TokenStream {
    let body = match arguments {
        GeneratorArguments::Struct(names) => {
            generate_equivalent_body(&from_generict_param_to_vec(names))
        }
        GeneratorArguments::Enum(branches) => {
            let mut branches_acc = quote! {};
            for (branch_name, names) in branches {
                let body = generate_equivalent_body(
                    &from_generict_param_to_vec(names),
                );
                let mut pattern = quote! {};
                let mut self_acc = quote! {};
                let mut other_acc = quote! {};
                for (name_self, name_other, is_ignored) in names {
                    match &name_self.field {
                        crate::equivalence_parser::DataFieldName::StructNamed(ident) => {
                            if !is_ignored{
                                let field_name = ident;
                                let pattern_name = name_self.access_name();
                                let other_pattern_name = name_other.access_name();
                                self_acc.extend(quote! {
                                   #field_name :  #pattern_name,
                                });
                                other_acc.extend(quote! {
                                   #field_name :  #other_pattern_name,
                                })
                            }
                        },
                        crate::equivalence_parser::DataFieldName::StructIndex(_) => {
                            if !is_ignored{
                                let pattern_name = name_self.access_name();
                                let other_pattern_name = name_other.access_name();
                                self_acc.extend(quote! {
                                   #pattern_name,
                                });
                                other_acc.extend(quote! {
                                   #other_pattern_name,
                                })
                            }else{
                               self_acc.extend(quote! { _,} );
                               other_acc.extend(quote! { _,} )

                            }
                        }
                    }
                }
                if names.len() > 0 {
                    let (first, _, _) = names
                        .get(0)
                        .expect("first item of a enum, the enum was empty!");
                    pattern.extend(match first.field {
                        DataFieldName::StructNamed(_) => {
                            self_acc.extend(quote! { .. });
                            other_acc.extend(quote! { .. });
                            quote! {(Self::#branch_name{#self_acc},Self::#branch_name{#other_acc}) => {#body}}
                        }
                        DataFieldName::StructIndex(_) => {
                            quote! {(Self::#branch_name(#self_acc),Self::#branch_name(#other_acc)) => {#body}}
                        }
                    });
                    branches_acc.extend(quote! {#pattern,})
                } else {
                    pattern.extend(quote! {(Self::#branch_name,Self::#branch_name) => {true}});
                    branches_acc.extend(quote! {#pattern,})
                }
            }
            quote! {match (self,other) {#branches_acc (_,_) => false}}
        }
    };

    quote! {
            fn equivalent(&self, other:&Self)->bool{
                #body
            }
    }
    .into()
}

fn generate_where(arguments: &EquivalenceInput) -> proc_macro2::TokenStream {
    let struct_equivalence_wheres =
        if arguments.relevant_type_parameters.is_empty() {
            quote! {}
        } else {
            let mut type_constraints = quote! {};
            for param in arguments.relevant_type_parameters.iter() {
                let name = param.to_owned();
                type_constraints.extend(
                    quote! { #name : octizys_common::equivalence::Equivalence,},
                );
            }
            type_constraints
        };
    match arguments.where_clause.clone() {
        Some(struct_wheres) => {
            quote! {
                #struct_wheres
                #struct_equivalence_wheres
            }
        }
        None => {
            if arguments.relevant_type_parameters.is_empty() {
                quote! {}
            } else {
                quote! { where #struct_equivalence_wheres }
            }
        }
    }
}

fn generate_impl(
    parsed: &EquivalenceInput,
    equivalent: proc_macro2::TokenStream,
    equivalence_or_diff: proc_macro2::TokenStream,
    represent: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let struct_params = &parsed.all_struct_parameters;
    let struct_name = &parsed.struct_name;
    let struct_where = generate_where(parsed);
    let out = quote! {
        impl #struct_params octizys_common::equivalence::Equivalence for #struct_name #struct_params
            #struct_where
        {
            #equivalent

            #equivalence_or_diff

            #represent
        }
    };
    out
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

pub(crate) fn derive_equivalence_impl(input: DeriveInput) -> TokenStream {
    let parsed = parse_input(input);
    generate(&parsed).into()
}
