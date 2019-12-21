use std::iter::FromIterator;

use proc_macro2::TokenStream;
use proc_macro_error::{abort, ResultExt};
use syn::{
    self, parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    parse2,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Expr, ExprLit, Ident, Lit, LitBool, LitStr, Token,
};

pub struct ClapAttributes {
    pub paren_token: syn::token::Paren,
    pub attrs: Punctuated<ClapAttr, Token![,]>,
}

impl Parse for ClapAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let attrs = content.parse_terminated(ClapAttr::parse)?;

        Ok(ClapAttributes { paren_token, attrs })
    }
}

#[allow(clippy::large_enum_variant)]
pub enum ClapAttr {
    // single-identifier attributes
    Short(Ident),
    Long(Ident),
    Env(Ident),
    Flatten(Ident),
    Subcommand(Ident),
    NoVersion(Ident),
    VerbatimDocComment(Ident),

    // ident [= "string literal"]
    About(Ident, Option<LitStr>),
    Author(Ident, Option<LitStr>),

    // ident = "string literal"
    Version(Ident, LitStr),
    RenameAllEnv(Ident, LitStr),
    RenameAll(Ident, LitStr),
    NameLitStr(Ident, LitStr),

    // parse(parser_kind [= parser_func])
    Parse(Ident, ParserSpec),

    // ident [= arbitrary_expr]
    Skip(Ident, Option<Expr>),

    // ident = arbitrary_expr
    NameExpr(Ident, Expr),

    // ident(arbitrary_expr,*)
    MethodCall(Ident, Vec<Expr>),
}

impl Parse for ClapAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use self::ClapAttr::*;

        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            let assign_token = input.parse::<Token![=]>()?; // skip '='

            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;
                let lit_str = lit.value();

                let check_empty_lit = |s| {
                    if lit_str.is_empty() {
                        abort!(
                            lit.span(),
                            "`#[clap({} = \"\")` is deprecated, \
                             now it's default behavior",
                            s
                        );
                    }
                };

                match &*name_str {
                    "rename_all" => Ok(RenameAll(name, lit)),
                    "rename_all_env" => Ok(RenameAllEnv(name, lit)),

                    "version" => {
                        check_empty_lit("version");
                        Ok(Version(name, lit))
                    }

                    "author" => {
                        check_empty_lit("author");
                        Ok(Author(name, Some(lit)))
                    }

                    "about" => {
                        check_empty_lit("about");
                        Ok(About(name, Some(lit)))
                    }

                    "skip" => {
                        let expr = ExprLit {
                            attrs: vec![],
                            lit: Lit::Str(lit),
                        };
                        let expr = Expr::Lit(expr);
                        Ok(Skip(name, Some(expr)))
                    }

                    _ => Ok(NameLitStr(name, lit)),
                }
            } else {
                match input.parse::<Expr>() {
                    Ok(expr) => {
                        if name_str == "skip" {
                            Ok(Skip(name, Some(expr)))
                        } else {
                            Ok(NameExpr(name, expr))
                        }
                    }

                    Err(_) => abort! {
                        assign_token.span(),
                        "expected `string literal` or `expression` after `=`"
                    },
                }
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            let nested;
            parenthesized!(nested in input);

            match name_str.as_ref() {
                "parse" => {
                    let parser_specs: Punctuated<ParserSpec, Token![,]> =
                        nested.parse_terminated(ParserSpec::parse)?;

                    if parser_specs.len() == 1 {
                        Ok(Parse(name, parser_specs[0].clone()))
                    } else {
                        abort!(name.span(), "parse must have exactly one argument")
                    }
                }

                "raw" => match nested.parse::<LitBool>() {
                    Ok(bool_token) => {
                        let expr = ExprLit {
                            attrs: vec![],
                            lit: Lit::Bool(bool_token),
                        };
                        let expr = Expr::Lit(expr);
                        Ok(MethodCall(name, vec![expr]))
                    }

                    Err(_) => {
                        abort!(name.span(),
                            "`#[clap(raw(...))` attributes are removed, \
                            they are replaced with raw methods";
                            help = "if you meant to call `clap::Arg::raw()` method \
                                you should use bool literal, like `raw(true)` or `raw(false)`";
                            note = raw_method_suggestion(nested);
                        );
                    }
                },

                _ => {
                    let method_args: Punctuated<_, Token![,]> =
                        nested.parse_terminated(Expr::parse)?;
                    Ok(MethodCall(name, Vec::from_iter(method_args)))
                }
            }
        } else {
            // Attributes represented with a sole identifier.
            match name_str.as_ref() {
                "long" => Ok(Long(name)),
                "short" => Ok(Short(name)),
                "env" => Ok(Env(name)),
                "flatten" => Ok(Flatten(name)),
                "subcommand" => Ok(Subcommand(name)),
                "no_version" => Ok(NoVersion(name)),
                "verbatim_doc_comment" => Ok(VerbatimDocComment(name)),

                "about" => (Ok(About(name, None))),
                "author" => (Ok(Author(name, None))),

                "skip" => Ok(Skip(name, None)),

                "version" => abort!(
                    name.span(),
                    "#[clap(version)] is invalid attribute, \
                     clap_derive inherits version from Cargo.toml by default, \
                     no attribute needed"
                ),

                _ => abort!(name.span(), "unexpected attribute: {}", name_str),
            }
        }
    }
}

#[derive(Clone)]
pub struct ParserSpec {
    pub kind: Ident,
    pub eq_token: Option<Token![=]>,
    pub parse_func: Option<Expr>,
}

impl Parse for ParserSpec {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let kind = input
            .parse()
            .map_err(|_| input.error("parser specification must start with identifier"))?;
        let eq_token = input.parse()?;
        let parse_func = match eq_token {
            None => None,
            Some(_) => Some(input.parse()?),
        };
        Ok(ParserSpec {
            kind,
            eq_token,
            parse_func,
        })
    }
}

fn raw_method_suggestion(ts: ParseBuffer) -> String {
    let do_parse = move || -> Result<(Ident, TokenStream), syn::Error> {
        let name = ts.parse()?;
        let _eq: Token![=] = ts.parse()?;
        let val: LitStr = ts.parse()?;
        Ok((name, syn::parse_str(&val.value())?))
    };
    if let Ok((name, val)) = do_parse() {
        let val = val.to_string().replace(" ", "").replace(",", ", ");
        format!(
            "if you need to call `clap::Arg/App::{}` method you \
             can do it like this: #[clap({}({}))]",
            name, name, val
        )
    } else {
        "if you need to call some method from `clap::Arg/App` \
         you should use raw method, see \
         https://docs.rs/structopt/0.3/structopt/#raw-methods"
            .into()
    }
}

pub fn parse_clap_attributes(all_attrs: &[Attribute]) -> Vec<ClapAttr> {
    all_attrs
        .iter()
        .filter(|attr| attr.path.is_ident("clap"))
        .flat_map(|attr| {
            let attrs: ClapAttributes = parse2(attr.tokens.clone())
                .map_err(|e| match &*e.to_string() {
                    // this error message is misleading and points to Span::call_site()
                    // so we patch it with something meaningful
                    "unexpected end of input, expected parentheses" => {
                        let span = attr.path.span();
                        let patch_msg = "expected parentheses after `clap`";
                        syn::Error::new(span, patch_msg)
                    }
                    _ => e,
                })
                .unwrap_or_abort();
            attrs.attrs
        })
        .collect()
}
