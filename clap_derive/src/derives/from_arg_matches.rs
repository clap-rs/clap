// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, Field, Ident};

use crate::{
    attrs::{Attrs, Kind, ParserKind},
    utils::{sub_type, Sp, Ty},
};

pub fn gen_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    parent_attribute: &Attrs,
) -> TokenStream {
    let constructor = gen_constructor(fields, parent_attribute);
    let updater = gen_updater(fields, parent_attribute, true);

    quote! {
        #[allow(dead_code, unreachable_code, unused_variables)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        impl clap::FromArgMatches for #struct_name {
            fn try_from_arg_matches(arg_matches: &clap::ArgMatches) -> clap::Result<Self> {
                Ok(#struct_name #constructor)
            }

            fn try_update_from_arg_matches(
                &mut self,
                arg_matches: &clap::ArgMatches
            ) -> clap::Result<()> {
                #updater
                Ok(())
            }
        }
    }
}

pub fn gen_for_enum(name: &Ident) -> TokenStream {
    quote! {
        #[allow(dead_code, unreachable_code, unused_variables)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        impl clap::FromArgMatches for #name {
            fn try_from_arg_matches(arg_matches: &clap::ArgMatches) -> clap::Result<Self> {
                <#name as clap::Subcommand>::from_subcommand(arg_matches.subcommand())
            }
            fn try_update_from_arg_matches(
                &mut self,
                arg_matches: &clap::ArgMatches
            ) -> clap::Result<()> {
                <#name as clap::Subcommand>::update_from_subcommand(self, arg_matches.subcommand())
            }
        }
    }
}

