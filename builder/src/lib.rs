mod expand;
mod ast;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, parse_quote};

/// Checks if the type is an option. If it is, will return Some(ty) or None
fn ty_is_option(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let innert_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref ty) = innert_ty {
                return Some(ty);
            }
        }
    }
    None
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::derive(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// /// The entry point for the derive macro
// #[proc_macro_derive(Builder, attributes(builder))]
// pub fn derive_builder(input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as DeriveInput);
//     let name = &ast.ident;
//     let bname = format!("{}Builder", name);
//     let bident = syn::Ident::new(&bname, name.span());
//     let fields = if let syn::Data::Struct(syn::DataStruct {
//         fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
//         ..
//     }) = ast.data
//     {
//         named
//     } else {
//         unimplemented!();
//     };

//     let optionized = fields.iter().map(|f| {
//         let name = &f.ident;
//         let ty = &f.ty;
//         if ty_is_option(ty).is_some() {
//             quote! { #name: #ty }
//         } else {
//             quote! { #name: std::option::Option<#ty> }
//         }
//     });

//     let methods = fields.iter().map(|f| {
//         let name = &f.ident;
//         let ty = &f.ty;
//         if let Some(inner_ty) = ty_is_option(ty) {
//             quote! {
//                 pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
//                     self.#name = Some(#name);
//                     self
//                 }
//             }
//         } else {
//             quote! {
//                 pub fn #name(&mut self, #name: #ty) -> &mut Self {
//                     self.#name = Some(#name);
//                     self
//                 }
//             }
//         }
//     });

//     // TODO: Use parsemeta instead <21-11-20, Brian> //
//     // eprintln!("{:#?}", fields);
//     let extend_methods = fields.iter().filter_map(|f| {
//         for attr in &f.attrs {
//             if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
//                 if let Some(proc_macro2::TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() {
//                     let mut tokens = g.stream().into_iter();
//                     match tokens.next().unwrap() {
//                         TokenTree::Ident(ref i) => assert_eq!(i, "each"),
//                         tt => panic!("expected 'each', found {}", tt)
//                     }
//                     match tokens.next().unwrap() {
//                         TokenTree::Punct(ref p) => assert_eq!(p.as_char(), '='),
//                         tt => panic!("expected '=', fount {}", tt),
//                     }
//                     let arg_tok = tokens.next().unwrap();
//                     let arg_lit: syn::Lit = syn::parse2(arg_tok.into()).expect("Failed to parse into literal");
//                     let arg_ident: syn::Ident = match arg_lit {
//                         syn::Lit::Str(lit_str) => lit_str.parse().expect("Failed to parse lit_str into Ident"),
//                         _ => panic!("It must be a string literal"),
//                     };
//                     return Some(quote! { fn #arg_ident() {} })
//                 } else {
//                     eprintln!("Error if let did not work")
//                 }
//             }
//         }
//         None
//     });

//     let build_fields = fields.iter().map(|f| {
//         let name = &f.ident;
//         let ty = &f.ty;
//         if ty_is_option(ty).is_some() {
//             quote! {
//                 #name: self.#name.clone()
//             }
//         } else {
//             quote! {
//                 #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
//             }
//         }
//     });

//     let build_empty = fields.iter().map(|f| {
//         let name = &f.ident;
//         quote! {
//             #name: None
//         }
//     });

//     let expanded = quote! {
//         pub struct #bident {
//             #(#optionized,)*
//         }

//         impl #bident {
//             #(#methods)*

//             #(#extend_methods)*

//             pub fn build(&self) -> std::result::Result<#name, Box<dyn std::error::Error>> {
//                 Ok(#name {
//                     #(#build_fields,)*
//                 })
//             }
//         }

//         impl #name {
//             fn builder() -> #bident {
//                 #bident {
//                     #(#build_empty,)*
//                 }
//             }
//         }
//     };

//     expanded.into()
// }
