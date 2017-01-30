#![feature(type_ascription)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod attr;
mod attrs;
mod field;
mod define_app;
mod from_arg_matches;
mod define_sub_commands;
mod sub_command_from_arg_matches;

#[proc_macro_derive(DefineApp, attributes(clap))]
pub fn define_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let (attrs, field_attrs) = attrs::extract_attrs(&ast);
    let expanded = define_app::expand(&ast, &attrs, &field_attrs);
    attrs.check_used(ast.ident.as_ref(), None);
    field_attrs.check_used(ast.ident.as_ref());
    expanded.parse().unwrap()
}

#[proc_macro_derive(FromArgMatches, attributes(clap))]
pub fn from_arg_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let (_, field_attrs) = attrs::extract_attrs(&ast);
    let expanded = from_arg_matches::expand(&ast, &field_attrs);
    field_attrs.check_used(ast.ident.as_ref());
    expanded.parse().unwrap()
}

#[proc_macro_derive(DefineSubCommands, attributes(clap))]
pub fn define_sub_commands(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = define_sub_commands::expand(&ast);
    expanded.parse().unwrap()
}

#[proc_macro_derive(SubCommandFromArgMatches, attributes(clap))]
pub fn sub_command_from_arg_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = sub_command_from_arg_matches::expand(&ast);
    expanded.parse().unwrap()
}
