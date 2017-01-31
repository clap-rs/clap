use syn;
use quote;

pub struct Attribute {
    key: String,
    values: Vec<syn::Lit>,
}

impl Attribute {
    pub fn new(key: String) -> Attribute {
        Attribute {
            key: key,
            values: vec![],
        }
    }

    pub fn push(&mut self, value: syn::Lit) { self.values.push(value) }

    pub fn values(&self) -> Vec<String> {
        self.values
            .iter()
            .map(|s| match *s {
                syn::Lit::Str(ref s, _) => s.clone(),
                _ => panic!("clap-macros: multi-valued attributes must be strings"),
            })
            .collect()
    }

    fn only_value(&self) -> &syn::Lit {
        if self.values.len() == 1 {
            &self.values[0]
        } else {
            panic!("clap-macros: expected a single value for attribute '{}' but had multiple",
                   self.key);
        }
    }
}

impl<'a> Into<syn::Lit> for &'a Attribute {
    fn into(self) -> syn::Lit { self.only_value().clone() }
}

impl<'a> Into<&'a syn::Lit> for &'a Attribute {
    fn into(self) -> &'a syn::Lit { self.only_value() }
}

impl<'a> Into<&'a str> for &'a Attribute {
    fn into(self) -> &'a str {
        if let &syn::Lit::Str(ref value, _) = self.only_value() {
            value
        } else {
            panic!("Expected string value for attribute {} but got a {:?}",
                   self.key,
                   self.only_value());
        }
    }
}

impl<'a> Into<&'a [u8]> for &'a Attribute {
    fn into(self) -> &'a [u8] {
        if let &syn::Lit::ByteStr(ref value, _) = self.only_value() {
            value
        } else if let &syn::Lit::Str(ref value, _) = self.only_value() {
            value.as_bytes()
        } else {
            panic!("Expected bytestring value for attribute {} but got a {:?}",
                   self.key,
                   self.only_value());
        }
    }
}

impl<'a> Into<u8> for &'a Attribute {
    fn into(self) -> u8 {
        if let &syn::Lit::Byte(ref value) = self.only_value() {
            *value
        } else if let &syn::Lit::Str(ref value, _) = self.only_value() {
            value.parse().unwrap()
        } else {
            panic!("Expected byte value for attribute {} but got a {:?}",
                   self.key,
                   self.only_value());
        }
    }
}

impl<'a> Into<char> for &'a Attribute {
    fn into(self) -> char {
        if let &syn::Lit::Char(ref value) = self.only_value() {
            *value
        } else if let &syn::Lit::Str(ref value, _) = self.only_value() {
            assert!(value.len() == 1);
            value.chars().next().unwrap()
        } else {
            panic!("Expected char value for attribute {} but got a {:?}",
                   self.key,
                   self.only_value());
        }
    }
}

impl<'a> Into<u64> for &'a Attribute {
    fn into(self) -> u64 {
        if let &syn::Lit::Int(ref value, _) = self.only_value() {
            *value
        } else if let &syn::Lit::Str(ref value, _) = self.only_value() {
            value.parse().unwrap()
        } else {
            panic!("Expected int value for attribute {} but got a {:?}",
                   self.key,
                   self.only_value());
        }
    }
}

impl<'a> Into<bool> for &'a Attribute {
    fn into(self) -> bool {
        if let &syn::Lit::Bool(ref value) = self.only_value() {
            *value
        } else if let &syn::Lit::Str(ref value, _) = self.only_value() {
            value.parse().unwrap()
        } else {
            panic!("Expected bool value for attribute {} but got a {:?}",
                   self.key,
                   self.only_value());
        }
    }
}

impl<'a> quote::ToTokens for &'a mut Attribute {
    fn to_tokens(&self, tokens: &mut quote::Tokens) { self.only_value().to_tokens(tokens) }
}

impl quote::ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut quote::Tokens) { self.only_value().to_tokens(tokens) }
}
