//! Special types handling

use super::spanned::Sp;

use syn::{
    spanned::Spanned, GenericArgument, Path, PathArguments, PathArguments::AngleBracketed,
    PathSegment, Type, TypePath,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Ty {
    Bool,
    Vec,
    Option,
    OptionOption,
    OptionVec,
    Other,
}

impl Ty {
    /// Detect whether `ty` is one of our special-cased types above, and if so,
    /// also return the inner type (eg. the `T` in `Vec<T>`).
    pub fn from_syn_ty(ty: &syn::Type) -> (Sp<Self>, &syn::Type) {
        use self::Ty::*;
        let t = |kind| Sp::new(kind, ty.span());

        if is_simple_ty(ty, "bool") {
            (t(Bool), ty)
        } else if let Some(elt) = subty_if_name(ty, "Vec") {
            (t(Vec), elt)
        } else if let Some(subty) = subty_if_name(ty, "Option") {
            if let Some(subsubty) = subty_if_name(subty, "Option") {
                (t(OptionOption), subsubty)
            } else if let Some(elt) = subty_if_name(subty, "Vec") {
                (t(OptionVec), elt)
            } else {
                (t(Option), subty)
            }
        } else {
            (t(Other), ty)
        }
    }
}

pub fn sub_type(ty: &syn::Type) -> Option<&syn::Type> {
    subty_if(ty, |_| true)
}

fn only_last_segment(mut ty: &syn::Type) -> Option<&PathSegment> {
    while let syn::Type::Group(syn::TypeGroup { elem, .. }) = ty {
        ty = elem;
    }
    match ty {
        Type::Path(TypePath {
            qself: None,
            path:
                Path {
                    leading_colon: None,
                    segments,
                },
        }) => only_one(segments.iter()),

        _ => None,
    }
}

fn subty_if<F>(ty: &syn::Type, f: F) -> Option<&syn::Type>
where
    F: FnOnce(&PathSegment) -> bool,
{
    only_last_segment(ty)
        .filter(|segment| f(segment))
        .and_then(|segment| {
            if let AngleBracketed(args) = &segment.arguments {
                only_one(args.args.iter()).and_then(|genneric| {
                    if let GenericArgument::Type(ty) = genneric {
                        Some(ty)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
}

pub fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type> {
    subty_if(ty, |seg| seg.ident == name)
}

pub fn is_simple_ty(ty: &syn::Type, name: &str) -> bool {
    only_last_segment(ty)
        .map(|segment| {
            if let PathArguments::None = segment.arguments {
                segment.ident == name
            } else {
                false
            }
        })
        .unwrap_or(false)
}

fn only_one<I, T>(mut iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
{
    iter.next().filter(|_| iter.next().is_none())
}
