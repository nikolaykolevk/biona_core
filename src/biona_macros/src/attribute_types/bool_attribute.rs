use syn::parse::{Parse, ParseStream, Result};

struct BoolAttribute {
    bool_literal: syn::LitBool,
}

impl Parse for BoolAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parenthesized_token = parenthesized!(content in input);

        Ok(BoolAttribute {
            bool_literal: content.parse()?,
        })
    }
}


pub fn from_field (item: &syn::Field, attr_name : &str) -> Option<syn::LitBool>{
    item
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == attr_name)
        .map(|attr| {
            syn::parse2::<BoolAttribute>(attr.tokens.clone())
                .unwrap()
                .bool_literal
        })
}

