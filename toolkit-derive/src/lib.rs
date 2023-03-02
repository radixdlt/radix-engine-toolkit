extern crate proc_macro;

use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;

macro_rules! token_stream {
    ($token: expr) => {
        proc_macro2::TokenStream::from_str($token)
            .expect("Failed to create token stream from trusted source")
    };
}

#[proc_macro_attribute]
pub fn serializable(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let json_schema_attribute = token_stream!("#[derive(schemars::JsonSchema)]");
    let serde_as_attribute = token_stream!("#[serde_with::serde_as]");
    let default_derive_attributes =
        token_stream!("#[derive(serde::Serialize, serde::Deserialize)]");
    let derive_debug_attribute = token_stream!("#[derive(Debug)]");
    let derive_clone_attribute = token_stream!("#[derive(Clone)]");

    TokenStream::from(quote! {
        #json_schema_attribute

        #serde_as_attribute

        #default_derive_attributes

        #derive_debug_attribute

        #derive_clone_attribute

        #input
    })
}
