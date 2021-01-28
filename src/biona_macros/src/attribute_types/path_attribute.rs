use syn::parse::{Parse, ParseStream, Result};

struct PathAttribute {
    path: syn::Path,
}

impl Parse for PathAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parenthesized_token = parenthesized!(content in input);

        Ok(PathAttribute {
            path: content.parse()?,
        })
    }
}

pub fn from_derive_input (item: &syn::DeriveInput, attr_name : &str) -> Option<syn::Path>{
    item
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == attr_name)
        .map(|attr| {
            syn::parse2::<PathAttribute>(attr.tokens.clone())
                .unwrap()
                .path
        })
}

pub fn from_field (item: &syn::Field, attr_name : &str) -> Option<syn::Path>{
    item
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == attr_name)
        .map(|attr| {
            syn::parse2::<PathAttribute>(attr.tokens.clone())
                .unwrap()
                .path
        })
}