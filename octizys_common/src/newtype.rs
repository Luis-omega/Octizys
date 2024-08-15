extern crate proc_macro;
use proc_macro::TokenStream;

pub trait Newtype<New, Original> {
    fn extract(self) -> Original;
}