fn gen_parsers(
    attrs: &Attrs,
    ty: &Sp<Ty>,
    inner: &TokenStream,
    field_name: &Ident,
    field: &Field,
    update: Option<&TokenStream>,
) -> TokenStream {
    use self::ParserKind::*;

    let parser = attrs.parser();
    let parse_func = &parser.parse_func;
    let span = parser.kind.span();

    // The operand type of the `parse` function.
    let parse_operand_type = match *parser.kind {
        FromStr | TryFromStr => quote_spanned!(ty.span()=> &str),
        Auto | FromOsStr | TryFromOsStr => quote_spanned!(ty.span()=> &::std::ffi::OsStr),
        FromOccurrences => quote_spanned!(ty.span()=> u64),
        FromFlag => quote_spanned!(ty.span()=> bool),
    };

    // Wrap `parse` in a closure so that we can give the operand a concrete type.
    let parse = if let Auto = *parser.kind {
        if parse_func.is_some() {
            abort!(
                parser.kind.span(),
                "`auto` may not be used with a custom parsing function"
            );
        }
        gen_auto_parser(&parse_operand_type, inner, attrs, span)
    } else {
        let func = match parse_func {
            None => match *parser.kind {
                Auto => panic!(), // Handled above.
                FromStr | FromOsStr => {
                    quote_spanned!(parser.kind.span()=> ::std::convert::From::from)
                }
                TryFromStr => quote_spanned!(parser.kind.span()=> ::std::str::FromStr::from_str),
                TryFromOsStr => abort!(
                    parser.kind.span(),
                    "you must set parser for `try_from_os_str` explicitly"
                ),
                FromOccurrences => quote_spanned!(parser.kind.span()=> { |v| v as _ }),
                FromFlag => quote_spanned!(parser.kind.span()=> ::std::convert::From::from),
            },

            Some(func) => match func {
                Expr::Path(_) => quote!(#func),
                _ => abort!(func, "`parse` argument must be a function path"),
            },
        };

        quote_spanned!(span=> |s: #parse_operand_type| #func(s))
    };

    let flag = *attrs.parser().kind == ParserKind::FromFlag;
    let occurrences = *attrs.parser().kind == ParserKind::FromOccurrences;
    let name = attrs.cased_name();
    // Use `quote!` to give this identifier the same hygiene
    // as the `arg_matches` parameter definition. This
    // allows us to refer to `arg_matches` within a `quote_spanned` block
    let arg_matches = quote! { arg_matches };

    let (value_of, values_of) = match *parser.kind {
        Auto => (
            quote_spanned!(span=> #arg_matches.parse_optional_t_auto(#name, #parse)?),
            quote_spanned!(span=> #arg_matches.parse_optional_vec_t_auto(#name, #parse)?),
        ),
        FromStr => (
            quote_spanned!(span=> #arg_matches.value_of(#name).map(#parse)),
            quote_spanned!(span=>
                #arg_matches.values_of(#name).map(|values| values.map(#parse)).collect()
            ),
        ),
        TryFromStr => (
            quote_spanned!(span=> #arg_matches.parse_optional_t(#name, #parse)?),
            quote_spanned!(span=> #arg_matches.parse_optional_vec_t(#name, #parse)?),
        ),
        FromOsStr => (
            quote_spanned!(span=> #arg_matches.value_of_os(#name).map(#parse)),
            quote_spanned!(span=>
                #arg_matches.values_of_os(#name).map(|values| values.map(#parse).collect())
            ),
        ),
        TryFromOsStr => (
            quote_spanned!(span=> #arg_matches.parse_optional_t_os(#name, #parse)?),
            quote_spanned!(span=> #arg_matches.parse_optional_vec_t_os(#name, #parse)?),
        ),
        FromOccurrences => (
            quote_spanned!(span=> (#parse)(#arg_matches.occurrences_of(#name))),
            quote!(),
        ),
        FromFlag => (
            quote_spanned!(span => (#parse)(#arg_matches.is_present(#name))),
            quote!(),
        ),
    };

    let field_value = match **ty {
        Ty::Bool => quote_spanned! { ty.span()=>
            #arg_matches.is_present(#name)
        },

        Ty::Option => quote_spanned! { ty.span()=>
            #value_of
        },

        Ty::OptionOption => quote_spanned! { ty.span()=>
            if #arg_matches.is_present(#name) {
                Some(#value_of)
            } else {
                None
            }
        },

        Ty::OptionVec => quote_spanned! { ty.span()=>
            if #arg_matches.is_present(#name) {
                Some(#values_of.unwrap_or_else(Vec::new))
            } else {
                None
            }
        },

        Ty::Vec => quote_spanned! { ty.span()=>
            #values_of.unwrap_or_else(Vec::new)
        },

        Ty::Other if occurrences => quote_spanned! { ty.span()=>
            #value_of
        },

        Ty::Other if flag => quote_spanned! { ty.span()=>
            #value_of
        },

        Ty::Other => quote_spanned! { ty.span()=>
            #value_of.unwrap()
        },
    };

    if let Some(access) = update {
        quote_spanned! { field.span()=>
            if #arg_matches.is_present(#name) {
                #access
                *#field_name = #field_value
            }
        }
    } else {
        quote_spanned!(field.span()=> #field_name: #field_value )
    }
}

/// Generate code to auto-detect which parsing trait a type supports and
/// perform parsing using it.
///
/// Normally, doing this kind of thing would require specialization, which
/// isn't available on stable Rust, however it turns out to be possible to
/// do a limited form of specialization using autoref. This limited form
/// only works in macro-like contexts, but that's what we're in here!
///
/// For more information on autoref specialization, see this blog post on
/// [generalized autoref-based specialization].
///
/// [generalized autoref-based specialization]: http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
fn gen_auto_parser(
    operand_type: &TokenStream,
    result_type: &TokenStream,
    attrs: &Attrs,
    span: Span,
) -> TokenStream {
    let ci = attrs.case_insensitive();

    quote_spanned!(span=> |s: #operand_type| {
        use std::convert::{Infallible, TryFrom};
        use std::ffi::{OsStr, OsString};
        use std::str::FromStr;
        use std::marker::PhantomData;

        struct Wrap<T>(T);
        trait Specialize7 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<'a, T: clap::ArgEnum> Specialize7 for &&&&&&&Wrap<(&'a OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<String, OsString>>;
            fn specialized(&self) -> Self::Return {
                match self.0.0.to_str() {
                    None => Err(Err(self.0.0.to_os_string())),
                    Some(s) => T::from_str(s, #ci).map_err(Ok),
                }
            }
        }
        trait Specialize6 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<'a, T: TryFrom<&'a OsStr>> Specialize6 for &&&&&&Wrap<(&'a OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<T::Error, OsString>>;
            fn specialized(&self) -> Self::Return {
                T::try_from(self.0.0).map_err(Ok)
            }
        }
        trait Specialize5 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<T: FromStr> Specialize5 for &&&&&Wrap<(&OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<T::Err, OsString>>;
            fn specialized(&self) -> Self::Return {
                match self.0.0.to_str() {
                    None => Err(Err(self.0.0.to_os_string())),
                    Some(s) => T::from_str(s).map_err(Ok),
                }
            }
        }
        trait Specialize4 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<'a, T: TryFrom<&'a str>> Specialize4 for &&&&Wrap<(&'a OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<T::Error, OsString>>;
            fn specialized(&self) -> Self::Return {
                match self.0.0.to_str() {
                    None => Err(Err(self.0.0.to_os_string())),
                    Some(s) => T::try_from(s).map_err(Ok),
                }
            }
        }
        trait Specialize3 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<'a, T: From<&'a OsStr>> Specialize3 for &&&Wrap<(&'a OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<Infallible, OsString>>;
            fn specialized(&self) -> Self::Return {
                Ok(T::from(self.0.0))
            }
        }
        trait Specialize2 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<'a, T: From<&'a str>> Specialize2 for &&Wrap<(&'a OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<Infallible, OsString>>;
            fn specialized(&self) -> Self::Return {
                match self.0.0.to_str() {
                    None => Err(Err(self.0.0.to_os_string())),
                    Some(s) => Ok(T::from(s)),
                }
            }
        }
        trait Specialize1 {
            type Return;
            fn specialized(&self) -> Self::Return;
        }
        impl<'a, T> Specialize1 for &Wrap<(&'a OsStr, PhantomData<T>)> {
            type Return = Result<T, Result<String, OsString>>;
            fn specialized(&self) -> Self::Return {
                Err(Ok(format!(
                    "Type `{}` does not implement any of the parsing traits: \
                    `clap::ArgEnum`, `TryFrom<&OsStr>`, `FromStr`, `TryFrom<&str>`, \
                    `From<&OsStr>`, or `From<&str>`",
                    stringify!(#result_type)
                )))
            }
        }
        (&&&&&&&Wrap((s, PhantomData::<#result_type>))).specialized()
    })
}

pub fn gen_constructor(fields: &Punctuated<Field, Comma>, parent_attribute: &Attrs) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let field_name = field.ident.as_ref().unwrap();
        let kind = attrs.kind();
        let arg_matches = quote! { arg_matches };
        match &*kind {
            Kind::ExternalSubcommand => {
                abort! { kind.span(),
                    "`external_subcommand` can be used only on enum variants"
                }
            }
            Kind::Subcommand(ty) => {
                let subcmd_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };
                let from_subcommand = quote_spanned! { kind.span() =>
                    <#subcmd_type as clap::Subcommand>::from_subcommand(#arg_matches.subcommand())
                };
                match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        #field_name: match #from_subcommand {
                            Ok(cmd) => Some(cmd),
                            Err(clap::Error { kind: clap::ErrorKind::UnrecognizedSubcommand, .. }) => None,
                            Err(e) => return Err(e),
                        }
                    },
                    _ => quote_spanned! { kind.span()=>
                        #field_name: #from_subcommand?
                    },
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=>
                #field_name: clap::FromArgMatches::try_from_arg_matches(#arg_matches)?
            },

            Kind::Skip(val) => match val {
                None => quote_spanned!(kind.span()=> #field_name: Default::default()),
                Some(val) => quote_spanned!(kind.span()=> #field_name: (#val).into()),
            },

            Kind::Arg(ty, inner) => gen_parsers(&attrs, ty, inner, field_name, field, None),
        }
    });

    quote! {{
        #( #fields ),*
    }}
}

pub fn gen_updater(
    fields: &Punctuated<Field, Comma>,
    parent_attribute: &Attrs,
    use_self: bool,
) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let field_name = field.ident.as_ref().unwrap();
        let kind = attrs.kind();

        let access = if use_self {
            quote! {
                #[allow(non_snake_case)]
                let #field_name = &mut self.#field_name;
            }
        } else {
            quote!()
        };
        let arg_matches = quote! { arg_matches };

        match &*kind {
            Kind::ExternalSubcommand => {
                abort! { kind.span(),
                    "`external_subcommand` can be used only on enum variants"
                }
            }
            Kind::Subcommand(ty) => {
                let subcmd_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };

                let updater = quote_spanned!{ ty.span()=>
                    <#subcmd_type as clap::Subcommand>::update_from_subcommand(#field_name, subcmd)?;
                };

                let updater = match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        if let Some(#field_name) = #field_name.as_mut() {
                            #updater
                        } else {
                            *#field_name = Some(<#subcmd_type as clap::Subcommand>::from_subcommand(
                                subcmd
                            )?)
                        }
                    },
                    _ => quote_spanned! { kind.span()=>
                        #updater
                    },
                };

                quote_spanned! { kind.span()=>
                    {
                        let subcmd = #arg_matches.subcommand();
                        #access
                        #updater
                    }
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=> {
                    #access
                    clap::FromArgMatches::update_from_arg_matches(#field_name, #arg_matches);
                }
            },

            Kind::Skip(_) => quote!(),

            Kind::Arg(ty, inner) => gen_parsers(&attrs, ty, &inner, field_name, field, Some(&access)),
        }
    });

    quote! {
        #( #fields )*
    }
}
