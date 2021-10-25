mod doc_comments;
mod spanned;
mod ty;

pub use self::{
    doc_comments::process_doc_comment,
    spanned::Sp,
    ty::{inner_type, is_simple_ty, sub_type, subty_if_name, Ty},
};
