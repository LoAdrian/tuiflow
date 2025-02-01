use buildable::buildable_impl;
use proc_macro::TokenStream;

mod buildable;

#[proc_macro_attribute]
pub fn buildable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    buildable_impl(item)
}