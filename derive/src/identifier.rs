extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

use crate::internals::{attr::Attrs, Ctxt};

pub fn expand_derive_identifier(input: &DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let cx = Ctxt::new();
    let attrs = Attrs::get(&cx, input);
    cx.check()?;

    let ident = &input.ident;
    let with = attrs.with();
    let params = attrs.params();

    let expanded = quote! {
        const _: () = {
            impl identifier::Identifier for #ident {
                type Id = #ident;
                type ParseError = identifier::ParseError;

                fn generate() -> Self::Id {
                    #ident(#with::generate(#params))
                }

                fn format(&self) -> String {
                    format!("{:032x}", self.0)
                }

                fn parse_str(s: &str) -> Result<Self::Id, Self::ParseError> {
                    let s = s.replace("-", "");
                    if s.len() != 32 {
                        Err(identifier::ParseError::InvalidLength)
                    } else {
                        let value = u128::from_str_radix(&s, 16).or(Err(identifier::ParseError::InvalidChars))?;
                        if (#with::validate(value, #params)) {
                            Ok(#ident(value))
                        } else {
                            Err(identifier::ParseError::Invalid)
                        }
                    }
                }
            }
        };
    };

    Ok(expanded)
}

pub fn expand_derive_display(input: &DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let ident = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.format())
            }
        }
    };

    Ok(expanded)
}

pub fn expand_derive_from_str(input: &DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    let ident = &input.ident;
    let trait_path: syn::Path = parse_quote!(core::str::FromStr);

    let expanded = quote! {
        impl #trait_path for #ident {
            type Err = identifier::ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                #ident::parse_str(s)
            }
        }
    };

    Ok(expanded)
}
