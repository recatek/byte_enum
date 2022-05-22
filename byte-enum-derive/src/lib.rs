#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![allow(clippy::bool_comparison)]

use proc_macro::{self, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Ident, Meta, NestedMeta, Variant,
};

#[proc_macro_derive(ByteEnum)]
pub fn describe(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match parse_enum_contents(&input) {
        Err(err) => {
            return err.to_compile_error().into();
        }
        Ok(variants) => {
            return generate_output(input.ident, variants);
        }
    }
}

fn parse_enum_contents(input: &DeriveInput) -> Result<Vec<Ident>, syn::Error> {
    check_attributes(&input.attrs)?;
    return parse_variants(input);
}

fn parse_variants(input: &DeriveInput) -> Result<Vec<Ident>, syn::Error> {
    let mut idents: Vec<Ident> = Vec::new();
    if let Data::Enum(DataEnum { variants, .. }) = &input.data {
        if variants.len() > 255 {
            return Err(error_too_many_variants());
        }
        for variant in variants.iter() {
            let _ = check_variant(variant)?;
            idents.push(variant.ident.clone());
        }
        return Ok(idents);
    };
    return Err(error_must_be_enum());
}

fn check_attributes(attrs: &Vec<Attribute>) -> Result<(), syn::Error> {
    for attr in attrs {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "repr" {
                return check_repr_attribute(attr);
            }
        }
    }
    return Err(error_invalid_representation_hint());
}

fn check_repr_attribute(attr: &Attribute) -> Result<(), syn::Error> {
    if let Meta::List(meta_list) = attr.parse_meta()? {
        let mut iter = meta_list.nested.iter();
        if iter.len() == 1 {
            if let NestedMeta::Meta(meta) = iter.next().unwrap() {
                if let Some(ident) = meta.path().get_ident() {
                    if ident == "u8" {
                        return Ok(());
                    }
                }
            }
        }
    }
    return Err(error_invalid_representation_hint());
}

fn check_variant(variant: &Variant) -> Result<(), syn::Error> {
    if variant.fields.is_empty() == false {
        return Err(error_must_be_fieldless());
    }
    if variant.discriminant.is_some() {
        return Err(error_no_explicit_discriminants());
    }
    return Ok(());
}

fn generate_output(name: Ident, variants: Vec<Ident>) -> TokenStream {
    let into_u8 = generate_into_u8(&name, &variants);
    let try_from_u8 = generate_try_from_u8(&name, &variants);

    return quote! {
        impl ByteEnum for #name {}
        #into_u8
        #try_from_u8
    }
    .into();
}

fn generate_into_u8(name: &Ident, variants: &Vec<Ident>) -> TokenStream2 {
    let index = 0_u8..variants.len() as u8;
    return quote! {
        impl From<#name> for u8 {
            #[inline(always)]
            fn from(value: #name) -> Self {
                return match value {
                    #( #name::#variants => { #index as u8 }, )*
                };
            }
        }
    };
}

fn generate_try_from_u8(name: &Ident, variants: &Vec<Ident>) -> TokenStream2 {
    let index = 0_u8..variants.len() as u8;
    return quote! {
        impl TryFrom<u8> for #name {
            type Error = TryEnumFromByteError;

            #[inline(always)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                return match value {
                    #( #index => { Ok(#name::#variants) }, )*
                    _ => Err(TryEnumFromByteError(())),
                };
            }
        }
    };
}

macro_rules! declare_error {
    ($name:ident, $text:expr) => {
        fn $name() -> syn::Error {
            return syn::Error::new(Span::call_site(), $text);
        }
    };
}

declare_error!(
    error_invalid_representation_hint,
    "#[derive(ByteEnum)] must be exclusively `#[repr(u8)]`"
);

declare_error!(
    error_must_be_fieldless,
    "#[derive(ByteEnum)] must be applied only to fieldless enums"
);

declare_error!(
    error_no_explicit_discriminants,
    "#[derive(ByteEnum)] enums may not have explicit discriminants"
);

declare_error!(
    error_must_be_enum,
    "#[derive(ByteEnum)] must be applied only to enum types"
);

declare_error!(
    error_too_many_variants,
    "#[derive(ByteEnum)] may not have more than 255 variants"
);
