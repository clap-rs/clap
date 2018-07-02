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
use std::{env, mem};
use syn::Type::Path;
use syn::{self, Attribute, Ident, LitStr, MetaList, MetaNameValue, TypePath};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    Arg(Ty),
    Subcommand(Ty),
    FlattenStruct,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Ty {
    Bool,
    Vec,
    Option,
    Other,
}
#[derive(Debug)]
pub struct Attrs {
    name: String,
    methods: Vec<Method>,
    parser: (Parser, TokenStream),
    has_custom_parser: bool,
    kind: Kind,
}
#[derive(Debug)]
struct Method {
    name: String,
    args: TokenStream,
}
#[derive(Debug, PartialEq)]
pub enum Parser {
    FromStr,
    TryFromStr,
    FromOsStr,
    TryFromOsStr,
    FromOccurrences,
}
impl ::std::str::FromStr for Parser {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "from_str" => Ok(Parser::FromStr),
            "try_from_str" => Ok(Parser::TryFromStr),
            "from_os_str" => Ok(Parser::FromOsStr),
            "try_from_os_str" => Ok(Parser::TryFromOsStr),
            "from_occurrences" => Ok(Parser::FromOccurrences),
            _ => Err(format!("unsupported parser {}", s)),
        }
    }
}

impl Attrs {
    fn new(name: String) -> Attrs {
        Attrs {
            name: name,
            methods: vec![],
            parser: (Parser::TryFromStr, quote!(::std::str::FromStr::from_str)),
            has_custom_parser: false,
            kind: Kind::Arg(Ty::Other),
        }
    }
    fn push_str_method(&mut self, name: &str, arg: &str) {
        match (name, arg) {
            ("about", "") | ("version", "") | ("author", "") => {
                let methods = mem::replace(&mut self.methods, vec![]);
                self.methods = methods.into_iter().filter(|m| m.name != name).collect();
            }
            ("name", new_name) => self.name = new_name.into(),
            (name, arg) => self.methods.push(Method {
                name: name.to_string(),
                args: quote!(#arg),
            }),
        }
    }
    fn push_attrs(&mut self, attrs: &[Attribute]) {
        use Lit::*;
        use Meta::*;
        use NestedMeta::*;

        let iter = attrs
            .iter()
            .filter_map(|attr| {
                let path = &attr.path;
                match quote!(#path).to_string() == "structopt" {
                    true => Some(
                        attr.interpret_meta()
                            .expect(&format!("invalid structopt syntax: {}", quote!(attr))),
                    ),
                    false => None,
                }
            })
            .flat_map(|m| match m {
                List(l) => l.nested,
                tokens => panic!("unsupported syntax: {}", quote!(#tokens).to_string()),
            })
            .map(|m| match m {
                Meta(m) => m,
                ref tokens => panic!("unsupported syntax: {}", quote!(#tokens).to_string()),
            });
        for attr in iter {
            match attr {
                NameValue(MetaNameValue {
                    ident,
                    lit: Str(value),
                    ..
                }) => self.push_str_method(&ident.to_string(), &value.value()),
                NameValue(MetaNameValue { ident, lit, .. }) => self.methods.push(Method {
                    name: ident.to_string(),
                    args: quote!(#lit),
                }),
                List(MetaList {
                    ref ident,
                    ref nested,
                    ..
                }) if ident == "parse" =>
                {
                    if nested.len() != 1 {
                        panic!("parse must have exactly one argument");
                    }
                    self.has_custom_parser = true;
                    self.parser = match nested[0] {
                        Meta(NameValue(MetaNameValue {
                            ref ident,
                            lit: Str(ref v),
                            ..
                        })) => {
                            let function: syn::Path = v.parse().expect("parser function path");
                            let parser = ident.to_string().parse().unwrap();
                            (parser, quote!(#function))
                        }
                        Meta(Word(ref i)) => {
                            use Parser::*;
                            let parser = i.to_string().parse().unwrap();
                            let function = match parser {
                                FromStr => quote!(::std::convert::From::from),
                                TryFromStr => quote!(::std::str::FromStr::from_str),
                                FromOsStr => quote!(::std::convert::From::from),
                                TryFromOsStr => panic!(
                                    "cannot omit parser function name with `try_from_os_str`"
                                ),
                                FromOccurrences => quote!({ |v| v as _ }),
                            };
                            (parser, function)
                        }
                        ref l @ _ => panic!("unknown value parser specification: {}", quote!(#l)),
                    };
                }
                List(MetaList {
                    ref ident,
                    ref nested,
                    ..
                }) if ident == "raw" =>
                {
                    for method in nested {
                        match *method {
                            Meta(NameValue(MetaNameValue {
                                ref ident,
                                lit: Str(ref v),
                                ..
                            })) => self.push_raw_method(&ident.to_string(), v),
                            ref mi @ _ => panic!("unsupported raw entry: {}", quote!(#mi)),
                        }
                    }
                }
                Word(ref w) if w == "subcommand" => {
                    self.set_kind(Kind::Subcommand(Ty::Other));
                }
                Word(ref w) if w == "flatten" => {
                    self.set_kind(Kind::FlattenStruct);
                }
                ref i @ List(..) | ref i @ Word(..) => panic!("unsupported option: {}", quote!(#i)),
            }
        }
    }
    fn push_raw_method(&mut self, name: &str, args: &LitStr) {
        let ts: TokenStream = args.value().parse().expect(&format!(
            "bad parameter {} = {}: the parameter must be valid rust code",
            name,
            quote!(#args)
        ));
        self.methods.push(Method {
            name: name.to_string(),
            args: quote!(#(#ts)*),
        })
    }
    fn push_doc_comment(&mut self, attrs: &[Attribute], name: &str) {
        let doc_comments: Vec<_> = attrs
            .iter()
            .filter_map(|attr| {
                let path = &attr.path;
                match quote!(#path).to_string() == "doc" {
                    true => attr.interpret_meta(),
                    false => None,
                }
            })
            .filter_map(|attr| {
                use Lit::*;
                use Meta::*;
                if let NameValue(MetaNameValue {
                    ident, lit: Str(s), ..
                }) = attr
                {
                    if ident != "doc" {
                        return None;
                    }
                    let value = s.value();
                    let text = value
                        .trim_left_matches("//!")
                        .trim_left_matches("///")
                        .trim_left_matches("/*!")
                        .trim_left_matches("/**")
                        .trim_right_matches("*/")
                        .trim();
                    if text.is_empty() {
                        Some("\n\n".to_string())
                    } else {
                        Some(text.to_string())
                    }
                } else {
                    None
                }
            })
            .collect();
        if doc_comments.is_empty() {
            return;
        }
        let arg = doc_comments
            .join(" ")
            .split('\n')
            .map(|l| l.trim().to_string())
            .collect::<Vec<_>>()
            .join("\n");
        self.methods.push(Method {
            name: name.to_string(),
            args: quote!(#arg),
        });
    }
    pub fn from_struct(attrs: &[Attribute], name: String) -> Attrs {
        let mut res = Self::new(name);
        let attrs_with_env = [
            ("version", "CARGO_PKG_VERSION"),
            ("about", "CARGO_PKG_DESCRIPTION"),
            ("author", "CARGO_PKG_AUTHORS"),
        ];
        attrs_with_env
            .iter()
            .filter_map(|&(m, v)| env::var(v).ok().and_then(|arg| Some((m, arg))))
            .filter(|&(_, ref arg)| !arg.is_empty())
            .for_each(|(name, arg)| {
                let new_arg = if name == "author" {
                    arg.replace(":", ", ")
                } else {
                    arg
                };
                res.push_str_method(name, &new_arg);
            });
        res.push_doc_comment(attrs, "about");
        res.push_attrs(attrs);
        if res.has_custom_parser {
            panic!("parse attribute is only allowed on fields");
        }
        match res.kind {
            Kind::Subcommand(_) => panic!("subcommand is only allowed on fields"),
            Kind::FlattenStruct => panic!("flatten is only allowed on fields"),
            Kind::Arg(_) => res,
        }
    }
    fn ty_from_field(ty: &syn::Type) -> Ty {
        if let Path(TypePath {
            path: syn::Path { ref segments, .. },
            ..
        }) = *ty
        {
            match segments.iter().last().unwrap().ident.to_string().as_str() {
                "bool" => Ty::Bool,
                "Option" => Ty::Option,
                "Vec" => Ty::Vec,
                _ => Ty::Other,
            }
        } else {
            Ty::Other
        }
    }
    pub fn from_field(field: &syn::Field) -> Attrs {
        let name = field.ident.as_ref().unwrap().to_string();
        let mut res = Self::new(name);
        res.push_doc_comment(&field.attrs, "help");
        res.push_attrs(&field.attrs);

        match res.kind {
            Kind::FlattenStruct => {
                if res.has_custom_parser {
                    panic!("parse attribute is not allowed for flattened entry");
                }
                if !res.methods.is_empty() {
                    panic!("methods and doc comments are not allowed for flattened entry");
                }
            }
            Kind::Subcommand(_) => {
                if res.has_custom_parser {
                    panic!("parse attribute is not allowed for subcommand");
                }
                if !res.methods.iter().all(|m| m.name == "help") {
                    panic!("methods in attributes is not allowed for subcommand");
                }
                res.kind = Kind::Subcommand(Self::ty_from_field(&field.ty));
            }
            Kind::Arg(_) => {
                let mut ty = Self::ty_from_field(&field.ty);
                if res.has_custom_parser {
                    match ty {
                        Ty::Option | Ty::Vec => (),
                        _ => ty = Ty::Other,
                    }
                }
                match ty {
                    Ty::Bool => {
                        if res.has_method("default_value") {
                            panic!("default_value is meaningless for bool")
                        }
                        if res.has_method("required") {
                            panic!("required is meaningless for bool")
                        }
                    }
                    Ty::Option => {
                        if res.has_method("default_value") {
                            panic!("default_value is meaningless for Option")
                        }
                        if res.has_method("required") {
                            panic!("required is meaningless for Option")
                        }
                    }
                    _ => (),
                }
                res.kind = Kind::Arg(ty);
            }
        }

        res
    }
    fn set_kind(&mut self, kind: Kind) {
        if let Kind::Arg(_) = self.kind {
            self.kind = kind;
        } else {
            panic!("subcommands cannot be flattened");
        }
    }
    pub fn has_method(&self, method: &str) -> bool {
        self.methods.iter().find(|m| m.name == method).is_some()
    }
    pub fn methods(&self) -> TokenStream {
        let methods = self.methods.iter().map(|&Method { ref name, ref args }| {
            let name = Ident::new(&name, Span::call_site());
            quote!( .#name(#args) )
        });
        quote!( #(#methods)* )
    }
    pub fn name(&self) -> &str { &self.name }
    pub fn parser(&self) -> &(Parser, TokenStream) { &self.parser }
    pub fn kind(&self) -> Kind { self.kind }
}
