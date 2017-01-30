use syn;
use quote;

use attrs::FieldAttributes;
use field::{ Arg, Field, Subcommand };

fn expand_parse_arg(arg: &Arg, matches: &syn::Ident) -> quote::Tokens {
    let ident = arg.ident;
    let name = arg.name;
    let value = if arg.is_counter {
        quote! { #matches.occurrences_of(#name) }
    } else {
        if arg.takes_value {
            if arg.multiple {
                quote! {
                    #matches
                        .values_of(#name)
                        .map(|vs| vs.map(|v| v.parse().unwrap()).collect())
                        .unwrap_or_else(|| Vec::new())
                }
            } else {
                if arg.is_optional {
                    quote! {
                        #matches
                            .value_of(#name)
                            .map(|a| a.parse().unwrap())
                    }
                } else {
                    quote! {
                        #matches
                            .value_of(#name).unwrap()
                            .parse().unwrap()
                    }
                }
            }
        } else {
            quote! { #matches.is_present(#name) }
        }
    };

    quote! {
        #ident: #value
    }
}

fn expand_parse_subcommand(cmd: &Subcommand, matches: &syn::Ident) -> quote::Tokens {
    let ident = cmd.ident;
    let ty = cmd.ty;

    let (default, wrapper);
    if cmd.is_optional {
        default = quote! { None };
        wrapper = Some(quote! { Some });
    } else {
        default = quote! { unreachable!() };
        wrapper = None;
    }

    quote! {
        #ident: match #matches.subcommand() {
            (name, Some(matches)) => #wrapper(<#ty as ::clap::stomp::SubCommandsFromArgMatches>::from(name, matches)),
            (_, None) => #default,
        }
    }
}

fn expand_parse_field(field: &Field, matches: &syn::Ident) -> quote::Tokens {
    match *field {
        Field::Arg(ref arg) => expand_parse_arg(arg, matches),
        Field::Subcommand(ref cmd) => expand_parse_subcommand(cmd, matches),
    }
}

fn expand_parse(ast: &syn::MacroInput, fields: &[Field], matches: &syn::Ident) -> quote::Tokens {
    let name = &ast.ident;
    let fields = fields.iter().map(|field| expand_parse_field(field, matches));
    quote! {
        #name {
            #( #fields ),*
        }
    }
}

pub fn expand(ast: &syn::MacroInput, field_attrs: &FieldAttributes) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Unit) => {
            Vec::new()
        }
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter()
                .map(|field| Field::from((field, field_attrs.get(field))))
                .collect()
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("#[derive(FromArgMatches)] is not supported on tuple structs")
        }
        syn::Body::Enum(_) => {
            panic!("#[derive(FromArgMatches)] is not supported on enums")
        }
    };

    let ident = &ast.ident;
    let matches = "matches".into(): syn::Ident;
    let parse = expand_parse(ast, &fields, &matches);
    let allow_unused = syn::Attribute {
        style: syn::AttrStyle::Outer,
        value: syn::MetaItem::List(syn::Ident::from("allow"), vec![
            syn::NestedMetaItem::MetaItem(
                syn::MetaItem::Word(syn::Ident::from("unused_variables"))
            ),
        ]),
        is_sugared_doc: false,
    };
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::clap::stomp::FromArgMatches for #ident #ty_generics #where_clause {
            #allow_unused
            fn from(#matches: &::clap::ArgMatches) -> Self {
                #parse
            }
        }
    }
}
