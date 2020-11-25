use std::fmt;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Attribute, Data, DataStruct, DeriveInput, Error, Generics, Member, Result, Type, Fields};

pub enum Input<'a> {
    Struct(Struct<'a>),
}

pub struct Struct<'a> {
    pub original: &'a DeriveInput,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: Attrs<'a>,
    pub member: Member,
    pub ty: &'a Type,
}

pub struct Attrs<'a> {
    pub each: Option<&'a Attribute>,
}

impl<'a> Input<'a> {
    pub fn from_syn(node: &'a DeriveInput) -> Result<Input<'a>> {
        match &node.data {
            Data::Struct(data) => Struct::from_syn(node, data).map(Input::Struct),
            Data::Enum(_) => Err(Error::new_spanned(
                node,
                "Enum builders are not yet supported",
            )),
            Data::Union(_) => Err(Error::new_spanned(
                node,
                "Union builders are not yet supported",
            )),
        }
    }
}

impl<'a> Struct<'a> {
    pub fn from_syn(node: &'a DeriveInput, data: &'a DataStruct) -> Result<Struct<'a>> {
        let attrs = Attrs::get(&node.attrs);
        let span = Span::call_site();
        let fields = Field::multiple_from_syn(&data.fields, span)?;
        eprintln!("The call site span is:");
        eprintln!("{:#?}", span);
        Ok(
            Struct {
                original: node,
                generics: &node.generics,
                fields,
            }
        )
    }
}

impl<'a> Attrs<'a> {
    pub fn get(input: &[Attribute]) -> Result<Attrs> {
        let mut attrs = Attrs { each: None };
        for attr in input {
            eprintln!("{:#?}", attr.to_token_stream())
        }
        Ok(attrs)
    }
}

impl<'a> Field<'a> {
    fn multiple_from_syn(fields: &'a Fields, span: Span) -> Result<Vec<Self>> {
        fields
            .iter()
            .enumerate()
            .map(|(i, field)| Field::from_syn(i, field, span))
            .collect()
    }

    fn from_syn(i: usize, node: &'a syn::Field, span: Span) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: Attrs::get(&node.attrs)?,
            member: node
                .ident
                .clone()
                .map(Member::Named)
                .ok_or_else(|| panic!("Fix this error"))
                .unwrap(),
            ty: &node.ty,
        })
    }
}
