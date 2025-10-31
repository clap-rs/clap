pub(crate) mod error;

mod default_field_value;
mod doc_comments;
mod spanned;
mod ty;

pub(crate) use default_field_value::DefaultField;
pub(crate) use doc_comments::extract_doc_comment;
pub(crate) use doc_comments::format_doc_comment;

pub(crate) use self::{
    spanned::Sp,
    ty::{inner_type, is_simple_ty, sub_type, subty_if_name, Ty},
};
