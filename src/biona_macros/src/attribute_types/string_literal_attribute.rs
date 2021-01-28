use syn::parse::{Parse, ParseStream, Result};

struct StringLiteralAttribute {
    string_literal: syn::LitStr,
}

impl Parse for StringLiteralAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parenthesized_token = parenthesized!(content in input);

        Ok(StringLiteralAttribute {
            string_literal: content.parse()?,
        })
    }
}

pub fn from_field (item: &syn::Field, attr_name : &str) -> Option<syn::LitStr>{
    item
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == attr_name)
        .map(|attr| {
            syn::parse2::<StringLiteralAttribute>(attr.tokens.clone())
                .unwrap()
                .string_literal
        })
}