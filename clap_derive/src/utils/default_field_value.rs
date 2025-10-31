//! Support for named fields with default field values

use syn::{
    parse::{Parse, ParseStream},
    Expr, Token, Type,
};

pub(crate) struct DefaultField {
    pub(crate) ty: Type,
    /// Default value: `field_name: i32 = 1`
    ///
    /// `#![feature(default_field_values)]`
    pub(crate) default: Option<(Token![=], Expr)>,
}

impl Parse for DefaultField {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let field = input.parse()?;
        let default = if input.peek(Token![=]) {
            let eq_token = input.parse()?;
            Some((eq_token, input.parse()?))
        } else {
            None
        };

        Ok(Self { ty: field, default })
    }
}

impl DefaultField {
    pub(crate) fn from_field_type(ty: Type) -> Self {
        match ty {
            Type::Verbatim(stream) => {
                if let Ok(parsed) = syn::parse2(stream.clone()) {
                    parsed
                } else {
                    Self {
                        ty: Type::Verbatim(stream),
                        default: None,
                    }
                }
            }
            other => Self {
                ty: other,
                default: None,
            },
        }
    }
}
