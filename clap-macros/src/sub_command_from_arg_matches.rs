use syn;
use quote;

fn expand_parse(me: &syn::Ident, cmds: &[(&syn::Ident, &syn::Ty)], name: &syn::Ident, matches: &syn::Ident) -> quote::Tokens {
    let variants = cmds.iter().map(|&(ident, ty)| {
        let name = ident.as_ref().to_lowercase();
        quote! { #name => #me::#ident(<#ty as ::clap::stomp::FromArgMatches>::from(#matches)) }
    });
    quote! {
        match #name {
            #(#variants,)*
            _ => unreachable!(),
        }
    }
}

pub fn expand(ast: &syn::MacroInput) -> quote::Tokens {
    let ident = &ast.ident;
    let name = "name".into(): syn::Ident;
    let matches = "matches".into(): syn::Ident;

    let cmds: Vec<_> = match ast.body {
        syn::Body::Enum(ref variants) => {
            variants.iter()
                .map(|variant| match variant.data {
                    syn::VariantData::Tuple(ref fields) => {
                        if fields.len() == 1 {
                            (&variant.ident, &fields[0].ty)
                        } else {
                            panic!("#[derive(SubCommandFromArgMatches)] does not support enum variants with multiple fields")
                        }
                    }
                    syn::VariantData::Struct(_) => {
                        panic!("#[derive(SubCommandFromArgMatches)] does not support struct enum variants")
                    }
                    syn::VariantData::Unit => {
                        panic!("#[derive(SubCommandFromArgMatches)] does not support unit enum variants")
                    }
                })
                .collect()
        }
        syn::Body::Struct(_) => {
            panic!("#[derive(SubCommandFromArgMatches)] is not supported on structs")
        }
    };

    let from = expand_parse(ident, &cmds, &name, &matches);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::clap::stomp::SubCommandFromArgMatches for #ident #ty_generics #where_clause {
            fn from(#name: &str, #matches: &::clap::ArgMatches) -> Self {
                #from
            }
        }
    }
}
