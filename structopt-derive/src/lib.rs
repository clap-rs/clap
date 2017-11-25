// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! ## How to `derive(StructOpt)`
//!
//! First, let's look at an example:
//!
//! ```ignore
//! #[derive(StructOpt)]
//! #[structopt(name = "example", about = "An example of StructOpt usage.")]
//! struct Opt {
//!     #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
//!     debug: bool,
//!     #[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
//!     speed: f64,
//!     #[structopt(help = "Input file")]
//!     input: String,
//!     #[structopt(help = "Output file, stdout if not present")]
//!     output: Option<String>,
//! }
//! ```
//!
//! So `derive(StructOpt)` tells Rust to generate a command line parser,
//! and the various `structopt` attributes are simply
//! used for additional parameters.
//!
//! First, define a struct, whatever its name.  This structure will
//! correspond to a `clap::App`.  Every method of `clap::App` in the
//! form of `fn function_name(self, &str)` can be use through attributes
//! placed on the struct. In our example above, the `about` attribute
//! will become an `.about("An example of StructOpt usage.")` call on the
//! generated `clap::App`. There are a few attributes that will default
//! if not specified:
//!
//!   - `name`: The binary name displayed in help messages. Defaults
//!      to the crate name given by Cargo.
//!   - `version`: Defaults to the crate version given by Cargo.
//!   - `author`: Defaults to the crate author name given by Cargo.
//!   - `about`: Defaults to the crate description given by Cargo.
//!
//! Then, each field of the struct not marked as a subcommand corresponds
//! to a `clap::Arg`. As with the struct attributes, every method of
//! `clap::Arg` in the form of `fn function_name(self, &str)` can be used
//! through specifying it as an attribute.
//! The `name` attribute can be used to customize the
//! `Arg::with_name()` call (defaults to the field name).
//! For functions that do not take a `&str` as argument, the attribute can be
//! called `function_name_raw`, e. g. `aliases_raw = "&[\"alias\"]"`.
//!
//! The type of the field gives the kind of argument:
//!
//! Type                 | Effect                               | Added method call to `clap::Arg`
//! ---------------------|--------------------------------------|--------------------------------------
//! `bool`               | `true` if present                    | `.takes_value(false).multiple(false)`
//! `u64`                | number of times the argument is used | `.takes_value(false).multiple(true)`
//! `Option<T: FromStr>` | optional argument                    | `.takes_value(true).multiple(false)`
//! `Vec<T: FromStr>`    | list of arguments                    | `.takes_value(true).multiple(true)`
//! `T: FromStr`         | required argument                    | `.takes_value(true).multiple(false).required(!has_default)`
//!
//! The `FromStr` trait is used to convert the argument to the given
//! type, and the `Arg::validator` method is set to a method using
//! `to_string()` (`FromStr::Err` must implement `std::fmt::Display`).
//! If you would like to use a custom string parser other than `FromStr`, see
//! the [same titled section](#custom-string-parsers) below.
//!
//! Thus, the `speed` argument is generated as:
//!
//! ```ignore
//! clap::Arg::with_name("speed")
//!     .takes_value(true)
//!     .multiple(false)
//!     .required(false)
//!     .validator(parse_validator::<f64>)
//!     .short("s")
//!     .long("speed")
//!     .help("Set speed")
//!     .default_value("42")
//! ```
//!
//! ## Help messages
//!
//! Help messages for the whole binary or individual arguments can be
//! specified using the `about` attribute on the struct/field, as we've
//! already seen. For convenience, they can also be specified using
//! doc comments. For example:
//!
//! ```ignore
//! #[derive(StructOpt)]
//! #[structopt(name = "foo")]
//! /// The help message that will be displayed when passing `--help`.
//! struct Foo {
//!   ...
//!   #[structopt(short = "b")]
//!   /// The description for the arg that will be displayed when passing `--help`.
//!   bar: String
//!   ...
//! }
//! ```
//!
//! ## Subcommands
//!
//! Some applications, especially large ones, split their functionality
//! through the use of "subcommands". Each of these act somewhat like a separate
//! command, but is part of the larger group.
//! One example is `git`, which has subcommands such as `add`, `commit`,
//! and `clone`, to mention just a few.
//!
//! `clap` has this functionality, and `structopt` supports it through enums:
//!
//! ```ignore
//! #[derive(StructOpt)]
//! #[structopt(name = "git", about = "the stupid content tracker")]
//! enum Git {
//!     #[structopt(name = "add")]
//!     Add {
//!         #[structopt(short = "i")]
//!         interactive: bool,
//!         #[structopt(short = "p")]
//!         patch: bool,
//!         files: Vec<String>
//!     },
//!     #[structopt(name = "fetch")]
//!     Fetch {
//!         #[structopt(long = "dry-run")]
//!         dry_run: bool,
//!         #[structopt(long = "all")]
//!         all: bool,
//!         repository: Option<String>
//!     },
//!     #[structopt(name = "commit")]
//!     Commit {
//!         #[structopt(short = "m")]
//!         message: Option<String>,
//!         #[structopt(short = "a")]
//!         all: bool
//!     }
//! }
//! ```
//!
//! Using `derive(StructOpt)` on an enum instead of a struct will produce
//! a `clap::App` that only takes subcommands. So `git add`, `git fetch`,
//! and `git commit` would be commands allowed for the above example.
//!
//! `structopt` also provides support for applications where certain flags
//! need to apply to all subcommands, as well as nested subcommands:
//!
//! ```ignore
//! #[derive(StructOpt)]
//! #[structopt(name = "make-cookie")]
//! struct MakeCookie {
//!     #[structopt(name = "supervisor", default_value = "Puck", required = false, long = "supervisor")]
//!     supervising_faerie: String,
//!     #[structopt(name = "tree")]
//!     /// The faerie tree this cookie is being made in.
//!     tree: Option<String>,
//!     #[structopt(subcommand)]  // Note that we mark a field as a subcommand
//!     cmd: Command
//! }
//!
//! #[derive(StructOpt)]
//! enum Command {
//!     #[structopt(name = "pound")]
//!     /// Pound acorns into flour for cookie dough.
//!     Pound {
//!         acorns: u32
//!     },
//!     #[structopt(name = "sparkle")]
//!     /// Add magical sparkles -- the secret ingredient!
//!     Sparkle {
//!         #[structopt(short = "m")]
//!         magicality: u64,
//!         #[structopt(short = "c")]
//!         color: String
//!     },
//!     #[structopt(name = "finish")]
//!     Finish {
//!         #[structopt(short = "t")]
//!         time: u32,
//!         #[structopt(subcommand)]  // Note that we mark a field as a subcommand
//!         type: FinishType
//!     }
//! }
//!
//! #[derive(StructOpt)]
//! enum FinishType {
//!     #[structopt(name = "glaze")]
//!     Glaze {
//!         applications: u32
//!     },
//!     #[structopt(name = "powder")]
//!     Powder {
//!         flavor: String,
//!         dips: u32
//!     }
//! }
//! ```
//!
//! Marking a field with `structopt(subcommand)` will add the subcommands of the
//! designated enum to the current `clap::App`. The designated enum *must* also
//! be derived `StructOpt`. So the above example would take the following
//! commands:
//!
//! + `make-cookie pound 50`
//! + `make-cookie sparkle -mmm --color "green"`
//! + `make-cookie finish 130 glaze 3`
//!
//! ### Optional subcommands
//!
//! A nested subcommand can be marked optional:
//!
//! ```ignore
//! #[derive(StructOpt)]
//! #[structopt(name = "foo")]
//! struct Foo {
//!     file: String,
//!     #[structopt(subcommand)]
//!     cmd: Option<Command>
//! }
//!
//! #[derive(StructOpt)]
//! enum Command {
//!     Bar,
//!     Baz,
//!     Quux
//! }
//! ```
//!
//! ## Custom string parsers
//!
//! If the field type does not have a `FromStr` implementation, or you would
//! like to provide a custom parsing scheme other than `FromStr`, you may
//! provide a custom string parser using `parse(...)` like this:
//!
//! ```ignore
//! use std::num::ParseIntError;
//! use std::path::PathBuf;
//!
//! fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
//!     u32::from_str_radix(src, 16)
//! }
//!
//! #[derive(StructOpt)]
//! struct HexReader {
//!     #[structopt(short = "n", parse(try_from_str = "parse_hex"))]
//!     number: u32,
//!     #[structopt(short = "o", parse(from_os_str))]
//!     output: PathBuf,
//! }
//! ```
//!
//! There are four kinds custom string parsers:
//!
//! | Kind              | Signature                             | Default                         |
//! |-------------------|---------------------------------------|---------------------------------|
//! | `from_str`        | `fn(&str) -> T`                       | `::std::convert::From::from`    |
//! | `try_from_str`    | `fn(&str) -> Result<T, E>`            | `::std::str::FromStr::from_str` |
//! | `from_os_str`     | `fn(&OsStr) -> T`                     | `::std::convert::From::from`    |
//! | `try_from_os_str` | `fn(&OsStr) -> Result<T, OsString>`   | (no default function)           |
//!
//! When supplying a custom string parser, `bool` and `u64` will not be treated
//! specially:
//!
//! Type        | Effect            | Added method call to `clap::Arg`
//! ------------|-------------------|--------------------------------------
//! `Option<T>` | optional argument | `.takes_value(true).multiple(false)`
//! `Vec<T>`    | list of arguments | `.takes_value(true).multiple(true)`
//! `T`         | required argument | `.takes_value(true).multiple(false).required(!has_default)`
//!
//! In the `try_from_*` variants, the function will run twice on valid input:
//! once to validate, and once to parse. Hence, make sure the function is
//! side-effect-free.


extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

/// Generates the `StructOpt` impl.
#[proc_macro_derive(StructOpt, attributes(structopt))]
pub fn structopt(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_structopt(&ast);
    gen.parse().unwrap()
}

#[derive(Copy, Clone, PartialEq)]
enum Ty {
    Bool,
    U64,
    Vec,
    Option,
    Other,
}

fn ty(t: &syn::Ty) -> Ty {
    if let syn::Ty::Path(None, syn::Path { segments: ref segs, .. }) = *t {
        match segs.last().unwrap().ident.as_ref() {
            "bool" => Ty::Bool,
            "u64" => Ty::U64,
            "Option" => Ty::Option,
            "Vec" => Ty::Vec,
            _ => Ty::Other,
        }
    } else {
        Ty::Other
    }
}

fn sub_type(t: &syn::Ty) -> Option<&syn::Ty> {
    let segs = match *t {
        syn::Ty::Path(None, syn::Path { ref segments, .. }) => segments,
        _ => return None,
    };
    match *segs.last().unwrap() {
        PathSegment {
            parameters: PathParameters::AngleBracketed(
                AngleBracketedParameterData { ref types, .. }),
            ..
        } if !types.is_empty() => Some(&types[0]),
            _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
enum AttrSource { Struct, Field, }

#[derive(Debug)]
enum Parser {
    /// Parse an option to using a `fn(&str) -> T` function. The function should never fail.
    FromStr,
    /// Parse an option to using a `fn(&str) -> Result<T, E>` function. The error will be
    /// converted to a string using `.to_string()`.
    TryFromStr,
    /// Parse an option to using a `fn(&OsStr) -> T` function. The function should never fail.
    FromOsStr,
    /// Parse an option to using a `fn(&OsStr) -> Result<T, OsString>` function.
    TryFromOsStr,
}

fn extract_attrs<'a>(attrs: &'a [Attribute], attr_source: AttrSource) -> Box<Iterator<Item = (Ident, Lit)> + 'a> {
    let settings_attrs = attrs.iter()
        .filter_map(|attr| match attr.value {
            MetaItem::List(ref i, ref v) if i.as_ref() == "structopt" => Some(v),
            _ => None,
        }).flat_map(|v| v.iter().filter_map(|mi| match *mi {
            NestedMetaItem::MetaItem(MetaItem::NameValue(ref i, ref l)) =>
                Some((i.clone(), l.clone())),
            _ => None,
        }));

    let doc_comments: Vec<String> = attrs.iter()
        .filter_map(move |attr| {
            if let Attribute {
                value: MetaItem::NameValue(ref name, Lit::Str(ref value, StrStyle::Cooked)),
                is_sugared_doc: true,
                ..
            } = *attr {
                if name != "doc" { return None; }
                let text = value.trim_left_matches("//!")
                    .trim_left_matches("///")
                    .trim_left_matches("/*!")
                    .trim_left_matches("/**")
                    .trim();
                Some(text.into())
            } else {
                None
            }
        })
        .collect();

    let doc_comments = if doc_comments.is_empty() {
        None
    } else {
        // Clap's `App` has an `about` method to set a description,
        // it's `Field`s have a `help` method instead.
        if let AttrSource::Struct = attr_source {
            Some(("about".into(), doc_comments.join(" ").into()))
        } else {
            Some(("help".into(), doc_comments.join(" ").into()))
        }
    };

    Box::new(doc_comments.into_iter().chain(settings_attrs))
}

