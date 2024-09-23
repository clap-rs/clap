pub(crate) mod error;

mod doc_comments;
mod spanned;
mod ty;

pub(crate) use self::doc_comments::{extract_doc_comment, format_doc_comment};
pub(crate) use self::spanned::Sp;
pub(crate) use self::ty::{inner_type, is_simple_ty, sub_type, subty_if_name, Ty};
