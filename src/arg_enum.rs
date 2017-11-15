use syn::DeriveInput;
use quote::Tokens;
use ClapDerive;
use helpers;
use errors::*;

pub struct ArgEnum;

impl ClapDerive for ArgEnum {
    fn generate_from(ast: &DeriveInput) -> Result<Tokens> {
        let from_str_block = impl_from_str(ast)?;
        let variants_block = impl_variants(ast)?;
        
        Ok(quote! {
            #from_str_block
            #variants_block
        })
    }
}

fn impl_from_str(ast: &DeriveInput) -> Result<Tokens> {
    let ident = &ast.ident;
    let is_case_sensitive = ast.attrs.iter().any(|v| v.name() == "case_sensitive");
    let variants = helpers::variants(ast)?;

    let strings = variants.iter()
        .map(|ref variant| String::from(variant.ident.as_ref()))
        .collect::<Vec<_>>();
    
    // All of these need to be iterators.
    let ident_slice = [ident.clone()];
    let idents = ident_slice.iter().cycle();

    let for_error_message = strings.clone();

    let condition_function_slice = [match is_case_sensitive {
        true => quote! { str::eq },
        false => quote! { ::std::ascii::AsciiExt::eq_ignore_ascii_case },
    }];
    let condition_function = condition_function_slice.iter().cycle();

    Ok(quote! {
        impl ::std::str::FromStr for #ident {
            type Err = String;

            fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
                match input {
                    #(val if #condition_function(val, #strings) => Ok(#idents::#variants),)*
                    _ => Err({
                        let v = #for_error_message;
                        format!("valid values: {}",
                            v.join(" ,"))
                    }),
                }
            }
        }
    })
}

fn impl_variants(ast: &DeriveInput) -> Result<Tokens> {
    let ident = &ast.ident;
    let variants = helpers::variants(ast)?
        .iter()
        .map(|ref variant| String::from(variant.ident.as_ref()))
        .collect::<Vec<_>>();
    let length = variants.len();

    Ok(quote! {
        impl #ident {
            fn variants() -> [&'static str; #length] {
                #variants
            }
        }
    })
}