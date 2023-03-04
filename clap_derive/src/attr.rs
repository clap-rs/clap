use std::iter::FromIterator;

use proc_macro2::TokenStream;
use proc_macro_error::abort;
use proc_macro_error::ResultExt;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, Ident, LitStr, Token,
};

use crate::utils::Sp;

#[derive(Clone)]
pub struct ClapAttr {
    pub kind: Sp<AttrKind>,
    pub name: Ident,
    pub magic: Option<MagicAttrName>,
    pub value: Option<AttrValue>,
}

impl ClapAttr {
    pub fn parse_all(all_attrs: &[Attribute]) -> Vec<Self> {
        all_attrs
            .iter()
            .filter_map(|attr| {
                let kind = if attr.path.is_ident("clap") {
                    Some(Sp::new(AttrKind::Clap, attr.path.span()))
                } else if attr.path.is_ident("structopt") {
                    Some(Sp::new(AttrKind::StructOpt, attr.path.span()))
                } else if attr.path.is_ident("command") {
                    Some(Sp::new(AttrKind::Command, attr.path.span()))
                } else if attr.path.is_ident("group") {
                    Some(Sp::new(AttrKind::Group, attr.path.span()))
                } else if attr.path.is_ident("arg") {
                    Some(Sp::new(AttrKind::Arg, attr.path.span()))
                } else if attr.path.is_ident("value") {
                    Some(Sp::new(AttrKind::Value, attr.path.span()))
                } else {
                    None
                };
                kind.map(|k| (k, attr))
            })
            .flat_map(|(k, attr)| {
                attr.parse_args_with(Punctuated::<ClapAttr, Token![,]>::parse_terminated)
                    .unwrap_or_abort()
                    .into_iter()
                    .map(move |mut a| {
                        a.kind = k;
                        a
                    })
            })
            .collect()
    }

    pub fn value_or_abort(&self) -> &AttrValue {
        self.value
            .as_ref()
            .unwrap_or_else(|| abort!(self.name, "attribute `{}` requires a value", self.name))
    }

    pub fn lit_str_or_abort(&self) -> &LitStr {
        let value = self.value_or_abort();
        match value {
            AttrValue::LitStr(tokens) => tokens,
            AttrValue::Expr(_) | AttrValue::Call(_) => {
                abort!(
                    self.name,
                    "attribute `{}` can only accept string literals",
                    self.name
                )
            }
        }
    }
}

impl Parse for ClapAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        let magic = match name_str.as_str() {
            "rename_all" => Some(MagicAttrName::RenameAll),
            "rename_all_env" => Some(MagicAttrName::RenameAllEnv),
            "skip" => Some(MagicAttrName::Skip),
            "next_display_order" => Some(MagicAttrName::NextDisplayOrder),
            "next_help_heading" => Some(MagicAttrName::NextHelpHeading),
            "default_value_t" => Some(MagicAttrName::DefaultValueT),
            "default_values_t" => Some(MagicAttrName::DefaultValuesT),
            "default_value_os_t" => Some(MagicAttrName::DefaultValueOsT),
            "default_values_os_t" => Some(MagicAttrName::DefaultValuesOsT),
            "long" => Some(MagicAttrName::Long),
            "short" => Some(MagicAttrName::Short),
            "value_parser" => Some(MagicAttrName::ValueParser),
            "action" => Some(MagicAttrName::Action),
            "env" => Some(MagicAttrName::Env),
            "flatten" => Some(MagicAttrName::Flatten),
            "value_enum" => Some(MagicAttrName::ValueEnum),
            "from_global" => Some(MagicAttrName::FromGlobal),
            "subcommand" => Some(MagicAttrName::Subcommand),
            "external_subcommand" => Some(MagicAttrName::ExternalSubcommand),
            "verbatim_doc_comment" => Some(MagicAttrName::VerbatimDocComment),
            "about" => Some(MagicAttrName::About),
            "long_about" => Some(MagicAttrName::LongAbout),
            "long_help" => Some(MagicAttrName::LongHelp),
            "author" => Some(MagicAttrName::Author),
            "version" => Some(MagicAttrName::Version),
            _ => None,
        };

        let value = if input.peek(Token![=]) {
            // `name = value` attributes.
            let assign_token = input.parse::<Token![=]>()?; // skip '='
            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;
                Some(AttrValue::LitStr(lit))
            } else {
                match input.parse::<Expr>() {
                    Ok(expr) => Some(AttrValue::Expr(expr)),

                    Err(_) => abort! {
                        assign_token,
                        "expected `string literal` or `expression` after `=`"
                    },
                }
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            let nested;
            parenthesized!(nested in input);

            let method_args: Punctuated<_, Token![,]> = nested.parse_terminated(Expr::parse)?;
            Some(AttrValue::Call(Vec::from_iter(method_args)))
        } else {
            None
        };

        Ok(Self {
            kind: Sp::new(AttrKind::Clap, name.span()),
            name,
            magic,
            value,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MagicAttrName {
    Short,
    Long,
    ValueParser,
    Action,
    Env,
    Flatten,
    ValueEnum,
    FromGlobal,
    Subcommand,
    VerbatimDocComment,
    ExternalSubcommand,
    About,
    LongAbout,
    LongHelp,
    Author,
    Version,
    RenameAllEnv,
    RenameAll,
    Skip,
    DefaultValueT,
    DefaultValuesT,
    DefaultValueOsT,
    DefaultValuesOsT,
    NextDisplayOrder,
    NextHelpHeading,
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum AttrValue {
    LitStr(LitStr),
    Expr(Expr),
    Call(Vec<Expr>),
}

impl ToTokens for AttrValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::LitStr(t) => t.to_tokens(tokens),
            Self::Expr(t) => t.to_tokens(tokens),
            Self::Call(t) => {
                let t = quote!(#(#t),*);
                t.to_tokens(tokens)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AttrKind {
    Clap,
    StructOpt,
    Command,
    Group,
    Arg,
    Value,
}

impl AttrKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Clap => "clap",
            Self::StructOpt => "structopt",
            Self::Command => "command",
            Self::Group => "group",
            Self::Arg => "arg",
            Self::Value => "value",
        }
    }
}
