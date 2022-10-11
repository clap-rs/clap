//! Special types handling

use super::spanned::Sp;

use syn::{
    spanned::Spanned, GenericArgument, Path, PathArguments, PathArguments::AngleBracketed,
    PathSegment, Type, TypePath,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Ty {
    Unit,
    Vec,
    Option,
    OptionOption,
    OptionVec,
    Other,
}

impl Ty {
    pub fn from_syn_ty(ty: &syn::Type) -> Sp<Self> {
        use self::Ty::*;
        let t = |kind| Sp::new(kind, ty.span());

        if is_unit_ty(ty) {
            t(Unit)
        } else if is_generic_ty(ty, "Vec") {
            t(Vec)
        } else if let Some(subty) = subty_if_name(ty, "Option") {
            if is_generic_ty(subty, "Option") {
                t(OptionOption)
            } else if is_generic_ty(subty, "Vec") {
                t(OptionVec)
            } else {
                t(Option)
            }
        } else {
            t(Other)
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unit => "()",
            Self::Vec => "Vec<T>",
            Self::Option => "Option<T>",
            Self::OptionOption => "Option<Option<T>>",
            Self::OptionVec => "Option<Vec<T>>",
            Self::Other => "...other...",
        }
    }
}

pub fn inner_type(field_ty: &syn::Type) -> &syn::Type {
    let ty = Ty::from_syn_ty(field_ty);
    match *ty {
        Ty::Vec | Ty::Option => sub_type(field_ty).unwrap_or(field_ty),
        Ty::OptionOption | Ty::OptionVec => {
            sub_type(field_ty).and_then(sub_type).unwrap_or(field_ty)
        }
        _ => field_ty,
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

fn is_generic_ty(ty: &syn::Type, name: &str) -> bool {
    subty_if_name(ty, name).is_some()
}

fn is_unit_ty(ty: &syn::Type) -> bool {
    if let syn::Type::Tuple(tuple) = ty {
        tuple.elems.is_empty()
    } else {
        false
    }
}

fn only_one<I, T>(mut iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
{
    iter.next().filter(|_| iter.next().is_none())
}
