// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
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

use crate::{
    parse::*,
    utils::{inner_type, is_simple_ty, process_doc_comment, Sp, Ty},
};

use std::env;

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{self, Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    self, ext::IdentExt, spanned::Spanned, Attribute, Expr, Field, Ident, LitStr, MetaNameValue,
    Type, Variant,
};

/// Default casing style for generated arguments.
pub const DEFAULT_CASING: CasingStyle = CasingStyle::Kebab;

/// Default casing style for environment variables
pub const DEFAULT_ENV_CASING: CasingStyle = CasingStyle::ScreamingSnake;

#[derive(Clone)]
pub struct Attrs {
    name: Name,
    casing: Sp<CasingStyle>,
    env_casing: Sp<CasingStyle>,
    ty: Option<Type>,
    doc_comment: Vec<Method>,
    methods: Vec<Method>,
    value_parser: Option<ValueParser>,
    action: Option<Action>,
    parser: Option<Sp<Parser>>,
    verbatim_doc_comment: Option<Ident>,
    next_display_order: Option<Method>,
    next_help_heading: Option<Method>,
    help_heading: Option<Method>,
    is_enum: bool,
    is_positional: bool,
    kind: Sp<Kind>,
}

impl Attrs {
    pub fn from_struct(
        span: Span,
        attrs: &[Attribute],
        name: Name,
        argument_casing: Sp<CasingStyle>,
        env_casing: Sp<CasingStyle>,
    ) -> Self {
        let mut res = Self::new(span, name, None, argument_casing, env_casing);
        res.push_attrs(attrs);
        res.push_doc_comment(attrs, "about");

        if let Some(value_parser) = res.value_parser.as_ref() {
            abort!(
                value_parser.span(),
                "`value_parser` attribute is only allowed on fields"
            );
        }
        if let Some(action) = res.action.as_ref() {
            abort!(
                action.span(),
                "`action` attribute is only allowed on fields"
            );
        }
        if let Some(parser) = res.parser.as_ref() {
            abort!(parser.span(), "`parse` attribute is only allowed on fields");
        }
        match &*res.kind {
            Kind::Subcommand(_) => abort!(res.kind.span(), "subcommand is only allowed on fields"),
            Kind::Skip(_) => abort!(res.kind.span(), "skip is only allowed on fields"),
            Kind::Arg(_) => res,
            Kind::FromGlobal(_) => abort!(res.kind.span(), "from_global is only allowed on fields"),
            Kind::Flatten => abort!(res.kind.span(), "flatten is only allowed on fields"),
            Kind::ExternalSubcommand => abort!(
                res.kind.span(),
                "external_subcommand is only allowed on fields"
            ),
        }
    }

    pub fn from_variant(
        variant: &Variant,
        struct_casing: Sp<CasingStyle>,
        env_casing: Sp<CasingStyle>,
    ) -> Self {
        let name = variant.ident.clone();
        let mut res = Self::new(
            variant.span(),
            Name::Derived(name),
            None,
            struct_casing,
            env_casing,
        );
        res.push_attrs(&variant.attrs);
        res.push_doc_comment(&variant.attrs, "about");

        match &*res.kind {
            Kind::Flatten => {
                if let Some(value_parser) = res.value_parser.as_ref() {
                    abort!(
                        value_parser.span(),
                        "`value_parser` attribute is not allowed for flattened entry"
                    );
                }
                if let Some(action) = res.action.as_ref() {
                    abort!(
                        action.span(),
                        "`action` attribute is not allowed for flattened entry"
                    );
                }
                if let Some(parser) = res.parser.as_ref() {
                    abort!(
                        parser.span(),
                        "parse attribute is not allowed for flattened entry"
                    );
                }
                if res.has_explicit_methods() {
                    abort!(
                        res.kind.span(),
                        "methods are not allowed for flattened entry"
                    );
                }

                // ignore doc comments
                res.doc_comment = vec![];
            }

            Kind::ExternalSubcommand => (),

            Kind::Subcommand(_) => {
                if let Some(value_parser) = res.value_parser.as_ref() {
                    abort!(
                        value_parser.span(),
                        "`value_parser` attribute is not allowed for subcommand"
                    );
                }
                if let Some(action) = res.action.as_ref() {
                    abort!(
                        action.span(),
                        "`action` attribute is not allowed for subcommand"
                    );
                }
                if let Some(parser) = res.parser.as_ref() {
                    abort!(
                        parser.span(),
                        "parse attribute is not allowed for subcommand"
                    );
                }

                use syn::Fields::*;
                use syn::FieldsUnnamed;
                let field_ty = match variant.fields {
                    Named(_) => {
                        abort!(variant.span(), "structs are not allowed for subcommand");
                    }
                    Unit => abort!(variant.span(), "unit-type is not allowed for subcommand"),
                    Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                        &unnamed[0].ty
                    }
                    Unnamed(..) => {
                        abort!(
                            variant,
                            "non single-typed tuple is not allowed for subcommand"
                        )
                    }
                };
                let ty = Ty::from_syn_ty(field_ty);
                match *ty {
                    Ty::OptionOption => {
                        abort!(
                            field_ty,
                            "Option<Option<T>> type is not allowed for subcommand"
                        );
                    }
                    Ty::OptionVec => {
                        abort!(
                            field_ty,
                            "Option<Vec<T>> type is not allowed for subcommand"
                        );
                    }
                    _ => (),
                }

                res.kind = Sp::new(Kind::Subcommand(ty), res.kind.span());
            }
            Kind::Skip(_) => (),
            Kind::FromGlobal(_) => {
                abort!(res.kind.span(), "from_global is not supported on variants");
            }
            Kind::Arg(_) => (),
        }

