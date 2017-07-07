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
//!
//! The type of the field gives the kind of argument:
//!
//! Type                 | Effect            | Added method call to `clap::Arg`
//! ---------------------|-------------------|--------------------------------------
//! `bool`               | `true` if present | `.takes_value(false).multiple(false)`
//! `u64`                | number of params  | `.takes_value(false).multiple(true)`
//! `Option<T: FromStr>` | optional argument | `.takes_value(true).multiple(false)`
//! `Vec<T: FromStr>`    | list of arguments | `.takes_value(true).multiple(true)`
//! `T: FromStr`         | required argument | `.takes_value(true).multiple(false).required(!has_default)`
//!
//! The `FromStr` trait is used to convert the argument to the given
//! type, and the `Arg::validator` method is set to a method using
//! `FromStr::Error::description()`.
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
//!     .long("debug")
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
//! Some applications, like `git`, support "subcommands;" an extra command that
//! is used to differentiate what the application should do. With `git`, these
//! would be `add`, `init`, `fetch`, `commit`, for a few examples.
//!
//! `clap` has this functionality, so `structopt` supports this through enums:
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

#[derive(Copy, Clone)]
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

    let doc_comments = attrs.iter()
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

                // Clap's `App` has an `about` method to set a description,
                // it's `Field`s have a `help` method instead.
                if let AttrSource::Struct = attr_source {
                    Some(("about".into(), text.into()))
                } else {
                    Some(("help".into(), text.into()))
                }
            } else {
                None
            }
        });

    Box::new(doc_comments.chain(settings_attrs))
}

