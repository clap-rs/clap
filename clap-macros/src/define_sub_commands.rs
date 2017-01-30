use syn;
use quote;

pub fn expand_subcommands(cmds: &[(&syn::Ident, &syn::Ty)]) -> quote::Tokens {
    let types = cmds.iter().map(|&(_ident, ty)| ty);
    quote! { vec![ #(<#types as ::clap::code_gen::App>::app()),* ] }
}

pub fn expand(ast: &syn::MacroInput) -> quote::Tokens {
    let ident = &ast.ident;

    let cmds: Vec<_> = match ast.body {
        syn::Body::Enum(ref variants) => {
            variants.iter()
                .map(|variant| match variant.data {
                    syn::VariantData::Tuple(ref fields) => {
                        if fields.len() == 1 {
                            (&variant.ident, &fields[0].ty)
                        } else {
                            panic!("#[derive(DefineSubCommands)] does not support enum variants with multiple fields")
                        }
                    }
                    syn::VariantData::Struct(_) => {
                        panic!("#[derive(DefineSubCommands)] does not support struct enum variants")
                    }
                    syn::VariantData::Unit => {
                        panic!("#[derive(DefineSubCommands)] does not support unit enum variants")
                    }
                })
                .collect()
        }
        syn::Body::Struct(_) => {
            panic!("#[derive(DefineSubCommands)] is not supported on structs")
        }
    };

    let subcommands = expand_subcommands(&cmds);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::clap::code_gen::SubCommands for #ident #ty_generics #where_clause {
            fn subcommands() -> ::std::vec::Vec<::clap::App<'static, 'static>> {
                #subcommands
            }
        }
    }
}
