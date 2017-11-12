/*!
Derives for clap-rs.
*/
#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate error_chain;

use proc_macro::TokenStream;
use syn::DeriveInput;
use quote::Tokens;
use errors::*;

mod arg_enum;
use arg_enum::ArgEnum;
mod helpers;
mod errors;

trait ClapDerive {
    /// Generate the output from a given input.
    fn generate_from(ast: &DeriveInput) -> Result<Tokens>;

    /// Wraps around `generate_from` and does some pre/post processing.
    fn derive(input: TokenStream) -> Result<TokenStream> {
        let derive_input = Self::parse_input(input)?;
        let generated_output = Self::generate_from(&derive_input)?;
        let stream = generated_output.parse()
            .map_err(|e| ErrorKind::ProcLexError(e))?;
        Ok(stream)
    }
    /// Parses the inputted stream.
    fn parse_input(input: TokenStream) -> Result<DeriveInput> {
        // Construct a string representation of the type definition
        let as_string = input.to_string();
        // Parse the string representation
        let parsed = syn::parse_derive_input(&as_string)
            .map_err(|e| ErrorKind::ParseError(e))?;
        Ok(parsed)
    }
}

/// It is required to have this seperate and specificly defined.
#[proc_macro_derive(ArgEnum)]
pub fn derive_arg_enum(input: TokenStream) -> TokenStream {
    ArgEnum::derive(input).unwrap()
}