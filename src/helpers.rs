use syn::{DeriveInput, Variant};
use syn::Body::Enum;
use errors::*;

pub fn variants(ast: &DeriveInput) -> Result<&Vec<Variant>> {
    match ast.body {
        Enum(ref variants) => Ok(variants),
        _ => Err(ErrorKind::WrongBodyType("enum"))?,
    }
}