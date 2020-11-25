use quote::quote;
use proc_macro2::TokenStream;
use syn::{Result, DeriveInput};

use crate::ast::{Input, Struct};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    Ok(quote! {/*                         */})
}

fn impl_struct(input: Struct) -> TokenStream {
    quote!{ /*                             */ }
}
