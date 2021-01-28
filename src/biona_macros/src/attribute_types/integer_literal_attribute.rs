use syn::parse::{Parse, ParseStream, Result};

struct IntegerLiteralAttribute {
    integer_literal: syn::LitInt,
}

impl Parse for IntegerLiteralAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parenthesized_token = parenthesized!(content in input);

        Ok(IntegerLiteralAttribute {
            integer_literal: content.parse()?,
        })
    }
}

pub fn from_field (item: &syn::Field, attr_name : &str) -> Option<syn::LitInt>{
    item
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == attr_name)
        .map(|attr| {
            syn::parse2::<IntegerLiteralAttribute>(attr.tokens.clone())
                .unwrap()
                .integer_literal
        })
}