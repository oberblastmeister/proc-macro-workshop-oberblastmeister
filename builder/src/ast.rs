use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, Fields, Generics, Ident, Member, Result, Type,
};

pub enum Input<'a> {
    Struct(Struct<'a>),
}

pub struct Struct<'a> {
    pub ident: &'a Ident,
    pub attrs: Attrs<'a>,
    pub original: &'a DeriveInput,
    pub generics: &'a Generics,
    pub fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    pub original: &'a syn::Field,
    pub attrs: Attrs<'a>,
    pub ident: &'a Ident,
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
        let attrs = Attrs::get(&node.attrs)?;
        let fields = Field::multiple_from_syn(&data.fields)?;
        let ident = &node.ident;
        Ok(Struct {
            ident,
            attrs,
            original: node,
            generics: &node.generics,
            fields,
        })
    }
}

impl<'a> Attrs<'a> {
    pub fn get(input: &[Attribute]) -> Result<Attrs> {
        let mut attrs = Attrs { each: None };
        for attr in input {
            let meta = attr.parse_meta()?;
            println!("Meta: {:#?}", meta);
            println!("Attr: {:#?}", attr);
        }
        Ok(attrs)
    }
}

impl<'a> Field<'a> {
    fn multiple_from_syn(fields: &'a Fields) -> Result<Vec<Self>> {
        fields.iter().map(|field| Field::from_syn(field)).collect()
    }

    fn from_syn(node: &'a syn::Field) -> Result<Self> {
        Ok(Field {
            original: node,
            attrs: Attrs::get(&node.attrs)?,
            ident: node
                .ident
                .as_ref()
                .ok_or_else(|| Error::new_spanned(node, "The struct's fields must be named"))?,
            ty: &node.ty,
        })
    }
}
