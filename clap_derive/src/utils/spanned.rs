use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::LitStr;

use std::ops::{Deref, DerefMut};

/// An entity with a span attached.
#[derive(Debug, Clone)]
pub struct Sp<T> {
    span: Span,
    val: T,
}

impl<T> Sp<T> {
    pub fn new(val: T, span: Span) -> Self {
        Sp { val, span }
    }

    pub fn call_site(val: T) -> Self {
        Sp {
            val,
            span: Span::call_site(),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl<T> Deref for Sp<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.val
    }
}

impl<T> DerefMut for Sp<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.val
    }
}

impl From<Ident> for Sp<String> {
    fn from(ident: Ident) -> Self {
        Sp {
            val: ident.to_string(),
            span: ident.span(),
        }
    }
}

impl From<LitStr> for Sp<String> {
    fn from(lit: LitStr) -> Self {
        Sp {
            val: lit.value(),
            span: lit.span(),
        }
    }
}

impl<'a> From<Sp<&'a str>> for Sp<String> {
    fn from(sp: Sp<&'a str>) -> Self {
        Sp::new(sp.val.into(), sp.span)
    }
}

impl<U, T: PartialEq<U>> PartialEq<U> for Sp<T> {
    fn eq(&self, other: &U) -> bool {
        self.val == *other
    }
}

impl<T: AsRef<str>> AsRef<str> for Sp<T> {
    fn as_ref(&self) -> &str {
        self.val.as_ref()
    }
}

impl<T: ToTokens> ToTokens for Sp<T> {
    fn to_tokens(&self, stream: &mut TokenStream) {
        // this is the simplest way out of correct ones to change span on
        // arbitrary token tree I could come up with
        let tt = self.val.to_token_stream().into_iter().map(|mut tt| {
            tt.set_span(self.span.clone());
            tt
        });

        stream.extend(tt);
    }
}
