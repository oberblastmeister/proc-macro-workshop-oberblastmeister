use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;
use syn::GenericArgument;
use syn::Ident;
use syn::PathArguments;
use syn::{DeriveInput, Result, Type};

use crate::ast::{Input, Struct};

pub fn derive(node: &DeriveInput) -> Result<TokenStream> {
    let input = Input::from_syn(node)?;
    input.validate()?;
    Ok(match input {
        Input::Struct(input) => impl_struct(input),
        _ => unreachable!(),
    })
}

fn impl_struct(input: Struct) -> TokenStream {
    let ident = &input.ident;

    let builder_ident = {
        let ident = &input.ident;
        let builder_name = format!("{}Builder", &input.ident);
        Ident::new(&builder_name, ident.span())
    };

    let optionized = input.fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        if ty_is_option(ty) {
            quote! { #ident: #ty }
        } else {
            quote! { #ident: ::std::option::Option<#ty> }
        }
    });

    let methods = input.fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        if let Ok(inner_ty) = inner_ty(ty, "Option") {
            quote! {
                pub fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        } else {
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        }
    });

    let builder_fields_empty = input.fields.iter().map(|f| {
        let ident = &f.ident;
        quote! { #ident: None }
    });

    let expanded = quote! {
        // create the builder struct with name builder_ident and optionized fields from the
        // original struct
        pub struct #builder_ident {
            #(#optionized,)*
        }

        // implement methods on to the builder struct that will set fields appropriately
        impl #builder_ident {
            // the regular methods that set individual fields
            #(#methods)*

            // extend methods
        }

        // implementations on the original struct (currently only the builder method)
        impl #ident {
            fn builder() -> #builder_ident {
                #builder_ident {
                    #(#builder_fields_empty,)*
                }
            }
        }
    };

    expanded.into()
}

/// Checks if the the type has the ident of the wrapper. If it does, give the inner type of the
/// wrapper.
fn inner_ty<'a>(ty: &'a Type, wrapper: &str) -> Result<&'a Type> {
    let last = match ty {
        Type::Path(type_path) => {
            type_path.path.segments.last().ok_or_else(|| Error::new_spanned(ty, "The typepath did not have segments"))?
        }
        _ => return Err(Error::new_spanned(ty, "The type must be a typepath")),
    };

    if last.ident != wrapper {
        return Err(Error::new_spanned(ty, format!("The type was not {}", wrapper)));
    }

    let type_arg = match &last.arguments {
        PathArguments::AngleBracketed(bracketed) => {
            let args = &bracketed.args;

            if args.len() != 1 {
                return Err(Error::new_spanned(ty, format!("The type had too many angle bracketed arguments, expected only one.")));
            }

            args.last().expect("Must be okay, len of args was checked above")
        }
        _ => return Err(Error::new_spanned(ty, format!("The type must have angle bracketed path arguments."))),
    };

    match type_arg {
        GenericArgument::Type(type_arg) => Ok(type_arg),
        _ => Err(Error::new_spanned(ty, format!("The GenericArgument must be a type"))),
    }
}

/// Checks if the type is an option. If it is, will return Some(ty) or None
fn ty_is_option(ty: &Type) -> bool {
    let path = match ty {
        Type::Path(ty) => &ty.path,
        _ => return false,
    };

    let last = path.segments.last().unwrap();
    if last.ident != "Option" {
        return false;
    }

    match &last.arguments {
        PathArguments::AngleBracketed(bracketed) => bracketed.args.len() == 1,
        _ => false,
    }
    // if let syn::Type::Path(ref p) = ty {
    //     if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
    //         return None;
    //     }

    //     if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
    //         if inner_ty.args.len() != 1 {
    //             return None;
    //         }

    //         let innert_ty = inner_ty.args.first().unwrap();
    //         if let syn::GenericArgument::Type(ref ty) = innert_ty {
    //             return Some(ty);
    //         }
    //     }
    // }
    // None
}
