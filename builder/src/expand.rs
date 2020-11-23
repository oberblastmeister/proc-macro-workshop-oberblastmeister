use quote::quote;
use proc_macro2::TokenStream;
use syn::{Result, DeriveInput};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    Ok(quote! {/*                         */})
}
