mod identifier;
mod internals;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Identifier, attributes(identifier))]
pub fn derive_identifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    identifier::expand_derive_identifier(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_derive(Display)]
pub fn derive_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    identifier::expand_derive_display(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

#[proc_macro_derive(FromStr)]
pub fn derive_from_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    identifier::expand_derive_from_str(&input)
        .unwrap_or_else(to_compile_errors)
        .into()
}

fn to_compile_errors(errors: Vec<syn::Error>) -> proc_macro2::TokenStream {
    let compile_errors = errors.iter().map(syn::Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
