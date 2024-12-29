extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod equivalence_impl;
mod equivalence_parser;
use crate::equivalence_impl::derive_equivalence_impl;

#[proc_macro_derive(Equivalence, attributes(equivalence))]
pub fn derive_equivalence(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    derive_equivalence_impl(input)
}