fn from_attr_or_env(attrs: &[(Ident, Lit)], key: &str, env: &str) -> Lit {
    let default = std::env::var(env).unwrap_or("".into());
    attrs.iter()
        .filter(|&&(ref i, _)| i.as_ref() == key)
        .last()
        .map(|&(_, ref l)| l.clone())
        .unwrap_or_else(|| Lit::Str(default, StrStyle::Cooked))
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

            quote!( let #app_var = #subcmd_type ::augment_clap( #app_var ); )
        })
        .collect();
    let args = fields.iter()
        .filter(|&field| !is_subcommand(field))
        .map(|field| {
            let name = gen_name(field);
            let cur_type = ty(&field.ty);
            let convert_type = match cur_type {
                Ty::Vec | Ty::Option => sub_type(&field.ty).unwrap_or(&field.ty),
                _ => &field.ty,
            };
            let validator = quote! {
                validator(|s| s.parse::<#convert_type>()
                          .map(|_| ())
                          .map_err(|e| e.description().into()))
            };
            let modifier = match cur_type {
                Ty::Bool => quote!( .takes_value(false).multiple(false) ),
                Ty::U64 => quote!( .takes_value(false).multiple(true) ),
                Ty::Option => quote!( .takes_value(true).multiple(false).#validator ),
                Ty::Vec => quote!( .takes_value(true).multiple(true).#validator ),
                Ty::Other => {
                    let required = extract_attrs(&field.attrs, AttrSource::Field)
                        .find(|&(ref i, _)| i.as_ref() == "default_value")
                        .is_none();
                    quote!( .takes_value(true).multiple(false).required(#required).#validator )
                },
            };
            let from_attr = extract_attrs(&field.attrs, AttrSource::Field)
                .filter(|&(ref i, _)| i.as_ref() != "name")
                .map(|(i, l)| quote!(.#i(#l)));
            quote!( .arg(_structopt::clap::Arg::with_name(stringify!(#name)) #modifier #(#from_attr)*) )
        });

    assert!(subcmds.len() <= 1, "cannot have more than one nested subcommand");

    quote! {{
        use std::error::Error;
        let #app_var = #app_var #( #args )* ;
        #( #subcmds )*
        #app_var
    }}
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
            let convert = match ty(&field.ty) {
                Ty::Bool => quote!(is_present(stringify!(#name))),
                Ty::U64 => quote!(occurrences_of(stringify!(#name))),
                Ty::Option => quote! {
                    value_of(stringify!(#name))
                        .as_ref()
                        .map(|s| s.parse().unwrap())
                },
                Ty::Vec => quote! {
                    values_of(stringify!(#name))
                        .map(|v| v.map(|s| s.parse().unwrap()).collect())
                        .unwrap_or_else(Vec::new)
                },
                Ty::Other => quote! {
                    value_of(stringify!(#name))
                        .as_ref()
                        .unwrap()
                        .parse()
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

fn gen_clap(struct_attrs: &[Attribute], subcmd_required: bool) -> quote::Tokens {
    let struct_attrs: Vec<_> = extract_attrs(struct_attrs, AttrSource::Struct).collect();
    let name = from_attr_or_env(&struct_attrs, "name", "CARGO_PKG_NAME");
    let version = from_attr_or_env(&struct_attrs, "version", "CARGO_PKG_VERSION");
    let author = from_attr_or_env(&struct_attrs, "author", "CARGO_PKG_AUTHORS");
    let about = from_attr_or_env(&struct_attrs, "about", "CARGO_PKG_DESCRIPTION");
    let setting = if subcmd_required {
        quote!( .setting(_structopt::clap::AppSettings::SubcommandRequired) )
    } else {
        quote!()
    };

    quote! {
        fn clap<'a, 'b>() -> _structopt::clap::App<'a, 'b> {
            let app = _structopt::clap::App::new(#name)
                .version(#version)
                .author(#author)
                .about(#about)
                #setting
                ;
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap(fields: &[Field]) -> quote::Tokens {
    let app_var = Ident::new("app");
    let augmentation = gen_augmentation(fields, &app_var);
    quote! {
        fn augment_clap<'a, 'b>(#app_var: _structopt::clap::App<'a, 'b>) -> _structopt::clap::App<'a, 'b> {
            #augmentation
        }
    }
}

fn gen_clap_enum(enum_attrs: &[Attribute]) -> quote::Tokens {
    let enum_attrs: Vec<_> = extract_attrs(enum_attrs, AttrSource::Struct).collect();
    let name = from_attr_or_env(&enum_attrs, "name", "CARGO_PKG_NAME");
    let version = from_attr_or_env(&enum_attrs, "version", "CARGO_PKG_VERSION");
    let author = from_attr_or_env(&enum_attrs, "author", "CARGO_PKG_AUTHORS");
    let about = from_attr_or_env(&enum_attrs, "about", "CARGO_PKG_DESCRIPTION");

    quote! {
        fn clap<'a, 'b>() -> _structopt::clap::App<'a, 'b> {
            let app = _structopt::clap::App::new(#name)
                .version(#version)
                .author(#author)
                .about(#about)
                .setting(_structopt::clap::AppSettings::SubcommandRequired);
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
            .map(|(i, l)| quote!( .#i(#l) ));

        quote! {
            .subcommand({
                let #app_var = _structopt::clap::SubCommand::with_name( #name )
                    #( #from_attr )* ;
                #arg_block
            })
        }
    });
   
    quote! {
        fn augment_clap<'a, 'b>(app: _structopt::clap::App<'a, 'b>) -> _structopt::clap::App<'a, 'b> {
            app #( #subcommands )*
        }
    }
}

fn gen_from_clap_enum(name: &Ident) -> quote::Tokens {
    quote! {
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
        fn from_subcommand<'a, 'b>(sub: (&'b str, Option<&'b _structopt::clap::ArgMatches<'a>>)) -> Option<Self> {
            match sub {
                #( #match_arms ),*,
                _ => None
            }
        }
    }
}

fn impl_structopt_for_struct(name: &Ident, fields: &[Field], attrs: &[Attribute]) -> quote::Tokens {
    let subcmd_required = fields.iter().any(|field| {
        let cur_type = ty(&field.ty);
        match cur_type {
            Ty::Option => false,
            _ => is_subcommand(field)
        }
    });
    let clap = gen_clap(attrs, subcmd_required);
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
