use syn::parse::{Parse, ParseStream, Result};

pub struct ExprMethodCallAttribute {
    expr_method_call: syn::ExprMethodCall
}

impl Parse for ExprMethodCallAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _parenthesized_token = parenthesized!(content in input);

        Ok(ExprMethodCallAttribute {
            expr_method_call: content.parse()?,
        })
    }
}

pub fn from_derive_input (item : &syn::DeriveInput, attr_name : &str) -> Option<syn::ExprMethodCall> {
    item
        .attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == attr_name)
        .map(|attr| {
            syn::parse2::<ExprMethodCallAttribute>(attr.tokens.clone())
                .unwrap()
                .expr_method_call
        })
}