        res
    }

    pub fn from_value_enum_variant(
        variant: &Variant,
        argument_casing: Sp<CasingStyle>,
        env_casing: Sp<CasingStyle>,
    ) -> Self {
        let mut res = Self::new(
            variant.span(),
            Name::Derived(variant.ident.clone()),
            None,
            argument_casing,
            env_casing,
        );
        res.push_attrs(&variant.attrs);
        res.push_doc_comment(&variant.attrs, "help");

        if let Some(value_parser) = res.value_parser.as_ref() {
            abort!(
                value_parser.span(),
                "`value_parser` attribute is only allowed on fields"
            );
        }
        if let Some(action) = res.action.as_ref() {
            abort!(
                action.span(),
                "`action` attribute is only allowed on fields"
            );
        }
        if let Some(parser) = res.parser.as_ref() {
            abort!(parser.span(), "`parse` attribute is only allowed on fields");
        }
        match &*res.kind {
            Kind::Subcommand(_) => abort!(res.kind.span(), "subcommand is only allowed on fields"),
            Kind::Skip(_) => res,
            Kind::Arg(_) => res,
            Kind::FromGlobal(_) => abort!(res.kind.span(), "from_global is only allowed on fields"),
            Kind::Flatten => abort!(res.kind.span(), "flatten is only allowed on fields"),
            Kind::ExternalSubcommand => abort!(
                res.kind.span(),
                "external_subcommand is only allowed on fields"
            ),
        }
    }

    pub fn from_field(
        field: &Field,
        struct_casing: Sp<CasingStyle>,
        env_casing: Sp<CasingStyle>,
    ) -> Self {
        let name = field.ident.clone().unwrap();
        let mut res = Self::new(
            field.span(),
            Name::Derived(name),
            Some(field.ty.clone()),
            struct_casing,
            env_casing,
        );
        res.push_attrs(&field.attrs);
        res.push_doc_comment(&field.attrs, "help");

        match &*res.kind {
            Kind::Flatten => {
                if let Some(value_parser) = res.value_parser.as_ref() {
                    abort!(
                        value_parser.span(),
                        "`value_parser` attribute is not allowed for flattened entry"
                    );
                }
                if let Some(action) = res.action.as_ref() {
                    abort!(
                        action.span(),
                        "`action` attribute is not allowed for flattened entry"
                    );
                }
                if let Some(parser) = res.parser.as_ref() {
                    abort!(
                        parser.span(),
                        "parse attribute is not allowed for flattened entry"
                    );
                }
                if res.has_explicit_methods() {
                    abort!(
                        res.kind.span(),
                        "methods are not allowed for flattened entry"
                    );
                }

                // ignore doc comments
                res.doc_comment = vec![];
            }

            Kind::ExternalSubcommand => {
                abort! { res.kind.span(),
                    "`external_subcommand` can be used only on enum variants"
                }
            }

            Kind::Subcommand(_) => {
                if let Some(value_parser) = res.value_parser.as_ref() {
                    abort!(
                        value_parser.span(),
                        "`value_parser` attribute is not allowed for subcommand"
                    );
                }
                if let Some(action) = res.action.as_ref() {
                    abort!(
                        action.span(),
                        "`action` attribute is not allowed for subcommand"
                    );
                }
                if let Some(parser) = res.parser.as_ref() {
                    abort!(
                        parser.span(),
                        "parse attribute is not allowed for subcommand"
                    );
                }
                if res.has_explicit_methods() {
                    abort!(
                        res.kind.span(),
                        "methods in attributes are not allowed for subcommand"
                    );
                }

                let ty = Ty::from_syn_ty(&field.ty);
                match *ty {
                    Ty::OptionOption => {
                        abort!(
                            field.ty,
                            "Option<Option<T>> type is not allowed for subcommand"
                        );
                    }
                    Ty::OptionVec => {
                        abort!(
                            field.ty,
                            "Option<Vec<T>> type is not allowed for subcommand"
                        );
                    }
                    _ => (),
                }

                res.kind = Sp::new(Kind::Subcommand(ty), res.kind.span());
            }
            Kind::Skip(_) => {
                if res.has_explicit_methods() {
                    abort!(
                        res.kind.span(),
                        "methods are not allowed for skipped fields"
                    );
                }
            }
            Kind::FromGlobal(orig_ty) => {
                let ty = Ty::from_syn_ty(&field.ty);
                res.kind = Sp::new(Kind::FromGlobal(ty), orig_ty.span());
            }
            Kind::Arg(_) => {
                let mut ty = Ty::from_syn_ty(&field.ty);
                if res.parser.is_some() {
                    if let Some(value_parser) = res.value_parser.as_ref() {
                        abort!(
                            value_parser.span(),
                            "`value_parser` attribute conflicts with `parse` attribute"
                        );
                    }
                    if let Some(action) = res.action.as_ref() {
                        abort!(
                            action.span(),
                            "`action` attribute conflicts with `parse` attribute"
                        );
                    }
                    match *ty {
                        Ty::Option | Ty::Vec | Ty::OptionVec => (),
                        _ => ty = Sp::new(Ty::Other, ty.span()),
                    }
                }

                match *ty {
                    Ty::Option => {
                        if let Some(m) = res.find_default_method() {
                            abort!(m.name, "default_value is meaningless for Option")
                        }
                    }
                    Ty::OptionOption => {
                        if res.is_positional() {
                            abort!(
                                field.ty,
                                "Option<Option<T>> type is meaningless for positional argument"
                            )
                        }
                    }
                    Ty::OptionVec => {
                        if res.is_positional() {
                            abort!(
                                field.ty,
                                "Option<Vec<T>> type is meaningless for positional argument"
                            )
                        }
                    }

                    _ => (),
                }
                res.kind = Sp::new(
                    Kind::Arg(ty),
                    field
                        .ident
                        .as_ref()
                        .map(|i| i.span())
                        .unwrap_or_else(|| field.ty.span()),
                );
            }
        }

        res
    }

    fn new(
        default_span: Span,
        name: Name,
        ty: Option<Type>,
        casing: Sp<CasingStyle>,
        env_casing: Sp<CasingStyle>,
    ) -> Self {
        Self {
            name,
            ty,
            casing,
            env_casing,
            doc_comment: vec![],
            methods: vec![],
            value_parser: None,
            action: None,
            parser: None,
            verbatim_doc_comment: None,
            next_display_order: None,
            next_help_heading: None,
            help_heading: None,
            is_enum: false,
            is_positional: true,
            kind: Sp::new(Kind::Arg(Sp::new(Ty::Other, default_span)), default_span),
        }
    }

    fn push_method(&mut self, name: Ident, arg: impl ToTokens) {
        if name == "name" || name == "id" {
            self.name = Name::Assigned(quote!(#arg));
        } else if name == "value_parser" {
            self.value_parser = Some(ValueParser::Explicit(Method::new(name, quote!(#arg))));
        } else if name == "action" {
            self.action = Some(Action::Explicit(Method::new(name, quote!(#arg))));
        } else {
            if name == "short" || name == "long" {
                self.is_positional = false;
            }
            self.methods.push(Method::new(name, quote!(#arg)));
        }
    }

    fn push_attrs(&mut self, attrs: &[Attribute]) {
        use ClapAttr::*;

        let parsed = parse_clap_attributes(attrs);
        for attr in &parsed {
            let attr = attr.clone();
            match attr {
                Short(ident) => {
                    self.push_method(ident, self.name.clone().translate_char(*self.casing));
                }

                Long(ident) => {
                    self.push_method(ident, self.name.clone().translate(*self.casing));
                }

                ValueParser(ident) => {
                    use crate::attrs::ValueParser;
                    self.value_parser = Some(ValueParser::Implicit(ident));
                }

                Action(ident) => {
                    use crate::attrs::Action;
                    self.action = Some(Action::Implicit(ident));
                }

                Env(ident) => {
                    self.push_method(ident, self.name.clone().translate(*self.env_casing));
                }

                ValueEnum(_) => self.is_enum = true,

                FromGlobal(ident) => {
                    let ty = Sp::call_site(Ty::Other);
                    let kind = Sp::new(Kind::FromGlobal(ty), ident.span());
                    self.set_kind(kind);
                }

                Subcommand(ident) => {
                    let ty = Sp::call_site(Ty::Other);
                    let kind = Sp::new(Kind::Subcommand(ty), ident.span());
                    self.set_kind(kind);
                }

                ExternalSubcommand(ident) => {
                    let kind = Sp::new(Kind::ExternalSubcommand, ident.span());
                    self.set_kind(kind);
                }

                Flatten(ident) => {
                    let kind = Sp::new(Kind::Flatten, ident.span());
                    self.set_kind(kind);
                }

                Skip(ident, expr) => {
                    let kind = Sp::new(Kind::Skip(expr), ident.span());
                    self.set_kind(kind);
                }

                VerbatimDocComment(ident) => self.verbatim_doc_comment = Some(ident),

                DefaultValueT(ident, expr) => {
                    let ty = if let Some(ty) = self.ty.as_ref() {
                        ty
                    } else {
                        abort!(
                            ident,
                            "#[clap(default_value_t)] (without an argument) can be used \
                            only on field level";

                            note = "see \
                                https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md#magic-attributes")
                    };

                    let val = if let Some(expr) = expr {
                        quote!(#expr)
                    } else {
                        quote!(<#ty as ::std::default::Default>::default())
                    };

                    let val = if parsed.iter().any(|a| matches!(a, ValueEnum(_))) {
                        quote_spanned!(ident.span()=> {
                            {
                                let val: #ty = #val;
                                clap::ValueEnum::to_possible_value(&val).unwrap().get_name()
                            }
                        })
                    } else {
                        quote_spanned!(ident.span()=> {
                            static DEFAULT_VALUE: clap::__macro_refs::once_cell::sync::Lazy<String> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                let val: #ty = #val;
                                ::std::string::ToString::to_string(&val)
                            });
                            &*DEFAULT_VALUE
                        })
                    };

                    let raw_ident = Ident::new("default_value", ident.span());
                    self.methods.push(Method::new(raw_ident, val));
                }

                DefaultValuesT(ident, expr) => {
                    let ty = if let Some(ty) = self.ty.as_ref() {
                        ty
                    } else {
                        abort!(
                            ident,
                            "#[clap(default_values_t)] (without an argument) can be used \
                            only on field level";

                            note = "see \
                                https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md#magic-attributes")
                    };

                    let container_type = Ty::from_syn_ty(ty);
                    if *container_type != Ty::Vec {
                        abort!(
                            ident,
                            "#[clap(default_values_t)] can be used only on Vec types";

                            note = "see \
                                https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md#magic-attributes")
                    }
                    let inner_type = inner_type(ty);

                    // Use `Borrow<#inner_type>` so we accept `&Vec<#inner_type>` and
                    // `Vec<#inner_type>`.
                    let val = if parsed.iter().any(|a| matches!(a, ValueEnum(_))) {
                        quote_spanned!(ident.span()=> {
                            {
                                fn iter_to_vals<T>(iterable: impl IntoIterator<Item = T>) -> Vec<&'static str>
                                where
                                    T: ::std::borrow::Borrow<#inner_type>
                                {
                                    iterable
                                        .into_iter()
                                        .map(|val| {
                                            clap::ValueEnum::to_possible_value(val.borrow()).unwrap().get_name()
                                        })
                                        .collect()

                                }

                                static DEFAULT_VALUES: clap::__macro_refs::once_cell::sync::Lazy<Vec<&str>> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                    iter_to_vals(#expr)
                                });
                                &*DEFAULT_VALUES.as_slice()
                            }
                        })
                    } else {
                        quote_spanned!(ident.span()=> {
                            {
                                fn iter_to_vals<T>(iterable: impl IntoIterator<Item = T>) -> Vec<String>
                                where
                                    T: ::std::borrow::Borrow<#inner_type>
                                {
                                    iterable.into_iter().map(|val| val.borrow().to_string()).collect()

                                }

                                static DEFAULT_STRINGS: clap::__macro_refs::once_cell::sync::Lazy<Vec<::std::string::String>> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                    iter_to_vals(#expr)
                                });

                                static DEFAULT_VALUES: clap::__macro_refs::once_cell::sync::Lazy<Vec<&str>> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                    DEFAULT_STRINGS.iter().map(::std::string::String::as_str).collect()
                                });
                                &*DEFAULT_VALUES.as_slice()
                            }
                        })
                    };

                    self.methods
                        .push(Method::new(Ident::new("default_values", ident.span()), val));
                }

                DefaultValueOsT(ident, expr) => {
                    let ty = if let Some(ty) = self.ty.as_ref() {
                        ty
                    } else {
                        abort!(
                            ident,
                            "#[clap(default_value_os_t)] (without an argument) can be used \
                            only on field level";

                            note = "see \
                                https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md#magic-attributes")
                    };

                    let val = if let Some(expr) = expr {
                        quote!(#expr)
                    } else {
                        quote!(<#ty as ::std::default::Default>::default())
                    };

                    let val = if parsed.iter().any(|a| matches!(a, ValueEnum(_))) {
                        quote_spanned!(ident.span()=> {
                            {
                                let val: #ty = #val;
                                clap::ValueEnum::to_possible_value(&val).unwrap().get_name()
                            }
                        })
                    } else {
                        quote_spanned!(ident.span()=> {
                            static DEFAULT_VALUE: clap::__macro_refs::once_cell::sync::Lazy<::std::ffi::OsString> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                let val: #ty = #val;
                                ::std::ffi::OsString::from(val)
                            });
                            &*DEFAULT_VALUE
                        })
                    };

                    let raw_ident = Ident::new("default_value_os", ident.span());
                    self.methods.push(Method::new(raw_ident, val));
                }

                DefaultValuesOsT(ident, expr) => {
                    let ty = if let Some(ty) = self.ty.as_ref() {
                        ty
                    } else {
                        abort!(
                            ident,
                            "#[clap(default_values_os_t)] (without an argument) can be used \
                            only on field level";

                            note = "see \
                                https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md#magic-attributes")
                    };

                    let container_type = Ty::from_syn_ty(ty);
                    if *container_type != Ty::Vec {
                        abort!(
                            ident,
                            "#[clap(default_values_os_t)] can be used only on Vec types";

                            note = "see \
                                https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md#magic-attributes")
                    }
                    let inner_type = inner_type(ty);

                    // Use `Borrow<#inner_type>` so we accept `&Vec<#inner_type>` and
                    // `Vec<#inner_type>`.
                    let val = if parsed.iter().any(|a| matches!(a, ValueEnum(_))) {
                        quote_spanned!(ident.span()=> {
                            {
                                fn iter_to_vals<T>(iterable: impl IntoIterator<Item = T>) -> Vec<&'static ::std::ffi::OsStr>
                                where
                                    T: ::std::borrow::Borrow<#inner_type>
                                {
                                    iterable
                                        .into_iter()
                                        .map(|val| {
                                            clap::ValueEnum::to_possible_value(val.borrow()).unwrap().get_name()
                                        })
                                        .map(::std::ffi::OsStr::new)
                                        .collect()

                                }

                                static DEFAULT_VALUES: clap::__macro_refs::once_cell::sync::Lazy<Vec<&::std::ffi::OsStr>> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                    iter_to_vals(#expr)
                                });
                                &*DEFAULT_VALUES.as_slice()
                            }
                        })
                    } else {
                        quote_spanned!(ident.span()=> {
                            {
                                fn iter_to_vals<T>(iterable: impl IntoIterator<Item = T>) -> Vec<::std::ffi::OsString>
                                where
                                    T: ::std::borrow::Borrow<#inner_type>
                                {
                                    iterable.into_iter().map(|val| val.borrow().into()).collect()

                                }

                                static DEFAULT_OS_STRINGS: clap::__macro_refs::once_cell::sync::Lazy<Vec<::std::ffi::OsString>> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                    iter_to_vals(#expr)
                                });

                                static DEFAULT_VALUES: clap::__macro_refs::once_cell::sync::Lazy<Vec<&::std::ffi::OsStr>> = clap::__macro_refs::once_cell::sync::Lazy::new(|| {
                                    DEFAULT_OS_STRINGS.iter().map(::std::ffi::OsString::as_os_str).collect()
                                });
                                &*DEFAULT_VALUES.as_slice()
                            }
                        })
                    };

                    self.methods.push(Method::new(
                        Ident::new("default_values_os", ident.span()),
                        val,
                    ));
                }

                NextDisplayOrder(ident, expr) => {
                    self.next_display_order = Some(Method::new(ident, quote!(#expr)));
                }

                HelpHeading(ident, expr) => {
                    self.help_heading = Some(Method::new(ident, quote!(#expr)));
                }
                NextHelpHeading(ident, expr) => {
                    self.next_help_heading = Some(Method::new(ident, quote!(#expr)));
                }

                About(ident) => {
                    if let Some(method) = Method::from_env(ident, "CARGO_PKG_DESCRIPTION") {
                        self.methods.push(method);
                    }
                }

                Author(ident) => {
                    if let Some(method) = Method::from_env(ident, "CARGO_PKG_AUTHORS") {
                        self.methods.push(method);
                    }
                }

                Version(ident) => {
                    if let Some(method) = Method::from_env(ident, "CARGO_PKG_VERSION") {
                        self.methods.push(method);
                    }
                }

                NameLitStr(name, lit) => {
                    self.push_method(name, lit);
                }

                NameExpr(name, expr) => {
                    self.push_method(name, expr);
                }

                MethodCall(name, args) => self.push_method(name, quote!(#(#args),*)),

                RenameAll(_, casing_lit) => {
                    self.casing = CasingStyle::from_lit(casing_lit);
                }

                RenameAllEnv(_, casing_lit) => {
                    self.env_casing = CasingStyle::from_lit(casing_lit);
                }

                Parse(ident, spec) => {
                    self.parser = Some(Parser::from_spec(ident, spec));
                }
            }
        }
    }

    fn push_doc_comment(&mut self, attrs: &[Attribute], name: &str) {
        use syn::Lit::*;
        use syn::Meta::*;

        let comment_parts: Vec<_> = attrs
            .iter()
            .filter(|attr| attr.path.is_ident("doc"))
            .filter_map(|attr| {
                if let Ok(NameValue(MetaNameValue { lit: Str(s), .. })) = attr.parse_meta() {
                    Some(s.value())
                } else {
                    // non #[doc = "..."] attributes are not our concern
                    // we leave them for rustc to handle
                    None
                }
            })
            .collect();

        self.doc_comment =
            process_doc_comment(comment_parts, name, self.verbatim_doc_comment.is_none());
    }

    fn set_kind(&mut self, kind: Sp<Kind>) {
        if let Kind::Arg(_) = *self.kind {
            self.kind = kind;
        } else {
            abort!(
                kind.span(),
                "`subcommand`, `flatten`, `external_subcommand` and `skip` cannot be used together"
            );
        }
    }

    pub fn find_method(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name == name)
    }

    pub fn find_default_method(&self) -> Option<&Method> {
        self.methods
            .iter()
            .find(|m| m.name == "default_value" || m.name == "default_value_os")
    }

    /// generate methods from attributes on top of struct or enum
    pub fn initial_top_level_methods(&self) -> TokenStream {
        let next_display_order = self.next_display_order.as_ref().into_iter();
        let next_help_heading = self.next_help_heading.as_ref().into_iter();
        let help_heading = self.help_heading.as_ref().into_iter();
        quote!(
            #(#next_display_order)*
            #(#next_help_heading)*
            #(#help_heading)*
        )
    }

    pub fn final_top_level_methods(&self) -> TokenStream {
        let methods = &self.methods;
        let doc_comment = &self.doc_comment;

        quote!( #(#doc_comment)* #(#methods)*)
    }

    /// generate methods on top of a field
    pub fn field_methods(&self, supports_long_help: bool) -> proc_macro2::TokenStream {
        let methods = &self.methods;
        let help_heading = self.help_heading.as_ref().into_iter();
        match supports_long_help {
            true => {
                let doc_comment = &self.doc_comment;
                quote!( #(#doc_comment)* #(#help_heading)* #(#methods)* )
            }
            false => {
                let doc_comment = self
                    .doc_comment
                    .iter()
                    .filter(|mth| mth.name != "long_help");
                quote!( #(#doc_comment)* #(#help_heading)* #(#methods)* )
            }
        }
    }

    pub fn next_display_order(&self) -> TokenStream {
        let next_display_order = self.next_display_order.as_ref().into_iter();
        quote!( #(#next_display_order)* )
    }

    pub fn next_help_heading(&self) -> TokenStream {
        let next_help_heading = self.next_help_heading.as_ref().into_iter();
        let help_heading = self.help_heading.as_ref().into_iter();
        quote!( #(#next_help_heading)*  #(#help_heading)* )
    }

    #[cfg(feature = "unstable-v4")]
    pub fn id(&self) -> TokenStream {
        self.name.clone().raw()
    }

    #[cfg(not(feature = "unstable-v4"))]
    pub fn id(&self) -> TokenStream {
        self.cased_name()
    }

    pub fn cased_name(&self) -> TokenStream {
        self.name.clone().translate(*self.casing)
    }

    pub fn value_name(&self) -> TokenStream {
        self.name.clone().translate(CasingStyle::ScreamingSnake)
    }

    pub fn value_parser(&self, field_type: &Type) -> Method {
        self.value_parser
            .clone()
            .map(|p| {
                let inner_type = inner_type(field_type);
                p.resolve(inner_type)
            })
            .unwrap_or_else(|| {
                if let Some(action) = self.action.as_ref() {
                    let inner_type = inner_type(field_type);
                    let span = action.span();
                    default_value_parser(inner_type, span)
                } else if !self.ignore_parser() || cfg!(not(feature = "unstable-v4")) {
                    self.parser(field_type).value_parser()
                } else {
                    let inner_type = inner_type(field_type);
                    let span = self
                        .action
                        .as_ref()
                        .map(|a| a.span())
                        .unwrap_or_else(|| self.kind.span());
                    default_value_parser(inner_type, span)
                }
            })
    }

    pub fn action(&self, field_type: &Type) -> Method {
        self.action
            .clone()
            .map(|p| p.resolve(field_type))
            .unwrap_or_else(|| {
                if let Some(value_parser) = self.value_parser.as_ref() {
                    let span = value_parser.span();
                    default_action(field_type, span)
                } else if !self.ignore_parser() || cfg!(not(feature = "unstable-v4")) {
                    self.parser(field_type).action()
                } else {
                    let span = self
                        .value_parser
                        .as_ref()
                        .map(|a| a.span())
                        .unwrap_or_else(|| self.kind.span());
                    default_action(field_type, span)
                }
            })
    }

    #[cfg(feature = "unstable-v4")]
    pub fn ignore_parser(&self) -> bool {
        self.parser.is_none()
    }

    #[cfg(not(feature = "unstable-v4"))]
    pub fn ignore_parser(&self) -> bool {
        self.value_parser.is_some() || self.action.is_some()
    }

    pub fn explicit_parser(&self) -> bool {
        self.parser.is_some()
    }

    pub fn parser(&self, field_type: &Type) -> Sp<Parser> {
        self.parser
            .clone()
            .unwrap_or_else(|| Parser::from_type(field_type, self.kind.span()))
    }

    pub fn kind(&self) -> Sp<Kind> {
        self.kind.clone()
    }

    pub fn is_enum(&self) -> bool {
        self.is_enum
    }

    pub fn is_positional(&self) -> bool {
        self.is_positional
    }

    pub fn ignore_case(&self) -> TokenStream {
        let method = self.find_method("ignore_case");

        if let Some(method) = method {
            method.args.clone()
        } else {
            quote! { false }
        }
    }

    pub fn casing(&self) -> Sp<CasingStyle> {
        self.casing.clone()
    }

    pub fn env_casing(&self) -> Sp<CasingStyle> {
        self.env_casing.clone()
    }

    pub fn has_explicit_methods(&self) -> bool {
        self.methods
            .iter()
            .any(|m| m.name != "help" && m.name != "long_help")
    }
}

#[derive(Clone)]
enum ValueParser {
    Explicit(Method),
    Implicit(Ident),
}

impl ValueParser {
    fn resolve(self, inner_type: &Type) -> Method {
        match self {
            Self::Explicit(method) => method,
            Self::Implicit(ident) => default_value_parser(inner_type, ident.span()),
        }
    }

    fn span(&self) -> Span {
        match self {
            Self::Explicit(method) => method.name.span(),
            Self::Implicit(ident) => ident.span(),
        }
    }
}

fn default_value_parser(inner_type: &Type, span: Span) -> Method {
    let func = Ident::new("value_parser", span);
    Method::new(
        func,
        quote_spanned! { span=>
            clap::value_parser!(#inner_type)
        },
    )
}

#[derive(Clone)]
pub enum Action {
    Explicit(Method),
    Implicit(Ident),
}

impl Action {
    pub fn resolve(self, field_type: &Type) -> Method {
        match self {
            Self::Explicit(method) => method,
            Self::Implicit(ident) => default_action(field_type, ident.span()),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Explicit(method) => method.name.span(),
            Self::Implicit(ident) => ident.span(),
        }
    }
}

fn default_action(field_type: &Type, span: Span) -> Method {
    let ty = Ty::from_syn_ty(field_type);
    let args = match *ty {
        Ty::Vec | Ty::OptionVec => {
            quote_spanned! { span=>
                clap::ArgAction::Append
            }
        }
        Ty::Option | Ty::OptionOption => {
            quote_spanned! { span=>
                clap::ArgAction::Set
            }
        }
        _ => {
            if is_simple_ty(field_type, "bool") {
                quote_spanned! { span=>
                    clap::ArgAction::SetTrue
                }
            } else {
                quote_spanned! { span=>
                    clap::ArgAction::Set
                }
            }
        }
    };

    let func = Ident::new("action", span);
    Method::new(func, args)
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Kind {
    Arg(Sp<Ty>),
    FromGlobal(Sp<Ty>),
    Subcommand(Sp<Ty>),
    Flatten,
    Skip(Option<Expr>),
    ExternalSubcommand,
}

#[derive(Clone)]
pub struct Method {
    name: Ident,
    args: TokenStream,
}

impl Method {
    pub fn new(name: Ident, args: TokenStream) -> Self {
        Method { name, args }
    }

    fn from_env(ident: Ident, env_var: &str) -> Option<Self> {
        let mut lit = match env::var(env_var) {
            Ok(val) => {
                if val.is_empty() {
                    return None;
                }
                LitStr::new(&val, ident.span())
            }
            Err(_) => {
                abort!(ident,
                    "cannot derive `{}` from Cargo.toml", ident;
                    note = "`{}` environment variable is not set", env_var;
                    help = "use `{} = \"...\"` to set {} manually", ident, ident;
                );
            }
        };

        if ident == "author" {
            let edited = process_author_str(&lit.value());
            lit = LitStr::new(&edited, lit.span());
        }

        Some(Method::new(ident, quote!(#lit)))
    }

    pub(crate) fn args(&self) -> &TokenStream {
        &self.args
    }
}

impl ToTokens for Method {
    fn to_tokens(&self, ts: &mut proc_macro2::TokenStream) {
        let Method { ref name, ref args } = self;

        let tokens = quote!( .#name(#args) );

        tokens.to_tokens(ts);
    }
}

/// replace all `:` with `, ` when not inside the `<>`
///
/// `"author1:author2:author3" => "author1, author2, author3"`
/// `"author1 <http://website1.com>:author2" => "author1 <http://website1.com>, author2"
fn process_author_str(author: &str) -> String {
    let mut res = String::with_capacity(author.len());
    let mut inside_angle_braces = 0usize;

    for ch in author.chars() {
        if inside_angle_braces > 0 && ch == '>' {
            inside_angle_braces -= 1;
            res.push(ch);
        } else if ch == '<' {
            inside_angle_braces += 1;
            res.push(ch);
        } else if inside_angle_braces == 0 && ch == ':' {
            res.push_str(", ");
        } else {
            res.push(ch);
        }
    }

    res
}

#[derive(Clone)]
pub struct Parser {
    pub kind: Sp<ParserKind>,
    pub func: TokenStream,
}

impl Parser {
    fn from_type(field_type: &Type, span: Span) -> Sp<Self> {
        if is_simple_ty(field_type, "bool") {
            let kind = Sp::new(ParserKind::FromFlag, span);
            let func = quote_spanned!(span=> ::std::convert::From::from);
            Sp::new(Parser { kind, func }, span)
        } else {
            let kind = Sp::new(ParserKind::TryFromStr, span);
            let func = quote_spanned!(span=> ::std::str::FromStr::from_str);
            Sp::new(Parser { kind, func }, span)
        }
    }

    fn from_spec(parse_ident: Ident, spec: ParserSpec) -> Sp<Self> {
        use self::ParserKind::*;

        let kind = match &*spec.kind.to_string() {
            "from_str" => FromStr,
            "try_from_str" => TryFromStr,
            "from_os_str" => FromOsStr,
            "try_from_os_str" => TryFromOsStr,
            "from_occurrences" => FromOccurrences,
            "from_flag" => FromFlag,
            s => abort!(spec.kind.span(), "unsupported parser `{}`", s),
        };

        let func = match spec.parse_func {
            None => match kind {
                FromStr | FromOsStr => {
                    quote_spanned!(spec.kind.span()=> ::std::convert::From::from)
                }
                TryFromStr => quote_spanned!(spec.kind.span()=> ::std::str::FromStr::from_str),
                TryFromOsStr => abort!(
                    spec.kind.span(),
                    "you must set parser for `try_from_os_str` explicitly"
                ),
                FromOccurrences => quote_spanned!(spec.kind.span()=> { |v| v as _ }),
                FromFlag => quote_spanned!(spec.kind.span()=> ::std::convert::From::from),
            },

            Some(func) => match func {
                Expr::Path(_) => quote!(#func),
                _ => abort!(func, "`parse` argument must be a function path"),
            },
        };

        let kind = Sp::new(kind, spec.kind.span());
        let parser = Parser { kind, func };
        Sp::new(parser, parse_ident.span())
    }

    fn value_parser(&self) -> Method {
        let func = Ident::new("value_parser", self.kind.span());
        match *self.kind {
            ParserKind::FromStr | ParserKind::TryFromStr => Method::new(
                func,
                quote_spanned! { self.kind.span()=>
                clap::builder::ValueParser::string()},
            ),
            ParserKind::FromOsStr | ParserKind::TryFromOsStr => Method::new(
                func,
                quote_spanned! { self.kind.span()=> clap::builder::ValueParser::os_string()},
            ),
            ParserKind::FromOccurrences => Method::new(
                func,
                quote_spanned! { self.kind.span()=> clap::value_parser!(u64)},
            ),
            ParserKind::FromFlag => Method::new(
                func,
                quote_spanned! { self.kind.span()=> clap::builder::ValueParser::bool()},
            ),
        }
    }

    fn action(&self) -> Method {
        let func = Ident::new("action", self.kind.span());
        match *self.kind {
            ParserKind::FromStr
            | ParserKind::TryFromStr
            | ParserKind::FromOsStr
            | ParserKind::TryFromOsStr => Method::new(
                func,
                quote_spanned! { self.kind.span()=> clap::ArgAction::StoreValue},
            ),
            ParserKind::FromOccurrences | ParserKind::FromFlag => Method::new(
                func,
                quote_spanned! { self.kind.span()=> clap::ArgAction::IncOccurrence},
            ),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParserKind {
    FromStr,
    TryFromStr,
    FromOsStr,
    TryFromOsStr,
    FromOccurrences,
    FromFlag,
}

/// Defines the casing for the attributes long representation.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CasingStyle {
    /// Indicate word boundaries with uppercase letter, excluding the first word.
    Camel,
    /// Keep all letters lowercase and indicate word boundaries with hyphens.
    Kebab,
    /// Indicate word boundaries with uppercase letter, including the first word.
    Pascal,
    /// Keep all letters uppercase and indicate word boundaries with underscores.
    ScreamingSnake,
    /// Keep all letters lowercase and indicate word boundaries with underscores.
    Snake,
    /// Keep all letters lowercase and remove word boundaries.
    Lower,
    /// Keep all letters uppercase and remove word boundaries.
    Upper,
    /// Use the original attribute name defined in the code.
    Verbatim,
}

impl CasingStyle {
    fn from_lit(name: LitStr) -> Sp<Self> {
        use self::CasingStyle::*;

        let normalized = name.value().to_upper_camel_case().to_lowercase();
        let cs = |kind| Sp::new(kind, name.span());

        match normalized.as_ref() {
            "camel" | "camelcase" => cs(Camel),
            "kebab" | "kebabcase" => cs(Kebab),
            "pascal" | "pascalcase" => cs(Pascal),
            "screamingsnake" | "screamingsnakecase" => cs(ScreamingSnake),
            "snake" | "snakecase" => cs(Snake),
            "lower" | "lowercase" => cs(Lower),
            "upper" | "uppercase" => cs(Upper),
            "verbatim" | "verbatimcase" => cs(Verbatim),
            s => abort!(name, "unsupported casing: `{}`", s),
        }
    }
}

#[derive(Clone)]
pub enum Name {
    Derived(Ident),
    Assigned(TokenStream),
}

impl Name {
    #[cfg(feature = "unstable-v4")]
    pub fn raw(self) -> TokenStream {
        match self {
            Name::Assigned(tokens) => tokens,
            Name::Derived(ident) => {
                let s = ident.unraw().to_string();
                quote_spanned!(ident.span()=> #s)
            }
        }
    }

    pub fn translate(self, style: CasingStyle) -> TokenStream {
        use CasingStyle::*;

        match self {
            Name::Assigned(tokens) => tokens,
            Name::Derived(ident) => {
                let s = ident.unraw().to_string();
                let s = match style {
                    Pascal => s.to_upper_camel_case(),
                    Kebab => s.to_kebab_case(),
                    Camel => s.to_lower_camel_case(),
                    ScreamingSnake => s.to_shouty_snake_case(),
                    Snake => s.to_snake_case(),
                    Lower => s.to_snake_case().replace('_', ""),
                    Upper => s.to_shouty_snake_case().replace('_', ""),
                    Verbatim => s,
                };
                quote_spanned!(ident.span()=> #s)
            }
        }
    }

    pub fn translate_char(self, style: CasingStyle) -> TokenStream {
        use CasingStyle::*;

        match self {
            Name::Assigned(tokens) => quote!( (#tokens).chars().next().unwrap() ),
            Name::Derived(ident) => {
                let s = ident.unraw().to_string();
                let s = match style {
                    Pascal => s.to_upper_camel_case(),
                    Kebab => s.to_kebab_case(),
                    Camel => s.to_lower_camel_case(),
                    ScreamingSnake => s.to_shouty_snake_case(),
                    Snake => s.to_snake_case(),
                    Lower => s.to_snake_case(),
                    Upper => s.to_shouty_snake_case(),
                    Verbatim => s,
                };

                let s = s.chars().next().unwrap();
                quote_spanned!(ident.span()=> #s)
            }
        }
    }
}