fn from_attr_or_env(attrs: &[(Ident, Lit)], key: &str, env: &str) -> String {
    let default = std::env::var(env).unwrap_or("".into());
    attrs.iter()
        .filter(|&&(ref i, _)| i.as_ref() == key)
        .last()
        .and_then(|&(_, ref l)| match *l {
            Lit::Str(ref s, _) => Some(s.clone()),
            _ => None
        })
        .unwrap_or(default)
}

fn is_subcommand(field: &Field) -> bool {
    field.attrs.iter()
        .map(|attr| &attr.value)
        .any(|meta| if let MetaItem::List(ref i, ref l) = *meta {
            if i != "structopt" { return false; }
            match l.first() {
                Some(&NestedMetaItem::MetaItem(MetaItem::Word(ref inner))) => inner == "subcommand",
                _ => false
            }
        } else {
          false
        })
}

fn get_default_parser() -> (Parser, quote::Tokens) {
    (Parser::TryFromStr, quote!(::std::str::FromStr::from_str))
}

fn get_parser(field: &Field) -> Option<(Parser, quote::Tokens)> {
    field.attrs.iter()
        .flat_map(|attr| {
            if let MetaItem::List(ref i, ref l) = attr.value {
                if i == "structopt" {
                    return &**l;
                }
            }
            &[]
        })
        .filter_map(|attr| {
            if let NestedMetaItem::MetaItem(MetaItem::List(ref i, ref l)) = *attr {
                if i == "parse" {
                    return l.first();
                }
            }
            None
        })
        .map(|attr| {
            match *attr {
                NestedMetaItem::MetaItem(MetaItem::NameValue(ref i, Lit::Str(ref v, _))) => {
                    let function = parse_path(v).expect("parser function path");
                    let parser = if i == "from_str" {
                        Parser::FromStr
                    } else if i == "try_from_str" {
                        Parser::TryFromStr
                    } else if i == "from_os_str" {
                        Parser::FromOsStr
                    } else if i == "try_from_os_str" {
                        Parser::TryFromOsStr
                    } else {
                        panic!("unsupported parser {}", i);
                    };
                    (parser, quote!(#function))
                }
                NestedMetaItem::MetaItem(MetaItem::Word(ref i)) => {
                    if i == "from_str" {
                        (Parser::FromStr, quote!(::std::convert::From::from))
                    } else if i == "try_from_str" {
                        (Parser::TryFromStr, quote!(::std::str::FromStr::from_str))
                    } else if i == "from_os_str" {
                        (Parser::FromOsStr, quote!(::std::convert::From::from))
                    } else if i == "try_from_os_str" {
                        panic!("cannot omit parser function name with `try_from_os_str`")
                    } else {
                        panic!("unsupported parser {}", i);
                    }
                }
                _ => panic!("unknown value parser specification"),
            }
        })
        .next()
}

fn convert_with_custom_parse(cur_type: Ty) -> Ty {
    match cur_type {
        Ty::Bool | Ty::U64 => Ty::Other,
        rest => rest,
    }
}

/// Generate a block of code to add arguments/subcommands corresponding to
/// the `fields` to an app.
fn gen_augmentation(fields: &[Field], app_var: &Ident) -> quote::Tokens {
    let subcmds: Vec<quote::Tokens> = fields.iter()
        .filter(|&field| is_subcommand(field))
        .map(|field| {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };
            let required = if cur_type == Ty::Option {
                quote!()
            } else {
                quote!( let #app_var = #app_var.setting(_structopt::clap::AppSettings::SubcommandRequiredElseHelp); )
            };

            quote!{
                let #app_var = #subcmd_type ::augment_clap( #app_var );
                #required
            }
        })
        .collect();

    assert!(subcmds.len() <= 1, "cannot have more than one nested subcommand");

    let args = fields.iter()
        .filter(|&field| !is_subcommand(field))
        .map(|field| {
            let name = gen_name(field);
            let mut cur_type = ty(&field.ty);
            let convert_type = match cur_type {
                Ty::Vec | Ty::Option => sub_type(&field.ty).unwrap_or(&field.ty),
                _ => &field.ty,
            };

            let parser = get_parser(field);
            if parser.is_some() {
                cur_type = convert_with_custom_parse(cur_type);
            }
            let validator = match parser.unwrap_or_else(get_default_parser) {
                (Parser::TryFromStr, f) => quote! {
                    .validator(|s| {
                        #f(&s)
                            .map(|_: #convert_type| ())
                            .map_err(|e| e.to_string())
                    })
                },
                (Parser::TryFromOsStr, f) => quote! {
                    .validator_os(|s| #f(&s).map(|_: #convert_type| ()))
                },
                _ => quote! {},
            };

            let modifier = match cur_type {
                Ty::Bool => quote!( .takes_value(false).multiple(false) ),
                Ty::U64 => quote!( .takes_value(false).multiple(true) ),
                Ty::Option => quote!( .takes_value(true).multiple(false) #validator ),
                Ty::Vec => quote!( .takes_value(true).multiple(true) #validator ),
                Ty::Other => {
                    let required = extract_attrs(&field.attrs, AttrSource::Field)
                        .find(|&(ref i, _)| i.as_ref() == "default_value"
                              || i.as_ref() == "default_value_raw")
                        .is_none();
                    quote!( .takes_value(true).multiple(false).required(#required) #validator )
                },
            };
            let from_attr = extract_attrs(&field.attrs, AttrSource::Field)
                .filter(|&(ref i, _)| i.as_ref() != "name")
                .map(|(i, l)| gen_attr_call(&i, &l));
            quote!( .arg(_structopt::clap::Arg::with_name(stringify!(#name)) #modifier #(#from_attr)*) )
        });

    quote! {{
        use std::error::Error;
        let #app_var = #app_var #( #args )* ;
        #( #subcmds )*
        #app_var
    }}
}

/// Interpret the value of `*_raw` attributes as code and the rest as strings.
fn gen_attr_call(key: &syn::Ident, val: &syn::Lit) -> quote::Tokens {
    if let Lit::Str(ref val, _) = *val {
        let key = key.as_ref();
        if key.ends_with("_raw") {
            let key = Ident::from(&key[..(key.len() - 4)]);
            // Call method without quoting the string
            let ts = syn::parse_token_trees(val)
                .expect(&format!("bad parameter {} = {}: the parameter must be valid rust code", key, val));
            return quote!(.#key(#(#ts)*));
        }
    }
    quote!(.#key(#val))
}

fn gen_constructor(fields: &[Field]) -> quote::Tokens {
    let fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let name = gen_name(field);
        if is_subcommand(field) {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };
            let unwrapper = match cur_type {
                Ty::Option => quote!(),
                _ => quote!( .unwrap() )
            };
            quote!( #field_name: #subcmd_type ::from_subcommand(matches.subcommand()) #unwrapper )
        } else {
            let mut cur_type = ty(&field.ty);
            let parser = get_parser(field);
            if parser.is_some() {
                cur_type = convert_with_custom_parse(cur_type);
            }

            let (value_of, values_of, parse) = match parser.unwrap_or_else(get_default_parser) {
                (Parser::FromStr, f) => (
                    quote!(value_of),
                    quote!(values_of),
                    f,
                ),
                (Parser::TryFromStr, f) => (
                    quote!(value_of),
                    quote!(values_of),
                    quote!(|s| #f(s).unwrap()),
                ),
                (Parser::FromOsStr, f) => (
                    quote!(value_of_os),
                    quote!(values_of_os),
                    f,
                ),
                (Parser::TryFromOsStr, f) => (
                    quote!(value_of_os),
                    quote!(values_of_os),
                    quote!(|s| #f(s).unwrap()),
                ),
            };

            let convert = match cur_type {
                Ty::Bool => quote!(is_present(stringify!(#name))),
                Ty::U64 => quote!(occurrences_of(stringify!(#name))),
                Ty::Option => quote! {
                    #value_of(stringify!(#name))
                        .as_ref()
                        .map(#parse)
                },
                Ty::Vec => quote! {
                    #values_of(stringify!(#name))
                        .map(|v| v.map(#parse).collect())
                        .unwrap_or_else(Vec::new)
                },
                Ty::Other => quote! {
                    #value_of(stringify!(#name))
                        .map(#parse)
                        .unwrap()
                },
            };
            quote!( #field_name: matches.#convert )
        }
    });

    quote! {{
        #( #fields ),*
    }}
}

fn gen_name(field: &Field) -> Ident {
    extract_attrs(&field.attrs, AttrSource::Field)
        .filter(|&(ref i, _)| i.as_ref() == "name")
        .last()
        .and_then(|(_, ref l)| match l {
            &Lit::Str(ref s, _) => Some(Ident::new(s.clone())),
            _ => None,
        })
        .unwrap_or(field.ident.as_ref().unwrap().clone())
}

fn gen_from_clap(struct_name: &Ident, fields: &[Field]) -> quote::Tokens {
    let field_block = gen_constructor(fields);

    quote! {
        fn from_clap(matches: _structopt::clap::ArgMatches) -> Self {
            #struct_name #field_block
        }
    }
}

fn format_author(raw_authors: String) -> String {
    raw_authors.replace(":", ", ")
}

fn method_if_arg(method: &str, arg: &str) -> Option<quote::Tokens> {
    if arg.is_empty() {
        None
    } else {
        let method: Ident = method.into();
        Some(quote!(.#method(#arg)))
    }
}

fn gen_clap(attrs: &[Attribute]) -> quote::Tokens {
    let attrs: Vec<_> = extract_attrs(attrs, AttrSource::Struct).collect();
    let name: Lit = from_attr_or_env(&attrs, "name", "CARGO_PKG_NAME").into();
    let version = from_attr_or_env(&attrs, "version", "CARGO_PKG_VERSION");
    let version = method_if_arg("version", &version);
    let author = format_author(from_attr_or_env(&attrs, "author", "CARGO_PKG_AUTHORS"));
    let author = method_if_arg("author", &author);
    let about = from_attr_or_env(&attrs, "about", "CARGO_PKG_DESCRIPTION");
    let about = method_if_arg("about", &about);
    let settings = attrs.iter()
        .filter(|&&(ref i, _)| !["name", "version", "author", "about"].contains(&i.as_ref()))
        .map(|&(ref i, ref l)| gen_attr_call(i, l))
        .collect::<Vec<_>>();

    quote! {
        _structopt::clap::App::new(#name)
            #version
            #author
            #about
            #( #settings )*
    }
}

fn gen_clap_struct(struct_attrs: &[Attribute]) -> quote::Tokens {
    let gen = gen_clap(struct_attrs);

    quote! {
        fn clap<'a, 'b>() -> _structopt::clap::App<'a, 'b> {
            let app = #gen;
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap(fields: &[Field]) -> quote::Tokens {
    let app_var = Ident::new("app");
    let augmentation = gen_augmentation(fields, &app_var);
    quote! {
        pub fn augment_clap<'a, 'b>(#app_var: _structopt::clap::App<'a, 'b>) -> _structopt::clap::App<'a, 'b> {
            #augmentation
        }
    }
}

fn gen_clap_enum(enum_attrs: &[Attribute]) -> quote::Tokens {
    let gen = gen_clap(enum_attrs);
    quote! {
        fn clap<'a, 'b>() -> _structopt::clap::App<'a, 'b> {
            let app = #gen
                .setting(_structopt::clap::AppSettings::SubcommandRequiredElseHelp);
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap_enum(variants: &[Variant]) -> quote::Tokens {
    let subcommands = variants.iter().map(|variant| {
        let name = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter_map(|attr| match attr {
                (ref i, Lit::Str(ref s, ..)) if i == "name" =>
                    Some(s.to_string()),
                _ => None
            })
            .next()
            .unwrap_or_else(|| variant.ident.to_string());
        let app_var = Ident::new("subcommand");
        let arg_block = match variant.data {
            VariantData::Struct(ref fields) => gen_augmentation(fields, &app_var),
            VariantData::Unit => quote!( #app_var ),
            _ => unreachable!()
        };
        let from_attr = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter(|&(ref i, _)| i != "name")
            .map(|(i, l)| gen_attr_call(&i, &l));

        quote! {
            .subcommand({
                let #app_var = _structopt::clap::SubCommand::with_name( #name )
                    #( #from_attr )* ;
                #arg_block
            })
        }
    });

    quote! {
        pub fn augment_clap<'a, 'b>(app: _structopt::clap::App<'a, 'b>) -> _structopt::clap::App<'a, 'b> {
            app #( #subcommands )*
        }
    }
}

fn gen_from_clap_enum(name: &Ident) -> quote::Tokens {
    quote! {
        #[doc(hidden)]
        fn from_clap(matches: _structopt::clap::ArgMatches) -> Self {
            #name ::from_subcommand(matches.subcommand())
                .unwrap()
        }
    }
}

fn gen_from_subcommand(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let match_arms = variants.iter().map(|variant| {
        let sub_name = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter_map(|attr| match attr {
                (ref i, Lit::Str(ref s, ..)) if i == "name" =>
                    Some(s.to_string()),
                _ => None
            })
            .next()
            .unwrap_or_else(|| variant.ident.as_ref().to_string());
        let variant_name = &variant.ident;
        let constructor_block = match variant.data {
            VariantData::Struct(ref fields) => gen_constructor(fields),
            VariantData::Unit => quote!(),  // empty
            _ => unreachable!()
        };

        quote! {
            (#sub_name, Some(matches)) =>
                Some(#name :: #variant_name #constructor_block)
        }
    });

    quote! {
        #[doc(hidden)]
        pub fn from_subcommand<'a, 'b>(sub: (&'b str, Option<&'b _structopt::clap::ArgMatches<'a>>)) -> Option<Self> {
            match sub {
                #( #match_arms ),*,
                _ => None
            }
        }
    }
}

fn impl_structopt_for_struct(name: &Ident, fields: &[Field], attrs: &[Attribute]) -> quote::Tokens {
    let clap = gen_clap_struct(attrs);
    let augment_clap = gen_augment_clap(fields);
    let from_clap = gen_from_clap(name, fields);

    quote! {
        impl _structopt::StructOpt for #name {
            #clap
            #from_clap
        }

        impl #name {
            #augment_clap
        }
    }
}

fn impl_structopt_for_enum(name: &Ident, variants: &[Variant], attrs: &[Attribute]) -> quote::Tokens {
    if variants.iter().any(|variant| {
            if let VariantData::Tuple(..) = variant.data { true } else { false }
        })
    {
        panic!("enum variants cannot be tuples");
    }

    let clap = gen_clap_enum(attrs);
    let augment_clap = gen_augment_clap_enum(variants);
    let from_clap = gen_from_clap_enum(name);
    let from_subcommand = gen_from_subcommand(name, variants);

    quote! {
        impl _structopt::StructOpt for #name {
            #clap
            #from_clap
        }

        impl #name {
            #augment_clap
            #from_subcommand
        }
    }
}

fn impl_structopt(ast: &DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let inner_impl = match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) =>
            impl_structopt_for_struct(struct_name, fields, &ast.attrs),
        Body::Enum(ref variants) =>
            impl_structopt_for_enum(struct_name, variants, &ast.attrs),
        _ => panic!("structopt only supports non-tuple structs and enums")
    };

    let dummy_const = Ident::new(format!("_IMPL_STRUCTOPT_FOR_{}", struct_name));
    quote! {
        #[allow(non_upper_case_globals)]
        #[allow(unused_attributes, unused_imports, unused_variables)]
        const #dummy_const: () = {
            extern crate structopt as _structopt;
            use structopt::StructOpt;
            #inner_impl
        };
    }
}
