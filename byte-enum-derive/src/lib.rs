use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Attribute, DataEnum, DeriveInput, Ident, Meta, Variant};

struct ParsedEnum {
    variants: Vec<Ident>,
}

#[proc_macro_derive(ByteEnum)]
pub fn describe(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    match parse_enum(&input) {
        Err(err) => {
            return err.to_compile_error().into();
        }
        Ok(parsed) => {
            return generate_output(&parsed).into();
        }
    }
}

fn parse_enum(input: &DeriveInput) -> Result<ParsedEnum, syn::Error> {
    check_attributes(&input.attrs)?;
    let variants = parse_variants(&input)?;
    return Ok(ParsedEnum { variants: variants });
}

fn parse_variants(input: &DeriveInput) -> Result<Vec<Ident>, syn::Error> {
    let mut idents: Vec<Ident> = Vec::new();
    match &input.data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            for variant in variants.iter() {
                let _ = check_variant(&variant)?;
                idents.push(variant.ident.clone());
            }
        }
        _ => {
            return Err(error_must_be_enum());
        }
    };
    return Ok(idents);
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
    match attr.parse_meta()? {
        Meta::List(metaList) => {
            let mut iter = metaList.nested.iter();
            if iter.len() == 1 {
                let value = iter.next().unwrap();
                return Ok(());
            }
        }
        _ => {}
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

fn generate_output(parsed: &ParsedEnum) -> TokenStream {
    let output = quote! {
        // impl #input.ident {
        //     fn describe() {
        //         println!("{} is {}.", stringify!(#input.ident), #description);
        //     }
        // }
    };
    return output.into();
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
    "#[derive(ByteEnum)] must be exclusively `#[repr(u8)]` or `#[repr(u16)]`"
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
